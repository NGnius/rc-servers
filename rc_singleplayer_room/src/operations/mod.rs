mod more_auth;
mod eac;
mod load_ai_robots;

use polariton_server::operations::OperationsHandler;

pub fn handler() -> OperationsHandler<crate::UserTy> {
    OperationsHandler::<crate::UserTy>::new()
        .without_state(more_auth::MoreLobbyAuth)
        .without_state(eac::EacChallengeIgnorer)
        .without_state(load_ai_robots::tdm_machines_provider())
        //.without_state(polariton_server::operations::Ack::<33, _>::default()) // get user clan info (this is equivalent to not being in a clan)
}
