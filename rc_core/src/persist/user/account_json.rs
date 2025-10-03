use argon2::PasswordVerifier;
use sha2::Digest;

use crate::persist::config::ConfigProvider;

pub struct AccountProvider {
    cubes: std::sync::Arc<Vec<u32>>,
    garage_upgrades: std::sync::Arc<crate::persist::config::GarageUpgrades>,
    fake_players: std::sync::Arc<Vec<crate::persist::config::FakePlayer>>,
    auto_signups: bool,
    cdn: std::sync::Arc<String>,
    secret: std::sync::Arc<Vec<u8>>,
    db: std::sync::Arc<oj_rc_database::Database>,
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
        Ok(Self {
            cubes: std::sync::Arc::new(<crate::persist::config::ConfigImpl as ConfigProvider<()>>::ids(conf)),
            garage_upgrades: std::sync::Arc::new(<crate::persist::config::ConfigImpl as ConfigProvider<()>>::garage_upgrades(conf)),
            fake_players: std::sync::Arc::new(<crate::persist::config::ConfigImpl as ConfigProvider<()>>::fake_players(conf)),
            auto_signups: server_settings.auto_signup,
            cdn: std::sync::Arc::new(server_settings.cdn_url),
            secret: std::sync::Arc::new(secret),
            db: std::sync::Arc::new(db),
        })
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
}

#[async_trait::async_trait]
impl <C: Clone> super::UserProvider<C> for AccountProvider {
    async fn authenticate(&self, token: super::UserToken) -> Result<Box<dyn super::User<C> + Send + Sync>, super::AuthError> {
        //let new_root = self.root.join(&token.uuid);
        let secret = jsonwebtoken::DecodingKey::from_secret(&self.secret);
        let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.set_required_spec_claims::<&str>(&[]);
        jsonwebtoken::decode::<libfj::robocraft::TokenPayload>(&token.token, &secret, &validation).map_err(|e| super::AuthError {
            message: e.to_string(),
            code: crate::data::error_codes::AuthErrorCode::BadCredentials,
        })?;
        let user_info = if let Some(user_info) = self.db.user_by_any_unique_id(token.uuid.clone()).await.map_err(|e| super::AuthError {
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
        //let account_info = AccountInfo::load(&new_root).map_err(|e| e.to_string())?;
        Ok(Box::new(UserData {
            account: user_info,
            perms: user_perms,
            cubes: self.cubes.clone(),
            garage_upgrades: self.garage_upgrades.clone(),
            fake_players: self.fake_players.clone(),
            cdn: self.cdn.clone(),
            db: self.db.clone(),
            secret: self.secret.clone(),
        }))
        //Err("Unable to authenticate".to_string())
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
            cdn: self.cdn.clone(),
            db: self.db.clone(),
            secret: self.secret.clone(),
        }))
    }
}

