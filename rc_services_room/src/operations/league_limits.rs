use polariton_server::operations::Immediate;
use polariton::{operation::{Dict, ParameterTable, Typed}, serdes::TypePrefix};

const PARAM_KEY: u8 = 1;

pub(super) fn league_battle_parameters_provider(conf: &oj_rc_core::ConfigImpl) -> Immediate<57, crate::UserTy> {
    let multiplayer_conf = <oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::multiplayer_settings(conf);
    let player_level = multiplayer_conf.min_level;
    let cpu = multiplayer_conf.min_cpu;
    Immediate::new(move || {
        let mut params = ParameterTable::with_capacity(3);
        params.insert(PARAM_KEY, Typed::Dict(Dict {
            key_ty: TypePrefix::Str, //str
            val_ty: TypePrefix::Any, // obj
            items: vec![
                (Typed::Str("playerLevelRequired".into()), Typed::Int(player_level)),
                (Typed::Str("minCpu".into()), Typed::Int(cpu)),
            ],
        }));
        params
    })
}
