use argon2::PasswordVerifier;
use oj_rc_database::sea_orm::IntoActiveModel;
use sha2::Digest;

use crate::persist::config::ConfigProvider;

pub struct AccountProvider {
    cubes: std::sync::Arc<Vec<u32>>,
    garage_upgrades: std::sync::Arc<crate::persist::config::GarageUpgrades>,
    fake_players: std::sync::Arc<Vec<crate::persist::config::FakePlayer>>,
    filler_players: std::sync::Arc<Vec<crate::persist::config::FakePlayer>>,
    auto_signups: bool,
    pub(super) domain: std::sync::Arc<String>,
    cdn: std::sync::Arc<String>,
    pub(super) auth: std::sync::Arc<String>,
    pub(super) intercom: std::sync::Arc<String>,
    pub(super) intercom_http_client: std::sync::Arc<reqwest::Client>,
    pub(super) secret: std::sync::Arc<Vec<u8>>,
    pub(super) db: std::sync::Arc<oj_rc_database::Database>,
    pub(super) federation: Option<crate::persist::config::Federation>,
}

impl AccountProvider {
    pub async fn load(root: impl AsRef<std::path::Path>, conf: &crate::persist::config::ConfigImpl) -> std::io::Result<Self> {
        let token_path = root.as_ref().join(super::TOKEN_SECRET_FILENAME);
        log::debug!("Loading secret from {}", token_path.display());
        let secret = std::fs::read(&token_path)?;
        let server_settings = <crate::persist::config::ConfigImpl as ConfigProvider<()>>::server_config(conf);
        let database_uri = server_settings.database;
        log::debug!("Connecting to user database URI: {}", database_uri);
        let db = oj_rc_database::Database::init(&database_uri).await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::NotConnected, e))?;
        let federation_conf = <crate::persist::config::ConfigImpl as ConfigProvider<()>>::federation(conf);
        Ok(Self {
            cubes: std::sync::Arc::new(<crate::persist::config::ConfigImpl as ConfigProvider<()>>::ids(conf)),
            garage_upgrades: std::sync::Arc::new(<crate::persist::config::ConfigImpl as ConfigProvider<()>>::garage_upgrades(conf)),
            fake_players: std::sync::Arc::new(<crate::persist::config::ConfigImpl as ConfigProvider<()>>::fake_players(conf)),
            filler_players: std::sync::Arc::new(<crate::persist::config::ConfigImpl as ConfigProvider<()>>::filler_players(conf)),
            auto_signups: server_settings.auto_signup,
            domain: std::sync::Arc::new(server_settings.domain),
            cdn: std::sync::Arc::new(server_settings.cdn_url),
            auth: std::sync::Arc::new(server_settings.auth_url),
            intercom: std::sync::Arc::new(server_settings.intercom_url),
            intercom_http_client: std::sync::Arc::new(
                reqwest::ClientBuilder::new()
                    .redirect(reqwest::redirect::Policy::none())
                    .build()
                    .expect("HTTP client did not init")
            ),
            secret: std::sync::Arc::new(secret),
            db: std::sync::Arc::new(db),
            federation: federation_conf,
        })
    }

    pub fn factory_impl(&self) -> oj_rc_database::FactoryDatabase {
        self.db.vehicle_factory(self.cdn.clone())
    }

    pub async fn multiplayer_init(&self) -> Result<(), oj_rc_database::sea_orm::DbErr> {
        self.db.complete_all_games().await
    }

    /*pub fn fake_user<C: Clone>(&self) -> Box<dyn super::User<C> + Send + Sync> {
        Box::new(UserData {
            token: super::UserToken { uuid: "fake user!".to_owned(), token: "".to_owned(), refresh_token: "".to_owned() },
            account: oj_rc_database::schema::user::Model {
                id: 0,
                creation_time: 0,
                public_id: "".to_owned(),
                display_name: "".to_owned(),
                password: "".to_owned(),
                email: "".to_owned(),
                steam_id: None,
            },
            perms: oj_rc_database::schema::permissions::Model {
                id: 0,
                user_id: 0,
                moderator: true,
                administrator: true,
                developer: true,
                royalty: false,
                banned: false,
            },
            cubes: self.cubes.clone(),
            garage_upgrades: self.garage_upgrades.clone(),
            singleplayer_vehicles: self.singleplayer_vehicles.clone(),
            extensions: Default::default(),
            db: self.db.clone(),
        })
    }*/

    async fn auth_internal(&self, token: &str) -> Result<UserData, super::AuthError> {
        let secret = jsonwebtoken::DecodingKey::from_secret(&self.secret);
        let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.set_required_spec_claims::<&str>(&[]);
        validation.aud = Some(vec![ self.domain.to_string() ].into_iter().collect());
        let token_data = jsonwebtoken::decode::<crate::auth::Token>(&token, &secret, &validation).map_err(|e| super::AuthError {
            message: e.to_string(),
            code: crate::data::error_codes::AuthErrorCode::BadCredentials,
        })?;
        let display_name = token_data.claims.client_details.display_name.clone();
        let user_info = if let Some(user_info) = self.db.user_by_display_name(display_name.clone()).await.map_err(|e| super::AuthError {
            message: e.to_string(),
            code: crate::data::error_codes::AuthErrorCode::Unknown,
        })? {
            user_info
        } else {
            return Err(super::AuthError {
                message: "User not found".to_owned(),
                code: crate::data::error_codes::AuthErrorCode::BadCredentials,
            });
        };
        let user_perms = if let Some(user_perms) = self.db.perms_by_user_id(user_info.id).await.map_err(|e| super::AuthError {
            message: e.to_string(),
            code: crate::data::error_codes::AuthErrorCode::Unknown
        })? {
            user_perms
        } else {
            return Err(super::AuthError {
                message: "User permissions not found".to_owned(),
                code: crate::data::error_codes::AuthErrorCode::BadCredentials,
            });
        };
        #[cfg(debug_assertions)]
        log::info!("Authenticated user {} with flags {:?}", display_name, token_data.claims.client_details.flags.as_slice());
        Ok(UserData {
            account: user_info,
            perms: user_perms,
            cubes: self.cubes.clone(),
            garage_upgrades: self.garage_upgrades.clone(),
            fake_players: self.fake_players.clone(),
            filler_players: self.filler_players.clone(),
            cdn: self.cdn.clone(),
            auth: self.auth.clone(),
            intercom: self.intercom.clone(),
            http_client: std::sync::Arc::new(reqwest::Client::new()),
            db: self.db.clone(),
            secret: self.secret.clone(),
        })
    }

    pub(super) async fn login_internal(&self, info: super::UserAuthInfo, audience: Option<String>) -> Result<super::UserLoginInfo, super::AuthError> {
        //let new_root = self.root.join(&info.payload.public_id);
        let is_fedi = audience.is_some();
        let is_new_user;
        let user_opt = match &info {
            super::UserAuthInfo::Steam { id } => self.db.user_by_steam_id(*id).await,
            super::UserAuthInfo::Email { email, .. } => self.db.user_by_email(email.to_owned()).await,
            super::UserAuthInfo::Username { username, .. } => self.db.user_by_display_name(username.to_owned()).await,
        }.map_err(|e| super::AuthError {
            message: e.to_string(),
            code: crate::data::error_codes::AuthErrorCode::BadCredentials,
        })?;
        let mut user_info = if let Some(user_info) = user_opt {
            is_new_user = false;
            user_info
        } else {
            is_new_user = true;
            if self.auto_signups && !is_fedi {
                log::info!("New user {}", info.display_id());
                let auto_name = match &info {
                    super::UserAuthInfo::Steam { id } => id.to_string(),
                    super::UserAuthInfo::Email { email, .. } => email.to_owned(),
                    super::UserAuthInfo::Username { username, .. } => username.to_owned(),
                };
                super::setup_new_user(&info, auto_name.clone(), &self.db).await.map_err(|e| super::AuthError {
                    message: e.to_string(),
                    code: crate::data::error_codes::AuthErrorCode::Unknown,
                })?;

                self.db.user_by_display_name(auto_name).await.map_err(|e| super::AuthError {
                    message: e.to_string(),
                    code: crate::data::error_codes::AuthErrorCode::Unknown,
                })?.unwrap()
            } else {
                log::info!("Rejecting user sign-in for `{}` (set settings.server.auto_signup=true to disable this behaviour)", info.display_id());
                return Err(super::AuthError {
                    message: "User not found".to_owned(),
                    code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                });
            }
        };
        let override_password = user_info.password.is_empty() && user_info.steam_id.is_none();
        match &info {
            super::UserAuthInfo::Steam { id } => {
                let id_str = id.to_string();
                if let Some(expected_steam_id) = user_info.steam_id {
                    if expected_steam_id != id_str {
                        return Err(super::AuthError {
                            message: "SteamID does not match".to_owned(),
                            code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                        });
                    }
                } else {
                    return Err(super::AuthError {
                        message: "SteamID not supported for this user".to_owned(),
                        code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                    });
                }
            },
            super::UserAuthInfo::Email { password, .. }
            | super::UserAuthInfo::Username { password, .. } => {
                use argon2::password_hash::PasswordHasher;
                let argon2_algo = argon2::Argon2::default();
                if override_password {
                    let salt = argon2::password_hash::SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
                    let password_hash = argon2_algo.hash_password(password.as_bytes(), &salt).map_err(|e| super::AuthError {
                        message: e.to_string(),
                        code: crate::data::error_codes::AuthErrorCode::Unknown,
                    })?.to_string();
                    user_info.password = password_hash;
                }
                if !user_info.password.is_empty() {
                    let expected = argon2::password_hash::PasswordHash::new(&user_info.password).map_err(|e| super::AuthError {
                        message: e.to_string(),
                        code: crate::data::error_codes::AuthErrorCode::Unknown,
                    })?;
                    argon2_algo.verify_password(password.as_bytes(), &expected).map_err(|e| super::AuthError {
                        message: e.to_string(),
                        code: crate::data::error_codes::AuthErrorCode::Unknown,
                    })?;
                } else {
                    return Err(super::AuthError {
                        message: "Password not supported for this user".to_owned(),
                        code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                    });
                }
            }
        }
        // check if user is banned
        if let Some(perms) = self.db.perms_by_user_id(user_info.id).await.map_err(|e| super::AuthError {
            message: e.to_string(),
            code: crate::data::error_codes::AuthErrorCode::Unknown,
        })? {
            if perms.banned {
                return Err(super::AuthError {
                    message: "User is banned".to_owned(),
                    code: crate::data::error_codes::AuthErrorCode::AccountBlocked301,
                });
            }
        } else {
            return Err(super::AuthError {
                message: "User has no permissions".to_owned(),
                code: crate::data::error_codes::AuthErrorCode::BadCredentials,
            });
        }
        // authentication has now definitely succeeded
        // build token
        let header = jsonwebtoken::Header {
            typ: Some("JWT".to_string()),
            alg: jsonwebtoken::Algorithm::HS256,
            ..Default::default()
        };

        let login_method = match &info {
            super::UserAuthInfo::Steam { .. } => crate::auth::LoginMethod::Steam,
            super::UserAuthInfo::Email { .. } => crate::auth::LoginMethod::Email,
            super::UserAuthInfo::Username { .. } => crate::auth::LoginMethod::Username,
        };

        let pub_id = if is_fedi { format!("{}#{}", user_info.public_id, self.domain) } else { user_info.public_id.clone() };
        let client_details = libfj::robocraft::TokenPayload {
            public_id: pub_id.clone(),
            display_name: if is_fedi { format!("{}#{}", user_info.display_name, self.domain) } else { user_info.display_name.clone() },
            robocraft_name: pub_id.clone(),
            email_address: if is_fedi { format!("{}@{}", user_info.public_id, self.domain) } else { user_info.email },
            email_verified: true,
            flags: vec![
                if is_fedi { "federated=true".to_owned() } else { "federated=false".to_owned() },
            ],
        };
        let now = chrono::Utc::now().timestamp();
        let payload = crate::auth::Token {
            client_details,
            federate: is_fedi,
            auth_time: now,
            qualified_name: format!("{}#{}", user_info.public_id, self.domain),
            source_domain: self.domain.to_string(),
            login_method: if is_fedi { crate::auth::LoginMethod::OAuth } else { login_method },
            iss: self.auth.to_string(),
            exp: now + 86400, // 1 day
            iat: now,
            sub: pub_id.clone(),
            aud: audience.unwrap_or_else(|| self.domain.to_string()),
            fedi_token: None,
        };
        #[cfg(debug_assertions)]
        log::debug!("Token payload\n{}", serde_json::to_string_pretty(&payload).unwrap());
        let secret = jsonwebtoken::EncodingKey::from_secret(&self.secret);
        let token = jsonwebtoken::encode(&header, &payload, &secret)
            .unwrap_or_else(|e| {
                log::error!("Failed to encode JWT: {}", e);
                libfj::robocraft::DEFAULT_TOKEN.to_owned()
            });

        Ok(super::UserLoginInfo {
            response: libfj::robocraft::AuthenticationResponseInfo {
                token,
                refresh_token: "qwertyuiop".to_string(), // TODO
                refresh_token_expiry: "0".to_string(), // TODO (seems like this isn't actually considered by the client)
            },
            is_new: is_new_user,
            id: user_info.id,
        })
    }
}