#[async_trait::async_trait]
impl super::UserAuthenticator for AccountProvider {
    async fn login(&self, info: super::UserInfo) -> Result<super::UserLoginInfo, super::AuthError> {
        //let new_root = self.root.join(&info.payload.public_id);
        let is_new_user;
        let user_opt = match &info.extra {
            super::ExtraUserInfo::Steam { id } => self.db.user_by_steam_id(*id).await,
            super::ExtraUserInfo::Email { .. } => self.db.user_by_email(info.payload.email_address.clone()).await,
            super::ExtraUserInfo::Username { .. } => self.db.user_by_display_name(info.payload.display_name.clone()).await,
        }.map_err(|e| super::AuthError {
            message: e.to_string(),
            code: crate::data::error_codes::AuthErrorCode::BadCredentials,
        })?;
        let mut user_info = if let Some(user_info) = user_opt {
            is_new_user = false;
            user_info
        } else {
            is_new_user = true;
            if self.auto_signups {
                log::info!("New user {}", info.payload.public_id);
                super::setup_new_user(&info, &self.db).await.map_err(|e| super::AuthError {
                    message: e.to_string(),
                    code: crate::data::error_codes::AuthErrorCode::Unknown,
                })?;
                self.db.user_by_display_name(info.payload.display_name.clone()).await.map_err(|e| super::AuthError {
                    message: e.to_string(),
                    code: crate::data::error_codes::AuthErrorCode::Unknown,
                })?.unwrap()
            } else {
                log::info!("Rejecting user sign-in for `{}` (set settings.server.auto_signup=true to disable this behaviour)", info.payload.public_id);
                return Err(super::AuthError {
                    message: "User not found".to_owned(),
                    code: crate::data::error_codes::AuthErrorCode::BadCredentials,
                });
            }
        };
        let override_password = user_info.password.is_empty() && user_info.steam_id.is_none();
        match info.extra {
            super::ExtraUserInfo::Steam { id } => {
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
            super::ExtraUserInfo::Email { password }
            | super::ExtraUserInfo::Username { password } => {
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
        let mut payload = info.payload;
        payload.public_id = user_info.public_id;
        payload.display_name = user_info.display_name.clone();
        payload.robocraft_name = user_info.display_name;
        payload.email_address = user_info.email;
        payload.email_verified = true;
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
        })
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
}

pub(super) struct UserData {
    pub(super) account: oj_rc_database::schema::user::Model,
    pub(super) perms: oj_rc_database::schema::permissions::Model,
    pub(super) cubes: std::sync::Arc<Vec<u32>>,
    pub(super) garage_upgrades: std::sync::Arc<crate::persist::config::GarageUpgrades>,
    pub(super) fake_players: std::sync::Arc<Vec<crate::persist::config::FakePlayer>>,
    pub(super) cdn: std::sync::Arc<String>,
    pub(super) db: std::sync::Arc<oj_rc_database::Database>,
    pub(super) secret: std::sync::Arc<Vec<u8>>,
}

impl UserData {
    async fn load_garage_by_slot(&self, slot: i32) -> Result<Option<oj_rc_database::schema::garage::Model>, oj_rc_database::sea_orm::DbErr> {
        //let path = self.root.join(super::GARAGE_DIR).join(format!("{}.json", id));
        //crate::persist::GarageSlot::load(&path)
        self.db.garage_by_user_id_and_slot(self.account.id, slot).await
    }

    async fn save_garage_by_slot(&self, data: oj_rc_database::schema::garage::ActiveModel, slot: i32) -> Result<(), oj_rc_database::sea_orm::DbErr> {
        self.db.update_garage_by_user_id_and_slot(data, self.account.id, slot).await?;
        Ok(())
    }

    async fn all_vehicles(&self) -> Result<Vec<oj_rc_database::schema::garage::Model>, oj_rc_database::sea_orm::DbErr> {
        self.db.garages_by_user_id(self.account.id).await
    }

    async fn double_check_permissions(&self) -> Result<oj_rc_database::schema::permissions::Model, oj_rc_database::sea_orm::DbErr> {
        Ok(self.db.perms_by_user_id(self.account.id).await?.unwrap())
    }

    async fn err_on_banned(&self) -> Result<(), i16> {
        let perms = self.double_check_permissions().await.map_err(|e| {
            log::error!("Failed to retrieve user {} permissions: {}", self.account.id, e);
            DATABASE_ERR
        })?;
        if perms.banned {
            Err(crate::data::error_codes::WebServicesError::Banned as i16)
        } else {
            Ok(())
        }
    }

    fn check_perms_to_exec(&self, ty: &super::SanctionType) -> Result<(), i16> {
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
                            ^ crate::persist::user::i64_join((1 << 30, *factory_id))
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
            };
            next_id += 1;
            players.push(enemy);
        }
        Ok(players)
    }
}

const INVALID_ROBOT_ERR: i16 = crate::data::error_codes::WebServicesError::InvalidRobot as i16; // 140
const DATABASE_ERR: i16 = crate::data::error_codes::WebServicesError::DatabaseError as i16; // 8
const UNEXPECTED_ERR: i16 = crate::data::error_codes::WebServicesError::UnexpectedError as i16; // 9

