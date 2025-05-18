use argon2::PasswordVerifier;

use crate::persist::config::ConfigProvider;

pub struct AccountProvider {
    cubes: std::sync::Arc<Vec<u32>>,
    garage_upgrades: std::sync::Arc<crate::persist::config::GarageUpgrades>,
    auto_signups: bool,
    secret: Vec<u8>,
    db: std::sync::Arc<rc_database::Database>,
}

impl AccountProvider {
    pub async fn load(root: impl AsRef<std::path::Path>, conf: &crate::persist::config::ConfigImpl) -> std::io::Result<Self> {
        let token_path = root.as_ref().join(super::TOKEN_SECRET_FILENAME);
        let server_settings = <crate::persist::config::ConfigImpl as ConfigProvider<()>>::server_config(conf);
        let database_uri = server_settings.database;
        log::debug!("Connecting to user database URI: {}", database_uri);
        let db = rc_database::Database::init(&database_uri).await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::NotConnected, e))?;
        Ok(Self {
            cubes: std::sync::Arc::new(<crate::persist::config::ConfigImpl as ConfigProvider<()>>::ids(conf)),
            garage_upgrades: std::sync::Arc::new(<crate::persist::config::ConfigImpl as ConfigProvider<()>>::garage_upgrades(conf)),
            auto_signups: server_settings.auto_signup,
            secret: std::fs::read(&token_path)?,
            db: std::sync::Arc::new(db),
        })
    }
}

#[async_trait::async_trait]
impl <C: Clone> super::UserProvider<C> for AccountProvider {
    async fn authenticate(&self, token: super::UserToken, ext: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any + Send + Sync + 'static>>) -> Result<Box<dyn super::User<C> + Send + Sync>, String> {
        //let new_root = self.root.join(&token.uuid);
        let secret = jsonwebtoken::DecodingKey::from_secret(&self.secret);
        let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.set_required_spec_claims::<&str>(&[]);
        jsonwebtoken::decode::<libfj::robocraft::TokenPayload>(&token.token, &secret, &validation).map_err(|e| e.to_string())?;
        let user_info = if let Some(user_info) = self.db.user_by_any_unique_id(token.uuid.clone()).await.map_err(|e| e.to_string())? {
            user_info
        } else {
            return Err("User not found".to_owned());
        };
        let user_perms = if let Some(user_perms) = self.db.perms_by_user_id(user_info.id).await.map_err(|e| e.to_string())? {
            user_perms
        } else {
            return Err("User permissions not found".to_owned());
        };
        //let account_info = AccountInfo::load(&new_root).map_err(|e| e.to_string())?;
        Ok(Box::new(UserData {
            token,
            account: user_info,
            perms: user_perms,
            cubes: self.cubes.clone(),
            garage_upgrades: self.garage_upgrades.clone(),
            extensions: ext,
            db: self.db.clone(),
        }))
        //Err("Unable to authenticate".to_string())
    }
}

