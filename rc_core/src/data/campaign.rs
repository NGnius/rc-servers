use polariton::operation::Typed;

pub struct CampaignsGameParameters {
    pub campaigns: Vec<CampaignParameters>,
}

impl CampaignsGameParameters {
    fn dump(&self, writer: &mut dyn std::io::Write) -> std::io::Result<usize> {
        writer.write_all(&(self.campaigns.len() as i32).to_le_bytes())?;
        let mut total_len = 4;
        for campaign in self.campaigns.iter() {
            total_len += campaign.dump(writer)?;
        }
        Ok(total_len)
    }

    pub fn as_transmissible<C>(&self) -> Typed<C> {
        let mut buf = Vec::new();
        self.dump(&mut std::io::Cursor::new(&mut buf)).unwrap();
        Typed::Bytes(buf.into())
    }
}

pub struct CampaignParameters {
    pub id: String,
    pub excluded_cubes: Vec<u32>, // encoded to hex strings
    pub categories: Vec<super::weapon_list::ItemCategory>,
    pub min_cpu: i32,
    pub max_cpu: i32,
    pub name: String,
    pub description: String,
    pub image: String,
    pub rules: Vec<String>,
    pub parameters: Vec<Vec<String>>,
    pub difficulties: Vec<CampaignDifficultyData>,
    pub completed: Vec<CampaignCompletionData>,
    pub map: String,
}

impl CampaignParameters {
    fn dump(&self, writer: &mut dyn std::io::Write) -> std::io::Result<usize> {
        let mut total_len = super::write_str_for_binreader(&self.id, writer)?;
        writer.write_all(&(self.excluded_cubes.len() as i32).to_le_bytes())?;
        total_len += 4;
        for excluded_cube in self.excluded_cubes.iter() {
            let s = super::cube_id_to_str(*excluded_cube);
            total_len += super::write_str_for_binreader(&s, writer)?;
        }
        writer.write_all(&(self.categories.len() as i32).to_le_bytes())?;
        total_len += 4;
        for cat in self.categories.iter() {
            total_len += super::write_str_for_binreader(cat.as_str(), writer)?;
        }
        writer.write_all(&self.min_cpu.to_le_bytes())?;
        writer.write_all(&self.max_cpu.to_le_bytes())?;
        total_len += 8;
        total_len += super::write_str_for_binreader(&self.name, writer)?;
        total_len += super::write_str_for_binreader(&self.description, writer)?;
        total_len += super::write_str_for_binreader(&self.image, writer)?;
        writer.write_all(&(self.rules.len() as i32).to_le_bytes())?;
        total_len += 4;
        for rule in self.rules.iter() {
            total_len += super::write_str_for_binreader(rule, writer)?;
        }
        writer.write_all(&(self.parameters.len() as i32).to_le_bytes())?;
        total_len += 4;
        for param_vec in self.parameters.iter() {
            writer.write_all(&(param_vec.len() as i32).to_le_bytes())?;
            total_len += 4;
            for param in param_vec.iter() {
                total_len += super::write_str_for_binreader(param, writer)?;
            }
        }
        writer.write_all(&(self.difficulties.len() as i32).to_le_bytes())?;
        total_len += 4;
        for difficulty in self.difficulties.iter() {
            writer.write_all(&(CampaignDifficultyData::WRITE_BYTES_LEN as i32).to_le_bytes())?;
            total_len += 4 + difficulty.dump(writer)?;
        }
        writer.write_all(&(self.completed.len() as i32).to_le_bytes())?;
        total_len += 4;
        for completion in self.completed.iter() {
            total_len += completion.dump(writer)?;
        }
        total_len += super::write_str_for_binreader(&self.map, writer)?;
        Ok(total_len)
    }
}

pub struct CampaignDifficultyData {
    pub level: i32,
    pub lives: i32,
    pub auto_heal: bool,
    pub single_wave_bonus: i32,
    pub initial_health_boost: f32,
    pub health_boost_wave_increase: f32,
    pub initial_damage_boost: f32,
    pub damage_boost_wave_increase: f32,
}

