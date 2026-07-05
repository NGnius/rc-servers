use actix_web::{get, web::{Data, Json}};
use oj_rc_core::persist::user::federation::DiscoveryMetadata;

const OAUTH_AUTH_URL: &str = "authenticate/oauth2/auth";
const OAUTH_JWKS_URL: &str = "authenticate/oauth2/jwks";
const OAUTH_TOKEN_URL: &str = "authenticate/oauth2/token";

#[get("/.well-known/openid-configuration")]
pub async fn get_openid_configuration(server_config: Data<oj_rc_core::persist::config::ServerConfig>) -> Json<DiscoveryMetadata> {
    let fallback_url = "http://127.0.0.1/fallback";
    let auth_url = format!("{}/{}", server_config.auth_url, OAUTH_AUTH_URL);
    let jwks_url = format!("{}/{}", server_config.auth_url, OAUTH_JWKS_URL); // TODO
    let token_url = format!("{}/{}", server_config.auth_url, OAUTH_TOKEN_URL);
    let meta = DiscoveryMetadata::new(
        openidconnect::IssuerUrl::new(server_config.auth_url.clone())
            .unwrap_or_else(|e| {
                log::error!("Failed to parse issuer url {}: {}", server_config.auth_url, e);
                openidconnect::IssuerUrl::new(fallback_url.to_owned()).unwrap()
            }),
        openidconnect::AuthUrl::new(auth_url.clone())
            .unwrap_or_else(|e| {
                log::error!("Failed to parse auth url {}: {}", auth_url, e);
                openidconnect::AuthUrl::new(fallback_url.to_owned()).unwrap()
            }),
        openidconnect::JsonWebKeySetUrl::new(jwks_url.clone())
            .unwrap_or_else(|e| {
                log::error!("Failed to parse jwks url {}: {}", jwks_url, e);
                openidconnect::JsonWebKeySetUrl::new(fallback_url.to_owned()).unwrap()
            }),
        vec![
            openidconnect::ResponseTypes::new(vec![openidconnect::core::CoreResponseType::Code]),
            openidconnect::ResponseTypes::new(vec![openidconnect::core::CoreResponseType::IdToken]),
        ],
        vec![
            openidconnect::core::CoreSubjectIdentifierType::Public,
        ],
        vec![
            //openidconnect::core::CoreJwsSigningAlgorithm::RsaSsaPkcs1V15Sha256,
            openidconnect::core::CoreJwsSigningAlgorithm::HmacSha256,
        ],
        openidconnect::EmptyAdditionalProviderMetadata::default(),
    )
    .set_token_endpoint(Some(openidconnect::TokenUrl::new(token_url)
        .unwrap_or_else(|e| {
            log::error!("Failed to parse token url {}: {}", server_config.auth_url, e);
            openidconnect::TokenUrl::new(fallback_url.to_owned()).unwrap()
        })
    ))
    .set_scopes_supported(Some(vec![
        openidconnect::Scope::new("openid".to_owned()),
        openidconnect::Scope::new("read".to_owned()),
        openidconnect::Scope::new("write".to_owned()),
        openidconnect::Scope::new("federate".to_owned()),
    ]))
    .set_claims_supported(Some(vec![
        openidconnect::core::CoreClaimName::new("aud".to_owned()),
        openidconnect::core::CoreClaimName::new("exp".to_owned()),
        openidconnect::core::CoreClaimName::new("iat".to_owned()),
        openidconnect::core::CoreClaimName::new("iss".to_owned()),
        openidconnect::core::CoreClaimName::new("sub".to_owned()),
        openidconnect::core::CoreClaimName::new("name".to_owned()),
        openidconnect::core::CoreClaimName::new("preferred_username".to_owned()),
    ]));
    Json(meta)
}
