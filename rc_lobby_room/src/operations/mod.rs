mod more_auth;
//mod eac;
mod no_quit;
mod join_queue;
mod reconnect_game;

use polariton_server::operations::OperationsHandler;

pub fn handler(init_ctx: &crate::InitConfig) -> OperationsHandler<crate::UserTy> {
    OperationsHandler::<crate::UserTy>::new()
        .modify(<oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::polariton_operation_modifier(&init_ctx.config))
        .add(more_auth::MoreLobbyAuth)
        //.add(eac::EacChallengeIgnorer)
        .add(polariton_server::operations::Ack::<2, _>::default())
        .add(no_quit::quit_blocker_provider())
        .add(join_queue::join_queue_provider(&init_ctx.queue))
        .add(reconnect_game::reconnect_provider(&init_ctx.config, &init_ctx.queue))
}
