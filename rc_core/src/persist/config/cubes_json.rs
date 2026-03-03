use std::collections::HashMap;
use indexmap::IndexMap;

use serde::{Serialize, Deserialize};

use polariton::operation::{Typed, Dict};
use polariton::serdes::TypePrefix;

use crate::persist::config::SelfValidator;

use super::super::{MovementCategoryData, MovementData, Cube, ItemCategory, ItemTier, BattleConfig, Settings, ChatConfig, FactoryConfig, ItemShopConfig};

const CUBE_CONFIG_FILENAME: &str = "config.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct CubeConfig {
    cubes: IndexMap<String, Cube>,
    movement: HashMap<ItemCategory, MovementCategoryData>,
    lerp_value: f32,
    battle: BattleConfig,
    chat: ChatConfig,
    factory: FactoryConfig,
    shop: ItemShopConfig,
    settings: Settings,
}

impl CubeConfig {
    pub fn load(root: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let file = std::fs::File::open(root.as_ref().join(CUBE_CONFIG_FILENAME))?;
        let buffered = std::io::BufReader::new(file);
        let result = serde_json::from_reader(buffered)?;
        #[cfg(debug_assertions)]
        {
            let filename = root.as_ref().join(format!("{}.expanded.json", CUBE_CONFIG_FILENAME.trim_end_matches(".json")));
            let file = std::fs::File::create(filename)?;
            let buffered = std::io::BufWriter::new(file);
            serde_json::to_writer_pretty(buffered, &result)?;
        }
        Ok(result)
    }

