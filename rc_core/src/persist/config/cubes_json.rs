use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use polariton::operation::{Typed, Dict};
use polariton::serdes::TypePrefix;

use super::super::{MovementCategoryData, MovementData, Cube, ItemCategory, ItemTier, BattleConfig, Settings, ChatConfig, FactoryConfig};

const CUBE_CONFIG_FILENAME: &str = "config.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct CubeConfig {
    cubes: HashMap<String, Cube>,
    movement: HashMap<ItemCategory, MovementCategoryData>,
    lerp_value: f32,
    battle: BattleConfig,
    chat: ChatConfig,
    factory: FactoryConfig,
    settings: Settings,
}

impl CubeConfig {
    pub fn load(root: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let file = std::fs::File::open(root.as_ref().join(CUBE_CONFIG_FILENAME))?;
        let buffered = std::io::BufReader::new(file);
        let result = serde_json::from_reader(buffered)?;
        Ok(result)
    }
}

#[async_trait::async_trait]
impl <C: Clone + Send> super::ConfigProvider<C> for CubeConfig {
    fn cube_list(&self) -> Typed<C> {
        Typed::Dict(Dict {
            key_ty: TypePrefix::Str,
            val_ty: TypePrefix::HashMap,
            items: self.cubes.values().map(|cube| {
                let cube_d: crate::data::cube_list::CubeInfo<C> = cube.info.clone().into();
                cube_d.as_transmissible_key_val(cube.id)
            }).collect(),
        })
    }

    fn movement_list(&self) -> Typed<C> {
        let mut movements_stats = HashMap::<ItemCategory, HashMap<ItemTier, MovementData>>::new();
        for cube in self.cubes.values() {
            if let Some(movement_data) = &cube.movement {
                let category_map = if let Some(x) = movements_stats.get_mut(&cube.info.category) {
                    x
                } else {
                    movements_stats.insert(cube.info.category, HashMap::new());
                    movements_stats.get_mut(&cube.info.category).unwrap()
                };
                category_map.insert(cube.info.size, movement_data.to_owned());
            }
        }
        let mut movement_cat_stats = Vec::with_capacity(self.movement.len());
        for (k, v) in self.movement.iter() {
            let stats: Vec<_> = if let Some(stats) = movements_stats.get(&k) {
                stats.iter().map(|(k, v)| (k.to_owned(), v.to_owned())).collect()
            } else {
                Vec::default()
            };
            let key: crate::data::weapon_list::ItemCategory = k.to_owned().into();
            let key_typed = Typed::<C>::Str(key.as_str().into());

            let value_data = v.to_owned().into_data(stats);
            movement_cat_stats.push((key_typed, value_data.as_transmissible()));
        }
        Typed::Dict(Dict {
            key_ty: TypePrefix::Str,
            val_ty: TypePrefix::HashMap,
            items: vec![
                (Typed::Str("Global".into()), Typed::HashMap(vec![
                    (Typed::Str("lerpValue".into()), Typed::Float(self.lerp_value)),
                ].into())),
                (Typed::Str("Movements".into()), Typed::HashMap(movement_cat_stats.into())),
            ],
        })
    }

    fn weapon_list(&self) -> Typed<C> {
        let mut weapon_stats = HashMap::new();
        for cube in self.cubes.values() {
            if let Some(weapon_data) = &cube.weapon {
                let category_map = if let Some(x) = weapon_stats.get_mut(&cube.info.category) {
                    x
                } else {
                    weapon_stats.insert(cube.info.category, HashMap::new());
                    weapon_stats.get_mut(&cube.info.category).unwrap()
                };
                category_map.insert(cube.info.size, weapon_data.to_owned());
            }
        }
        let mut weapons_vec: Vec<(Typed<C>, Typed<C>)> = Vec::with_capacity(weapon_stats.len());
        for (k, v) in weapon_stats {
            let cat_data: crate::data::weapon_list::ItemCategory = k.into();
            let mut tiers_vec = Vec::with_capacity(v.len());
            for (k, v) in v {
                let tier_data: crate::data::cube_list::ItemTier = k.into();
                let val_data: crate::data::weapon_list::WeaponData = v.into();
                tiers_vec.push((Typed::Str(tier_data.as_str().into()), val_data.as_transmissible()));
            }
            weapons_vec.push((Typed::Str(cat_data.as_str().into()), Typed::HashMap(tiers_vec.into())));
        }
        Typed::Dict(Dict {
            key_ty: TypePrefix::Str,
            val_ty: TypePrefix::HashMap,
            items: weapons_vec,
        })
    }

