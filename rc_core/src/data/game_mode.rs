use polariton::{operation::Typed, serdes::TypePrefix};

pub struct GameModeConfig {
    pub respawn_heal_duration: f32,
    pub respawn_full_heal_duration: f32,
    pub kill_limit: i32,
    pub game_time_minutes: i32,
}

impl GameModeConfig {
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        Typed::HashMap(vec![
            (Typed::Str("respawnHealDuration".into()), Typed::Float(self.respawn_heal_duration)),
            (Typed::Str("respawnFullHealDuration".into()), Typed::Float(self.respawn_full_heal_duration)),
            (Typed::Str("killLimit".into()), Typed::Int(self.kill_limit)),
            (Typed::Str("gameTimeMinutes".into()), Typed::Int(self.game_time_minutes)),
        ].into())
    }
}

pub struct GameModeConfigs {
    pub battle_arena: GameModeConfig,
    pub elimination: GameModeConfig,
    pub the_pit: GameModeConfig,
    pub team_deathmatch: GameModeConfig,
}

impl GameModeConfigs {
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        Typed::Dict(polariton::operation::Dict {
            key_ty: TypePrefix::Str,
            val_ty: TypePrefix::HashMap,
            items: vec![
                (Typed::Str("BattleArena".into()), self.battle_arena.as_transmissible()),
                (Typed::Str("Elimination".into()), self.elimination.as_transmissible()),
                (Typed::Str("ThePit".into()), self.the_pit.as_transmissible()),
                (Typed::Str("TeamDeathmatch".into()), self.team_deathmatch.as_transmissible()),
            ],
        })
    }
}
