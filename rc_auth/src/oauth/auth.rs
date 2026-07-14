use actix_web::{get, post, web::{Data, Form, Query, Redirect}, HttpRequest};
use serde::{Serialize, Deserialize};

use oj_rc_core::persist::user::FederatedAuthenticator;

const DEFAULT_COOLDOWN_PERIOD: std::time::Duration = std::time::Duration::from_millis(123);

struct RateTracker {
    successes: std::sync::atomic::AtomicU32,
    failures: std::sync::atomic::AtomicU32,
    request_pending: std::sync::atomic::AtomicBool,
    first_seen: i64,
    client_id: String,
}

impl RateTracker {
    fn new(client_id: String) -> Self {
        Self {
            successes: std::sync::atomic::AtomicU32::new(0),
            failures: std::sync::atomic::AtomicU32::new(0),
            request_pending: std::sync::atomic::AtomicBool::new(true),
            first_seen: chrono::Utc::now().timestamp(),
            client_id,
        }
    }

    fn slowdown(&self, client_id: &str) -> std::time::Duration {
        let now = chrono::Utc::now();
        let successes = self.successes.load(std::sync::atomic::Ordering::Relaxed);
        let failures = self.failures.load(std::sync::atomic::Ordering::Relaxed);
        let is_already_pending = self.request_pending.load(std::sync::atomic::Ordering::SeqCst);
        if self.client_id.to_lowercase() != client_id.to_lowercase() {
            return DEFAULT_COOLDOWN_PERIOD.mul_f32(1.95);
        }
        let first_seen = if let Some(t) = chrono::DateTime::from_timestamp_secs(self.first_seen) {
            t
        } else {
            return DEFAULT_COOLDOWN_PERIOD;
        };
        let delta_time_sqrt = (now.signed_duration_since(first_seen).abs().num_seconds() as f64).sqrt();
        let failure_rate = (failures as f64) / (successes as f64 + failures as f64);
        let slowdown_ms = (10_000.0 * failure_rate) / delta_time_sqrt;
        let slowdown_ms_clamped = (slowdown_ms as u64).clamp(2, 246);
        if slowdown_ms_clamped == 246 && is_already_pending {
            DEFAULT_COOLDOWN_PERIOD.mul_f32(5.0)
        } else {
            std::time::Duration::from_millis(slowdown_ms_clamped)
        }
    }

    fn complete_ok(&self) {
        self.successes.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.request_pending.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    fn complete_err(&self) {
        self.failures.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.request_pending.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}

static RATE_LIMITS: std::sync::OnceLock<tokio::sync::RwLock<std::collections::HashMap<u128, RateTracker>>> = std::sync::OnceLock::new();

fn init_rate_limits() -> tokio::sync::RwLock<std::collections::HashMap<u128, RateTracker>> {
    tokio::sync::RwLock::new(std::collections::HashMap::new())
}

fn key_from_address(addr: std::net::SocketAddr) -> u128 {
    let addr_str = addr.ip().to_canonical().to_string();
    let addr_bytes = addr_str.as_bytes();
    let end = addr_bytes.len() - 1;
    u128::from_le_bytes([
        addr_bytes[end.saturating_sub(15)],
        addr_bytes[end.saturating_sub(14)],
        addr_bytes[end.saturating_sub(13)],
        addr_bytes[end.saturating_sub(12)],
        addr_bytes[end.saturating_sub(11)],
        addr_bytes[end.saturating_sub(10)],
        addr_bytes[end.saturating_sub(9)],
        addr_bytes[end.saturating_sub(8)],
        addr_bytes[end.saturating_sub(7)],
        addr_bytes[end.saturating_sub(6)],
        addr_bytes[end.saturating_sub(5)],
        addr_bytes[end.saturating_sub(4)],
        addr_bytes[end.saturating_sub(3)],
        addr_bytes[end.saturating_sub(2)],
        addr_bytes[end.saturating_sub(1)],
        addr_bytes[end],
    ])
}

#[derive(Serialize, Deserialize, Clone)]
struct AuthQuery {
    pub response_type: Option<String>,
    pub client_id: String,
    pub redirect_uri: Option<String>,
    pub scope: String,
    pub state: String,
    pub code_challenge: String,
    pub code_challenge_method: String,
}

#[post("/authenticate/oauth2/auth")]
pub async fn post_oauth_auth(body: Form<oj_rc_core::persist::user::federation::FederatedAuthenticationPayload>, query: Query<AuthQuery>, config: Data<crate::robocraft::RcConfig>, req: HttpRequest) -> impl actix_web::Responder {
    let rate_limits = RATE_LIMITS.get_or_init(init_rate_limits);
    if let Some(peer_addr) = req.peer_addr() {
        let key = key_from_address(peer_addr);
        let read_lock = rate_limits.read().await;
        let cooldown_dur = if let Some(tracker) = read_lock.get(&key) {
            let slowdown = tracker.slowdown(&query.client_id);
            drop(read_lock);
            slowdown
        } else {
            drop(read_lock);
            let mut write_lock = rate_limits.write().await;
            write_lock.insert(key, RateTracker::new(query.client_id.clone()));
            DEFAULT_COOLDOWN_PERIOD.div_f32(1.95)
        };
        if config.rate_limit_fediverse {
            log::debug!("Cooldown for {} is {}us", query.client_id, cooldown_dur.as_micros());
            tokio::time::sleep(cooldown_dur).await;
        } else {
            log::debug!("Cooldown for {} would've been {}us", query.client_id, cooldown_dur.as_micros());
        }
    } else {
        // this is just sketchy but should never occur; always rate limit this
        tokio::time::sleep(DEFAULT_COOLDOWN_PERIOD).await;
    }

    let access_token = match config.account_provider.remote_auth(&body, &query.code_challenge).await {
        Ok(x) => x,
        Err(e) => {
            if let Some(peer_addr) = req.peer_addr() {
                let key = key_from_address(peer_addr);
                rate_limits.read().await.get(&key).unwrap().complete_err();
            }
            log::error!("Failed to OAuth authenticate {} from {}: {}", body.display_name, body.domain_source, e.message);
            return Redirect::to("/")
                .temporary()
        }
    };
    let redirect_root = query.redirect_uri.as_ref().map(|x| x.to_owned()).unwrap_or_else(|| "/authenticate/oauth2/auth".to_owned());
    let redirect_url = format!("{}?code={}&state={}", redirect_root, access_token, query.state);
    #[cfg(debug_assertions)]
    log::debug!("Redirecting to {}", redirect_url);
    if let Some(peer_addr) = req.peer_addr() {
        let key = key_from_address(peer_addr);
        rate_limits.read().await.get(&key).unwrap().complete_ok();
    }
    Redirect::to(redirect_url)
        .temporary()
}

#[get("/authenticate/oauth2/auth")]
pub async fn get_oauth_auth() -> &'static str {
    "This is unimplemented and should not be used for standard OAuth flows"
}
