use polariton::operation::Typed;

pub(super) struct CompleteCampaignData {
    pub(super) difficulty_map: std::collections::HashMap<i32, crate::persist::CampaignDifficulty>,
    pub(super) waves: Vec<crate::persist::Wave>,
}

pub struct CompleteCampaignProvider {
    map: std::collections::HashMap<String, CompleteCampaignData>,
}

impl CompleteCampaignProvider {
    pub(super) fn new(map: std::collections::HashMap<String, CompleteCampaignData>) -> Self {
        Self { map }
    }

    pub async fn get<C>(
        &self,
        id: &str,
        difficulty: &i32,
        user: &dyn crate::persist::user::CommonUser,
        factory: &dyn oj_rc_factory::VehicleFactoryAdapter,
        weapon_order: &crate::cubes::WeaponListParser,
        cpu_counter: &crate::cubes::CpuListParser,
    ) -> Result<Typed<C>, i16> {
        if let Some(campaign) = self.map.get(id) {
            if let Some(difficulty_conf) = campaign.difficulty_map.get(difficulty) {
                let mut waves = Vec::with_capacity(campaign.waves.len());
                for wave in campaign.waves.iter() {
                    let mut vehicles = Vec::with_capacity(wave.robots_in_wave.len());
                    for vehicle in wave.robots_in_wave.iter() {
                        let resolved = user.resolve_config_vehicle(&vehicle.vehicle.into_conf(), factory, weapon_order, cpu_counter).await?;
                        vehicles.push(crate::data::campaign::CompleteWaveRobotData {
                            name: resolved.robot_name,
                            robot_data: resolved.robot_map,
                            colour_data: resolved.colour_map,
                            time_to_spawn: vehicle.time_to_spawn,
                            kills_to_spawn: vehicle.kills_to_spawn,
                            time_to_despawn: vehicle.time_to_despawn,
                            kills_to_despawn: vehicle.kills_to_despawn,
                            initial_robot_amount: vehicle.initial_robot_amount,
                            periodic_robot_amount: vehicle.periodic_robot_amount,
                            spawn_interval: vehicle.spawn_interval,
                            min_robot_amount: vehicle.min_robot_amount,
                            max_robot_amount: vehicle.max_robot_amount,
                            is_boss: vehicle.is_boss,
                            is_kill_requirement: vehicle.is_kill_requirement,
                        });
                    }
                    waves.push(crate::data::campaign::CompleteWaveData {
                        player_spawn_location: wave.player_spawn_location,
                        robots_in_wave: vehicles,
                        kill_target: wave.kill_target,
                        time_min: wave.time_min,
                        time_max: wave.time_max,
                    });
                }
                let details_data = crate::data::campaign::CampaignWavesDifficultyData {
                    difficulty: difficulty_conf.clone().into(),
                    waves,
                };
                Ok(details_data.as_transmissible())
            } else {
                log::warn!("Couldn't find difficulty {} in campaign `{}`", difficulty, id);
                Err(crate::data::error_codes::WebServicesError::DatabaseError as i16)
            }
        } else {
            log::warn!("Couldn't find campaign {} (ignoring difficulty {})", id, difficulty);
            Err(crate::data::error_codes::WebServicesError::DatabaseError as i16)
        }
    }
}

pub struct CampaignResolver {
    pub(super) singleplayer: crate::persist::SingleplayerConfig,
}

impl CampaignResolver {
    pub fn campaigns_parameters<C>(&self) -> Typed<C> {
        self.singleplayer.clone().into_campaign_params().as_transmissible()
    }

    pub async fn campaign_waves<C>(
        &self,
        user: &dyn crate::persist::user::CommonUser,
        factory: &dyn oj_rc_factory::VehicleFactoryAdapter,
        weapon_order: &crate::cubes::WeaponListParser,
        cpu_counter: &crate::cubes::CpuListParser,
    ) -> Result<Typed<C>, polariton_server::operations::SimpleOpError> {
        let mut campaigns = Vec::with_capacity(self.singleplayer.campaigns.len());
        for campaign in self.singleplayer.campaigns.iter() {
            let mut waves = Vec::with_capacity(campaign.waves.len());
            for wave in campaign.waves.iter() {
                let mut vehicles = Vec::with_capacity(wave.robots_in_wave.len());
                for robot in wave.robots_in_wave.iter() {
                    let resolved = user.resolve_config_vehicle(&robot.vehicle.into_conf(), factory, weapon_order, cpu_counter).await?;
                    vehicles.push(crate::data::campaign::WaveRobotData {
                        name: resolved.robot_name,
                        weapon: robot.weapon.clone(),
                        movement: robot.movement.clone(),
                        rank: robot.rank.clone(),
                        count: robot.count,
                    });
                }
                waves.push(crate::data::campaign::WaveData {
                    robots_in_wave: vehicles,
                });
            }
            campaigns.push(crate::data::campaign::WavesData {
                id: campaign.id.clone(),
                waves,
                campaign_type: campaign.campaign_type.into(),
            });
        }
        Ok(crate::data::campaign::LiveCampaignWaves {
            waves: campaigns,
        }.as_transmissible())
    }

    pub fn campaign_version<C>(&self) -> Typed<C> {
        let mut locked_map = std::collections::HashMap::with_capacity(self.singleplayer.campaigns.len());
        for campaign in self.singleplayer.campaigns.iter() {
            locked_map.insert(campaign.id.clone(), true);
        }
        crate::data::campaign::GameModeVersionParameters {
            current_version: 0,
            is_locked: locked_map,
        }.as_transmissible()
    }
}