impl CampaignDifficultyData {
    const WRITE_BYTES_LEN: usize = 29;
    fn dump(&self, writer: &mut dyn std::io::Write) -> std::io::Result<usize> {
        writer.write_all(&self.level.to_le_bytes())?;
        writer.write_all(&self.lives.to_le_bytes())?;
        writer.write_all(&[self.auto_heal as u8])?;
        writer.write_all(&self.single_wave_bonus.to_le_bytes())?;
        writer.write_all(&self.initial_health_boost.to_le_bytes())?;
        writer.write_all(&self.health_boost_wave_increase.to_le_bytes())?;
        writer.write_all(&self.initial_damage_boost.to_le_bytes())?;
        writer.write_all(&self.damage_boost_wave_increase.to_le_bytes())?;
        Ok(Self::WRITE_BYTES_LEN)
    }
}

pub struct CampaignCompletionData {
    pub index: i32,
    pub wave: i32,
    pub difficulty: bool,
}

impl CampaignCompletionData {
    const WRITE_BYTES_LEN: usize = 9;
    fn dump(&self, writer: &mut dyn std::io::Write) -> std::io::Result<usize> {
        writer.write_all(&self.index.to_le_bytes())?;
        writer.write_all(&self.wave.to_le_bytes())?;
        writer.write_all(&[self.difficulty as u8])?;
        Ok(Self::WRITE_BYTES_LEN)
    }
}

pub struct LiveCampaignWaves {
    pub waves: Vec<WavesData>,
}

impl LiveCampaignWaves {
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        Typed::HashMap(self.waves.iter().flat_map(|waves| waves.as_transmissible_key_val()).collect::<Vec<_>>().into())
    }
}

pub struct WavesData {
    pub id: String,
    pub waves: Vec<WaveData>,
    pub campaign_type: CampaignType,
}

impl WavesData {
    pub fn as_transmissible_key_val<C>(&self) -> [(Typed<C>, Typed<C>); 3] {
        [
            (Typed::Str(format!("wavesNumberInCurrentCampaign_{}", self.id).into()), Typed::Int(self.waves.len() as _)),
            (Typed::Str(self.id.clone().into()), Typed::HashMap(self.waves.iter().enumerate().flat_map(|(i, wave)| wave.as_transmissible_key_val(i)).collect::<Vec<_>>().into())),
            (Typed::Str(format!("campaignType_{}", self.id).into()), Typed::Int(self.campaign_type as _)),
        ]
    }
}

pub struct WaveData {
    pub robots_in_wave: Vec<WaveRobotData>,
}

impl WaveData {
    pub fn as_transmissible_key_val<C>(&self, index: usize) -> [(Typed<C>, Typed<C>); 2] {
        [
            (Typed::Str(format!("numberOfDifferentRobotsInCurrentWave_{}", index).into()), Typed::Int(self.robots_in_wave.len() as _)),
            (Typed::Int(index as _), Typed::HashMap(self.robots_in_wave.iter().enumerate().map(|(i, robot)| (Typed::Int(i as _), robot.as_transmissible())).collect::<Vec<_>>().into())),
        ]
    }
}

pub struct WaveRobotData {
    pub name: String,
    pub weapon: String,
    pub movement: String,
    pub rank: String,
    pub count: i32,
}

impl WaveRobotData {
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        Typed::HashMap(vec![
            (Typed::Str("RobotName".into()), Typed::Str(self.name.clone().into())),
            (Typed::Str("RobotWeapon".into()), Typed::Str(self.weapon.clone().into())),
            (Typed::Str("RobotMovementPart".into()), Typed::Str(self.movement.clone().into())),
            (Typed::Str("RobotRank".into()), Typed::Str(self.rank.clone().into())),
            (Typed::Str("RobotCount".into()), Typed::Int(self.count)),
        ].into())
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum CampaignType {
    TimedElimination = 0,
    Survival = 1,
    Elimination = 2,
}

pub struct GameModeVersionParameters {
    pub current_version: i32,
    pub is_locked: std::collections::HashMap<String, bool>,
}

impl GameModeVersionParameters {
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        Typed::HashMap(vec![
            (Typed::Str("CurrentVersionNumber".into()), Typed::Int(self.current_version)),
            (Typed::Str("LockedCampaignsInfo".into()), Typed::HashMap(self.is_locked.iter().map(|(key, val)| (Typed::Str(key.into()), Typed::Bool(*val))).collect::<Vec<_>>().into())),
        ].into())
    }
}

