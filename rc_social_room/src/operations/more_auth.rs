use polariton::operation::Typed;
use polariton_server::operations::{Operation, OperationCode};

pub fn more_lobby_auth(init_ctx: &crate::InitConfig) -> MoreLobbyAuth {
    MoreLobbyAuth {
        social: init_ctx.social.clone(),
    }
}

pub struct MoreLobbyAuth {
    social: std::sync::Arc<crate::SocialMesh>,
}

impl MoreLobbyAuth {
    const AUTH_PAYLOAD_KEY: u8 = 245;
}

#[async_trait::async_trait]
impl <C: Send + 'static> Operation<C> for MoreLobbyAuth {
    type User = crate::UserTy;

    async fn handle_async(&self, params: polariton::operation::ParameterTable<C>, user: &Self::User) -> polariton::operation::OperationResponse<C> {
        let params_dict = params.to_dict();
        if let Some(Typed::Str(auth_payload)) = params_dict.get(&Self::AUTH_PAYLOAD_KEY) {
            if user.update_with_auth(&auth_payload.string).await {
                self.social.add_user(
                    user.user().unwrap().public_id().to_owned(),
                    user.event_sender().to_owned().downgrade(),
                ).await;
                crate::update_status(user.user().unwrap().as_ref().as_ref()).await;
                let mut resp_params = std::collections::HashMap::with_capacity(1);
                resp_params.insert(Self::AUTH_PAYLOAD_KEY, polariton::operation::Typed::Byte(0));
                return polariton::operation::OperationResponse {
                    code: Self::op_code(),
                    return_code: 0,
                    message: polariton::operation::Typed::Null,
                    params: resp_params.into(),
                }
            }
        }
        polariton::operation::OperationResponse {
            code: Self::op_code(),
            return_code: 120,
            message: polariton::operation::Typed::Null,
            params: std::collections::HashMap::new().into(),
        }
    }
}

impl OperationCode for MoreLobbyAuth {
    fn op_code() -> u8 {
        230
    }
}