    fn weapon_upgrade_list(&self) -> Typed<C> {
        let mut seen_keys = std::collections::HashSet::new();
        let mut weapon_upgrades = Vec::new();
        for cube in self.cubes.values() {
            if let Some(weapon_up) = &cube.weapon_upgrade {
                let key = (cube.info.category, cube.info.size);
                if seen_keys.contains(&key) {
                    log::warn!("Weapon upgrade info for {:?} already exists, skipping", key);
                } else {
                    seen_keys.insert(key);
                    let weapon_upgrade_data = weapon_up.to_owned().into_data(cube.info.size, cube.info.category);
                    weapon_upgrades.push(weapon_upgrade_data.as_transmissible());
                }
            }
        }
        Typed::ObjArr(weapon_upgrades.into())
    }

    fn weapon_keys(&self) -> Typed<C> {
        let mut seen_keys = std::collections::HashSet::new();
        for cube in self.cubes.values() {
            if cube.weapon.is_some() {
                let key = crate::data::cube_list::item_key(cube.info.category.into(), cube.info.size.into());
                seen_keys.insert(key);
            }
        }
        let keys_vec: Vec<i32> = seen_keys.into_iter().collect();
        Typed::IntArr(keys_vec.into())
    }

    fn tech_tree_nodes(&self, unlocked_cubes: &std::collections::HashSet<u32>) -> Typed<C> {
        let mut seen_cubes = std::collections::HashSet::with_capacity(self.cubes.len());
        let mut needed_cubes = std::collections::HashSet::with_capacity(self.cubes.len());
        let mut typed_nodes = Vec::new();
        for cube in self.cubes.values() {
            if let Some(tree_data) = &cube.tree {
                let is_unlocked = unlocked_cubes.contains(&cube.id);
                let is_unlockable = tree_data.requires.iter().all(|id| unlocked_cubes.contains(id));
                tree_data.neighbours.iter().for_each(|id| { needed_cubes.insert(*id); });
                seen_cubes.insert(cube.id);
                let node_data = tree_data.to_owned().into_data(cube.id, is_unlocked, is_unlockable);
                typed_nodes.push(node_data.as_transmissible_key_val());
            }
        }
        for needed_cube_id in needed_cubes {
            if !seen_cubes.contains(&needed_cube_id) {
                log::warn!("Tech tree needs cube {} but it doesn't have tree info", needed_cube_id);
            }
        }
        Typed::Dict(Dict {
            key_ty: TypePrefix::Str,
            val_ty: TypePrefix::HashMap,
            items: typed_nodes,
        })
    }

    fn ids(&self) -> Vec<u32> {
        self.cubes.values().map(|cube| cube.id).collect()
    }

    fn regen_config(&self) -> Typed<C> {
        let regen_data: crate::data::auto_regen::AutoRegenHealthConfig = self.battle.regen.clone().into();
        regen_data.as_transmissible()
    }

    fn after_battle_vote_config(&self) -> Typed<C> {
        let mut vote_data = Vec::with_capacity(self.battle.votes.len()); // probably len() == 2
        for (key, val) in self.battle.votes.iter() {
            let key_data: crate::data::voting::Vote = key.to_owned().into();
            let mut val_data = Vec::with_capacity(val.len());
            for item in val {
                let vote_data: crate::data::voting::VoteThresholdData = item.to_owned().into();
                val_data.push(vote_data.as_transmissible());
            }
            vote_data.push((Typed::Str(key_data.as_str().into()), Typed::ObjArr(val_data.into())));
        }
        Typed::Dict(Dict {
            key_ty: TypePrefix::Str,
            val_ty: TypePrefix::Any,
            items: vote_data,
        })
    }

    fn game_mode_config(&self) -> Typed<C> {
        let game_mode_data: crate::data::game_mode::GameModeConfigs = self.battle.games.into();
        game_mode_data.as_transmissible()
    }

    fn campaigns_parameters(&self) -> Typed<C> {
        self.battle.singleplayer.clone().into_campaign_params().as_transmissible()
    }