#[async_trait::async_trait]
impl <C: Clone + Send> super::UserProvider<C> for AccountProvider {
    async fn authenticate(&self, token: super::UserToken) -> Result<Box<dyn super::User<C> + Send + Sync>, super::AuthError> {
        Ok(Box::new(self.auth_internal(&token.token).await?))
    }

    async fn multiplayer_authenticate(&self, user: String) -> Result<Box<dyn super::User<C> + Send + Sync>, super::AuthError> {
        let user_info = if let Some(user_info) = self.db.user_by_display_name(user).await.map_err(|e| super::AuthError {
            message: e.to_string(),
            code: crate::data::error_codes::AuthErrorCode::Unknown,
        })? {
            user_info
        } else {
            return Err(super::AuthError {
                message: "User not found".to_owned(),
                code: crate::data::error_codes::AuthErrorCode::BadCredentials,
            });
        };
        let user_perms = if let Some(user_perms) = self.db.perms_by_user_id(user_info.id).await.map_err(|e| super::AuthError {
            message: e.to_string(),
            code: crate::data::error_codes::AuthErrorCode::Unknown,
        })? {
            user_perms
        } else {
            return Err(super::AuthError {
                message: "User permissions not found".to_owned(),
                code: crate::data::error_codes::AuthErrorCode::BadCredentials,
            });
        };
        Ok(Box::new(UserData {
            account: user_info,
            perms: user_perms,
            cubes: self.cubes.clone(),
            garage_upgrades: self.garage_upgrades.clone(),
            fake_players: self.fake_players.clone(),
            filler_players: self.filler_players.clone(),
            cdn: self.cdn.clone(),
            auth: self.auth.clone(),
            intercom: self.intercom.clone(),
            http_client: std::sync::Arc::new(reqwest::Client::new()),
            db: self.db.clone(),
            secret: self.secret.clone(),
        }))
    }

    async fn web_authenticate(&self, token: String) -> Result<Box<dyn super::WebUser>, super::AuthError> {
        Ok(Box::new(self.auth_internal(&token).await?))
    }
}

#[async_trait::async_trait]
impl super::UserAuthenticator for AccountProvider {
    async fn login(&self, info: super::UserAuthInfo) -> Result<super::UserLoginInfo, super::AuthError> {
        self.login_internal(info, None).await
    }

    async fn user_exists(&self, user: super::UserId) -> Result<bool, String> {
        Ok(match user {
            super::UserId::SteamId(steam_id) => {
                self.db.user_by_steam_id(steam_id).await.map_err(|e| e.to_string())?.is_some()
            },
            super::UserId::Email(email) => {
                self.db.user_by_email(email).await.map_err(|e| e.to_string())?.is_some()
            },
            super::UserId::Username(display_name) => {
                self.db.user_by_display_name(display_name).await.map_err(|e| e.to_string())?.is_some()
            },
        })
    }

    async fn register(&self, info: super::RegistrationInfo) -> Result<i32, String> {
        super::register_new_user(&info, &self.db).await.map_err(|e| e.to_string())
    }

    async fn verify(&self, token: String) -> Result<crate::auth::Token, super::AuthError> {
        let secret = jsonwebtoken::DecodingKey::from_secret(&self.secret);
        let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.set_required_spec_claims::<&str>(&[]);
        jsonwebtoken::decode::<crate::auth::Token>(&token, &secret, &validation)
            .map_err(|e| super::AuthError {
                message: e.to_string(),
                code: crate::data::error_codes::AuthErrorCode::BadCredentials,
            })
            .map(|decoded| decoded.claims)
    }
}

pub(super) struct UserData {
    pub(super) account: oj_rc_database::schema::user::Model,
    pub(super) perms: oj_rc_database::schema::permissions::Model,
    pub(super) cubes: std::sync::Arc<Vec<u32>>,
    pub(super) garage_upgrades: std::sync::Arc<crate::persist::config::GarageUpgrades>,
    pub(super) fake_players: std::sync::Arc<Vec<crate::persist::config::FakePlayer>>,
    pub(super) filler_players: std::sync::Arc<Vec<crate::persist::config::FakePlayer>>,
    pub(super) cdn: std::sync::Arc<String>,
    pub(super) auth: std::sync::Arc<String>,
    pub(super) intercom: std::sync::Arc<String>,
    pub(super) http_client: std::sync::Arc<reqwest::Client>,
    pub(super) db: std::sync::Arc<oj_rc_database::Database>,
    pub(super) secret: std::sync::Arc<Vec<u8>>,
}

impl UserData {
    pub(super) async fn load_garage_by_slot(&self, slot: i32) -> Result<Option<oj_rc_database::schema::garage::Model>, oj_rc_database::sea_orm::DbErr> {
        //let path = self.root.join(super::GARAGE_DIR).join(format!("{}.json", id));
        //crate::persist::GarageSlot::load(&path)
        self.db.garage_by_user_id_and_slot(self.account.id, slot).await
    }

    async fn save_garage_by_slot(&self, data: oj_rc_database::schema::garage::ActiveModel, slot: i32) -> Result<(), oj_rc_database::sea_orm::DbErr> {
        self.db.update_garage_by_user_id_and_slot(data, self.account.id, slot).await?;
        Ok(())
    }

    pub(super) async fn all_vehicles(&self) -> Result<Vec<oj_rc_database::schema::garage::Model>, oj_rc_database::sea_orm::DbErr> {
        self.db.garages_by_user_id(self.account.id).await
    }

    async fn double_check_permissions(&self) -> Result<oj_rc_database::schema::permissions::Model, oj_rc_database::sea_orm::DbErr> {
        Ok(self.db.perms_by_user_id(self.account.id).await?.unwrap())
    }

    pub(super) async fn err_on_banned(&self) -> Result<(), polariton_server::operations::SimpleOpError> {
        let perms = self.double_check_permissions().await.map_err(|e| {
            log::error!("Failed to retrieve user {} permissions: {}", self.account.id, e);
            polariton_server::operations::SimpleOpError::with_message(
                DATABASE_ERR,
                format!("Failed to retrieve user permissions: {}", e),
            )
        })?;
        if perms.banned {
            Err(polariton_server::operations::SimpleOpError::with_code(crate::data::error_codes::WebServicesError::Banned as i16))
        } else {
            Ok(())
        }
    }

    pub(super) fn check_perms_to_exec(&self, ty: &super::SanctionType) -> Result<(), i16> {
        match ty {
            super::SanctionType::Warn
            | super::SanctionType::Mute
            | super::SanctionType::Note => if self.has_any_elevated_perms() {
                Ok(())
            } else {
                Err(crate::data::error_codes::ChatErrorCodes::ModeratorsOnly as i16)
            },
            super::SanctionType::Ban
            | super::SanctionType::Kick => if self.has_admin_or_better_perms() {
                Ok(())
            } else {
                Err(crate::data::error_codes::ChatErrorCodes::AdminsOnly as i16)
            },
        }
    }

    #[inline]
    fn has_any_elevated_perms(&self) -> bool {
        self.perms.moderator | self.perms.administrator | self.perms.developer
    }

    fn has_admin_or_better_perms(&self) -> bool {
        self.perms.administrator | self.perms.developer
    }

