use serde::{Serialize, Deserialize};

use crate::persist::config::ConfigProvider;

pub struct AccountProvider {
    root: std::path::PathBuf,
    cubes: std::sync::Arc<Vec<u32>>,
}

impl AccountProvider {
    pub fn load(root: impl AsRef<std::path::Path>, cubes: &crate::persist::config::ConfigImpl) -> std::io::Result<Self> {
        let root = root.as_ref().join(super::USERS_DIR);
        std::fs::create_dir_all(&root)?;
        Ok(Self {
            root,
            cubes: std::sync::Arc::new(<crate::persist::config::ConfigImpl as ConfigProvider<()>>::ids(cubes)),
        })
    }
}

impl <C: Clone> super::UserProvider<C> for AccountProvider {
    fn authenticate(&self, token: super::UserToken) -> Result<Box<dyn super::User<C> + Send + Sync>, String> {
        let new_root = self.root.join(&token.uuid);
        if !new_root.exists() {
            std::fs::create_dir(&new_root).map_err(|e| e.to_string())?;
            log::info!("New user {}", token.uuid);
            super::setup_directory(&new_root).map_err(|e| e.to_string())?;
        }
        let account_info = AccountInfo::load(&new_root).map_err(|e| e.to_string())?;
        Ok(Box::new(UserData {
            root: new_root,
            token,
            account: account_info,
            cubes: self.cubes.clone(),
        }))
        //Err("Unable to authenticate".to_string())
    }
}

#[allow(dead_code)]
struct UserData {
    root: std::path::PathBuf,
    token: super::UserToken,
    account: AccountInfo,
    cubes: std::sync::Arc<Vec<u32>>,
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

}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccountInfo {
    pub is_mod: bool,
    pub is_admin: bool,
    pub is_dev: bool,
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