#[async_trait::async_trait]
impl super::UserAuthenticator for AccountProvider {
    async fn login(&self, info: super::UserInfo) -> Result<super::UserLoginInfo, String> {
        //let new_root = self.root.join(&info.payload.public_id);
        let is_new_user;
        let user_opt = match &info.extra {
            super::ExtraUserInfo::Steam { id } => self.db.user_by_steam_id(*id).await,
            super::ExtraUserInfo::Email { .. } => self.db.user_by_email(info.payload.email_address.clone()).await,
            super::ExtraUserInfo::Username { .. } => self.db.user_by_display_name(info.payload.display_name.clone()).await,
        }.map_err(|e| e.to_string())?;
        let mut user_info = if let Some(user_info) = user_opt {
            is_new_user = false;
            user_info
        } else {
            is_new_user = true;
            if self.auto_signups {
                log::info!("New user {}", info.payload.public_id);
                super::setup_new_user(&info, &self.db).await.map_err(|e| e.to_string())?;
                self.db.user_by_display_name(info.payload.display_name.clone()).await.map_err(|e| e.to_string())?.unwrap()
            } else {
                log::info!("Rejecting user sign-in for `{}` (set settings.server.auto_signup=true to disable this behaviour)", info.payload.public_id);
                return Err(format!("User does not exist"));
            }
        };
        let override_password = user_info.password.is_empty() && user_info.steam_id.is_none();
        match info.extra {
            super::ExtraUserInfo::Steam { id } => {
                let id_str = id.to_string();
                if let Some(expected_steam_id) = user_info.steam_id {
                    if expected_steam_id != id_str {
                        return Err("SteamID does not match".to_owned())
                    }
                } else {
                    return Err("SteamID not supported for this user".to_owned());
                }
            },
            super::ExtraUserInfo::Email { password }
            | super::ExtraUserInfo::Username { password } => {
                use argon2::password_hash::PasswordHasher;
                let argon2_algo = argon2::Argon2::default();
                if override_password {
                    let salt = argon2::password_hash::SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
                    let password_hash = argon2_algo.hash_password(password.as_bytes(), &salt).map_err(|e| e.to_string())?.to_string();
                    user_info.password = password_hash;
                }
                if !user_info.password.is_empty() {
                    let expected = argon2::password_hash::PasswordHash::new(&user_info.password).map_err(|e| e.to_string())?;
                    argon2_algo.verify_password(password.as_bytes(), &expected).map_err(|e| e.to_string())?;
                } else {
                    return Err("Password not supported for this user".to_owned())
                }
            }
        }
        // authentication has now definitely succeeded
        // build token
        let header = jsonwebtoken::Header {
            typ: Some("JWT".to_string()),
            alg: jsonwebtoken::Algorithm::HS256,
            ..Default::default()
        };
        let secret = jsonwebtoken::EncodingKey::from_secret(&self.secret);
        let token = jsonwebtoken::encode(&header, &info.payload, &secret)
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

    async fn register(&self, info: super::RegistrationInfo) -> Result<u32, String> {
        super::register_new_user(&info, &self.db).await.map_err(|e| e.to_string())
    }
}

#[allow(dead_code)]
struct UserData {
    token: super::UserToken,
    account: rc_database::schema::user::Model,
    perms: rc_database::schema::permissions::Model,
    cubes: std::sync::Arc<Vec<u32>>,
    garage_upgrades: std::sync::Arc<crate::persist::config::GarageUpgrades>,
    extensions: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any + Send + Sync + 'static>>,
    db: std::sync::Arc<rc_database::Database>,
}

impl UserData {
    async fn load_garage_by_slot(&self, slot: u32) -> Result<Option<rc_database::schema::garage::Model>, rc_database::sea_orm::DbErr> {
        //let path = self.root.join(super::GARAGE_DIR).join(format!("{}.json", id));
        //crate::persist::GarageSlot::load(&path)
        self.db.garage_by_user_id_and_slot(self.account.id, slot).await
    }

    async fn save_garage_by_slot(&self, data: rc_database::schema::garage::ActiveModel, slot: u32) -> Result<(), rc_database::sea_orm::DbErr> {
        self.db.update_garage_by_user_id_and_slot(data, self.account.id, slot).await?;
        Ok(())
    }

    async fn all_vehicles(&self) -> Result<Vec<rc_database::schema::garage::Model>, rc_database::sea_orm::DbErr> {
        self.db.garages_by_user_id(self.account.id).await
    }
}

const INVALID_ROBOT_ERR: i16 = crate::data::error_codes::WebServicesError::InvalidRobot as i16; // 140
const DATABASE_ERR: i16 = crate::data::error_codes::WebServicesError::DatabaseError as i16; // 8

#[async_trait::async_trait]
impl <C: Clone> super::User<C> for UserData {
    fn ext(&self, ty: std::any::TypeId) -> Option<&'_ (dyn std::any::Any + Send + Sync + 'static)> {
        self.extensions.get(&ty).map(|x| x.as_ref())
    }

    fn token(&self) -> &'_ super::UserToken {
        &self.token
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

