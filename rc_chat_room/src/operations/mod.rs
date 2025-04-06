mod more_auth;
mod chat_ignores;
mod pending_sanctions;
mod all_joined_channels;

use polariton_server::operations::OperationsHandler;

pub fn handler() -> OperationsHandler<crate::UserTy> {
    OperationsHandler::new()
        .add(more_auth::MoreLobbyAuth)
        .add(chat_ignores::ignores_provider())
        .add(pending_sanctions::pending_sanctions_checker())
        .add(all_joined_channels::all_channels_provider())
        .add(polariton_server::operations::Ack::<12, _>::default())
        //.add(polariton_server::operations::Ack::<00000, _>::default())
}