    fn campaign_waves(&self) -> Typed<C> {
        self.battle.singleplayer.clone().into_waves().as_transmissible()
    }

    fn campaign_version(&self) -> Typed<C> {
        let mut locked_map = std::collections::HashMap::with_capacity(self.battle.singleplayer.campaigns.len());
        for campaign in self.battle.singleplayer.campaigns.iter() {
            locked_map.insert(campaign.id.clone(), true);
        }
        crate::data::campaign::GameModeVersionParameters {
            current_version: 0,
            is_locked: locked_map,
        }.as_transmissible()
    }

    fn campaign_details(&self) -> super::CompleteCampaignProvider {
        let mut map = std::collections::HashMap::with_capacity(self.battle.singleplayer.campaigns.len());
        for campaign in self.battle.singleplayer.campaigns.iter() {
            //let waves_data: Vec<crate::data::campaign::CompleteWaveData> = campaign.waves.iter().map(|x| x.clone().into()).collect();
            let mut difficulty_map = std::collections::HashMap::with_capacity(campaign.difficulties.len());
            for difficulty in campaign.difficulties.iter() {
                let difficulty_data: crate::data::campaign::CampaignDifficultyData = difficulty.clone().into();
                let complete_campaign = crate::data::campaign::CampaignWavesDifficultyData {
                    difficulty: difficulty_data,
                    waves: campaign.waves.iter().map(|x| x.clone().into()).collect(),
                };
                difficulty_map.insert(difficulty.level, complete_campaign);
            }
            map.insert(campaign.id.clone(), difficulty_map);
        }
        super::CompleteCampaignProvider::new(map)
    }

    fn client_config(&self) -> Typed<C> {
        let conf_data: crate::data::client_config::GameplaySettings = self.settings.gameplay.clone().into();
        Typed::Dict(Dict {
            key_ty: TypePrefix::Str,
            val_ty: TypePrefix::HashMap,
            items: vec![
                (Typed::Str("GameplaySettings".into()), conf_data.as_transmissible()),
            ].into(),
        })
    }

    fn login_messages(&self) -> super::DevMessageProvider<C> {
        super::DevMessageProvider::new(self.settings.banners.iter().map(|msg| (msg.message.clone(), msg.duration as i32)).collect())
    }

    fn public_channels(&self) -> Typed<C> {
        Typed::Arr(polariton::operation::Arr {
            ty: TypePrefix::Str,
            items: self.chat.public_channels.iter().map(|s| Typed::Str(s.into())).collect(),
        })
    }

    fn server_config(&self) -> super::ServerConfig {
        super::ServerConfig {
            database: self.settings.server.database.clone(),
            auto_signup: self.settings.server.auto_signup,
            queue_mode: super::QueueChangeMode::from_persist(self.settings.server.queue_mode.clone()),
        }
    }

    fn garage_upgrades(&self) -> super::GarageUpgrades {
        super::GarageUpgrades {
            increments: self.settings.garage_upgrades.iter().map(|inc| super::GarageUpgradeIncrement {
                cpu: inc.cpu,
                cost: inc.cost,
            }).collect(),
        }
    }

