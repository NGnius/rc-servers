mod more_auth;
mod eac;
mod load_ai_robots;

use polariton_server::operations::OperationsHandler;

pub fn handler() -> OperationsHandler<crate::UserTy> {
    OperationsHandler::<crate::UserTy>::new()
        .add(more_auth::MoreLobbyAuth)
        .add(eac::EacChallengeIgnorer)
        .add(load_ai_robots::tdm_machines_provider())
        //.add(polariton_server::operations::Ack::<33, _>::default()) // get user clan info (this is equivalent to not being in a clan)
}
