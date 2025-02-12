use polariton::operation::Typed;
use base64::{Engine, engine::general_purpose::STANDARD};

pub struct BattleArenaData {
    pub protonium_health: i64,
    pub respawn_time_seconds: i64,
    pub heal_over_time_per_tower: Vec<u64>,
    pub base_machine_map: Vec<u8>, // aka team base model, converted into base64
    pub equalizer_model: Vec<u8>, // converted into base64
    pub equalizer_health: i64,
    pub equalizer_trigger_time_seconds: Vec<u64>,
    pub equalizer_warning_seconds: i64,
    pub equalizer_duration_seconds: Vec<u64>,
    pub capture_time_seconds_per_player: Vec<i64>,
    pub num_segments: i32,
    pub heal_escalation_time_seconds: i64,
}

fn to_obj_arr_u(slice: &[u64]) -> Typed {
    Typed::ObjArr(slice.iter().map(|x| Typed::Long(*x as i64)).collect::<Vec<Typed>>().into())
}

fn to_obj_arr_i(slice: &[i64]) -> Typed {
    Typed::ObjArr(slice.iter().map(|x| Typed::Long(*x)).collect::<Vec<Typed>>().into())
}

impl BattleArenaData {
    pub fn as_transmissible(&self) -> Typed {
        Typed::HashMap(vec![
            (Typed::Str("protoniumHealth".into()), Typed::Long(self.protonium_health)),
            (Typed::Str("respawnTimeSeconds".into()), Typed::Long(self.respawn_time_seconds)),
            (Typed::Str("healOverTimePerTower".into()), to_obj_arr_u(&self.heal_over_time_per_tower)),
            (Typed::Str("baseMachineMap".into()), Typed::Str(STANDARD.encode(&self.base_machine_map).into())),
            (Typed::Str("equalizerModel".into()), Typed::Str(STANDARD.encode(&self.equalizer_model).into())),
            (Typed::Str("equalizerHealth".into()), Typed::Long(self.equalizer_health)),
            (Typed::Str("equalizerTriggerTimeSeconds".into()), to_obj_arr_u(&self.equalizer_trigger_time_seconds)),
            (Typed::Str("equalizerWarningSeconds".into()), Typed::Long(self.equalizer_warning_seconds)),
            (Typed::Str("equalizerDurationSeconds".into()), to_obj_arr_u(&self.equalizer_duration_seconds)),
            (Typed::Str("captureTimeSecondsPerPlayer".into()), to_obj_arr_i(&self.capture_time_seconds_per_player)),
            (Typed::Str("numSegments".into()), Typed::Int(self.num_segments)),
            (Typed::Str("healEscalationTimeSeconds".into()), Typed::Long(self.heal_escalation_time_seconds)),
        ].into())
    }
}