    async fn factory(&self) -> Result<crate::factory::Factory, Box<dyn std::error::Error + 'static>> {
        crate::factory::Factory::from_config(&self.factory).await
    }

    fn cubes(&self) -> &'_ std::collections::HashMap<String, crate::persist::Cube> {
        &self.cubes
    }

    fn chat_system_config(&self) -> super::ChatSystemConfig {
        super::ChatSystemConfig {
            command_channel: self.chat.command_channel.clone(),
            commands: self.chat.commands.clone(),
        }
    }

    fn gamemode_events(&self) -> super::GameEventSequence {
        let strategy = self.battle.rotation.strategy.into_conf();
        let first = strategy.next(self.battle.rotation.modes.len() - 1, self.battle.rotation.modes.len());
        super::GameEventSequence {
            strategy: self.battle.rotation.strategy.into_conf(),
            modes: self.battle.rotation.modes.clone().into_iter().map(|event| super::GameEvents {
                singleplayer: event.singleplayer.into_conf(),
                multiplayer: event.multiplayer.into_conf(),
                duration: std::time::Duration::from_secs(event.duration_s),
            }).collect(),
            index: first,
            started: chrono::Utc::now().timestamp(),
        }
    }

    fn gamemodes(&self) -> crate::data::game_mode::GameModeConfigs {
        self.battle.games.clone().into()
    }

    fn singleplayer_details(&self) -> super::SingleplayerConfig {
        self.battle.singleplayer.into_singleplayer_conf()
    }

    /*async fn prefab_vehicles(&self, user: &(dyn crate::persist::user::User<C> + Sync), factory: &crate::factory::Factory) -> Typed<C> {
        let mut next_id = 0;
        let mut id_map = Vec::with_capacity(self.battle.singleplayer.vehicles.len());
        let mut debug_str_map = Vec::with_capacity(self.battle.singleplayer.vehicles.len());
        for vehicle in self.battle.singleplayer.vehicles.clone().into_iter() {
            let current_id = next_id;
            let uuid_i64 = crate::persist::user::uuid_sanitize(crate::persist::user::i64_join((i32::MAX as u32, current_id)));
            let uuid_str = crate::persist::user::i64_as_uuid_str(uuid_i64);
            let prebuilt = match &vehicle.id {
                crate::persist::PrefabId::Factory { factory: factory_id } => {
                    use oj_rc_factory::VehicleFactoryAdapter;
                    let factory_vehicle = factory.vehicle(*factory_id).await
                        .expect("Failed to retrieve prefab vehicle from factory") // result
                        .expect("Prefab vehicle does not exist in factory"); // option
                    crate::data::robot_data::PrebuiltRobotInfo {
                        name: vehicle.name.unwrap_or(factory_vehicle.1.name),
                        class: vehicle.class,
                        category: "RE_robot_category0".to_owned(),
                        robot_data: factory_vehicle.0.cube_data,
                        colour_data: factory_vehicle.0.colour_data,
                    }
                },
                crate::persist::PrefabId::Database { garage } => {
                    let db_vehicle = user.garage_by_id(*garage).await
                        .expect("Prefab vehicle does not exist in main garage database");
                    crate::data::robot_data::PrebuiltRobotInfo {
                        name: vehicle.name.unwrap_or(db_vehicle.name.expect("Prefab vehicle name is required")),
                        class: vehicle.class,
                        category: "RE_robot_category0".to_owned(),
                        robot_data: db_vehicle.robot_data,
                        colour_data: db_vehicle.colour_data,
                    }
                },
            };
            debug_str_map.push(format!("`{}` -> `{}`", uuid_str, prebuilt.name));
            id_map.push((Typed::Str(uuid_str.into()), prebuilt.as_transmissible()));
            next_id += 1;
        }
        log::debug!("Prefab mapping generated `<uuid>` -> `<name>`:\n\t{}", debug_str_map.join("\n\t"));
        Typed::Dict(Dict {
            key_ty: TypePrefix::Str, // str
            val_ty: TypePrefix::HashMap, // hashmap
            items: id_map,
        })
    }*/

    fn players_per_game(&self) -> usize {
        self.battle.multiplayer.players_per_game
    }

    fn is_multiplayer_enabled(&self) -> bool {
        self.battle.multiplayer.enabled
    }

    fn network_config(&self) -> crate::persist::NetworkConf {
        self.battle.multiplayer.network.clone()
    }

    fn maps(&self) -> std::collections::HashMap<super::GameMap, super::MapConfig> {
        self.battle.maps.map.iter().map(|(map, conf)| {
            let mut spawns = std::collections::HashMap::<u8, Vec<super::Point>>::with_capacity(2); // usually 2 teams
            for point in conf.spawn_points.iter() {
                if let Some(list) = spawns.get_mut(&point.team) {
                    list.push(super::Point {
                        x: point.x,
                        y: point.y,
                        z: point.z,
                    });
                } else {
                    let mut list = Vec::with_capacity(10); // usually 10 spawn points (suddent death has the most)
                    list.push(super::Point {
                        x: point.x,
                        y: point.y,
                        z: point.z,
                    });
                    spawns.insert(point.team, list);
                }
            }
            let bases = conf.bases.iter().map(|base| (base.team, super::Sphere {
                radius: base.radius,
                center: super::Point {
                    x: base.x,
                    y: base.y,
                    z: base.z,
                },
            })).collect();
            let map_conf = super::MapConfig {
                spawns,
                bases,
            };
            (map.into_conf(), map_conf)
        }).collect()
    }
}
