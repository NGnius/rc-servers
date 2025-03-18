use polariton::operation::Typed;

pub struct ScoreMultipliersData {
    pub max_cpu: f32,
    pub stat_multipliers: std::collections::HashMap<InGameStat, ScoreMultiplier>,
    pub completed_battle_base_multiplier: f32,
    pub completed_battle_bonus_multiplier: f32,
    pub delta_scaler: f32,
    pub defeat_score: u32,
    pub victory_score: u32,
    pub max_score_ratio: f32,
}

impl ScoreMultipliersData {
    fn dump(&self, writer: &mut dyn std::io::Write) -> std::io::Result<usize> {
        writer.write_all(&self.max_cpu.to_le_bytes())?;
        for (key, val) in self.stat_multipliers.iter() {
            writer.write_all(&(*key as u32).to_le_bytes())?;
            writer.write_all(&val.base.to_le_bytes())?;
            writer.write_all(&val.bonus.to_le_bytes())?;
        }
        writer.write_all(&self.completed_battle_base_multiplier.to_le_bytes())?;
        writer.write_all(&self.completed_battle_bonus_multiplier.to_le_bytes())?;
        writer.write_all(&self.delta_scaler.to_le_bytes())?;
        writer.write_all(&self.defeat_score.to_le_bytes())?;
        writer.write_all(&self.victory_score.to_le_bytes())?;
        writer.write_all(&self.max_score_ratio.to_le_bytes())?;
        Ok(28 + (12 * self.stat_multipliers.len()))
    }

    pub fn as_transmissible<C>(&self) -> Typed<C> {
        let mut buf = Vec::new();
        self.dump(&mut std::io::Cursor::new(&mut buf)).unwrap();
        Typed::Bytes(buf.into())
    }
}

pub struct ScoreMultiplier {
    pub base: f32,
    pub bonus: f32,
}

impl std::default::Default for ScoreMultiplier {
    fn default() -> Self {
        Self {
            base: 0.5,
            bonus: 0.9
        }
    }
}

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InGameStat {
    None = 0,
    DestroyedCubes = 1,
    DestroyedCubesInProtection = 2,
    DestroyedCubesDefendingTheBase = 3,
    Kill = 4,
    KillAssist = 5,
    HealCubes = 6,
    HealAssist = 7,
    DestroyedProtoniumCubes = 8,
    BaseCaptureClassicMode = 9,
    RobotDestroyed = 10,
    Score = 11,
    HealthPercentageBonusClassicMode = 12,
    Points = 13,
    CurrentKillStreak = 14,
    BestKillStreak = 15,
    CapturePointBattleArenaMode = 16,
    EqualiserDestroyedBattleArenaMode = 17,
    BattleArenaObjectives = 18,
}

impl std::default::Default for ScoreMultipliersData {
    fn default() -> Self {
        Self {
            max_cpu: 1000.0,
            stat_multipliers: vec![
                (InGameStat::DestroyedCubes, ScoreMultiplier::default()),
                (InGameStat::DestroyedCubesInProtection, ScoreMultiplier::default()),
                (InGameStat::DestroyedCubesDefendingTheBase, ScoreMultiplier::default()),
                (InGameStat::Kill, ScoreMultiplier::default()),
                (InGameStat::KillAssist, ScoreMultiplier::default()),
                (InGameStat::HealCubes, ScoreMultiplier::default()),
                (InGameStat::HealAssist, ScoreMultiplier::default()),
                (InGameStat::DestroyedProtoniumCubes, ScoreMultiplier::default()),
                (InGameStat::BaseCaptureClassicMode, ScoreMultiplier::default()),
                (InGameStat::RobotDestroyed, ScoreMultiplier::default()),
                (InGameStat::Score, ScoreMultiplier::default()),
                (InGameStat::HealthPercentageBonusClassicMode, ScoreMultiplier::default()),
                (InGameStat::Points, ScoreMultiplier::default()),
                (InGameStat::CurrentKillStreak, ScoreMultiplier::default()),
                (InGameStat::BestKillStreak, ScoreMultiplier::default()),
                (InGameStat::CapturePointBattleArenaMode, ScoreMultiplier::default()),
                (InGameStat::EqualiserDestroyedBattleArenaMode, ScoreMultiplier::default()),
                (InGameStat::BattleArenaObjectives, ScoreMultiplier::default()),
            ].into_iter().collect(),
            completed_battle_base_multiplier: 1.0,
            completed_battle_bonus_multiplier: 1.2,
            delta_scaler: 0.5,
            defeat_score: 500,
            victory_score: 2_000,
            max_score_ratio: 2.0,
        }
    }
}