    pub(super) async fn user_player_data(&self, cpu_counter: &crate::cubes::CpuListParser) -> Result<crate::data::player_data::PlayerData, polariton_server::operations::SimpleOpError> {
        let current_slot = self.db.garage_selected(self.account.id).await.map_err(|e| {
            log::error!("Failed to retrieve selected vehicle for user_id {} (user_player_data): {}", self.account.id, e);
            polariton_server::operations::SimpleOpError::with_message(DATABASE_ERR, format!("Could not retrieve selected garage: {}", e))
        })?
        .ok_or_else(|| {
            log::error!("Failed to find selected vehicle for user_id {} (user_player_data)", self.account.id);
            polariton_server::operations::SimpleOpError::with_message(INVALID_ROBOT_ERR, "No selected garage".to_owned())
        })?;
        let user_uuid = self.account.public_id.clone();
        let weapon_orders = oj_rc_database::schema::parse_int_csv(&current_slot.weapon_order).into_iter().map(|x| x as i32).collect::<Vec<_>>();
        let weapon_ranks = oj_rc_database::schema::parse_int_csv(&current_slot.weapon_order).into_iter().map(|x| (x as i32, if x == 0 { 0 } else { 1 })).collect();
        let user_avatar_aux = self.db.user_aux_by_user_id_and_descriptor(self.account.id, oj_rc_database::schema::user_aux::Descriptor::AvatarId).await.map_err(|e| {
            log::error!("Failed to retrieve avatar for user_id {} (user_player_data): {}", self.account.id, e);
            polariton_server::operations::SimpleOpError::with_message(DATABASE_ERR, format!("Could not retrieve avatar: {}", e))
        })?
        .ok_or_else(|| {
            log::error!("Failed to find avatar for user_id {} (user_player_data)", self.account.id);
            polariton_server::operations::SimpleOpError::with_message(UNEXPECTED_ERR, "No avatar".to_owned())
        })?;
        let avatar_id: Result<i32, _> = user_avatar_aux.data.parse();
        let cpu_count = if current_slot.total_robot_cpu <= 0 {
            cpu_counter.calculate_cpu(&mut std::io::Cursor::new(&current_slot.robot_data)).total as i32
        } else {
            current_slot.total_robot_cpu
        };
        let clan_opt = self.db.clan_by_user_id(self.account.id).await.map_err(|e| {
            log::error!("Failed to retrieve clan for user_id {} (user_player_data): {}", self.account.id, e);
            polariton_server::operations::SimpleOpError::with_message(DATABASE_ERR, format!("Could not retrieve clan: {}", e))
        })?;

        Ok(crate::data::player_data::PlayerData {
            name: user_uuid,
            display_name: self.account.display_name.clone(),
            mastery: current_slot.mastery_level as i32,
            tier: 1, // FIXME
            robot_name: current_slot.name,
            robot_map: current_slot.robot_data,
            group: None, // no platoon
            team: 0,
            has_premium: false, // FIXME
            robot_uuid: super::i64_as_uuid_str(current_slot.uuid),
            cpu: cpu_count,
            avatar_id: avatar_id.ok(),
            weapon_order: weapon_orders,
            colour_map: current_slot.colour_data,
            is_ai: false,
            spawn_effect: current_slot.spawn_animation_id,
            death_effect: current_slot.death_animation_id,
            player_rank: 1, // FIXME
            weapon_rank: weapon_ranks,
            clan_name: clan_opt.map(|(clan, _member)| clan.name),
        })
    }

    pub(super) async fn resolve_vehicle(&self, vehicle: &crate::persist::config::VehicleInfo, factory: &dyn oj_rc_factory::VehicleFactoryAdapter, weapon_order: &crate::cubes::WeaponListParser, cpu_counter: &crate::cubes::CpuListParser) -> Result<super::ResolvedVehicle, polariton_server::operations::SimpleOpError> {
        let sha_bytes = sha2::Sha256::digest(vehicle.username.as_bytes());
        let u32_bytes = [
            sha_bytes[0],
            sha_bytes[1],
            sha_bytes[2],
            sha_bytes[3],
        ];
        let standard_uuid_uniqueness = (u32::from_be_bytes(u32_bytes) as i64) << 16; // middle 32 bits
        match &vehicle.id {
            crate::persist::config::VehicleDescriptor::Factory { factory: factory_id } => {
                match factory.vehicle(*factory_id).await {
                    Ok(Some(factory_vehicle)) => {
                        let uuid_i64 = crate::persist::user::uuid_sanitize(
                            standard_uuid_uniqueness
                            ^ crate::persist::user::i64_join((1 << 30, *factory_id as u32))
                        );
                        let uuid_str = crate::persist::user::i64_as_uuid_str(uuid_i64);
                        let weapons_guess = weapon_order.guess_weapons(&mut std::io::Cursor::new(&factory_vehicle.0.cube_data));
                        let weapons_guess = vec![weapons_guess[0], 0, 0];
                        let weapon_ranks = weapons_guess.iter().map(|&x| (x, if x == 0 { 0 } else { 1 })).collect();
                        let cpu_count = if factory_vehicle.1.cpu == 0 {
                            cpu_counter.calculate_cpu(&mut std::io::Cursor::new(&factory_vehicle.0.cube_data)).total as i32
                        } else {
                            factory_vehicle.1.cpu as i32
                        };
                        Ok(super::ResolvedVehicle {
                            mastery: 1,
                            tier: 1, // FIXME
                            robot_name: vehicle.name.clone().unwrap_or_else(|| factory_vehicle.1.name.clone()),
                            robot_map: factory_vehicle.0.cube_data,
                            robot_uuid: uuid_str,
                            cpu: cpu_count,
                            weapon_order: weapons_guess,
                            colour_map: factory_vehicle.0.colour_data,
                            spawn_effect: "Spawn".to_owned(),
                            death_effect: "Explosion".to_owned(),
                            weapon_rank: weapon_ranks,
                        })
                    },
                    Ok(None) => {
                        log::error!("Prefab vehicle {} does not exist in factory", factory_id);
                        Err(polariton_server::operations::SimpleOpError::with_message(
                            crate::data::error_codes::SingleplayerErrorCode::UnexpectedError as i16,
                            format!("Prefab vehicle {} does not exist in factory", factory_id),
                        ))
                    },
                    Err(e) => {
                        log::error!("Failed to retrieve prefab vehicle {} from factory: {}", factory_id, e);
                        Err(polariton_server::operations::SimpleOpError::with_message(
                            crate::data::error_codes::SingleplayerErrorCode::DatabaseError as i16,
                            format!("Failed to retrieve prefab vehicle {} from factory: {}", factory_id, e),
                        ))
                    }
                }
            },
            crate::persist::config::VehicleDescriptor::Database { garage } => {
                match self.db.garage_by_id(*garage).await {
                    Ok(Some(db_vehicle)) => {
                        let cpu_count = if db_vehicle.total_robot_cpu <= 0 {
                            cpu_counter.calculate_cpu(&mut std::io::Cursor::new(&db_vehicle.robot_data)).total as i32
                        } else {
                            db_vehicle.total_robot_cpu
                        };
                        let uuid_i64 = crate::persist::user::uuid_sanitize(
                            standard_uuid_uniqueness
                            ^ crate::persist::user::i64_join((1 << 31, *garage as u32))
                        );
                        let uuid_str = crate::persist::user::i64_as_uuid_str(uuid_i64);
                        Ok(super::ResolvedVehicle {
                            mastery: 1,
                            tier: 1, // FIXME
                            robot_name: vehicle.name.clone().unwrap_or_else(|| db_vehicle.name.clone()),
                            robot_map: db_vehicle.robot_data,
                            robot_uuid: uuid_str,
                            cpu: cpu_count,
                            weapon_order: oj_rc_database::schema::parse_int_csv(&db_vehicle.weapon_order).into_iter().map(|x| x as i32).collect::<Vec<_>>(),
                            colour_map: db_vehicle.colour_data,
                            spawn_effect: db_vehicle.spawn_animation_id,
                            death_effect: db_vehicle.death_animation_id,
                            weapon_rank: oj_rc_database::schema::parse_int_csv(&db_vehicle.weapon_order).into_iter().map(|x| (x as i32, if x == 0 { 0 } else { 1 })).collect(),
                        })
                    },
                    Ok(None) => {
                        log::error!("Prefab vehicle {} does not exist in main garage database", garage);
                        Err(polariton_server::operations::SimpleOpError::with_message(
                            crate::data::error_codes::SingleplayerErrorCode::UnexpectedError as i16,
                            format!("Prefab vehicle {} does not exist in main garage database", garage),
                        ))
                    }
                    Err(e) => {
                        log::error!("Failed to retrieve prefab vehicle {} from main garage database: {}", garage, e);
                        Err(polariton_server::operations::SimpleOpError::with_message(
                            crate::data::error_codes::SingleplayerErrorCode::DatabaseError as i16,
                            format!("Failed to retrieve prefab vehicle {} from main garage database: {}", garage, e),
                        ))
                    }
                }
            },
            crate::persist::config::VehicleDescriptor::Raw {
                cube_data,
                colour_data,
            } => {
                let sha_bytes = sha2::Sha256::digest(cube_data);
                let u32_bytes = [
                    sha_bytes[0],
                    sha_bytes[1],
                    sha_bytes[2],
                    sha_bytes[3],
                ];
                let uuid_i64 = crate::persist::user::uuid_sanitize(
                    standard_uuid_uniqueness
                    ^ crate::persist::user::i64_join((1 << 29, u32::from_le_bytes(u32_bytes)))
                );
                let uuid_str = crate::persist::user::i64_as_uuid_str(uuid_i64);
                let weapons_guess = weapon_order.guess_weapons(&mut std::io::Cursor::new(&cube_data));
                let weapons_guess = vec![
                    weapons_guess.first().copied().unwrap_or(0),
                    weapons_guess.get(1).copied().unwrap_or(0),
                    weapons_guess.get(2).copied().unwrap_or(0),
                ];
                let weapon_ranks = weapons_guess.iter().map(|&x| (x, if x == 0 { 0 } else { 1 })).collect();
                let cpu_counts = cpu_counter.calculate_cpu(&mut std::io::Cursor::new(&cube_data));
                Ok(super::ResolvedVehicle {
                    mastery: 1,
                    tier: 1, // FIXME
                    robot_name: vehicle.name.clone().unwrap_or_else(|| "Raw Robot".to_owned()),
                    robot_map: cube_data.to_owned(),
                    robot_uuid: uuid_str,
                    cpu: cpu_counts.total as i32,
                    weapon_order: weapons_guess,
                    colour_map: colour_data.to_owned(),
                    spawn_effect: "Spawn".to_owned(),
                    death_effect: "Explosion".to_owned(),
                    weapon_rank: weapon_ranks,
                })
            }
        }
    }

