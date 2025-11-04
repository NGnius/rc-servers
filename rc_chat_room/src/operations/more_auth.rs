use polariton::operation::Typed;
use polariton_server::operations::{Operation, OperationCode};

//use crate::persist::chat_user::{ChatUser, ChatUserImpl};
//use oj_rc_core::persist::user::ChatUser;

pub struct MoreLobbyAuth;

impl MoreLobbyAuth {
    const AUTH_PAYLOAD_KEY: u8 = 245;

    #[inline]
    pub fn new() -> Self {
        Self
    }

    /*fn build_ext_map(&self, token: &oj_rc_core::persist::user::UserToken) -> Option<std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any + Send + Sync + 'static>>> {
        let user_dir = self.root.join(oj_rc_core::persist::user::USERS_DIR).join(&token.uuid);
        let data = if let Ok(data) = ChatUserImpl::load(&user_dir) {
            data
        } else {
            let data = ChatUserImpl::default_load(&user_dir);
            data
        };
        let mut map = std::collections::HashMap::with_capacity(1);
        map.insert(std::any::TypeId::of::<ChatUserImpl>(), Box::new(data) as _);
        Some(map)
    }*/

    async fn do_auth<C>(&self, params: std::collections::HashMap<u8, Typed<C>>, user: &crate::UserTy) -> Result<polariton::operation::ParameterTable<C>, i16> {
        if let Some(Typed::Str(auth_payload)) = params.get(&Self::AUTH_PAYLOAD_KEY) {
            if user.update_with_auth(&auth_payload.string).await {
                //let user_impl = user.user()?;
                //let name = user_impl.public_id().to_owned();
                //let chat_user = super::get_chat_user(user_impl.as_ref().as_ref());
                //let channels = user_impl.subscribed_channels_strings().await?;
                //let event_tx = user.event_chann();
                //self.chat_system.system_mut().connect_user(name, channels, event_tx);
                crate::update_status(user.user().unwrap().as_ref().as_ref()).await;
                let mut resp_params = std::collections::HashMap::new();
                resp_params.insert(Self::AUTH_PAYLOAD_KEY, polariton::operation::Typed::Byte(0));
                return Ok(resp_params.into());
            }
        }
        Err(120)
    }
}

#[async_trait::async_trait]
impl <C: Send + 'static> Operation<C> for MoreLobbyAuth {
    type User = crate::UserTy;

    async fn handle_async(&self, params: polariton::operation::ParameterTable<C>, user: &Self::User) -> polariton::operation::OperationResponse<C> {
        let params_dict = params.to_dict();
        match self.do_auth(params_dict, user).await {
            Ok(params) => {
                polariton::operation::OperationResponse {
                    code: Self::op_code(),
                    return_code: 0,
                    message: polariton::operation::Typed::Null,
                    params,
                }
            },
            Err(code) => {
                polariton::operation::OperationResponse {
                    code: Self::op_code(),
                    return_code: code,
                    message: polariton::operation::Typed::Null,
                    params: std::collections::HashMap::new().into(),
                }
            }
        }
    }
}

impl OperationCode for MoreLobbyAuth {
    fn op_code() -> u8 {
        230
    }
}
