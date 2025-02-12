use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Dict};

use crate::data::battle_arena_config::*;

const PARAM_KEY: u8 = 1;

pub(super) fn battle_arena_config_provider() -> SimpleFunc<53, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: 115, // str
            val_ty: 104, // obj
            items: vec![
                (Typed::Str("BattleArenaSettings".into()), BattleArenaData {
                    protonium_health: 1_000,
                    respawn_time_seconds: 10,
                    heal_over_time_per_tower: vec![10, 10, 10, 10],
                    base_machine_map: Vec::default(),
                    equalizer_model: Vec::default(),
                    equalizer_health: 1_000_000,
                    equalizer_trigger_time_seconds: vec![10, 10, 10, 10, 10],
                    equalizer_warning_seconds: 10,
                    equalizer_duration_seconds: vec![20, 20, 20, 20, 20],
                    capture_time_seconds_per_player: vec![30, 20, 10, 5, 1],
                    num_segments: 4,
                    heal_escalation_time_seconds: 5,
                }.as_transmissible())
            ],
        }));
        Ok(params.into())
    })
}