    async fn resolve_some_singleplayer_vehicles(&self, factory: &dyn oj_rc_factory::VehicleFactoryAdapter, weapon_order: &crate::cubes::WeaponListParser, singleplayer_config: &crate::persist::config::SingleplayerConfig, cpu_counter: &crate::cubes::CpuListParser) -> Result<Vec<crate::data::player_data::PlayerData>, i16> {
        use rand::seq::IndexedRandom;
        let mut players = Vec::with_capacity((singleplayer_config.max_enemies + singleplayer_config.max_teammates + 1) as usize);
        let mut next_id = 0;
        let mut seen_usernames = std::collections::HashSet::<String>::new();
        #[allow(clippy::explicit_counter_loop)] // this is really bad to read with this suggested refactor
        for i in 0..(singleplayer_config.max_enemies + singleplayer_config.max_teammates) {
            let vehicle = singleplayer_config.vehicles.choose(&mut rand::rng())
                .ok_or(crate::data::error_codes::SingleplayerErrorCode::UnexpectedError as i16)?;
            let current_id = next_id;
            let uuid_i64 = crate::persist::user::uuid_sanitize(crate::persist::user::i64_join((i32::MAX as u32, current_id)));
            let uuid_str = crate::persist::user::i64_as_uuid_str(uuid_i64);
            let username = if seen_usernames.contains(&vehicle.username) {
                let mut username = vehicle.username.clone();
                while seen_usernames.contains(&username) {
                    username = format!("{}{}", username, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].choose(&mut rand::rng()).unwrap());
                }
                username
            } else {
                vehicle.username.clone()
            };
            seen_usernames.insert(username.clone());
            let team_num = if i < singleplayer_config.max_enemies { 1 } else { 0 };
            let enemy_vehicle = self.resolve_vehicle(vehicle, factory, weapon_order, cpu_counter).await?;
            let weapons = vec![enemy_vehicle.weapon_order[0], 0, 0];
            let weapon_ranks = weapons.iter().map(|&x| (x, if x == 0 { 0 } else { 1 })).collect();
            let enemy = crate::data::player_data::PlayerData {
                name: username.clone(),
                display_name: username.clone(),
                mastery: enemy_vehicle.mastery,
                tier: enemy_vehicle.tier,
                robot_name: enemy_vehicle.robot_name,
                robot_map: enemy_vehicle.robot_map,
                group: None,
                team: team_num,
                has_premium: false,
                robot_uuid: uuid_str,
                cpu: enemy_vehicle.cpu,
                avatar_id: None, // not serialised
                weapon_order: weapons,
                colour_map: enemy_vehicle.colour_map,
                is_ai: true,
                spawn_effect: enemy_vehicle.spawn_effect,
                death_effect: enemy_vehicle.death_effect,
                player_rank: 1,
                weapon_rank: weapon_ranks,
                clan_name: None,
            };
            next_id += 1;
            players.push(enemy);
        }
        Ok(players)
    }

    #[inline]
    fn currency_ty_to_db(ty: super::CurrencyType) -> oj_rc_database::schema::user_aux::Descriptor {
        match ty {
            super::CurrencyType::Free => oj_rc_database::schema::user_aux::Descriptor::UserFreeCurrency,
            super::CurrencyType::Paid => oj_rc_database::schema::user_aux::Descriptor::UserPaidCurrency,
            super::CurrencyType::TechPoints => oj_rc_database::schema::user_aux::Descriptor::TechPoints,
            super::CurrencyType::Experience => oj_rc_database::schema::user_aux::Descriptor::UserXP,
        }
    }

    async fn currency_sub_checked(&self, ty: super::CurrencyType, to_sub: u64) -> Result<bool, oj_rc_database::sea_orm::DbErr> {
        let desc = Self::currency_ty_to_db(ty);
        let model_opt = self.db.user_aux_by_user_id_and_descriptor(self.account.id, desc.clone()).await?;
        if let Some(model) = model_opt {
            let existing_funds = model.data.parse::<u64>().unwrap_or_default();
            if existing_funds < to_sub {
                Ok(false)
            } else {
                let mut active = model.into_active_model();
                active.data = oj_rc_database::sea_orm::ActiveValue::Set((existing_funds - to_sub).to_string());
                self.db.update_user_aux_by_user_id_and_descriptor(active, self.account.id, desc).await?;
                Ok(true)
            }
        } else {
            Ok(false)
        }
    }

    pub(super) async fn currency_op(&self, ty: super::CurrencyType, op: super::CurrencyOp) -> Result<u64, oj_rc_database::sea_orm::DbErr> {
        let desc = Self::currency_ty_to_db(ty);
        let model_opt = match op {
            super::CurrencyOp::Get => {
                self.db.update_user_aux_by_user_id_and_descriptor_custom(
                    self.account.id,
                    desc.clone(),
                    |_model| None
                ).await?
            },
            super::CurrencyOp::Add(to_add) => {
                self.db.update_user_aux_by_user_id_and_descriptor_custom(
                    self.account.id,
                    desc.clone(),
                    move |model| {
                        let new_currency = model.data.parse::<u64>().unwrap_or_default() + to_add;
                        let mut am = model.to_owned().into_active_model();
                        am.data = oj_rc_database::sea_orm::ActiveValue::Set(new_currency.to_string());
                        Some(am)
                    }
                ).await?
            },
            super::CurrencyOp::Sub(to_sub) => {
                self.db.update_user_aux_by_user_id_and_descriptor_custom(
                    self.account.id,
                    desc.clone(),
                    move |model| {
                        let new_currency = model.data.parse::<u64>().unwrap_or_default().saturating_sub(to_sub);
                        let mut am = model.to_owned().into_active_model();
                        am.data = oj_rc_database::sea_orm::ActiveValue::Set(new_currency.to_string());
                        Some(am)
                    }
                ).await?
            },
            super::CurrencyOp::AddSub(to_addsub) => {
                self.db.update_user_aux_by_user_id_and_descriptor_custom(
                    self.account.id,
                    desc.clone(),
                    move |model| {
                        let new_currency = (model.data.parse::<u64>().unwrap_or_default() as i64) + to_addsub;
                        let mut am = model.to_owned().into_active_model();
                        am.data = oj_rc_database::sea_orm::ActiveValue::Set(new_currency.clamp(0, i64::MAX).to_string());
                        Some(am)
                    }
                ).await?
            },
        };
        let num: u64 = if let Some(model) = model_opt {
            model.data.parse().unwrap_or_default()
        } else {
            log::warn!("No {:?} user_aux found for user {}", desc, self.account.id);
            0
        };
        Ok(num)
    }
}

const INVALID_ROBOT_ERR: i16 = crate::data::error_codes::WebServicesError::InvalidRobot as i16; // 140
const DATABASE_ERR: i16 = crate::data::error_codes::WebServicesError::DatabaseError as i16; // 8
const UNEXPECTED_ERR: i16 = crate::data::error_codes::WebServicesError::UnexpectedError as i16; // 9

#[async_trait::async_trait]
impl <C: Clone + Send> super::User<C> for UserData {
    async fn unlocked_parts(&self) -> Vec<u32> {
        match self.db.user_aux_by_user_id_and_descriptor(self.account.id, oj_rc_database::schema::user_aux::Descriptor::UnlockedParts).await {
            Ok(Some(parts)) => {
                match serde_json::from_str::<super::inventory::UnlockedParts>(&parts.data) {
                    Ok(json) => {
                        match json.override_ {
                            super::inventory::UnlockOverride::Normal => json.unlocked.clone(),
                            super::inventory::UnlockOverride::UnlockNone => Vec::default(),
                            super::inventory::UnlockOverride::UnlockAll => self.cubes.as_ref().to_owned(),
                        }
                    },
                    Err(e) => {
                        log::error!("Failed to deserialize Descriptor::UnlockedParts for user_id {}: {}", self.account.id, e);
                        Vec::default()
                    }
                }
            },
            Ok(None) => Vec::default(),
            Err(e) => {
                log::error!("Failed to retrieve Descriptor::UnlockedParts for user_id {}: {}", self.account.id, e);
                Vec::default()
            }
        }
    }

