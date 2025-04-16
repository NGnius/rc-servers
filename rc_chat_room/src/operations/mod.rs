mod more_auth;
mod chat_ignores;
mod pending_sanctions;
mod all_joined_channels;
mod send_message;
mod public_channels;
mod join_channel;
mod user_online;

use polariton_server::operations::OperationsHandler;

pub fn handler(chat_system: crate::state::chat::ChatImpl, data_root: impl AsRef<std::path::Path>, conf: &rc_core::persist::config::ConfigImpl) -> OperationsHandler<crate::UserTy> {
    OperationsHandler::new()
        .modify(rc_core::polariton::OpIdCopy)
        .add(more_auth::MoreLobbyAuth::new(chat_system.clone(), data_root))
        .add(chat_ignores::ignores_provider())
        .add(pending_sanctions::pending_sanctions_checker())
        .add(all_joined_channels::all_channels_provider(chat_system.clone()))
        .add(polariton_server::operations::Ack::<12, _>::default())
        .add(send_message::send_public_message_handler(chat_system.clone()))
        .add(public_channels::public_channels_provider(conf))
        .add(join_channel::join_channel_provider(chat_system.clone()))
        .add(user_online::is_online_provider(chat_system.clone()))
        .add(send_message::send_private_message_handler(chat_system.clone()))
        .add(join_channel::leave_channel_provider(chat_system.clone()))
        //.add(polariton_server::operations::Ack::<00000, _>::default())
}

pub(self) fn get_chat_user<'a, C>(user: &'a dyn rc_core::persist::user::User<C>) -> &'a crate::persist::chat_user::ChatUserImpl {
    user.ext(std::any::TypeId::of::<crate::persist::chat_user::ChatUserImpl>()).unwrap().downcast_ref().unwrap()
}
