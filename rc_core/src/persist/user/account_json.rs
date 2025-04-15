use argon2::PasswordVerifier;
use serde::{Serialize, Deserialize};

use crate::persist::config::ConfigProvider;

pub struct AccountProvider {
    root: std::path::PathBuf,
    cubes: std::sync::Arc<Vec<u32>>,
    secret: Vec<u8>,
}

impl AccountProvider {
    pub fn load(root: impl AsRef<std::path::Path>, cubes: &crate::persist::config::ConfigImpl) -> std::io::Result<Self> {
        let token_path = root.as_ref().join(super::TOKEN_SECRET_FILENAME);
        let root = root.as_ref().join(super::USERS_DIR);
        std::fs::create_dir_all(&root)?;
        Ok(Self {
            root,
            cubes: std::sync::Arc::new(<crate::persist::config::ConfigImpl as ConfigProvider<()>>::ids(cubes)),
            secret: std::fs::read(&token_path)?,
        })
    }

    pub fn load_for_auth(root: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let token_path = root.as_ref().join(super::TOKEN_SECRET_FILENAME);
        let root = root.as_ref().join(super::USERS_DIR);
        std::fs::create_dir_all(&root)?;
        Ok(Self {
            root,
            cubes: std::sync::Arc::new(Vec::default()),
            secret: std::fs::read(&token_path)?,
        })
    }
}

impl <C: Clone> super::UserProvider<C> for AccountProvider {
    fn authenticate(&self, token: super::UserToken, ext: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any + Send + Sync + 'static>>) -> Result<Box<dyn super::User<C> + Send + Sync>, String> {
        let new_root = self.root.join(&token.uuid);
        let secret = jsonwebtoken::DecodingKey::from_secret(&self.secret);
        let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.set_required_spec_claims::<&str>(&[]);
        jsonwebtoken::decode::<libfj::robocraft::TokenPayload>(&token.token, &secret, &validation).map_err(|e| e.to_string())?;
        let account_info = AccountInfo::load(&new_root).map_err(|e| e.to_string())?;
        Ok(Box::new(UserData {
            root: new_root,
            token,
            account: account_info,
            cubes: self.cubes.clone(),
            extensions: ext,
        }))
        //Err("Unable to authenticate".to_string())
    }
}
impl super::UserAuthenticator for AccountProvider {
    fn login(&self, info: super::UserInfo) -> Result<super::UserLoginInfo, String> {
        let new_root = self.root.join(&info.payload.public_id);
        let is_new_user = !new_root.exists();
        if is_new_user {
            std::fs::create_dir(&new_root).map_err(|e| e.to_string())?;
            log::info!("New user {}", info.payload.public_id);
            super::setup_directory(&new_root).map_err(|e| e.to_string())?;
        }
        let mut account_info = AccountInfo::load(&new_root).map_err(|e| e.to_string())?;
        let is_new_user = is_new_user || (account_info.password.is_none() && account_info.steam_id.is_none()); // migration
        match info.extra {
            super::ExtraUserInfo::Steam { id } => {
                if is_new_user {
                    account_info.steam_id = Some(id);
                }
                if let Some(expected_steam_id) = account_info.steam_id {
                    if expected_steam_id != id {
                        return Err("SteamID does not match".to_owned())
                    }
                } else {
                    return Err("SteamID not supported for this user".to_owned());
                }
            },
            super::ExtraUserInfo::Standalone { password } => {
                use argon2::password_hash::PasswordHasher;
                let argon2_algo = argon2::Argon2::default();
                if is_new_user {
                    let salt = argon2::password_hash::SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
                    let password_hash = argon2_algo.hash_password(password.as_bytes(), &salt).map_err(|e| e.to_string())?.to_string();
                    account_info.password = Some(password_hash);
                }
                if let Some(expected_password) = &account_info.password {
                    let expected = argon2::password_hash::PasswordHash::new(expected_password).map_err(|e| e.to_string())?;
                    argon2_algo.verify_password(password.as_bytes(), &expected).map_err(|e| e.to_string())?;
                } else {
                    return Err("Password not supported for this user".to_owned())
                }
            }
        }
        // authentication has now definitely succeeded
        if is_new_user {
            account_info.save(new_root).map_err(|e| e.to_string())?;
        }
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
}

#[allow(dead_code)]
struct UserData {
    root: std::path::PathBuf,
    token: super::UserToken,
    account: AccountInfo,
    cubes: std::sync::Arc<Vec<u32>>,
    extensions: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any + Send + Sync + 'static>>,
}

impl UserData {
    fn load_garage_by_id(&self, id: u32) -> std::io::Result<crate::persist::GarageSlot> {
        let path = self.root.join(super::GARAGE_DIR).join(format!("{}.json", id));
        crate::persist::GarageSlot::load(&path)
    }