    async fn unlock_parts(&self, parts: &[u32]) -> Result<(), polariton_server::operations::SimpleOpError> {
        let parts_row = self.db.user_aux_by_user_id_and_descriptor(self.account.id, oj_rc_database::schema::user_aux::Descriptor::UnlockedParts).await
            .map_err(|e| {
                log::error!("Failed to retrieve UnlockedParts to unlock parts for user {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::WebServicesError::DatabaseError as i16,
                    format!("Failed to retrieve UnlockedParts to unlock parts: {}", e),
                )
            })?
            .ok_or_else(|| {
                log::error!("Failed to find UnlockedParts to unlock parts for user {}", self.account.id);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::WebServicesError::DatabaseError as i16,
                    "Failed to find UnlockedParts to unlock parts".to_owned(),
                )
            })?;
        let mut unlocked_parts = serde_json::from_str::<super::inventory::UnlockedParts>(&parts_row.data)
            .map_err(|e| {
                log::error!("Failed to deserialize UnlockedParts to unlock parts for user {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::WebServicesError::DatabaseError as i16,
                    format!("Failed to deserialize UnlockedParts to unlock parts: {}", e),
                )
            })?;
        for new_part in parts {
            unlocked_parts.unlocked.push(*new_part);
        }
        let mut parts_row = parts_row.into_active_model();
        let parts_json = serde_json::to_string(&unlocked_parts)
            .map_err(|e| {
                log::error!("Failed to serialize UnlockedParts to unlock parts for user {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::WebServicesError::DatabaseError as i16,
                    format!("Failed to serialize UnlockedParts to unlock parts: {}", e),
                )
            })?;
        parts_row.data = oj_rc_database::sea_orm::ActiveValue::Set(parts_json);
        self.db.update_user_aux_by_user_id_and_descriptor(
            parts_row,
            self.account.id,
            oj_rc_database::schema::user_aux::Descriptor::UnlockedParts
        ).await
            .map_err(|e| {
                log::error!("Failed to update UnlockedParts to unlock parts for user {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::WebServicesError::DatabaseError as i16,
                    format!("Failed to update UnlockedParts to unlock parts: {}", e),
                )
            })?;
        Ok(())
    }

    async fn selected_garage(&self) -> (String, u32) {
        match self.db.garage_selected(self.account.id).await {
            Ok(Some(selected)) => (super::i64_as_uuid_str(selected.uuid), selected.slot as u32),
            Ok(None) => {
                log::warn!("User {} does not have a selected garage", self.account.id);
                ("0_0".to_owned(), 0)
            },
            Err(e) => {
                log::error!("Failed to retrieve selected garage for user_id {}: {}", self.account.id, e);
                ("0_0".to_owned(), 0)
            }
        }
    }

    async fn select_garage(&self, slot: i32) -> Result<(), i16> {
        self.err_on_banned().await?;
        self.db.update_garage_selected_by_user_id_and_slot(self.account.id, slot).await.map_err(|e| {
            log::error!("Failed to select vehicle slot {} user_id {}: {}", slot, self.account.id, e);
            DATABASE_ERR
        })
    }

    async fn selected_vehicle_data(&self) -> Result<super::VehicleData, polariton_server::operations::SimpleOpError> {
        let garage = self.db.garage_selected(self.account.id).await
            .map_err(|e| {
                log::error!("Failed to retrieve selected garage data for user_id {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::WebServicesError::DatabaseError as i16,
                    "Failed to retrieve selected garage data for user".to_owned(),
                )
            })?
            .ok_or_else(|| {
                log::error!("No selected garage data for user_id {}", self.account.id);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::WebServicesError::UnexpectedError as i16,
                    "No selected garage data for user".to_owned(),
                )
            })?;
        Ok(super::VehicleData {
            name: Some(garage.name),
            slot: garage.slot,
            robot_data: garage.robot_data,
            colour_data: garage.colour_data,
            weapon_order: oj_rc_database::schema::parse_int_csv(&garage.weapon_order)
                .into_iter()
                .map(|x| x as i32)
                .collect(),
            crf_id: garage.crf_id,
            was_rated: Some(garage.was_rated),
        })
    }

    async fn all_slots(&self) -> super::UserSlots<C> {
        let slots = match self.all_vehicles().await {
            Ok(slots) => slots,
            Err(e) => {
                log::error!("Failed to load all vehicles: {}", e);
                Vec::default()
            }
        };
        let slot_order = match self.db.user_aux_by_user_id_and_descriptor(self.account.id, oj_rc_database::schema::user_aux::Descriptor::GarageSlotOrder).await{
            Ok(Some(slot_order_db)) => {
                let slots = serde_json::from_str::<Vec<u32>>(&slot_order_db.data).unwrap_or_default();
                polariton::operation::Typed::ObjArr(slots.iter().map(|slot| polariton::operation::Typed::Int(*slot as _)).collect::<Vec<_>>().into())
            },
            Ok(None) => {
                log::error!("No vehicle slot order for user_id {}", self.account.id);
                polariton::operation::Typed::ObjArr(slots.iter().map(|slot| polariton::operation::Typed::Int(slot.slot as _)).collect::<Vec<_>>().into())
            },
            Err(e) => {
                log::error!("Failed to get vehicle slot order for user_id {}: {}", self.account.id, e);
                polariton::operation::Typed::ObjArr(slots.iter().map(|slot| polariton::operation::Typed::Int(slot.slot as _)).collect::<Vec<_>>().into())
            },
        };
        //let slot_order = polariton::operation::Typed::ObjArr(slots.iter().map(|slot| polariton::operation::Typed::Int(slot.slot as _)).collect::<Vec<_>>().into());
        let slot_info = polariton::operation::Typed::Dict(polariton::operation::Dict {
            key_ty: polariton::serdes::TypePrefix::Int,
            val_ty: polariton::serdes:: TypePrefix::HashMap,
            items: slots.into_iter().map(|slot| {
                let slot_index = slot.slot;
                let garage_data: crate::data::garage_bay::GarageSlotInfo = crate::persist::garage::db_into_data(slot);
                (polariton::operation::Typed::Int(slot_index as _), garage_data.as_transmissible())
            }).collect(),
        });
        super::UserSlots {
            slot_info, slot_order,
        }
    }

    async fn slot_by_id(&self, id: i32) -> Result<crate::persist::user::UserSlotData<C>, i16> {
        self.err_on_banned().await?;
        match self.load_garage_by_slot(id as _).await {
            Ok(Some(slot)) => {
                let cube_count = slot.cube_count() as i32;
                let control_ty: crate::data::garage_bay::ControlType = crate::persist::garage::control_ty_into_data(slot.control_type);
                let control_options = crate::data::garage_bay::ControlOptions {
                    vertical_strafing: slot.vertical_strafing,
                    sideways_driving: slot.sideways_driving,
                    tracks_turn_on_spot: slot.tracks_turn_on_spot,
                };
                Ok(crate::persist::user::UserSlotData {
                    data: polariton::operation::Typed::Bytes(slot.robot_data.into()),
                    colour_data: polariton::operation::Typed::Bytes(slot.colour_data.into()),
                    cube_count: polariton::operation::Typed::Int(cube_count),
                    weapon_order: polariton::operation::Typed::IntArr(oj_rc_database::schema::parse_int_csv(&slot.weapon_order).into_iter().map(|x| x as i32).collect::<Vec<_>>().into()),
                    movement_categories: polariton::operation::Typed::IntArr(oj_rc_database::schema::parse_int_csv(&slot.movement_categories).into_iter().map(|x| x as i32).collect::<Vec<_>>().into()),
                    control_type: polariton::operation::Typed::Int(control_ty as _),
                    control_options: control_options.as_transmissible(),
                    mastery_level: polariton::operation::Typed::Int(slot.mastery_level),
                    robot_rank: polariton::operation::Typed::Int(slot.total_robot_ranking as _),
                    cpu: polariton::operation::Typed::Int(slot.total_robot_cpu as _),
                    cosmetic_cpu: polariton::operation::Typed::Int(slot.total_cosmetic_cpu as _),
                    uuid: polariton::operation::Typed::Str(super::i64_as_uuid_str(slot.uuid).into()),
                })
            },
            Ok(None) => {
                log::error!("Failed to find vehicle slot {} for user_id {}", id, self.account.id);
                Err(DATABASE_ERR)
            }
            Err(e) => {
                log::error!("Failed to retrieve vehicle {} for user_id {}: {}", id, self.account.id, e);
                Err(INVALID_ROBOT_ERR)
            }
        }
    }

    async fn save_slot(&self, vehicle: crate::persist::user::VehicleData, cpu_counter: &crate::cubes::CpuListParser) -> Result<(), i16> {
        self.err_on_banned().await?;
        let cpu_counts = cpu_counter.calculate_cpu(&mut std::io::Cursor::new(&vehicle.robot_data));
        let entity = oj_rc_database::schema::garage::ActiveModel {
            weapon_order: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::dump_csv(&vehicle.weapon_order)),
            robot_data: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.robot_data),
            colour_data: oj_rc_database::sea_orm::ActiveValue::Set(vehicle.colour_data),
            crf_id: if let Some(crf_id) = vehicle.crf_id { oj_rc_database::sea_orm::ActiveValue::Set(Some(crf_id)) } else { Default::default() },
            name: if let Some(new_name) = vehicle.name { oj_rc_database::sea_orm::ActiveValue::Set(new_name) } else { Default::default() },
            total_robot_cpu: oj_rc_database::sea_orm::ActiveValue::Set(cpu_counts.total as _),
            total_cosmetic_cpu: oj_rc_database::sea_orm::ActiveValue::Set(cpu_counts.cosmetic as _),
            was_rated: if let Some(is_rated) = vehicle.was_rated { oj_rc_database::sea_orm::ActiveValue::Set(is_rated) } else { oj_rc_database::sea_orm::ActiveValue::NotSet },
            ..Default::default()
        };
        self.save_garage_by_slot(entity, vehicle.slot).await.map_err(|e| {
            log::error!("Failed to save vehicle slot {} for user_id {}: {}", vehicle.slot, self.account.id, e);
            DATABASE_ERR
        })?;
        Ok(())
    }

    async fn save_slot_order(&self, slots: Vec<i32>) -> Result<(), i16> {
        self.err_on_banned().await?;
        let slots: Vec<u32> = slots.into_iter().map(|x| x as u32).collect();
        let entity = oj_rc_database::schema::user_aux::ActiveModel {
            data: oj_rc_database::sea_orm::ActiveValue::Set(serde_json::to_string_pretty(&slots).unwrap()),
            ..Default::default()
        };
        self.db.update_user_aux_by_user_id_and_descriptor(entity, self.account.id, oj_rc_database::schema::user_aux::Descriptor::GarageSlotOrder).await.map_err(|e| {
            log::error!("Failed to update garage slot order for user_id {}: {}", self.account.id, e);
            DATABASE_ERR
        })?;
        Ok(())
    }

    async fn new_slot(&self, reset_slot: Option<i32>) -> Result<super::NewSlotData<C>, i16> {
        self.err_on_banned().await?;
        let model = if let Some(slot) = reset_slot {
            let new_data = super::initial_data::default_reset_slot();
            if let Some(reset_g) = self.db.update_garage_by_user_id_and_slot(new_data, self.account.id, slot).await.map_err(|e| {
                log::error!("Failed to reset vehicle slot {} for user_id {}: {}", slot, self.account.id, e);
                DATABASE_ERR
            })? {
                reset_g
            } else {
                log::warn!("No vehicle slot {} to reset for user_id {}, creating new slot", slot, self.account.id);
                return self.new_slot(None).await;
            }
        } else {
            let max_slot = self.db.garage_max_slot_by_user_id(self.account.id).await.map_err(|e| {
                log::error!("Failed to get max slot for user_id {}: {}", self.account.id, e);
                DATABASE_ERR
            })?;
            let next_slot = max_slot + 1;
            let new_data = super::initial_data::default_new_slot(self.account.id, next_slot, 2_000);
            self.db.insert_garage(new_data).await.map_err(|e| {
                log::error!("Failed to create new vehicle slot {} for user_id {}: {}", next_slot, self.account.id, e);
                DATABASE_ERR
            })?
        };
        let split_uuid = super::i64_split(model.uuid);
        Ok(super::NewSlotData {
            name: polariton::operation::Typed::Str(model.name.into()),
            uuid_0: polariton::operation::Typed::Str(split_uuid.0.to_string().into()), // yes, seriously
            uuid_1: polariton::operation::Typed::Str(split_uuid.1.to_string().into()), // also yes, seriously
            slot: polariton::operation::Typed::Int(model.slot as _),
            bay_cpu: polariton::operation::Typed::Int(model.bay_cpu as _),
            mastery_level: polariton::operation::Typed::Int(model.mastery_level as _),
            slot_i: model.slot as _,
        })
    }

    async fn copy_slot(&self, slot: i32, into_slot: Option<i32>, append: &str) -> Result<(), i16> {
        let slot_to_copy = self.db.garage_by_user_id_and_slot(self.account.id, slot).await.map_err(|e| {
            log::error!("Failed to retrieve vehicle slot {} to copy for user_id {}: {}", slot, self.account.id, e);
            DATABASE_ERR
        })?.ok_or_else(|| {
            log::error!("No vehicle slot {} to copy for user_id {}", slot, self.account.id);
            UNEXPECTED_ERR
        })?;
        let vehicle_cpu = slot_to_copy.total_robot_cpu as u32;
        let minimum_upgrade_to_cpu = self.garage_upgrades.increments.iter()
            .find(|x| x.cpu >= vehicle_cpu)
            .map(|x| x.cpu)
            .unwrap_or(vehicle_cpu); // increments probably misconfigured
        log::info!("Minimum upgrade to bay CPU of {} (vehicle CPU {})", minimum_upgrade_to_cpu, vehicle_cpu);
        let new_name = format!("{} {}", slot_to_copy.name, append);
        let new_slot = if let Some(existing_slot) = into_slot {
            log::info!("Copy slot {} -> {} as `{}`", slot, existing_slot, new_name);
            if let Some(existing_g) = self.db.garage_by_user_id_and_slot(self.account.id, existing_slot).await.map_err(|e| {
                log::error!("Failed to retrieve vehicle slot {} for user_id {}: {}", slot, self.account.id, e);
                DATABASE_ERR
            })? {
                use oj_rc_database::sea_orm::IntoActiveModel;
                let existing_bay_cpu = existing_g.bay_cpu as u32;
                let mut to_update = existing_g.into_active_model();
                // subtract bay upgrade cost from user free currency
                let total_cost: u32 = self.garage_upgrades.increments.iter()
                    .map(|x| if x.cpu <= existing_bay_cpu || x.cpu > minimum_upgrade_to_cpu { 0 } else { x.cost })
                    .sum();
                self.currency_sub_checked(super::CurrencyType::Free, total_cost as u64).await.map_err(|e| {
                    log::error!("Failed to debit user for cpu upgrade during clone of {} to slot {} for user_id {}: {}", slot, existing_slot, self.account.id, e);
                    DATABASE_ERR
                })?;
                // copy everything except id, user_id, creation_time, slot, was_rated, bay_cpu, mastery_level, selected
                to_update.name = oj_rc_database::sea_orm::ActiveValue::Set(new_name);
                to_update.crf_id = oj_rc_database::sea_orm::ActiveValue::Set(slot_to_copy.crf_id);
                to_update.movement_categories = oj_rc_database::sea_orm::ActiveValue::Set(slot_to_copy.movement_categories);
                to_update.total_robot_cpu = oj_rc_database::sea_orm::ActiveValue::Set(slot_to_copy.total_robot_cpu);
                to_update.total_cosmetic_cpu = oj_rc_database::sea_orm::ActiveValue::Set(slot_to_copy.total_cosmetic_cpu);
                to_update.total_robot_ranking = oj_rc_database::sea_orm::ActiveValue::Set(slot_to_copy.total_robot_ranking);
                to_update.tutorial_robot = oj_rc_database::sea_orm::ActiveValue::Set(false);
                to_update.starter_robot_index = oj_rc_database::sea_orm::ActiveValue::Set(None);
                to_update.control_type = oj_rc_database::sea_orm::ActiveValue::Set(slot_to_copy.control_type);
                to_update.vertical_strafing = oj_rc_database::sea_orm::ActiveValue::Set(slot_to_copy.vertical_strafing);
                to_update.sideways_driving = oj_rc_database::sea_orm::ActiveValue::Set(slot_to_copy.sideways_driving);
                to_update.tracks_turn_on_spot = oj_rc_database::sea_orm::ActiveValue::Set(slot_to_copy.tracks_turn_on_spot);
                to_update.bay_skin_id = oj_rc_database::sea_orm::ActiveValue::Set(slot_to_copy.bay_skin_id);
                to_update.death_animation_id = oj_rc_database::sea_orm::ActiveValue::Set(slot_to_copy.death_animation_id);
                to_update.spawn_animation_id = oj_rc_database::sea_orm::ActiveValue::Set(slot_to_copy.spawn_animation_id);
                to_update.weapon_order = oj_rc_database::sea_orm::ActiveValue::Set(slot_to_copy.weapon_order);
                to_update.robot_data = oj_rc_database::sea_orm::ActiveValue::Set(slot_to_copy.robot_data);
                to_update.colour_data = oj_rc_database::sea_orm::ActiveValue::Set(slot_to_copy.colour_data);
                self.db.update_garage(to_update).await.map_err(|e| {
                    log::error!("Failed to update garage slot {} copied from {} for user_id {}: {}", existing_slot, slot, self.account.id, e);
                    DATABASE_ERR
                })?;
                existing_slot
            } else {
                log::warn!("No existing vehicle slot {} for user_id {}, copying to new slot", slot, self.account.id);
                return <Self as super::User<C>>::copy_slot(self, slot, None, append).await;
            }
        } else {
            log::info!("Copy slot {} -> <new slot> as `{}`", slot, new_name);
            use oj_rc_database::sea_orm::IntoActiveModel;
            // subtract bay upgrade cost from user free currency
            let total_cost: u32 = self.garage_upgrades.increments.iter()
                .map(|x| if x.cpu > minimum_upgrade_to_cpu { 0 } else { x.cost })
                .sum();
            log::debug!("Bay CPU upgrade costs {}", total_cost);
            self.currency_sub_checked(super::CurrencyType::Free, total_cost as u64).await.map_err(|e| {
                log::error!("Failed to debit user for cpu upgrade during fresh clone to slot {} for user_id {}: {}", slot, self.account.id, e);
                DATABASE_ERR
            })?;
            let mut to_insert = slot_to_copy.into_active_model();
            let max_slot = self.db.garage_max_slot_by_user_id(self.account.id).await.map_err(|e| {
                log::error!("Failed to get max garage slot during copy for user_id {}: {}", self.account.id, e);
                DATABASE_ERR
            })?;
            let now = chrono::Utc::now().timestamp();
            let uuid = super::uuid_sanitize(now);
            to_insert.id = Default::default();
            to_insert.creation_time = oj_rc_database::sea_orm::ActiveValue::Set(now);
            to_insert.uuid = oj_rc_database::sea_orm::ActiveValue::Set(uuid);
            to_insert.slot = oj_rc_database::sea_orm::ActiveValue::Set((max_slot + 1) as i32);
            to_insert.name = oj_rc_database::sea_orm::ActiveValue::Set(new_name);
            self.db.insert_garage(to_insert).await.map_err(|e| {
                log::error!("Failed to insert garage slot copied from {} for user_id {}: {}", slot, self.account.id, e);
                DATABASE_ERR
            })?;
            max_slot + 1
        };
        self.db.update_garage_selected_by_user_id_and_slot(self.account.id, new_slot).await.map_err(|e| {
            log::error!("Failed to select copied garage slot {} for user_id {}: {}", new_slot, self.account.id, e);
            DATABASE_ERR
        })?;
        Ok(())
    }

    async fn upgrade_slot(&self, increments: i32) -> Result<polariton::operation::Typed<C>, i16> {
        self.err_on_banned().await?;
        if increments <= 0 {
            // no-op
            return Ok(polariton::operation::Typed::Bool(true));
        }
        let selected_slot = self.db.garage_selected(self.account.id).await.map_err(|e| {
            log::error!("Failed to retrieve selected vehicle slot for user_id {}: {}", self.account.id, e);
            DATABASE_ERR
        })?.ok_or_else(|| {
            log::error!("No selected vehicle slot for user_id {}", self.account.id);
            DATABASE_ERR
        })?;
        let inc_opt = self.garage_upgrades.increments.iter().enumerate().rfind(|(_i, inc)| inc.cpu <= selected_slot.bay_cpu as u32);
        if let Some((i, _)) = inc_opt {
            let max_upgrade = self.garage_upgrades.increments.len() - 1;
            let upgrade_to = i + (increments as usize);
            if upgrade_to > max_upgrade {
                // over-upgraded
                Ok(polariton::operation::Typed::Bool(false))
            } else {
                let upgrade_to_cpu = self.garage_upgrades.increments[upgrade_to].cpu;
                // subtract bay cpu upgrade cost from user free currency
                let total_cost: u32 = self.garage_upgrades.increments[i+1..=upgrade_to].iter().map(|x| x.cost).sum();
                self.currency_sub_checked(super::CurrencyType::Free, total_cost as u64).await.map_err(|e| {
                    log::error!("Failed to debit user for cpu upgrade of {} to selected vehicle slot for user_id {}: {}", upgrade_to_cpu, self.account.id, e);
                    DATABASE_ERR
                })?;
                // apply upgrade to bay
                let entity = oj_rc_database::schema::garage::ActiveModel {
                    bay_cpu: oj_rc_database::sea_orm::ActiveValue::Set(upgrade_to_cpu as i32),
                    ..Default::default()
                };
                self.db.update_garage_by_user_id_and_slot(entity, self.account.id, selected_slot.slot).await.map_err(|e| {
                    log::error!("Failed to upgrade selected vehicle slot to bay cpu of {} for user_id {}: {}", upgrade_to_cpu, self.account.id, e);
                    DATABASE_ERR
                })?;
                Ok(polariton::operation::Typed::Bool(true))
            }
        } else {
            // probably a bad/changed garage update config
            log::warn!("No vehicle slot upgrade found for bay CPU {} for user_id {}", selected_slot.bay_cpu, self.account.id);
            Ok(polariton::operation::Typed::Bool(false))
        }
    }

    async fn save_slot_controls(&self, controls: super::ControlData) -> Result<(), i16> {
        self.err_on_banned().await?;
        let entity = oj_rc_database::schema::garage::ActiveModel {
            control_type: oj_rc_database::sea_orm::ActiveValue::Set(controls.control_ty.into_db()),
            vertical_strafing: oj_rc_database::sea_orm::ActiveValue::Set(controls.vertical_strafing),
            sideways_driving: oj_rc_database::sea_orm::ActiveValue::Set(controls.sideways_driving),
            tracks_turn_on_spot: oj_rc_database::sea_orm::ActiveValue::Set(controls.tracks_turn_on_spot),
            ..Default::default()
        };
        self.save_garage_by_slot(entity, controls.slot).await.map_err(|e| {
            log::error!("Failed to save controls for slot {} for user_id {}: {}", controls.slot, self.account.id, e);
            DATABASE_ERR
        })?;
        Ok(())
    }

    async fn save_slot_customisations(&self, customs: super::CustomisationData) -> Result<(), i16> {
        self.err_on_banned().await?;
        if let Some(uuid) = super::str_to_i64(&customs.uuid) {
            let entity = oj_rc_database::schema::garage::ActiveModel {
                bay_skin_id: oj_rc_database::sea_orm::ActiveValue::Set(customs.bay),
                spawn_animation_id: oj_rc_database::sea_orm::ActiveValue::Set(customs.spawn),
                death_animation_id: oj_rc_database::sea_orm::ActiveValue::Set(customs.death),
                ..Default::default()
            };
            self.db.update_garage_by_uuid(entity, uuid).await.map_err(|e| {
                log::error!("Failed to save customisations for garage {} for user_id {}: {}", uuid, self.account.id, e);
                DATABASE_ERR
            })?;
        }
        Ok(())
    }

    async fn get_slot_customisations(&self, uuid: &str) -> Result<super::GetCustomisationData<C>, i16> {
        if let Some(uuid) = super::str_to_i64(uuid) {
            let garage_opt = self.db.garage_by_uuid(uuid).await.map_err(|e| {
                log::error!("Failed to retrieve garage {} for user_id {}: {}", uuid, self.account.id, e);
                DATABASE_ERR
            })?;
            if let Some(garage) = garage_opt {
                Ok(super::GetCustomisationData {
                    bay: polariton::operation::Typed::Str(garage.bay_skin_id.into()),
                    spawn: polariton::operation::Typed::Str(garage.spawn_animation_id.into()),
                    death: polariton::operation::Typed::Str(garage.death_animation_id.into()),
                })
            } else {
                Err(DATABASE_ERR)
            }
        } else {
            Err(UNEXPECTED_ERR)
        }
    }

    async fn set_slot_name(&self, slot: i32, name: String) -> Result<(), i16> {
        let to_save = oj_rc_database::schema::garage::ActiveModel {
            name: oj_rc_database::sea_orm::ActiveValue::Set(name),
            ..Default::default()
        };
        self.db.update_garage_by_user_id_and_slot(to_save, self.account.id, slot).await.map_err(|e| {
            log::error!("Failed to save garage {} for user_id {}: {}", slot, self.account.id, e);
            DATABASE_ERR
        })?;
        Ok(())
    }

    fn signup_date(&self) -> i64 {
        super::since_windows_epoch(self.account.creation_time)
    }

    async fn singleplayer_robots(&self, factory: &dyn oj_rc_factory::VehicleFactoryAdapter, weapon_order: &crate::cubes::WeaponListParser, singleplayer_config: &crate::persist::config::SingleplayerConfig, cpu_counter: &crate::cubes::CpuListParser) ->  Result<polariton::operation::Typed<C>, i16> {
        //self.err_on_banned().await?;
        let mut vehicles = self.resolve_some_singleplayer_vehicles(factory, weapon_order, singleplayer_config, cpu_counter).await?;
        let user_bot = self.user_player_data(cpu_counter).await?;

        // real user MUST be last
        vehicles.push(user_bot);
        Ok(crate::data::player_data::PlayerDatas {
            players: vehicles,
        }.as_transmissible())
    }

    async fn last_seen(&self) -> Result<u64, i16> {
        self.err_on_banned().await?;
        let last_seen_aux_opt = self.db.user_aux_by_user_id_and_descriptor(self.account.id, oj_rc_database::schema::user_aux::Descriptor::LastSeen).await
            .map_err(|e| {
                log::error!("Failed to retrieve LastSeen (user_aux) for user_id {}: {}", self.account.id, e);
                DATABASE_ERR
            })?;
        let now = chrono::Utc::now().timestamp();
        if let Some(last_seen) = last_seen_aux_opt {
            let to_update = oj_rc_database::schema::user_aux::ActiveModel {
                data: oj_rc_database::sea_orm::ActiveValue::Set((now as u64).to_string()),
                ..Default::default()
            };
            self.db.update_user_aux_by_user_id_and_descriptor(to_update, self.account.id, oj_rc_database::schema::user_aux::Descriptor::LastSeen).await
                .map_err(|e| {
                    log::error!("Failed to update LastSeen (user_aux) for user_id {}: {}", self.account.id, e);
                    DATABASE_ERR
                })?;
            Ok(last_seen.data.parse().unwrap_or_default())
        } else {
            let to_insert = oj_rc_database::schema::user_aux::ActiveModel {
                user_id: oj_rc_database::sea_orm::ActiveValue::Set(self.account.id),
                creation_time: oj_rc_database::sea_orm::ActiveValue::Set(now),
                descriptor: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::user_aux::Descriptor::LastSeen),
                data: oj_rc_database::sea_orm::ActiveValue::Set((now as u64).to_string()),
                ..Default::default()
            };
            self.db.insert_user_aux(vec![to_insert]).await.map_err(|e| {
                log::error!("Failed to insert LastSeen (user_aux) for user_id {}: {}", self.account.id, e);
                DATABASE_ERR
            })?;
            Ok(0)
        }
    }

    async fn get_avatar_info(&self) -> Result<super::GetAvatarInfo<C>, i16> {
        self.err_on_banned().await?;
        let avatar_id_aux = self.db.user_aux_by_user_id_and_descriptor(self.account.id, oj_rc_database::schema::user_aux::Descriptor::AvatarId).await
            .map_err(|e| {
                log::error!("Failed to retrieve AvatarId (user_aux) for user_id {}: {}", self.account.id, e);
                DATABASE_ERR
            })?
            .ok_or_else(|| {
                log::error!("Failed to find AvatarId (user_aux) for user_id {}", self.account.id);
                DATABASE_ERR
            })?;
        let avatar_id: u32 = avatar_id_aux.data.parse()
            .map_err(|e| {
                log::error!("Failed to parse AvatarId (user_aux) for user_id {}: {}", self.account.id, e);
                crate::data::error_codes::WebServicesError::UnexpectedError as i16
            })?;
        Ok(super::GetAvatarInfo {
            avatar_id: polariton::operation::Typed::Int(if avatar_id == u32::MAX { 0 } else { avatar_id as i32 }),
            use_custom: polariton::operation::Typed::Bool(avatar_id == u32::MAX),
        })
    }

    async fn set_avatar_info(&self, info: super::AvatarInfo) -> Result<(), i16> {
        self.err_on_banned().await?;
        let to_update = oj_rc_database::schema::user_aux::ActiveModel {
            data: oj_rc_database::sea_orm::ActiveValue::Set(if info.use_custom { u32::MAX } else { info.avatar_id as u32 }.to_string()),
            ..Default::default()
        };
        self.db.update_user_aux_by_user_id_and_descriptor(to_update, self.account.id, oj_rc_database::schema::user_aux::Descriptor::AvatarId).await
            .map_err(|e| {
                log::error!("Failed to update AvatarId (user_aux) for user_id {}: {}", self.account.id, e);
                DATABASE_ERR
            })?;
        Ok(())
    }

    async fn list_avatar_info(&self, public_ids: &[String]) -> Result<Vec<super::SocialInfo>, polariton_server::operations::SimpleOpError> {
        let users = self.db.users_by_public_id(public_ids.iter()).await
            .map_err(|e| {
                log::error!("Failed to retrieve friend avatars for user {} : {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::WebServicesError::DatabaseError as i16,
                    format!("Failed to retrieve friend avatars: {}", e),
                )
            })?;
        let user_ids = users.iter().map(|user| user.id);
        let user_avatars = self.db.user_auxs_by_user_ids_and_descriptor(user_ids, oj_rc_database::schema::user_aux::Descriptor::AvatarId).await
            .map_err(|e| {
                log::error!("Failed to retrieve friend avatars for user {} : {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::WebServicesError::DatabaseError as i16,
                    format!("Failed to retrieve friend avatars: {}", e),
                )
            })?;
        let avatar_map: std::collections::HashMap<i32, u32> = user_avatars.iter()
            .filter_map(|avatar| avatar.data.parse().ok().map(|avatar_id| (avatar.user_id, avatar_id)))
            .collect();
        Ok(users.iter()
            .map(|user| super::SocialInfo {
                public_id: user.public_id.clone(),
                display_name: user.display_name.clone(),
                avatar_id: avatar_map.get(&user.id).and_then(|&avatar_id| if avatar_id == u32::MAX { None } else { Some(avatar_id as i32) }),
            })
            .collect()
        )
    }

    fn current_game_event_setter(&self) -> Box<dyn super::GameEventSetter> {
        Box::new(GameEventSetterImpl {
            db: self.db.clone(),
        })
    }

    async fn apply_purchase(&self, action: &crate::persist::config::ShopAction) -> Result<super::PurchaseResult, polariton_server::operations::SimpleOpError> {
        if action.cost_free != 0 {
            let is_ok = self.currency_sub_checked(super::CurrencyType::Free, action.cost_free as _).await
            .map_err(|e| {
                log::error!("Failed to apply free cost for purchase: {}", e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::WebServicesError::DatabaseError as i16,
                    format!("Failed to apply free cost for purchase: {}", e),
                )
            })?;
            if !is_ok {
                log::debug!("Rejected purchase costing {} for user {} (insufficient free funds)", action.cost_free, self.account.id);
                return Err(polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::WebServicesError::NotEnoughMoney as i16,
                    "Not enough free funds for purchase".to_owned(),
                ))
            }
        }
        if action.cost_paid != 0 {
            let is_ok = self.currency_sub_checked(super::CurrencyType::Paid, action.cost_paid as _).await
            .map_err(|e| {
                log::error!("Failed to apply paid cost for purchase: {}", e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::WebServicesError::DatabaseError as i16,
                    format!("Failed to apply paid cost for purchase: {}", e),
                )
            })?;
            if !is_ok {
                log::debug!("Rejected purchase costing {} for user {} (insufficient paid funds)", action.cost_paid, self.account.id);
                return Err(polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::WebServicesError::NotEnoughMoney as i16,
                    "Not enough paid funds for purchase".to_owned(),
                ))
            }
        }
        let mut new_cubes = std::collections::HashMap::new();
        let mut paid_currency = 0;
        for award in action.gives.iter() {
            match award {
                crate::persist::config::ShopGain::Cube(x) => {
                    new_cubes.insert(hex::encode((*x as i32).to_be_bytes()), 1);
                },
                crate::persist::config::ShopGain::Experience(xp) => {
                    self.currency_op(
                        crate::persist::user::CurrencyType::Experience,
                        crate::persist::user::CurrencyOp::AddSub(*xp as _),
                    ).await
                    .map_err(|e| {
                        log::error!("Failed to apply experience for purchase: {}", e);
                        polariton_server::operations::SimpleOpError::with_message(
                            crate::data::error_codes::WebServicesError::DatabaseError as i16,
                            format!("Failed to apply experience for purchase: {}", e),
                        )
                    })?;
                },
                crate::persist::config::ShopGain::FreeCurrency(c) => {
                    self.currency_op(
                        crate::persist::user::CurrencyType::Free,
                        crate::persist::user::CurrencyOp::AddSub(*c as _),
                    ).await
                    .map_err(|e| {
                        log::error!("Failed to apply free currency for purchase: {}", e);
                        polariton_server::operations::SimpleOpError::with_message(
                            crate::data::error_codes::WebServicesError::DatabaseError as i16,
                            format!("Failed to apply free currency for purchase: {}", e),
                        )
                    })?;
                },
                crate::persist::config::ShopGain::PaidCurrency(c) => {
                    self.currency_op(
                        crate::persist::user::CurrencyType::Paid,
                        crate::persist::user::CurrencyOp::AddSub(*c as _),
                    ).await
                    .map_err(|e| {
                        log::error!("Failed to apply paid currency for purchase: {}", e);
                        polariton_server::operations::SimpleOpError::with_message(
                            crate::data::error_codes::WebServicesError::DatabaseError as i16,
                            format!("Failed to apply paid currency for purchase: {}", e),
                        )
                    })?;
                    paid_currency += *c;
                },
                crate::persist::config::ShopGain::TechPoints(tp) => {
                    self.currency_op(
                        crate::persist::user::CurrencyType::TechPoints,
                        crate::persist::user::CurrencyOp::AddSub(*tp as _),
                    ).await
                    .map_err(|e| {
                        log::error!("Failed to apply tech point for purchase: {}", e);
                        polariton_server::operations::SimpleOpError::with_message(
                            crate::data::error_codes::WebServicesError::DatabaseError as i16,
                            format!("Failed to apply tech points for purchase: {}", e),
                        )
                    })?;
                },
            }
        }
        Ok(super::PurchaseResult {
            success: true,
            cube_awards: new_cubes,
            robopass_award: false,
            paid_currency_award: paid_currency,
        })
    }

    async fn currency_debit(&self, ty: super::CurrencyType, to_sub: u64) -> Result<(), polariton_server::operations::SimpleOpError> {
        let is_ok = self.currency_sub_checked(ty, to_sub).await.map_err(|e| {
            log::error!("Failed to apply currency debit of {} {:?} for user {}: {}", to_sub, ty, self.account.id, e);
            polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::WebServicesError::DatabaseError as i16,
                format!("Failed to apply debit of {} {:?}: {}", to_sub, ty, e),
            )
        })?;
        if is_ok {
            Ok(())
        } else {
            Err(polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::WebServicesError::NotEnoughMoney as i16,
                format!("Not enough funds to apply debit of {} {:?}", to_sub, ty),
            ))
        }
    }

    async fn mark_code_redeemed(&self, code: String) -> Result<bool, polariton_server::operations::SimpleOpError> {
        // TODO support serial (single global use) codes
        // FIXME this should probably be a transaction
        let codes_opt = self.db.user_aux_by_user_id_and_descriptor(self.account.id, oj_rc_database::schema::user_aux::Descriptor::RedeemedPromoCodes).await
            .map_err(|e| {
                log::error!("Failed to retrieve RedeemedPromoCodes (user_aux) for user_id {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::WebServicesError::DatabaseError as i16,
                    "Failed to retrieve RedeemedPromoCodes".to_owned(),
                )
            })?;
        if let Some(codes_entity) = codes_opt {
            match serde_json::from_str::<Vec<String>>(&codes_entity.data) {
                Ok(mut codes) => {
                    if codes.contains(&code) {
                        Ok(false)
                    } else {
                        codes.push(code);
                        let active_model = oj_rc_database::schema::user_aux::ActiveModel {
                            data: oj_rc_database::sea_orm::ActiveValue::Set(serde_json::to_string(&codes).unwrap()),
                            ..Default::default()
                        };
                        self.db.update_user_aux_by_user_id_and_descriptor(
                            active_model,
                            self.account.id,
                            oj_rc_database::schema::user_aux::Descriptor::RedeemedPromoCodes,
                        ).await.map_err(|e| {
                            log::error!("Failed to update RedeemedPromoCodes (user_aux id {}) for user_id {}: {}", codes_entity.id, self.account.id, e);
                            polariton_server::operations::SimpleOpError::with_message(
                                crate::data::error_codes::WebServicesError::DatabaseError as i16,
                                "Failed to update RedeemedPromoCodes".to_owned(),
                            )
                        })?;
                        Ok(true)
                    }
                },
                Err(e) => {
                    log::error!("Failed to parse RedeemedPromoCodes (user_aux id {}) for user_id {}: {}", self.account.id, codes_entity.id, e);
                    return Err(polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::WebServicesError::DatabaseError as i16,
                        "Failed to parse RedeemedPromoCodes JSON".to_owned(),
                    ));
                }
            }
        } else {
            // user_aux entry needs to be created
            let new_codes = oj_rc_database::schema::user_aux::ActiveModel {
                id: oj_rc_database::sea_orm::ActiveValue::NotSet,
                user_id: oj_rc_database::sea_orm::ActiveValue::Set(self.account.id),
                creation_time: oj_rc_database::sea_orm::ActiveValue::Set(chrono::Utc::now().timestamp()),
                descriptor: oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::user_aux::Descriptor::RedeemedPromoCodes),
                data: oj_rc_database::sea_orm::ActiveValue::Set(serde_json::to_string(&vec![code]).unwrap()),
            };
            self.db.insert_user_aux(vec![new_codes]).await
                .map_err(|e| {
                    log::error!("Failed to insert RedeemedPromoCodes (user_aux) for user_id {}: {}", self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::WebServicesError::DatabaseError as i16,
                        "Failed to insert updated RedeemedPromoCodes".to_owned(),
                    )
                })?;
            Ok(true)
        }
    }

    async fn get_emotes(&self) -> Result<Vec<String>, polariton_server::operations::SimpleOpError> {
        let user_aux_opt = self.db.user_aux_by_user_id_and_descriptor(self.account.id, oj_rc_database::schema::user_aux::Descriptor::EmotigramWheel).await
            .map_err(|e| {
                log::error!("Failed to retrieve EmotigramWheel for user_id {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::WebServicesError::DatabaseError as i16,
                    "Failed to update EmotigramWheel".to_owned(),
                )
            })?;
        if let Some(data) = user_aux_opt {
            let val: Vec<String> = serde_json::from_str(&data.data)
                .map_err(|e| {
                    log::error!("Failed to deserialize EmotigramWheel for user_id {}: {}", self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::WebServicesError::DatabaseError as i16,
                        "Failed to deserialize EmotigramWheel".to_owned(),
                    )
                })?;
            Ok(val)
        } else {
            // database entry not created yet
            Ok(Vec::default())
        }
    }

    async fn set_emotes(&self, emotes: &[String]) -> Result<(), polariton_server::operations::SimpleOpError> {
        let json_str = serde_json::to_string_pretty(emotes).expect("Bad emotes");
        //log::debug!("Settings emotes to {}", json_str);
        let to_update = oj_rc_database::schema::user_aux::ActiveModel {
            data: oj_rc_database::sea_orm::ActiveValue::Set(json_str),
            ..Default::default()
        };
        let is_update_missed = self.db.update_user_aux_by_user_id_and_descriptor(to_update.clone(), self.account.id, oj_rc_database::schema::user_aux::Descriptor::EmotigramWheel).await
            .map_err(|e| {
                log::error!("Failed to update EmotigramWheel for user_id {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::WebServicesError::DatabaseError as i16,
                    "Failed to update EmotigramWheel".to_owned(),
                )
            })?.is_none();
        if is_update_missed { // needs to be created
            let mut to_insert = to_update;
            to_insert.user_id = oj_rc_database::sea_orm::ActiveValue::Set(self.account.id);
            to_insert.creation_time = oj_rc_database::sea_orm::ActiveValue::Set(chrono::Utc::now().timestamp());
            to_insert.descriptor = oj_rc_database::sea_orm::ActiveValue::Set(oj_rc_database::schema::user_aux::Descriptor::EmotigramWheel);
            self.db.insert_user_aux(vec![to_insert]).await
                .map_err(|e| {
                    log::error!("Failed to insert EmotigramWheel for user_id {}: {}", self.account.id, e);
                    polariton_server::operations::SimpleOpError::with_message(
                        crate::data::error_codes::WebServicesError::DatabaseError as i16,
                        "Failed to insert EmotigramWheel".to_owned(),
                    )
                })?;
        }
        Ok(())
    }
}

