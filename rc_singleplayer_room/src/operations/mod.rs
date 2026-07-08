mod more_auth;
mod eac;
mod load_ai_robots;
mod save_result;

use polariton_server::operations::OperationsHandler;

pub fn handler(init_ctx: &crate::InitConfig) -> OperationsHandler<crate::UserTy> {
    OperationsHandler::<crate::UserTy>::new()
        .modify(<oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::polariton_operation_modifier(&init_ctx.config))
        .add(more_auth::MoreLobbyAuth)
        .add(eac::EacChallengeIgnorer)
        .add(load_ai_robots::tdm_machines_provider(&init_ctx.factory, init_ctx.parsers.weapon_order(), &init_ctx.config, init_ctx.parsers.cpu_counter()))
        //.add(polariton_server::operations::Ack::<33, _>::default()) // get user clan info (this is equivalent to not being in a clan)
        //.add(polariton_server::operations::Ack::<2, _>::default()) // Save singleplayer result (parameter-less response)
        .add(save_result::save_result_provider())
}
