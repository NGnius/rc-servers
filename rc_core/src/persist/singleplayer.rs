use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SingleplayerConfig {
    #[serde(default = "default_campaigns")]
    pub campaigns: Vec<Campaign>,
    pub vehicles: Vec<super::PrefabVehicle>,
    pub max_teammates: u32,
    pub max_enemies: u32,
}

impl SingleplayerConfig {
    pub fn into_campaign_params(self) -> crate::data::campaign::CampaignsGameParameters {
        crate::data::campaign::CampaignsGameParameters { campaigns: self.campaigns.into_iter().map(|x| x.into_campaign_params()).collect() }
    }

    pub fn into_waves(self) -> crate::data::campaign::LiveCampaignWaves {
        crate::data::campaign::LiveCampaignWaves { waves: self.campaigns.into_iter().map(|x| x.into_waves()).collect() }
    }

    pub fn into_singleplayer_conf(&self) -> crate::persist::config::SingleplayerConfig {
        crate::persist::config::SingleplayerConfig {
            max_teammates: self.max_teammates,
            max_enemies: self.max_enemies,
            vehicles: self.vehicles.iter().map(|v| v.into_conf()).collect()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Campaign {
    pub id: String,
    pub excluded_cubes: Vec<u32>, // encoded to hex strings
    pub categories: Vec<super::ItemCategory>,
    pub min_cpu: i32,
    pub max_cpu: i32,
    pub name: String,
    pub description: String,
    pub image: String,
    pub rules: Vec<String>,
    pub parameters: Vec<Vec<String>>,
    pub difficulties: Vec<CampaignDifficulty>,
    pub completed: Vec<CampaignCompletion>,
    pub map: String,
    pub campaign_type: CampaignType,
    pub waves: Vec<Wave>,
}

impl Campaign {
    pub fn into_campaign_params(self) -> crate::data::campaign::CampaignParameters {
        crate::data::campaign::CampaignParameters {
            id: self.id,
            excluded_cubes: self.excluded_cubes,
            categories: self.categories.into_iter().map(|x| x.into()).collect(),
            min_cpu: self.min_cpu,
            max_cpu: self.max_cpu,
            name: self.name,
            description: self.description,
            image: self.image,
            rules: self.rules,
            parameters: self.parameters,
            difficulties: self.difficulties.into_iter().map(|x| x.into()).collect(),
            completed: self.completed.into_iter().enumerate().map(|(i, x)| x.into_data(i as _)).collect(),
            map: self.map,
        }
    }

    pub fn into_waves(self) -> crate::data::campaign::WavesData {
        crate::data::campaign::WavesData {
            id: self.id,
            waves: self.waves.into_iter().map(|x| x.into()).collect(),
            campaign_type: self.campaign_type.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CampaignDifficulty {
    pub level: i32,
    pub lives: i32,
    pub auto_heal: bool,
    pub single_wave_bonus: i32,
    pub initial_health_boost: f32,
    pub health_boost_wave_increase: f32,
    pub initial_damage_boost: f32,
    pub damage_boost_wave_increase: f32,
}

impl std::convert::From<CampaignDifficulty> for crate::data::campaign::CampaignDifficultyData {
    fn from(val: CampaignDifficulty) -> Self {
        crate::data::campaign::CampaignDifficultyData {
            level: val.level,
            lives: val.lives,
            auto_heal: val.auto_heal,
            single_wave_bonus: val.single_wave_bonus,
            initial_health_boost: val.initial_health_boost,
            health_boost_wave_increase: val.health_boost_wave_increase,
            initial_damage_boost: val.initial_damage_boost,
            damage_boost_wave_increase: val.damage_boost_wave_increase,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CampaignCompletion {
    pub wave: i32,
    pub difficulty: bool,
}

impl CampaignCompletion {
    pub fn into_data(self, index: i32) -> crate::data::campaign::CampaignCompletionData {
        crate::data::campaign::CampaignCompletionData {
            index,
            wave: self.wave,
            difficulty: self.difficulty,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Wave {
    #[serde(default)]
    pub player_spawn_location: i32,
    pub robots_in_wave: Vec<WaveRobot>,
    pub kill_target: i32,
    #[serde(default)]
    pub time_min: i32,
    pub time_max: i32,
}

impl std::convert::From<Wave> for crate::data::campaign::WaveData {
    fn from(val: Wave) -> Self {
        crate::data::campaign::WaveData {
            robots_in_wave: val.robots_in_wave.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl std::convert::From<Wave> for crate::data::campaign::CompleteWaveData {
    fn from(val: Wave) -> Self {
        crate::data::campaign::CompleteWaveData {
            player_spawn_location: val.player_spawn_location,
            robots_in_wave: val.robots_in_wave.into_iter().map(|x| x.into()).collect(),
            kill_target: val.kill_target,
            time_min: val.time_min,
            time_max: val.time_max,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WaveRobot { // times appear to be in seconds
    pub name: String,
    pub weapon: String,
    pub movement: String,
    pub rank: String,
    pub count: i32,
    pub robot_data: Vec<u8>,
    pub colour_data: Vec<u8>,
    #[serde(default)]
    pub time_to_spawn: i32,
    #[serde(default)]
    pub kills_to_spawn: i32,
    #[serde(default)]
    pub time_to_despawn: i32,
    #[serde(default)]
    pub kills_to_despawn: i32,
    #[serde(default = "default_1")]
    pub initial_robot_amount: i32,
    #[serde(default)]
    pub periodic_robot_amount: i32,
    #[serde(default = "default_1")]
    pub spawn_interval: i32,
    #[serde(default = "default_1")]
    pub min_robot_amount: i32,
    #[serde(default)]
    pub max_robot_amount: i32,
    #[serde(default)]
    pub is_boss: bool,
    #[serde(default)]
    pub is_kill_requirement: bool,
}

fn default_1() -> i32 {
    1
}

impl std::convert::From<WaveRobot> for crate::data::campaign::WaveRobotData {
    fn from(val: WaveRobot) -> Self {
        crate::data::campaign::WaveRobotData {
            name: val.name,
            weapon: val.weapon,
            movement: val.movement,
            rank: val.rank,
            count: val.count,
        }
    }
}

impl std::convert::From<WaveRobot> for crate::data::campaign::CompleteWaveRobotData {
    fn from(val: WaveRobot) -> Self {
        crate::data::campaign::CompleteWaveRobotData {
            name: val.name,
            robot_data: val.robot_data,
            colour_data: val.colour_data,
            time_to_spawn: val.time_to_spawn,
            kills_to_spawn: val.kills_to_spawn,
            time_to_despawn: val.time_to_despawn,
            kills_to_despawn: val.kills_to_despawn,
            initial_robot_amount: val.initial_robot_amount,
            periodic_robot_amount: val.periodic_robot_amount,
            spawn_interval: val.spawn_interval,
            min_robot_amount: val.min_robot_amount,
            max_robot_amount: val.max_robot_amount,
            is_boss: val.is_boss,
            is_kill_requirement: val.is_kill_requirement,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum CampaignType {
    TimedElimination = 0,
    Survival = 1,
    Elimination = 2,
}

impl std::convert::From<CampaignType> for crate::data::campaign::CampaignType {
    fn from(val: CampaignType) -> Self {
        match val {
            CampaignType::TimedElimination => crate::data::campaign::CampaignType::TimedElimination,
            CampaignType::Survival => crate::data::campaign::CampaignType::Survival,
            CampaignType::Elimination => crate::data::campaign::CampaignType::Elimination,
        }
    }
}

fn default_campaigns() -> Vec<Campaign> {
    super::combat::default_campaigns().campaigns
}
