use polariton::operation::{ParameterTable, Typed};
use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};

const MAX_MESSAGE_LEN: usize = 512;

const CHANNEL_TYPE_PARAM_KEY: u8 = 1; // in; int
const MESSAGE_TEXT_PARAM_KEY: u8 = 2; // in; str
const CHANNEL_NAME_PARAM_KEY: u8 = 3; // in; str
const CHAT_LOCATION_PARAM_KEY: u8 = 29; // in; str

pub(super) struct PublicMessageSender {
    chat: crate::state::chat::ChatImpl
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for PublicMessageSender {
    type User = crate::UserTy;
    const CODE: u8 = 2;

    async fn handle(&self, params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let mut params = params.to_dict();
        if let Some(Typed::Int(channel_ty)) = params.remove(&CHANNEL_TYPE_PARAM_KEY) {
            let channel_enum = crate::data::channel::ChatChannelType::from_u8(channel_ty as u8)?;
            if let Some(Typed::Str(channel_name)) = params.remove(&CHANNEL_NAME_PARAM_KEY) {
                if let Some(Typed::Str(message_text)) = params.remove(&MESSAGE_TEXT_PARAM_KEY) {
                    let user = user.user()?;
                    if message_text.string.len() > MAX_MESSAGE_LEN {
                        log::warn!("Rejecting too long chat message from {}", user.public_id());
                        return Err((oj_rc_core::data::error_codes::ChatErrorCodes::Flood as i16).into())
                    }
                    let chat_loc = if let Some(Typed::Str(chat_loc)) = params.remove(&CHAT_LOCATION_PARAM_KEY) {
                        chat_loc.string.clone()
                    } else {
                        "<unknown location>".to_owned()
                    };
                    let chat_system = self.chat.system().await;
                    log::debug!("Got message `{}` from user {} ({} @ {}/{:?})", message_text.string, user.public_id(), channel_name.string, chat_loc, channel_enum);
                    chat_system.handle_public_message(user.as_ref().as_ref(), message_text.string, channel_name.string, channel_enum).await;
                }
            }
        }
        Ok(params.into())
    }
}

pub(super) fn send_public_message_handler<C: Send + 'static>(chat_system: crate::state::chat::ChatImpl) -> SimpleOpImpl<C, crate::UserTy, PublicMessageSender> {
    SimpleOpImpl::new(PublicMessageSender {
        chat: chat_system,
    })
}

pub const USERNAME_PARAM_KEY: u8 = 7; // in; str

pub(super) struct PrivateMessageSender {
    chat: crate::state::chat::ChatImpl
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for PrivateMessageSender {
    type User = crate::UserTy;
    const CODE: u8 = 3;

    async fn handle(&self, params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let mut params = params.to_dict();
        if let Some(Typed::Str(username)) = params.remove(&USERNAME_PARAM_KEY) {
            if let Some(Typed::Str(message_text)) = params.remove(&MESSAGE_TEXT_PARAM_KEY) {
                let user = user.user()?;
                if message_text.string.len() > MAX_MESSAGE_LEN {
                    log::warn!("Rejecting too long chat message from {}", user.public_id());
                    return Err((oj_rc_core::data::error_codes::ChatErrorCodes::Flood as i16).into())
                }
                let chat_loc = if let Some(Typed::Str(chat_loc)) = params.remove(&CHAT_LOCATION_PARAM_KEY) {
                    chat_loc.string.clone()
                } else {
                    "<unknown location>".to_owned()
                };
                let chat_system = self.chat.system().await;
                log::debug!("Got message `{}` from user {} (@ {} to {})", message_text.string, user.public_id(), chat_loc, username.string);
                chat_system.handle_private_message(user.as_ref().as_ref(), message_text.string, username.string).await;
            }
        }
        Ok(params.into())
    }
}

pub fn send_private_message_handler<C: Send + 'static>(chat_system: crate::state::chat::ChatImpl) -> SimpleOpImpl<C, crate::UserTy, PrivateMessageSender> {
    SimpleOpImpl::new(PrivateMessageSender {
        chat: chat_system,
    })
}
