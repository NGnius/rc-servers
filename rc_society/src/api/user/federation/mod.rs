mod user;
pub use user::get;

mod outbox;
pub use outbox::get as outbox_get;

mod inbox;
pub use inbox::get as inbox_get;
pub use inbox::post as inbox_post;

static SOCIETY_URL: std::sync::OnceLock<String> = std::sync::OnceLock::new();
static CDN_URL: std::sync::OnceLock<String> = std::sync::OnceLock::new();

pub(super) fn init(config: &dyn oj_rc_core::ConfigProvider<()>) {
    SOCIETY_URL.get_or_init(|| {
        let urls = config.server_config();
        urls.society_url
    });
    CDN_URL.get_or_init(|| {
        let urls = config.server_config();
        urls.cdn_url
    });
}

pub async fn auth_user(auth: actix_web::web::Data<Box<oj_rc_core::UserImpl>>, token: &actix_web::http::header::HeaderValue) -> Result<Box<dyn oj_rc_core::persist::user::WebUser>, Box<dyn std::error::Error>> {
    use oj_rc_core::UserProvider;
    let token = token.to_str()?;
    if let Some((_, token)) = token.split_once(' ') {
        <oj_rc_core::UserImpl as UserProvider<()>>::web_authenticate(&auth, token.to_owned()).await
            .map_err(|e| Box::from(e.message))
    } else {
        Err("Invalid Authorization header".into())
    }
}

pub fn url(public_id: &str) -> url::Url {
    url::Url::parse(
        &format!("{}/api/v1/activitypub/user/{}", SOCIETY_URL.get().unwrap(), public_id)
    ).unwrap()
}

pub fn inbox(public_id: &str) -> url::Url {
    url::Url::parse(
        &format!("{}/api/v1/activitypub/user/{}/inbox", SOCIETY_URL.get().unwrap(), public_id)
    ).unwrap()
}

pub fn outbox(public_id: &str) -> url::Url {
    url::Url::parse(
        &format!("{}/api/v1/activitypub/user/{}/outbox", SOCIETY_URL.get().unwrap(), public_id)
    ).unwrap()
}

fn avatar_url(public_id: &str) -> url::Url {
    url::Url::parse(
        &format!("{}/customavatar/Live/{}", CDN_URL.get().unwrap(), public_id)
    ).unwrap()
}

async fn build_person_from_user(user: &dyn oj_rc_core::persist::user::WebUser) -> Result<oj_serdes::society::activitypub::Person, Box<dyn std::error::Error>> {
    let pub_id = user.public_id();
    let account_info = user.account_stats().await?;
    let url_id = url(pub_id);
    let mut key_url_id = url_id.clone();
    key_url_id.set_fragment(Some("main-key"));
    Ok(oj_serdes::society::activitypub::Person {
        kind: oj_serdes::society::activitypub::ActivityPubObjectKind::Person,
        id: url_id.to_string(),
        name: user.display_name().to_owned(),
        url: url_id.clone(),
        preferred_username: pub_id.to_owned(),
        icon: if account_info.avatar_id.is_some() { None } else {
            Some(oj_serdes::society::activitypub::Image::new(avatar_url(pub_id)))
        },
        published: chrono::DateTime::from_timestamp(account_info.creation_time, 0).unwrap_or_else(chrono::Utc::now),
        updated: chrono::DateTime::from_timestamp(account_info.last_seen_time, 0),
        inbox: inbox(pub_id),
        outbox: outbox(pub_id),
        public_key: Some(oj_serdes::society::activitypub::PublicKey {
            id: key_url_id.to_string(),
            owner: url_id,
            public_key_pem: String::new(),
        })
    })
}

fn build_person_from_public_id(public_id: &str) -> oj_serdes::society::activitypub::Person {
    let url_id = url(public_id);
    let now = chrono::Utc::now();
    oj_serdes::society::activitypub::Person {
        kind: oj_serdes::society::activitypub::ActivityPubObjectKind::Person,
        id: url_id.to_string(),
        name: public_id.to_owned(),
        url: url_id.clone(),
        preferred_username: public_id.to_owned(),
        icon: Some(oj_serdes::society::activitypub::Image::new(avatar_url(public_id))),
        published: now,
        updated: Some(now),
        inbox: inbox(public_id),
        outbox: outbox(public_id),
        public_key: None,
    }
}