#[async_trait::async_trait]
impl <C: Clone> super::User<C> for UserData {
    fn public_id(&self) -> &'_ str {
        &self.account.public_id
    }

    fn is_mod(&self) -> bool {
        self.perms.moderator
    }

    fn is_admin(&self) -> bool {
        self.perms.administrator
    }

    fn is_dev(&self) -> bool {
        self.perms.developer
    }

    fn is_banned(&self) -> bool {
        self.perms.banned
    }

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
        let new_name = format!("{} {}", slot_to_copy.name, append);
        let new_slot = if let Some(existing_slot) = into_slot {
            log::info!("Copy slot {} -> {} as `{}`", slot, existing_slot, new_name);
            if let Some(existing_g) = self.db.garage_by_user_id_and_slot(self.account.id, existing_slot).await.map_err(|e| {
                log::error!("Failed to retrieve vehicle slot {} for user_id {}: {}", slot, self.account.id, e);
                DATABASE_ERR
            })? {
                use oj_rc_database::sea_orm::IntoActiveModel;
                let mut to_update = existing_g.into_active_model();
                // copy everything except id, user_id, creation_time, slot, was_rated, bay_cpu, mastery_level, selected
                to_update.name = oj_rc_database::sea_orm::ActiveValue::Set(new_name);
                to_update.crf_id = oj_rc_database::sea_orm::ActiveValue::Set(slot_to_copy.crf_id);
                to_update.movement_categories = oj_rc_database::sea_orm::ActiveValue::Set(slot_to_copy.movement_categories);
                to_update.thumbnail_version = oj_rc_database::sea_orm::ActiveValue::Set(slot_to_copy.thumbnail_version);
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
        let inc_opt = self.garage_upgrades.increments.iter().enumerate().filter(|(_i, inc)| inc.cpu <= selected_slot.bay_cpu as u32).next_back();
        if let Some((i, _)) = inc_opt {
            let max_upgrade = self.garage_upgrades.increments.len() - 1;
            let upgrade_to = i + (increments as usize);
            if upgrade_to > max_upgrade {
                // over-upgraded
                Ok(polariton::operation::Typed::Bool(false))
            } else {
                let upgrade_to_cpu = self.garage_upgrades.increments[upgrade_to].cpu;
                let entity = oj_rc_database::schema::garage::ActiveModel {
                    bay_cpu: oj_rc_database::sea_orm::ActiveValue::Set(upgrade_to_cpu as i32),
                    ..Default::default()
                };
                self.db.update_garage_by_user_id_and_slot(entity, self.account.id, selected_slot.slot).await.map_err(|e| {
                    log::error!("Failed to upgrade selected vehicle slot to bay cpu of {} for user_id {}: {}", upgrade_to_cpu, self.account.id, e);
                    DATABASE_ERR
                })?;
                // TODO subtract upgrade cost from user free currency total
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

    async fn prepare_factory_upload(&self, vehicle: super::VehicleUploadData) -> Result<oj_rc_factory::VehicleUploadInfo, i16> {
        self.err_on_banned().await?;
        let slot = self.load_garage_by_slot(vehicle.slot).await.map_err(|e| {
            log::error!("Failed to retrieve vehicle slot {} for user_id {} (prepare_factory_upload): {}", vehicle.slot, self.account.id, e);
            DATABASE_ERR
        })?.ok_or_else(|| {
            log::error!("Failed to find vehicle slot {} for user_id {} (prepare_factory_upload)", vehicle.slot, self.account.id);
            INVALID_ROBOT_ERR
        })?;
        Ok(oj_rc_factory::VehicleUploadInfo {
            name: vehicle.name,
            description: vehicle.description,
            thumbnail: vehicle.thumbnail,
            added_by: self.account.public_id.clone(),
            added_by_display_name: self.account.display_name.clone(),
            cpu: slot.total_robot_cpu as u32,
            total_robot_ranking: slot.total_robot_ranking as u32,
            build_version: vehicle.version,
            cube_data: slot.robot_data,
            colour_data: slot.colour_data,
        })
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

    fn current_game_event_setter(&self) -> Box<dyn super::GameEventSetter> {
        Box::new(GameEventSetterImpl {
            db: self.db.clone(),
        })
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

#[async_trait::async_trait]
impl super::ChatUser for UserData {
    async fn subscribed_channels(&self) -> Result<polariton::operation::Typed<()>, i16> {
        let channels = self.subscribed_channels_strings().await?;
        log::info!("User is subscribed to channels {:?}", channels);
        Ok(polariton::operation::Typed::Arr(polariton::operation::Arr {
            ty: polariton::serdes::TypePrefix::HashMap, // hashtable
            items: channels.into_iter().map(|name| crate::data::channel::ChatChannelInfo {
                channel_name: name,
                members: vec![
                    crate::data::channel::ChatChannelMember {
                        name: self.account.display_name.clone(),
                        use_custom_avatar: false,
                        state: crate::data::channel::ChatPlayerState::Idk0,
                        custom_avatar: Vec::default(),
                        avatar_id: 0,
                    },
                ],
                channel_ty: crate::data::channel::ChatChannelType::Public,
            }.as_transmissible()).collect()
        }))
    }

    async fn subscribed_channels_strings(&self) -> Result<Vec<String>, i16> {
        let channels = self.db.user_aux_by_user_id_and_descriptor(self.account.id, oj_rc_database::schema::user_aux::Descriptor::SubscribedChannels).await.map_err(|e| {
                log::error!("Failed to retrieve SubscribedChannels (user_aux) for user_id {}: {}", self.account.id, e);
                crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
            })?.ok_or_else(|| {
                log::error!("Failed to find SubscribedChannels (user_aux) for user_id {}", self.account.id);
                crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
            })?;
        let channels = serde_json::from_str::<Vec<String>>(&channels.data).map_err(|e| {
            log::error!("Failed to parse SubscribedChannels (user_aux) for user_id {}: {}", self.account.id, e);
            crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
        })?;
        Ok(channels)
    }

    async fn add_subscribed_channel(&self, channel: String, channel_ty: crate::data::channel::ChatChannelType) -> Result<polariton::operation::Typed<()>, i16> {
        if let crate::data::channel::ChatChannelType::Public = channel_ty {
            let mut sub_channels = self.subscribed_channels_strings().await?;
            sub_channels.push(channel.clone());
            let new_data = serde_json::to_string(&sub_channels).map_err(|e| {
                log::error!("Failed to convert to JSON SubscribedChannels (user_aux) for user_id {}: {}", self.account.id, e);
                crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
            })?;
            self.db.update_user_aux_by_user_id_and_descriptor(oj_rc_database::schema::user_aux::ActiveModel {
                data: oj_rc_database::sea_orm::ActiveValue::Set(new_data),
                ..Default::default()
            }, self.account.id, oj_rc_database::schema::user_aux::Descriptor::SubscribedChannels).await.map_err(|e| {
                log::error!("Failed to update SubscribedChannels (user_aux) for user_id {}: {}", self.account.id, e);
                crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
            })?;
        }

         Ok(crate::data::channel::ChatChannelInfo {
            channel_name: channel,
            members: Vec::default(),
            channel_ty,
        }.as_transmissible())
    }

    async fn remove_subscribed_channel(&self, channel: String, channel_ty: crate::data::channel::ChatChannelType) -> Result<(), i16> {
        if let crate::data::channel::ChatChannelType::Public = channel_ty {
            let mut sub_channels = self.subscribed_channels_strings().await?;
            if let Some(index) = sub_channels.iter().position(|chann| chann == &channel) {
                sub_channels.swap_remove(index);
                let new_data = serde_json::to_string(&sub_channels).map_err(|e| {
                    log::error!("Failed to convert to JSON SubscribedChannels (user_aux) for user_id {}: {}", self.account.id, e);
                    crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
                })?;
                self.db.update_user_aux_by_user_id_and_descriptor(oj_rc_database::schema::user_aux::ActiveModel {
                    data: oj_rc_database::sea_orm::ActiveValue::Set(new_data),
                    ..Default::default()
                }, self.account.id, oj_rc_database::schema::user_aux::Descriptor::SubscribedChannels).await.map_err(|e| {
                    log::error!("Failed to update SubscribedChannels (user_aux) for user_id {}: {}", self.account.id, e);
                    crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
                })?;
            }
        }
        Ok(())
    }

    /*async fn has_pending_sanctions(&self) -> Result<bool, i16> {
        let count = self.db.count_sanctions_to_ack_by_user_id_and_descriptor(self.account.id, oj_rc_database::schema::sanction::Descriptor::Warn).await.map_err(|e| {
            log::error!("Failed to count pending sanctions for user_id {}: {}", self.account.id, e);
            crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
        })?;
        Ok(count != 0)
    }*/

    async fn get_sanctions(&self, username: String) -> Result<polariton::operation::Typed<()>, i16> {
        let user_opt = self.db.user_by_display_name(username.clone()).await.map_err(|e| {
            log::error!("Failed to retrieve user by username {} for user_id {}: {}", username, self.account.id, e);
            crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
        })?;
        if let Some(user) = user_opt {
            let sanctions = self.db.sanctions_by_user_id(user.id).await.map_err(|e| {
                log::error!("Failed to retrieve sanctions by username {} for user_id {}: {}", username, self.account.id, e);
                crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
            })?;
            Ok(polariton::operation::Typed::Arr(polariton::operation::Arr {
                ty: polariton::serdes::TypePrefix::Str,
                items: sanctions.into_iter().map(|x| {
                    let data = crate::data::sanction::SanctionJson {
                        type_: crate::data::sanction::SanctionType::from_db(x.descriptor),
                        reason: x.reason,
                        reporter: x.issuer_name,
                        issued: chrono::DateTime::from_timestamp(x.creation_time, 0).unwrap(),
                    };
                    polariton::operation::Typed::Str(data.as_json().into())
                }).collect(),
            }))
        } else {
            Err(crate::data::error_codes::ChatErrorCodes::DoesNotExist as i16)
        }
    }

    async fn set_sanction(&self, sanction: super::SetSanction) -> Result<(), i16> {
        self.check_perms_to_exec(&sanction.type_)?;
        let user_opt = self.db.user_by_display_name(sanction.username.clone()).await.map_err(|e| {
            log::error!("Failed to retrieve user by username {} for user_id {}: {}", sanction.username, self.account.id, e);
            crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
        })?;
        if let Some(user) = user_opt {
            if sanction.is_adding {
                let now = chrono::Utc::now().timestamp();
                let sanction_ty = crate::data::sanction::SanctionType::from_persist(sanction.type_).to_db();
                let to_add = oj_rc_database::schema::sanction::ActiveModel {
                    user_id: oj_rc_database::sea_orm::ActiveValue::Set(user.id),
                    creation_time: oj_rc_database::sea_orm::ActiveValue::Set(now),
                    issuer_id: oj_rc_database::sea_orm::ActiveValue::Set(self.account.id),
                    issuer_name: oj_rc_database::sea_orm::ActiveValue::Set(self.account.display_name.clone()),
                    descriptor: oj_rc_database::sea_orm::ActiveValue::Set(sanction_ty.clone()),
                    reason: oj_rc_database::sea_orm::ActiveValue::Set(sanction.reason),
                    duration: oj_rc_database::sea_orm::ActiveValue::Set(if sanction.duration <= 0 { None } else { Some(sanction.duration as i64) }),
                    ..Default::default()
                };
                if matches!(sanction_ty, oj_rc_database::schema::sanction::Descriptor::Ban) {
                    self.db.update_perms_by_user_id(oj_rc_database::schema::permissions::ActiveModel {
                        banned: oj_rc_database::sea_orm::ActiveValue::Set(true),
                        ..Default::default()
                    }, user.id).await.map_err(|e| {
                        log::error!("Failed to update permissions (to ban) for user_id {} by user_id {}: {}", user.id, self.account.id, e);
                        crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
                    })?;
                }
                self.db.insert_sanction(to_add).await.map_err(|e| {
                    log::error!("Failed to insert sanction for user_id {} by user_id {}: {}", user.id, self.account.id, e);
                    crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16
                })?;
                Ok(())
            } else {
                // FIXME
                log::error!("Modifying sanctions is not currently supported");
                Err(crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16)
            }
        } else {
            Err(crate::data::error_codes::ChatErrorCodes::DoesNotExist as i16)
        }
    }

    async fn get_total_registered_users(&self) -> Result<u64, polariton_server::operations::SimpleOpError> {
        self.db.user_count().await
            .map_err(|e| {
                log::error!("Failed to retrieve total user count for {}: {}", self.account.id, e);
                polariton_server::operations::SimpleOpError::with_message(
                    crate::data::error_codes::ChatErrorCodes::UnexpectedError as i16,
                    format!("Failed to retrieve total user count: {}", e),
                )
            })
    }
}
