mod more_auth;
mod chat_ignores;
mod pending_sanctions;

use polariton_server::operations::OperationsHandler;

pub fn handler() -> OperationsHandler<crate::UserTy> {
    OperationsHandler::new()
        .without_state(more_auth::MoreLobbyAuth)
        .without_state(chat_ignores::ignores_provider())
        .without_state(pending_sanctions::pending_sanctions_checker())
        //.without_state(polariton_server::operations::Ack::<00000, _>::default())
}