struct GameEventSetterImpl {
    db: std::sync::Arc<oj_rc_database::Database>,
}

impl GameEventSetterImpl {
    async fn insert_event(&self, variant: oj_rc_database::schema::game_event::EventVariant, event: super::CurrentGameEvent) {
        let now = chrono::Utc::now().timestamp();
        let model = oj_rc_database::schema::game_event::ActiveModel {
            id: oj_rc_database::sea_orm::ActiveValue::NotSet,
            creation_time: oj_rc_database::sea_orm::ActiveValue::Set(now),
            map: oj_rc_database::sea_orm::ActiveValue::Set(event.map),
            mode: oj_rc_database::sea_orm::ActiveValue::Set(event.mode.to_db()),
            visibility: oj_rc_database::sea_orm::ActiveValue::Set(event.visibility.to_db()),
            auto_heal: oj_rc_database::sea_orm::ActiveValue::Set(event.auto_heal),
            start: oj_rc_database::sea_orm::ActiveValue::Set(event.start),
            end: oj_rc_database::sea_orm::ActiveValue::Set(event.end),
            variant: oj_rc_database::sea_orm::ActiveValue::Set(variant),
        };
        if let Err(e) = self.db.insert_game_event(model).await {
            log::error!("Failed to save new game event: {}", e);
        }
    }

