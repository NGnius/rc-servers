mod more_auth;
mod eac;
mod no_quit;
mod join_queue;

use polariton_server::operations::OperationsHandler;

pub fn handler(_init_ctx: &crate::InitConfig) -> OperationsHandler<crate::UserTy> {
    OperationsHandler::<crate::UserTy>::new()
        .modify(oj_rc_core::polariton::OpIdCopy)
        .add(more_auth::MoreLobbyAuth)
        //.add(eac::EacChallengeIgnorer)
        //.add(polariton_server::operations::Ack::<2, _>::default())
        .add(no_quit::quit_blocker_provider())
        .add(join_queue::join_queue_provider())
}