pub struct CampaignWavesDifficultyData {
    pub difficulty: CampaignDifficultyData,
    pub waves: Vec<CompleteWaveData>,
}

impl CampaignWavesDifficultyData {
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        let mut buf = Vec::new();
        self.dump(&mut std::io::Cursor::new(&mut buf)).unwrap();
        Typed::Bytes(buf.into())
    }

    fn dump(&self, writer: &mut dyn std::io::Write) -> std::io::Result<usize> {
        writer.write_all(&(CampaignDifficultyData::WRITE_BYTES_LEN as i32).to_le_bytes())?;
        self.difficulty.dump(writer)?;
        writer.write_all(&(self.waves.len() as i32).to_le_bytes())?;
        let mut waves_total_len = 4;
        for wave in self.waves.iter() {
            waves_total_len += wave.dump(writer)?;
        }
        Ok(4 + CampaignDifficultyData::WRITE_BYTES_LEN + waves_total_len)
    }
}

pub struct CompleteWaveData {
    pub player_spawn_location: i32,
    pub robots_in_wave: Vec<CompleteWaveRobotData>,
    pub kill_target: i32,
    pub time_min: i32,
    pub time_max: i32,
}

impl CompleteWaveData {
    fn dump(&self, writer: &mut dyn std::io::Write) -> std::io::Result<usize> {
        writer.write_all(&self.player_spawn_location.to_le_bytes())?;
        writer.write_all(&(self.robots_in_wave.len() as i32).to_le_bytes())?;
        let mut robots_total_len = 4;
        for robot in self.robots_in_wave.iter() {
            robots_total_len += robot.dump(writer)?;
        }
        writer.write_all(&self.kill_target.to_le_bytes())?;
        writer.write_all(&self.time_min.to_le_bytes())?;
        writer.write_all(&self.time_max.to_le_bytes())?;
        Ok(16 + robots_total_len)
    }
}

pub struct CompleteWaveRobotData {
    pub name: String,
    pub robot_data: Vec<u8>,
    pub colour_data: Vec<u8>,
    pub time_to_spawn: i32,
    pub kills_to_spawn: i32,
    pub time_to_despawn: i32,
    pub kills_to_despawn: i32,
    pub initial_robot_amount: i32,
    pub periodic_robot_amount: i32,
    pub spawn_interval: i32,
    pub min_robot_amount: i32,
    pub max_robot_amount: i32,
    pub is_boss: bool,
    pub is_kill_requirement: bool,
}

impl CompleteWaveRobotData {
    fn dump(&self, writer: &mut dyn std::io::Write) -> std::io::Result<usize> {
        let mut total_len = super::write_str_for_binreader(&self.name, writer)?;
        writer.write_all(&(self.robot_data.len() as i32).to_le_bytes())?;
        total_len += 4;
        writer.write_all(&self.robot_data)?;
        total_len += self.robot_data.len();
        writer.write_all(&(self.colour_data.len() as i32).to_le_bytes())?;
        total_len += 4;
        writer.write_all(&self.colour_data)?;
        total_len += self.colour_data.len();
        writer.write_all(&self.time_to_spawn.to_le_bytes())?;
        writer.write_all(&self.kills_to_spawn.to_le_bytes())?;
        writer.write_all(&self.time_to_despawn.to_le_bytes())?;
        writer.write_all(&self.kills_to_despawn.to_le_bytes())?;
        writer.write_all(&self.initial_robot_amount.to_le_bytes())?;
        writer.write_all(&self.periodic_robot_amount.to_le_bytes())?;
        writer.write_all(&self.spawn_interval.to_le_bytes())?;
        writer.write_all(&self.min_robot_amount.to_le_bytes())?;
        writer.write_all(&self.max_robot_amount.to_le_bytes())?;
        writer.write_all(&[self.is_boss as u8])?;
        writer.write_all(&[self.is_kill_requirement as u8])?;
        Ok(total_len)
    }
}