    fn save_garage(&self, slot: &crate::persist::GarageSlot) -> std::io::Result<()> {
        let path = self.root.join(super::GARAGE_DIR).join(format!("{}.json", slot.slot));
        slot.save(path)
    }

    fn all_vehicles(&self) -> std::io::Result<Vec<crate::persist::GarageSlot>> {
        let path = self.root.join(super::GARAGE_DIR);
        let mut slots = Vec::new();
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let filepath = entry.path();
            if filepath.is_file() {
                let slot = crate::persist::GarageSlot::load(&filepath)?;
                slots.push(slot);
            } else {
                log::warn!("Ignoring non-file {} in {} dir", filepath.display(), super::GARAGE_DIR);
            }
        }
        slots.sort_by_key(|slot| slot.slot);
        Ok(slots)
    }
}

const INVALID_ROBOT_ERR: i16 = 140;
const DATABASE_ERR: i16 = 8;

impl <C: Clone> super::User<C> for UserData {
    fn ext(&self, ty: std::any::TypeId) -> Option<&'_ (dyn std::any::Any + Send + Sync + 'static)> {
        self.extensions.get(&ty).map(|x| x.as_ref())
    }

    fn token(&self) -> &'_ super::UserToken {
        &self.token
    }

    fn is_mod(&self) -> bool {
        self.account.is_mod
    }

    fn is_admin(&self) -> bool {
        self.account.is_admin
    }

    fn is_dev(&self) -> bool {
        self.account.is_dev
    }

    fn unlocked_parts(&self) -> Vec<u32> {
        match self.account.inventory.override_ {
            super::inventory::UnlockOverride::Normal => self.account.inventory.unlocked.clone(),
            super::inventory::UnlockOverride::UnlockNone => Vec::default(),
            super::inventory::UnlockOverride::UnlockAll => self.cubes.as_ref().to_owned(),
        }
    }

    fn selected_garage_uuid(&self) -> String {
        self.account.garage.uuid_str()
    }

    fn selected_garage_slot(&self) -> u32 {
        self.account.garage.slot
    }

    fn all_slots_by_id(&self) -> super::UserSlots<C> {
        let slots = match self.all_vehicles() {
            Ok(slots) => slots,
            Err(e) => {
                log::error!("Failed to load all vehicles: {}", e);
                Vec::default()
            }
        };
        let slot_order = polariton::operation::Typed::ObjArr(slots.iter().map(|slot| polariton::operation::Typed::Int(slot.slot as _)).collect::<Vec<_>>().into());
        let slot_info = polariton::operation::Typed::Dict(polariton::operation::Dict {
            key_ty: polariton::serdes::TypePrefix::Int,
            val_ty: polariton::serdes:: TypePrefix::HashMap,
            items: slots.into_iter().map(|slot| {
                let slot_index = slot.slot;
                let garage_data: crate::data::garage_bay::GarageSlotInfo = slot.into();
                (polariton::operation::Typed::Int(slot_index as _), garage_data.as_transmissible())
            }).collect(),
        });
        super::UserSlots {
            slot_info, slot_order,
        }
    }

    fn slot_by_id(&self, id: i32) -> Result<crate::persist::user::UserSlotData<C>, i16> {
        match self.load_garage_by_id(id as _) {
            Ok(slot) => {
                let control_ty: crate::data::garage_bay::ControlType = slot.control_type.into();
                let control_options: crate::data::garage_bay::ControlOptions = slot.control_options.into();
                Ok(crate::persist::user::UserSlotData {
                    data: polariton::operation::Typed::Bytes(slot.robot_data.into()),
                    colour_data: polariton::operation::Typed::Bytes(slot.colour_data.into()),
                    cube_count: polariton::operation::Typed::Int(slot.cubes as _),
                    weapon_order: polariton::operation::Typed::IntArr(slot.weapon_order.clone().into()),
                    movement_categories: polariton::operation::Typed::IntArr(slot.movement_categories.into_iter().map(|cat| {
                        let cat: crate::data::weapon_list::ItemCategory = cat.into();
                        cat.but_bigger()
                    }).collect::<Vec<_>>().into()),
                    control_type: polariton::operation::Typed::Int(control_ty as _),
                    control_options: control_options.as_transmissible(),
                    mastery_level: polariton::operation::Typed::Int(0), // TODO
                    robot_rank: polariton::operation::Typed::Int(slot.total_robot_ranking as _),
                    cpu: polariton::operation::Typed::Int(slot.total_robot_cpu as _),
                    cosmetic_cpu: polariton::operation::Typed::Int(slot.total_cosmetic_cpu as _),
                    uuid: polariton::operation::Typed::Str(format!("{}_{}", slot.uuid.0, slot.uuid.1).into()),
                })
            },
            Err(e) => {
                log::error!("Failed to load vehicle {}: {}", id, e);
                Err(INVALID_ROBOT_ERR)
            }
        }
    }

    fn save_slot(&self, vehicle: crate::persist::user::VehicleData) -> Result<(), i16> {
        let id = vehicle.id as u32;
        let mut existing_data = self.load_garage_by_id(id).map_err(|e| {
            log::error!("Failed to load vehicle {}: {}", id, e);
            INVALID_ROBOT_ERR
        })?;
        existing_data.slot = id;
        existing_data.robot_data = vehicle.robot_data;
        existing_data.colour_data = vehicle.colour_data;
        log::debug!("weapon order: {:?}", vehicle.weapon_order);
        existing_data.weapon_order = vehicle.weapon_order;
        self.save_garage(&existing_data).map_err(|e| {
            log::error!("Failed to save vehicle {}: {}", id, e);
            DATABASE_ERR
        })?;
        Ok(())
    }

    fn signup_date(&self) -> i64 {
        match self.root.metadata() {
            Ok(meta) => {
                match meta.created() {
                    Ok(created) => {
                        match created.duration_since(std::time::SystemTime::UNIX_EPOCH) {
                            Ok(dur) => {
                                return super::since_windows_epoch(dur.as_secs() as i64);
                            },
                            Err(e) => log::error!("could not get duration since unix epoch of {}: {}", self.root.display(), e),
                        }
                    },
                    Err(e) => log::error!("could not read creation time of {}: {}", self.root.display(), e),
                }
            },
            Err(e) => log::error!("could not retrieve metadata of {}: {}", self.root.display(), e),
        }
        super::since_windows_epoch(0)
    }

    fn singleplayer_robots(&self) ->  Result<polariton::operation::Typed<C>, i16> {
        let current_slot = self.load_garage_by_id(self.account.garage.slot).map_err(|e| {
            log::error!("Failed to load current vehicle: {}", e);
            INVALID_ROBOT_ERR
        })?;
        let user_uuid = self.token.uuid.clone();
        Ok(crate::data::player_data::PlayerDatas {
            players: vec![
                crate::data::player_data::PlayerData {
                    name: user_uuid.clone(),
                    display_name: user_uuid,
                    mastery: current_slot.mastery_level,
                    tier: 1, // FIXME
                    robot_name: current_slot.name,
                    robot_map: current_slot.robot_data,
                    team: 0,
                    has_premium: true, // FIXME
                    robot_uuid: format!("{}_{}", current_slot.uuid.0, current_slot.uuid.1),
                    cpu: current_slot.total_robot_cpu as i32,
                    weapon_order: current_slot.weapon_order.clone(),
                    colour_map: current_slot.colour_data,
                    is_ai: false,
                    spawn_effect: "Spawn_Warp".to_owned(), // FIXME
                    death_effect: "Explosion_Warp".to_owned(), // FIXME
                    player_rank: 1, // FIXME
                    weapon_rank: current_slot.weapon_order.into_iter().map(|x| (x, if x == 0 { 0 } else { 1 })).collect(),
                }
            ],
        }.as_transmissible())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccountInfo {
    pub is_mod: bool,
    pub is_admin: bool,
    pub is_dev: bool,
    pub password: Option<String>,
    pub steam_id: Option<u64>,
    pub inventory: super::UnlockedParts,
    pub garage: super::SelectedGarage,
}

impl AccountInfo {
    fn load(root: impl AsRef<std::path::Path>) -> std::io::Result<AccountInfo> {
        let file = std::fs::File::open(root.as_ref().join(super::USER_FILE))?;
        let buffered = std::io::BufReader::new(file);
        let result = serde_json::from_reader(buffered)?;
        Ok(result)
    }

    pub fn save(&self, root: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        let file = std::fs::File::create(root.as_ref().join(super::USER_FILE))?;
        let buffered = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(buffered, self)?;
        Ok(())
    }
}

