use openidconnect::{OAuth2TokenResponse, TokenResponse};
use serde::{Serialize, Deserialize};

const SOCIETY_URLS_API_ENDPOINT: &str = "api/v1/services.json";
const ACCESS_CODE_AAD: &[u8] = b"oj-access-code";

pub type DiscoveryMetadata = openidconnect::core::CoreProviderMetadata;
pub type TokenResponsePayload = openidconnect::core::CoreTokenResponse;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Federation {
    pub enabled: bool,
    pub defederated: Vec<String>,
}

impl std::default::Default for Federation {
    fn default() -> Self {
        Self {
            enabled: true,
            defederated: Vec::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FederatedAuthenticationPayload {
    pub display_name: String,
    pub password: String,
    pub domain_source: String,
    pub domain_target: String,
}

#[derive(Deserialize, Serialize)]
struct AccessCode {
    aud: String,
    exp: i64,
    iat: i64,
    iss: String,
    sub: String,
    secured: String,
}

#[derive(Deserialize, Serialize)]
struct SecuredCodes {
    access_token: String,
    refresh_token: String,
    code_challenge: String,
}

struct NonceProvider {
    issuer: std::sync::Arc<String>,
    secret: std::sync::Arc<Vec<u8>>,
    generated_time: i64,
    fuse: bool,
}

impl NonceProvider {
    fn reset(&mut self, iat: i64) {
        self.generated_time = iat;
        self.fuse = false;
    }
}

impl ring::aead::NonceSequence for NonceProvider {
    // realistically this shouldn't ever be called again
    fn advance(&mut self) -> Result<ring::aead::Nonce, ring::error::Unspecified> {
        if self.fuse {
            Err(ring::error::Unspecified)
        } else {
            use sha2::Digest;
            self.fuse = true;
            let hash = sha2::Sha512::new()
                .chain_update(self.issuer.as_bytes())
                .chain_update(self.secret.as_slice())
                .chain_update(self.generated_time.to_ne_bytes())
                .finalize();
            let mut nonce = Vec::from(hash.as_slice());
            nonce.truncate(12);
            Ok(ring::aead::Nonce::try_assume_unique_for_key(&nonce).unwrap())
        }
    }
}

impl super::AccountProvider {
    fn nonce_provider(&self) -> NonceProvider {
        NonceProvider {
            issuer: self.auth.clone(),
            secret: self.secret.clone(),
            generated_time: 0,
            fuse: true,
        }
    }

    fn is_defederated_from(&self, domain: &str, fedi_conf: &crate::persist::config::Federation) -> bool {
        for defederated in fedi_conf.defederated.iter() {
            if domain.ends_with(defederated) {
                return true;
            }
        }
        false
    }

    async fn local_login_impl(&self, auth_info: super::FederatedAuthInfo, federation: &Option<crate::persist::config::Federation>) -> Result<super::UserLoginInfo, super::AuthError> {
        if auth_info.display_name.is_empty() {
            return Err(super::AuthError {
                message: "Refusing federation login with empty username".to_string(),
                code: crate::data::error_codes::AuthErrorCode::InvalidDisplayName,
            });
        }
        if let Some(fedi_conf) = federation {
            if auth_info.domain.is_empty() {
                return Err(super::AuthError {
                    message: format!("Refusing federation login with empty domain for logging in {}", auth_info.display_name),
                    code: crate::data::error_codes::AuthErrorCode::PasswordInvalidated,
                });
            }
            let sanitised_domain = auth_info.domain.trim().to_lowercase();
            let target_domain = if let Some(alias) = fedi_conf.aliases.get(&sanitised_domain) {
                alias.to_owned()
            } else {
                sanitised_domain
            };
            if self.is_defederated_from(&target_domain, fedi_conf) {
                return Err(super::AuthError {
                    message: format!("Refusing federation with {} for logging in {}", auth_info.domain, auth_info.display_name),
                    code: crate::data::error_codes::AuthErrorCode::PasswordInvalidated,
                });
            }
            let is_localhost = target_domain == "localhost"
                || target_domain == "::1"
                || target_domain == "0:0:0:0:0:0:0:1"
                || target_domain.starts_with("127.0.0.")
                || target_domain.starts_with("localhost:");
            // contact other domain to get oauth (auth) and root server
            let social_urls_api = if is_localhost {
                format!("http://{}/{}", target_domain, SOCIETY_URLS_API_ENDPOINT)
            } else {
                format!("https://{}/{}", target_domain, SOCIETY_URLS_API_ENDPOINT)
            };
            let urls: oj_serdes::society::ServiceDomains = self.intercom_http_client.get(&social_urls_api).send().await
                .map_err(|e| {
                    log::error!("Failed to get {}: {}", social_urls_api, e);
                    super::AuthError {
                        message: format!("Failed to get {} for logging in {}", social_urls_api, auth_info.display_name),
                        code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                    }
                })?
                .json().await
                .map_err(|e| {
                    log::error!("Failed to deserialize {}: {}", social_urls_api, e);
                    super::AuthError {
                        message: format!("Failed to deserialize {} for logging in {}", social_urls_api, auth_info.display_name),
                        code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                    }
                })?;
            if self.is_defederated_from(&urls.root, fedi_conf) {
                return Err(super::AuthError {
                    message: format!("Refusing federation with root {} for logging in {}", urls.root, auth_info.display_name),
                    code: crate::data::error_codes::AuthErrorCode::PasswordInvalidated,
                });
            }
            let sani_soc = urls.society.trim_start_matches("http://").trim_start_matches("https://").trim_matches('/').to_lowercase();
            if sani_soc != target_domain {
                return Err(super::AuthError {
                    message: format!("Bad society federation with {} for logging in {}", target_domain, auth_info.display_name),
                    code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                });
            }
            let issuer_url = openidconnect::IssuerUrl::new(urls.auth.clone())
                .map_err(|e| super::AuthError {
                    message: format!("Failed to parse issuer url {} for logging in {}: {}", urls.auth, auth_info.display_name, e),
                    code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                })?;
            // openidconnect relies on an old dependency
            let oauth_http_client = openidconnect::reqwest::ClientBuilder::new()
                // Following redirects opens the client up to SSRF vulnerabilities.
                .redirect(openidconnect::reqwest::redirect::Policy::none())
                .build()
                .expect("Client should build");
            // access openid discovery endpoint to self-configure
            let provider_metadata = DiscoveryMetadata::discover_async(issuer_url, &oauth_http_client).await
                .map_err(|e| super::AuthError {
                    message: format!("Failed to discover OAuth on {} for logging in {}: {}", urls.auth, auth_info.display_name, e),
                    code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                })?;
            let redirect_url_s = format!("http://{}/federation/redirect", self.domain);
            let redirect_url = openidconnect::RedirectUrl::new(redirect_url_s.clone())
                .map_err(|e| super::AuthError {
                    message: format!("Failed to parse redirect url {} for logging in {}: {}", redirect_url_s, auth_info.display_name, e),
                    code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                })?;
            // do oauth exchange
            let oauth_client = openidconnect::core::CoreClient::from_provider_metadata(
                provider_metadata,
                openidconnect::ClientId::new(self.domain.to_string()),
                None, // no client secret
            ).set_redirect_uri(redirect_url);
            let (pkce_challenge, pkce_verifier) = openidconnect::PkceCodeChallenge::new_random_sha256();
            let (auth_url, csrf_token, _nonce) = oauth_client
                .authorize_url(
                    openidconnect::core::CoreAuthenticationFlow::AuthorizationCode,
                    openidconnect::CsrfToken::new_random,
                    openidconnect::Nonce::new_random,
                )
                .add_scope(openidconnect::Scope::new("read".to_string()))
                .add_scope(openidconnect::Scope::new("federate".to_string()))
                .set_pkce_challenge(pkce_challenge)
                .url();
            // bypass browser because we don't need to ask permission and there's no mechanism to open a browser
            let remote_login = FederatedAuthenticationPayload {
                display_name: auth_info.display_name.clone(),
                password: auth_info.password.clone(),
                domain_source: self.domain.to_string(),
                domain_target: auth_info.domain.clone(),
            };
            let auth_resp = oauth_http_client.post(auth_url)
                .form(&remote_login)
                .send()
                .await
                .map_err(|e| super::AuthError {
                    message: format!("Failed to authenticate federated login in for {}: {}", auth_info.display_name, e),
                    code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                })?;
            let auth_redirected_url = if let Some(loc_header) = auth_resp.headers().get("location") {
                let url = loc_header.to_str().map_err(|e| super::AuthError {
                    message: format!("Failed to stringify location header for {}: {}", auth_info.display_name, e),
                    code: crate::data::error_codes::AuthErrorCode::InvalidDisplayName,
                })?;
                reqwest::Url::parse(url).map_err(|e| super::AuthError {
                    message: format!("Failed to parse location URL for {}: {}", auth_info.display_name, e),
                    code: crate::data::error_codes::AuthErrorCode::InvalidDisplayName,
                })?
            } else {
                return Err(super::AuthError {
                    message: format!("Bad OAuth2 auth response from {} for logging in {} (missing location header)", target_domain, auth_info.display_name),
                    code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                });
            };
            log::debug!("OAuth auth response URL: {}", auth_redirected_url);
            let mut query_map: std::collections::HashMap<_, _>  = auth_redirected_url.query_pairs().collect();
            let auth_code = if let Some(auth_code) = query_map.remove("code") {
                openidconnect::AuthorizationCode::new(auth_code.to_string())
            } else {
                return Err(super::AuthError {
                    message: format!("Bad OAuth2 auth response from {} for logging in {} (missing code)", target_domain, auth_info.display_name),
                    code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                });
            };
            if let Some(state) = query_map.remove("state") {
                if state != csrf_token.into_secret() {
                    return Err(super::AuthError {
                        message: format!("Bad OAuth2 auth response from {} for logging in {} (invalid state)", target_domain, auth_info.display_name),
                        code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                    });
                }
            } else {
                return Err(super::AuthError {
                    message: format!("Bad OAuth2 auth response from {} for logging in {} (missing state)", target_domain, auth_info.display_name),
                    code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                });
            };
            let token_resp: TokenResponsePayload = oauth_client.exchange_code(auth_code)
                .map_err(|e| super::AuthError {
                    message: format!("Failed to exchange code for {}: {}", auth_info.display_name, e),
                    code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                })?
                .set_pkce_verifier(pkce_verifier)
                .request_async(&oauth_http_client).await
                .map_err(|e| super::AuthError {
                    message: format!("Failed to exchange code for {}: {}", auth_info.display_name, e),
                    code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                })?;
            let _id_token = token_resp.id_token()
                .ok_or_else(|| super::AuthError {
                    message: format!("OAuth2 Token response missing ID token for {}", auth_info.display_name),
                    code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                })?;
            // Don't verify since it's not using the openidconnect::core's signing algorithm
            /*let id_token_verifier = oauth_client.id_token_verifier();
            let claims = id_token.claims(&id_token_verifier, &nonce)
                .map_err(|e| super::AuthError {
                    message: format!("Failed to verify OAuth2 claims for {}: {}", auth_info.display_name, e),
                    code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                })?;
            if let Some(expected_access_token_hash) = claims.access_token_hash() {
                let actual_access_token_hash = openidconnect::AccessTokenHash::from_token(
                    token_resp.access_token(),
                    id_token.signing_alg().map_err(|e| super::AuthError {
                        message: format!("Failed to verify OAuth2 signing algorithm for {}: {}", auth_info.display_name, e),
                        code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                    })?,
                    id_token.signing_key(&id_token_verifier).map_err(|e| super::AuthError {
                        message: format!("Failed to verify OAuth2 signing key for {}: {}", auth_info.display_name, e),
                        code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                    })?,
                ).map_err(|e| super::AuthError {
                    message: format!("Failed to verify OAuth2 access token for {}: {}", auth_info.display_name, e),
                    code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                })?;
                if actual_access_token_hash != *expected_access_token_hash {
                    return Err(super::AuthError {
                        message: format!("Failed to verify OAuth2 access token for {} (no match)", auth_info.display_name),
                        code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                    });
                }
            }*/
            let refresh_token = token_resp.refresh_token()
                .ok_or_else(|| super::AuthError {
                    message: format!("OAuth2 Token response missing refresh token for {}", auth_info.display_name),
                    code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                })?
                .secret()
                .to_owned();
            let remote_token = token_resp.access_token().secret();
            #[cfg(debug_assertions)]
            log::debug!("User {} authenticated to {} with access token {}", auth_info.display_name, target_domain, remote_token);
            // create/update federated user entry in DB
            let user_id = self.update_local_database(&auth_info, &urls).await.map_err(|e| super::AuthError {
                message: format!("Failed to update DB entries for {}: {}", auth_info.display_name, e),
                code: crate::data::error_codes::AuthErrorCode::Unknown,
            })?;
            // return local success token
            let local_token = self.localify_token(remote_token)?;
            Ok(super::UserLoginInfo {
                response: libfj::robocraft::AuthenticationResponseInfo {
                    token: local_token,
                    refresh_token,
                    refresh_token_expiry: "0".to_string(), // TODO (seems like this isn't actually considered by the client)
                },
                is_new: false,
                id: user_id,
            })
        } else {
            Err(super::AuthError {
                message: format!("Refusing federation with {} for logging in {} (federation not configured)", auth_info.domain, auth_info.display_name),
                code: crate::data::error_codes::AuthErrorCode::PasswordInvalidated,
            })
        }
    }

    fn localify_token(&self, remote_token: &str) -> Result<String, super::AuthError> {
        let remote_token_data = jsonwebtoken::dangerous::insecure_decode::<crate::auth::Token>(remote_token)
            .map_err(|e| super::AuthError {
                message: e.to_string(),
                code: crate::data::error_codes::AuthErrorCode::BadCredentials,
            })?;
        let local_token_data = crate::auth::Token {
            iss: self.auth.to_string(),
            fedi_token: Some(remote_token.to_owned()),
            ..remote_token_data.claims
        };
        let header = jsonwebtoken::Header {
            typ: Some("JWT".to_string()),
            alg: jsonwebtoken::Algorithm::HS256,
            ..Default::default()
        };
        let secret = jsonwebtoken::EncodingKey::from_secret(&self.secret);
        let token = jsonwebtoken::encode(&header, &local_token_data, &secret)
            .unwrap_or_else(|e| {
                log::error!("Failed to encode fedi JWT: {}", e);
                libfj::robocraft::DEFAULT_TOKEN.to_owned()
            });
        Ok(token)
    }

    async fn update_local_database(&self, auth_info: &super::FederatedAuthInfo, services_info: &oj_serdes::society::ServiceDomains) -> Result<i32, oj_rc_database::sea_orm::DbErr> {
        use oj_rc_database::sea_orm::IntoActiveModel;
        let now = chrono::Utc::now().timestamp();
        let fedi_id = if let Some(existing_fedi) = self.db.federation_by_domain(&services_info.root).await? {
            let mut active = existing_fedi.into_active_model();
            active.last_used_time = oj_rc_database::sea_orm::ActiveValue::Set(now);
            active.auth = oj_rc_database::sea_orm::ActiveValue::Set(services_info.auth.clone());
            active.cdn = oj_rc_database::sea_orm::ActiveValue::Set(services_info.cdn.clone());
            active.factory = oj_rc_database::sea_orm::ActiveValue::Set(services_info.factory.clone());
            active.society = oj_rc_database::sea_orm::ActiveValue::Set(services_info.society.clone());
            self.db.update_federation(active).await?.id
        } else {
            let new_entity = oj_rc_database::schema::federation::ActiveModel {
                id: oj_rc_database::sea_orm::ActiveValue::NotSet,
                creation_time: oj_rc_database::sea_orm::ActiveValue::Set(now),
                last_used_time: oj_rc_database::sea_orm::ActiveValue::Set(now),
                domain: oj_rc_database::sea_orm::ActiveValue::Set(services_info.root.clone()),
                auth: oj_rc_database::sea_orm::ActiveValue::Set(services_info.auth.clone()),
                cdn: oj_rc_database::sea_orm::ActiveValue::Set(services_info.cdn.clone()),
                factory: oj_rc_database::sea_orm::ActiveValue::Set(services_info.factory.clone()),
                society: oj_rc_database::sea_orm::ActiveValue::Set(services_info.society.clone()),
            };
            self.db.insert_federation(new_entity).await?.id
        };
        let qualified_name = format!("{}#{}", auth_info.display_name, services_info.root);
        let user_id = if let Some(existing_user) = self.db.user_by_display_name_and_federation(qualified_name.clone(), fedi_id).await? {
            log::info!("Using existing federated user with id {} for {} from {}", existing_user.id, auth_info.display_name, auth_info.domain);
            existing_user.id
        } else {
            let new_id = super::initial_data::register_new_federated_user(auth_info, fedi_id, &qualified_name, self.db.as_ref()).await?;
            log::info!("Created federated user with id {} for {} from {}", new_id, auth_info.display_name, auth_info.domain);
            new_id
        };
        Ok(user_id)
    }

    async fn remote_auth_impl(&self, auth_info: &FederatedAuthenticationPayload, challenge: &str, federation: &Option<crate::persist::config::Federation>) -> Result<String, super::AuthError> {
        if auth_info.display_name.is_empty() {
            return Err(super::AuthError {
                message: "Refusing federation login with empty username".to_string(),
                code: crate::data::error_codes::AuthErrorCode::InvalidDisplayName,
            });
        }
        if let Some(fedi_conf) = federation {
            if auth_info.domain_source.is_empty() {
                return Err(super::AuthError {
                    message: "Refusing federation login with empty domain_source".to_string(),
                    code: crate::data::error_codes::AuthErrorCode::InvalidDisplayName,
                });
            }
            if auth_info.domain_target != *self.domain {
                return Err(super::AuthError {
                    message: "Refusing federation login with not my domain_target".to_string(),
                    code: crate::data::error_codes::AuthErrorCode::InvalidDisplayName,
                });
            }
            if self.is_defederated_from(&auth_info.domain_source, fedi_conf) {
                return Err(super::AuthError {
                    message: format!("Refusing federation with {} for logging in {} (server defed)", auth_info.domain_source, auth_info.display_name),
                    code: crate::data::error_codes::AuthErrorCode::PasswordInvalidated,
                });
            }
            let user_info = super::UserAuthInfo::Username {
                username: auth_info.display_name.clone(),
                password: auth_info.password.clone(),
            };
            let login_info = self.login_internal(user_info, Some(auth_info.domain_source.clone())).await?;
            let user_fedi_aux = self.db.user_aux_by_user_id_and_descriptor(login_info.id, oj_rc_database::schema::user_aux::Descriptor::Federation).await
                .map_err(|e| super::AuthError {
                    message: format!("Failed to retrieve federation config for logging in {}: {}", auth_info.display_name, e),
                    code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                })?;
            if let Some(user_fedi_aux) = user_fedi_aux {
                let settings: super::Federation = serde_json::from_str(&user_fedi_aux.data)
                    .map_err(|e| super::AuthError {
                        message: format!("Failed to deserialize user_aux federation for logging in {}: {}", auth_info.display_name, e),
                        code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                    })?;
                let faked_server_config = crate::persist::config::Federation {
                    aliases: Default::default(),
                    defederated: settings.defederated,
                };
                if !settings.enabled || self.is_defederated_from(&auth_info.domain_source, &faked_server_config) {
                    return Err(super::AuthError {
                        message: format!("Refusing federation with {} for logging in {} (user defed)", auth_info.domain_source, auth_info.display_name),
                        code: crate::data::error_codes::AuthErrorCode::PasswordInvalidated,
                    });
                }
            } // default is to allow for user settings (NOT server config)
            Ok(Self::generate_access_code(
                &self.auth,
                challenge,
                &auth_info.display_name,
                &login_info.response.token,
                &login_info.response.refresh_token,
                &self.secret,
                self.nonce_provider(),
            ))
        } else {
            Err(super::AuthError {
                message: format!("Refusing federation with {} for logging in {} (federation not configured)", auth_info.domain_source, auth_info.display_name),
                code: crate::data::error_codes::AuthErrorCode::PasswordInvalidated,
            })
        }
    }

    async fn remote_token_impl(&self, access_code: &str, verifier: &str) -> Result<super::UserLoginInfo, super::AuthError> {
        let (_code, tokens) = Self::read_access_code(access_code, &self.auth, &self.secret, self.nonce_provider())
            .map_err(|_| super::AuthError {
                message: "Failed to read access code".to_owned(),
                code: crate::data::error_codes::AuthErrorCode::BadCredentials,
            })?;
        if !Self::validate_pkce(&tokens.code_challenge, verifier) {
            return Err(super::AuthError {
                message: "Failed to validate PKCE".to_owned(),
                code: crate::data::error_codes::AuthErrorCode::AccountUnconfirmed,
            });
        }
        Ok(super::UserLoginInfo {
            response: libfj::robocraft::AuthenticationResponseInfo {
                token: tokens.access_token,
                refresh_token: tokens.refresh_token,
                refresh_token_expiry: "0".to_owned(), // TODO
            },
            is_new: false,
            id: -1, // unused
        })
    }

    fn build_key(secret: &[u8], issuer: &str) -> Vec<u8> {
        if secret.len() < 32 {
            let mut temp_key = Vec::from(secret);
            for b in issuer.bytes() {
                temp_key.push(b);
                if temp_key.len() >= 32 {
                    break;
                }
            }
            if temp_key.len() < 32 {
                temp_key.extend(std::iter::repeat_n(0, 32 - temp_key.len()));
            }
            temp_key
        } else if secret.len() > 32 {
            let mut temp_key = Vec::from(secret);
            temp_key.truncate(32);
            temp_key
        } else {
            Vec::from(secret)
        }
    }

    fn generate_access_code(issuer: &str, challenge: &str, display_name: &str, access_token: &str, refresh_token: &str, secret: &[u8], mut noncer: NonceProvider) -> String {
        use ring::aead::BoundKey;
        use base64::Engine;
        let now = chrono::Utc::now().timestamp();
        noncer.reset(now);
        let secure_data = SecuredCodes {
            access_token: access_token.to_owned(),
            refresh_token: refresh_token.to_owned(),
            code_challenge: challenge.to_owned(),
        };
        let secure_data_str = serde_json::to_string(&secure_data).unwrap();
        let key = Self::build_key(secret, issuer);
        let enc_key = ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &key).unwrap();
        let mut seal_key = ring::aead::SealingKey::new(enc_key, noncer);
        let mut enc_data = Vec::from(secure_data_str.as_bytes());
        seal_key.seal_in_place_append_tag(ring::aead::Aad::from(ACCESS_CODE_AAD), &mut enc_data).unwrap();
        let secured_b64 = base64::engine::general_purpose::STANDARD.encode(enc_data);
        let token_data = AccessCode {
            aud: issuer.to_owned(),
            exp: now + 60, // 60s
            iat: now,
            iss: issuer.to_owned(),
            sub: display_name.to_owned(),
            secured: secured_b64,
        };
        let data_str = serde_json::to_string(&token_data).unwrap();
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data_str)
    }

    fn read_access_code(code: &str, issuer: &str, secret: &[u8], mut noncer: NonceProvider) -> Result<(AccessCode, SecuredCodes), ()> {
        use ring::aead::BoundKey;
        use base64::Engine;
        let json_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(code)
            .map_err(|e| {
                log::error!("Failed to decode Base64 access code: {}", e);
            })?;
        let access_code: AccessCode = serde_json::from_slice(&json_bytes)
            .map_err(|e| {
                log::error!("Failed to decode JSON access code: {}", e);
            })?;
        let mut enc_data = base64::engine::general_purpose::STANDARD.decode(&access_code.secured)
            .map_err(|e| {
                log::error!("Failed to decode Base64 secure code: {}", e);
            })?;
        noncer.reset(access_code.iat);
        let key = Self::build_key(secret, issuer);
        let enc_key = ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &key).unwrap();
        let mut opening_key = ring::aead::OpeningKey::new(enc_key, noncer);
        let plaintext = opening_key.open_in_place(ring::aead::Aad::from(ACCESS_CODE_AAD), &mut enc_data)
            .map_err(|e| {
                log::error!("Failed to decrypt secure code: {}", e);
            })?;
        let secure_data: SecuredCodes = serde_json::from_slice(plaintext)
            .map_err(|e| {
                log::error!("Failed to decode JSON secure code: {}", e);
            })?;
        Ok((access_code, secure_data))
    }

    fn validate_pkce(challenge: &str, verifier: &str) -> bool {
        use sha2::Digest;
        use base64::Engine;
        let hash = sha2::Sha256::new()
            .chain_update(verifier.as_bytes())
            .finalize();
        let expected = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(hash);
        expected == challenge
    }
}

#[async_trait::async_trait]
impl super::FederatedAuthenticator for super::AccountProvider {
    async fn local_login(&self, info: super::FederatedAuthInfo) -> Result<super::UserLoginInfo, super::AuthError> {
        self.local_login_impl(info, &self.federation).await
    }

    async fn remote_auth(&self, info: &FederatedAuthenticationPayload, challenge: &str) -> Result<String, super::AuthError> {
        self.remote_auth_impl(info, challenge, &self.federation).await
    }

    async fn remote_token(&self, access_code: &str, verifier: &str) -> Result<super::UserLoginInfo, super::AuthError> {
        self.remote_token_impl(access_code, verifier).await
    }
}
