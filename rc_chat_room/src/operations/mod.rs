mod more_auth;
mod chat_ignores;
mod pending_sanctions;
mod all_joined_channels;
mod send_message;
mod public_channels;
mod join_channel;
mod user_online;
mod subscribed_channels;
mod add_modify_sanction;
mod list_sanctions;

use polariton_server::operations::OperationsHandler;

pub fn handler(chat_system: crate::state::chat::ChatImpl, conf: &oj_rc_core::persist::config::ConfigImpl) -> OperationsHandler<crate::UserTy> {
    OperationsHandler::new()
        .modify(oj_rc_core::polariton::RcOpModifier)
        .add(more_auth::MoreLobbyAuth::new())
        .add(chat_ignores::ignores_provider())
        .add(pending_sanctions::pending_sanctions_checker())
        .add(all_joined_channels::all_channels_provider(chat_system.clone()))
        //.add(polariton_server::operations::Ack::<12, _>::default())
        .add(send_message::send_public_message_handler(chat_system.clone()))
        .add(public_channels::public_channels_provider(conf))
        .add(join_channel::join_channel_provider(chat_system.clone()))
        .add(user_online::is_online_provider(chat_system.clone()))
        .add(send_message::send_private_message_handler(chat_system.clone()))
        .add(join_channel::leave_channel_provider(chat_system.clone()))
        .add(subscribed_channels::all_subbed_channels_provider())
        .add(add_modify_sanction::add_modify_sanction_provider())
        .add(list_sanctions::list_sanctions_provider())
        //.add(polariton_server::operations::Ack::<00000, _>::default())
}