    async fn select_event_now(&self, variant: oj_rc_database::schema::game_event::EventVariant) -> Option<super::CurrentGameEvent> {
        let now = chrono::Utc::now().timestamp();
        match self.db.game_event_at_time(now, variant).await {
            Err(e) => {
                log::error!("Failed to retrieve current game event: {}", e);
                None
            },
            Ok(None) => {
                log::warn!("Failed to find current game event");
                None
            },
            Ok(Some(event)) => {
                Some(super::CurrentGameEvent {
                    map: event.map,
                    visibility: crate::data::game_mode::MapVisibility::from_db(event.visibility),
                    mode: crate::data::game_mode::GameMode::from_db(event.mode),
                    auto_heal: event.auto_heal,
                    start: event.start,
                    end: event.end,
                })
            },
        }
    }
}

#[async_trait::async_trait]
impl super::GameEventSetter for GameEventSetterImpl {
    async fn set_multiplayer(&self, event: super::CurrentGameEvent) {
        self.insert_event(oj_rc_database::schema::game_event::EventVariant::Multiplayer, event).await
    }

    async fn get_multiplayer(&self) -> Option<super::CurrentGameEvent> {
        self.select_event_now(oj_rc_database::schema::game_event::EventVariant::Multiplayer).await
    }

    async fn set_singleplayer(&self, event: super::CurrentGameEvent) {
        self.insert_event(oj_rc_database::schema::game_event::EventVariant::Singleplayer, event).await
    }

    async fn get_singleplayer(&self) -> Option<super::CurrentGameEvent> {
        self.select_event_now(oj_rc_database::schema::game_event::EventVariant::Singleplayer).await
    }
}