    /// Performs configuration checks
    /// Returns true if validation succeeds, false if failed
    pub fn self_validate(&self, data_path: impl AsRef<std::path::Path>) -> bool {
        let mut validation_info = super::ValidationInfo::default();
        validation_info.info(super::ValidationMessage {
            path: vec![],
            message: format!("Validation started at {}", chrono::Utc::now()),
        });
        let token_path = data_path.as_ref().join(crate::persist::user::TOKEN_SECRET_FILENAME);
        if !token_path.exists() {
            validation_info.error(crate::persist::config::ValidationMessage {
                path: vec![],
                message: format!("Token secret file does not exist; create it at {}", token_path.display()),
            });
        }
        // TODO cubes
        // TODO movement
        let battle_res = self.battle.validate_in(&mut validation_info, self, "battle");
        let chat_res = self.chat.validate_in(&mut validation_info, self, "chat");
        let factory_res = self.factory.validate_in(&mut validation_info, self, "factory");
        let shop_res = self.shop.validate_in(&mut validation_info, self, "shop");
        let settings_res = self.settings.validate_in(&mut validation_info, self, "settings");

        validation_info.info(super::ValidationMessage {
            path: vec![],
            message: format!("Validation ended at {}", chrono::Utc::now()),
        });

        validation_info.print_messages();
        battle_res
            && chat_res
            && factory_res
            && shop_res
            && settings_res
            && validation_info.is_ok()
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
            let stats: Vec<_> = if let Some(stats) = movements_stats.get(k) {
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

    fn tech_tree_nodes(&self) -> super::TechTreeNodeProvider {
        let mut nodes = indexmap::IndexMap::with_capacity(self.cubes.len());
        for cube in self.cubes.values() {
            if let Some(tree_data) = &cube.tree {
                nodes.insert(cube.id, tree_data.to_owned());
            }
        }
        nodes.shrink_to_fit();
        super::TechTreeNodeProvider {
            tree: nodes,
        }
    }

    fn tech_tree_costs(&self) -> std::collections::HashMap<String, u32> { // cube id (hex) -> tech point cost
        let mut costs = std::collections::HashMap::with_capacity(self.cubes.len());
        for cube in self.cubes.values() {
            if let Some(tree_data) = &cube.tree {
                costs.insert(hex::encode(cube.id.to_be_bytes()), tree_data.tech_points);
            }
        }
        costs.shrink_to_fit(); // probably unnecessary, but free memory usage reduction!
        costs
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

    fn campaign_details(&self) -> super::CompleteCampaignProvider {
        let mut map = std::collections::HashMap::with_capacity(self.battle.singleplayer.campaigns.len());
        for campaign in self.battle.singleplayer.campaigns.iter() {
            let mut difficulty_map = std::collections::HashMap::with_capacity(campaign.difficulties.len());
            for difficulty in campaign.difficulties.iter() {
                difficulty_map.insert(difficulty.level, difficulty.to_owned());
            }
            map.insert(campaign.id.clone(), super::campaign::CompleteCampaignData {
                difficulty_map,
                waves: campaign.waves.clone(),
            });
        }
        super::CompleteCampaignProvider::new(map)
    }

    fn campaigns(&self) -> super::CampaignResolver {
        super::CampaignResolver {
            singleplayer: self.battle.singleplayer.clone(),
        }
    }

    fn client_config(&self) -> Typed<C> {
        let conf_data: crate::data::client_config::GameplaySettings = self.settings.gameplay.clone().into();
        Typed::Dict(Dict {
            key_ty: TypePrefix::Str,
            val_ty: TypePrefix::HashMap,
            items: vec![
                (Typed::Str("GameplaySettings".into()), conf_data.as_transmissible()),
            ],
        })
    }

    fn login_messages(&self) -> super::DevMessageProvider<C> {
        super::DevMessageProvider::new(self.settings.banners.iter().map(|msg| (msg.message.clone(), msg.duration as i32)).collect())
    }

    fn public_channels(&self) -> Typed<C> {
        Typed::Arr(polariton::operation::Arr {
            ty: TypePrefix::Str,
            custom_ty: None,
            items: self.chat.public_channels.iter().map(|s| Typed::Str(s.into())).collect(),
        })
    }

    fn server_config(&self) -> super::ServerConfig {
        super::ServerConfig {
            database: self.settings.server.database.clone(),
            auto_signup: self.settings.server.auto_signup,
            queue_mode: super::QueueChangeMode::from_persist(self.settings.server.queue_mode.clone()),
            domain: self.settings.server.domain.to_owned(),
            cdn_url: self.settings.server.cdn_url.trim_end_matches('/').to_owned(),
            auth_url: self.settings.server.auth_url.trim_end_matches('/').to_owned(),
            intercom_url: self.settings.server.intercom_url.trim_end_matches('/').to_owned(),
            minimum_version: self.settings.server.min_version as i32,
            dos_protect: self.settings.server.dos_protection,
            maintenance_message: self.settings.server.maintenance_message.clone(),
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

    async fn factory(&self, builtin_factory_provider: &(dyn (Fn() -> oj_rc_database::FactoryDatabase) + Sync)) -> Result<crate::factory::Factory, Box<dyn std::error::Error + 'static>> {
        crate::factory::Factory::from_config(&self.factory, &<Self as super::ConfigProvider<()>>::server_config(self), builtin_factory_provider).await
    }

    fn cubes(&self) -> &'_ indexmap::IndexMap<String, crate::persist::Cube> {
        &self.cubes
    }

    fn chat_system_config(&self) -> super::ChatSystemConfig {
        super::ChatSystemConfig {
            command_channel: self.chat.command_channel.clone(),
            commands: self.chat.commands.clone(),
            default_channel: self.chat.default_channel.clone(),
            can_create_channels: self.chat.can_create_channels,
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
            needs_to_be_saved: true,
        }
    }

    fn gamemodes(&self) -> crate::data::game_mode::GameModeConfigs {
        self.battle.games.into()
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

    /*fn is_multiplayer_enabled(&self) -> bool {
        self.battle.multiplayer.enabled
    }
    
    fn multiplayer_autostart_after(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.battle.multiplayer.autostart_after_s)
    }*/

    fn multiplayer_settings(&self) -> super::MultiplayerSettings {
        super::MultiplayerSettings {
            is_enabled: self.battle.multiplayer.enabled,
            lobby_autostart_after: self.battle.multiplayer.autostart_after_s.map(std::time::Duration::from_secs),
            loading_autostart_after: self.battle.multiplayer.continue_loading_after_s.map(std::time::Duration::from_secs),
        }
    }

    fn network_config(&self) -> crate::persist::NetworkConf {
        self.battle.multiplayer.network.clone()
    }

    fn maps(&self) -> std::collections::HashMap<super::GameMap, super::MapConfig> {
        self.battle.maps.map.iter().map(|(map, conf)| {
            let mut spawns = std::collections::HashMap::<u8, Vec<super::Point>>::with_capacity(2); // usually 2 teams
            let mut pit_spawns = Vec::default();
            for point in conf.spawn_points.iter() {
                if let Some(point_team) = &point.team {
                    if let Some(list) = spawns.get_mut(point_team) {
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
                        spawns.insert(*point_team, list);
                    }
                } else {
                    pit_spawns.push(super::Point {
                        x: point.x,
                        y: point.y,
                        z: point.z,
                    });
                }
            }
            let bases = conf.bases.iter().map(|base| (base.team, (super::Sphere {
                radius: base.radius,
                center: super::Point {
                    x: base.x,
                    y: base.y,
                    z: base.z,
                },
            }, base.percent_per_second))).collect();
            let capture_points = conf.capture_points.iter().map(|point| (super::Sphere {
                radius: point.radius,
                center: super::Point {
                    x: point.x,
                    y: point.y,
                    z: point.z,
                }
            }, point.percent_per_second)).collect();
            let equalizer = super::Point {
                x: conf.equalizer.x,
                y: conf.equalizer.y,
                z: conf.equalizer.z,
            };
            let map_conf = super::MapConfig {
                spawns,
                pit_spawns,
                bases,
                capture_points,
                equalizer,
            };
            (map.into_conf(), map_conf)
        }).collect()
    }

    fn url_links(&self) -> super::LinksConfig {
        super::LinksConfig {
            feedback_url: self.settings.server.feedback_url.clone(),
            support_url: self.settings.server.support_url.clone(),
            wiki_url: self.settings.server.wiki_url.clone(),
        }
    }

    fn fake_players(&self) -> Vec<super::FakePlayer> {
        self.battle.multiplayer.fakes.iter().map(|player| super::FakePlayer {
            team: player.team,
            vehicle: player.vehicle.into_conf(),
            implementation: player.implementation.clone().to_config(),
        }).collect()
    }

    fn filler_players(&self) -> Vec<super::FakePlayer> {
        self.battle.multiplayer.filler.iter().map(|player| super::FakePlayer {
            team: player.team,
            vehicle: player.vehicle.into_conf(),
            implementation: player.implementation.clone().to_config(),
        }).collect()
    }

    fn energy(&self) -> super::EnergyConfig {
        super::EnergyConfig {
            refill_rate: self.battle.energy.refill_rate_per_s,
            total: self.battle.energy.total,
        }
    }

    fn ba_settings(&self) -> super::BattleArenaResolver {
        super::BattleArenaResolver {
            data: self.battle.multiplayer.battle_arena.clone(),
        }
    }

    fn pit_settings(&self) -> super::PitSettings {
        self.battle.multiplayer.pit_config.clone().into()
    }

    fn tdm_settings(&self) -> super::TeamDeathMatchSettings {
        self.battle.multiplayer.team_death_match.clone().into()
    }

    fn shop_entries(&self) -> super::ShopEntriesResolver {
        super::ShopEntriesResolver::new(self.shop.items.clone())
    }

    fn promo_codes(&self) -> std::collections::HashMap<String, super::PromoCode> {
        let mut map = std::collections::HashMap::with_capacity(self.shop.promo_codes.len());
        for (key, val) in self.shop.promo_codes.iter() {
            let tx = super::ShopAction {
                cost_free: 0,
                cost_paid: 0,
                gives: val.gives.iter().map(|x| x.to_owned().into()).collect(),
            };
            map.insert(key.to_owned(), super::PromoCode {
                message: val.message.clone(),
                bundle_id: val.bundle_id.to_owned().unwrap_or_else(|| key.to_owned()),
                promo_id: val.promo_id.to_owned().unwrap_or_else(|| key.to_owned()),
                is_serial: val.is_serial,
                is_repeatable: val.is_repeatable,
                value: val.value,
                transaction: tx,
            });
        }
        map
    }

    fn vehicle_validation(&self) -> super::VehicleValidators {
        super::VehicleValidators {
            multiplayer: self.battle.multiplayer.vehicle_validator.clone(),
            custom_game: crate::persist::VehicleValidator::None, // TODO
            singleplayer: self.battle.singleplayer.vehicle_validator.clone(),
            campaigns: self.battle.singleplayer.campaigns.iter()
                .map(|campaign| (campaign.id.clone(), campaign.vehicle_validator.clone()))
                .collect(),
        }
    }
}