    async fn unlocked_parts(&self) -> Vec<u32> {
        match self.db.user_aux_by_user_id_and_descriptor(self.account.id, rc_database::schema::user_aux::Descriptor::UnlockedParts).await {
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
            Ok(Some(selected)) => (super::i64_as_uuid_str(selected.uuid), selected.slot),
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
        self.db.update_garage_selected_by_user_id_and_slot(self.account.id, slot as u32).await.map_err(|e| {
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
        let slot_order = match self.db.user_aux_by_user_id_and_descriptor(self.account.id, rc_database::schema::user_aux::Descriptor::GarageSlotOrder).await{
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
                    weapon_order: polariton::operation::Typed::IntArr(rc_database::schema::parse_int_csv(&slot.weapon_order).into_iter().map(|x| x as i32).collect::<Vec<_>>().into()),
                    movement_categories: polariton::operation::Typed::IntArr(rc_database::schema::parse_int_csv(&slot.movement_categories).into_iter().map(|x| x as i32).collect::<Vec<_>>().into()),
                    control_type: polariton::operation::Typed::Int(control_ty as _),
                    control_options: control_options.as_transmissible(),
                    mastery_level: polariton::operation::Typed::Int(slot.mastery_level as i32),
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

    async fn save_slot(&self, vehicle: crate::persist::user::VehicleData) -> Result<(), i16> {
        let entity = rc_database::schema::garage::ActiveModel {
            weapon_order: rc_database::sea_orm::ActiveValue::Set(rc_database::schema::dump_csv(&vehicle.weapon_order)),
            robot_data: rc_database::sea_orm::ActiveValue::Set(vehicle.robot_data),
            colour_data: rc_database::sea_orm::ActiveValue::Set(vehicle.colour_data),
            ..Default::default()
        };
        self.save_garage_by_slot(entity, vehicle.slot as u32).await.map_err(|e| {
            log::error!("Failed to save vehicle slot {} for user_id {}: {}", vehicle.slot, self.account.id, e);
            DATABASE_ERR
        })?;
        Ok(())
    }

    async fn save_slot_order(&self, slots: Vec<i32>) -> Result<(), i16> {
        let slots: Vec<u32> = slots.into_iter().map(|x| x as u32).collect();
        let entity = rc_database::schema::user_aux::ActiveModel {
            data: rc_database::sea_orm::ActiveValue::Set(serde_json::to_string_pretty(&slots).unwrap()),
            ..Default::default()
        };
        self.db.update_user_aux_by_user_id_and_descriptor(entity, self.account.id, rc_database::schema::user_aux::Descriptor::GarageSlotOrder).await.map_err(|e| {
            log::error!("Failed to update garage slot order for user_id {}: {}", self.account.id, e);
            DATABASE_ERR
        })?;
        Ok(())
    }

    async fn new_slot(&self, reset_slot: Option<i32>) -> Result<super::NewSlotData<C>, i16> {
        let model = if let Some(slot) = reset_slot {
            let new_data = super::initial_data::default_reset_slot();
            if let Some(reset_g) = self.db.update_garage_by_user_id_and_slot(new_data, self.account.id, slot as u32).await.map_err(|e| {
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

    async fn upgrade_slot(&self, increments: i32) -> Result<polariton::operation::Typed<C>, i16> {
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
        let inc_opt = self.garage_upgrades.increments.iter().enumerate().filter(|(_i, inc)| inc.cpu <= selected_slot.bay_cpu).last();
        if let Some((i, _)) = inc_opt {
            let max_upgrade = self.garage_upgrades.increments.len() - 1;
            let upgrade_to = i + (increments as usize);
            if upgrade_to > max_upgrade {
                // over-upgraded
                Ok(polariton::operation::Typed::Bool(false))
            } else {
                let upgrade_to_cpu = self.garage_upgrades.increments[upgrade_to].cpu;
                let entity = rc_database::schema::garage::ActiveModel {
                    bay_cpu: rc_database::sea_orm::ActiveValue::Set(upgrade_to_cpu),
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

    fn signup_date(&self) -> i64 {
        super::since_windows_epoch(self.account.creation_time)
    }

    async fn singleplayer_robots(&self) ->  Result<polariton::operation::Typed<C>, i16> {
        let current_slot = self.db.garage_selected(self.account.id).await.map_err(|e| {
            log::error!("Failed to retrieve selected vehicle for user_id {} (singleplayer_robots): {}", self.account.id, e);
            DATABASE_ERR
        })?
        .ok_or_else(|| {
            log::error!("Failed to find selected vehicle for user_id {} (singleplayer_robots)", self.account.id);
            INVALID_ROBOT_ERR
        })?;
        let user_uuid = self.token.uuid.clone();
        Ok(crate::data::player_data::PlayerDatas {
            players: vec![
                crate::data::player_data::PlayerData {
                    name: user_uuid.clone(),
                    display_name: self.account.display_name.clone(),
                    mastery: current_slot.mastery_level as i32,
                    tier: 1, // FIXME
                    robot_name: current_slot.name,
                    robot_map: current_slot.robot_data,
                    team: 0,
                    has_premium: true, // FIXME
                    robot_uuid: super::i64_as_uuid_str(current_slot.uuid).into(),
                    cpu: current_slot.total_robot_cpu as i32,
                    weapon_order: rc_database::schema::parse_int_csv(&current_slot.weapon_order).into_iter().map(|x| x as i32).collect::<Vec<_>>(),
                    colour_map: current_slot.colour_data,
                    is_ai: false,
                    spawn_effect: "Spawn_Warp".to_owned(), // FIXME
                    death_effect: "Explosion_Warp".to_owned(), // FIXME
                    player_rank: 1, // FIXME
                    weapon_rank: rc_database::schema::parse_int_csv(&current_slot.weapon_order).into_iter().map(|x| (x as i32, if x == 0 { 0 } else { 1 })).collect(),
                }
            ],
        }.as_transmissible())
    }
}
