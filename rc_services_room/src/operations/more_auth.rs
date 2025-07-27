use polariton::operation::Typed;
use polariton_server::operations::{Operation, OperationCode};

pub struct MoreLobbyAuth;

impl MoreLobbyAuth {
    const AUTH_PAYLOAD_KEY: u8 = 245;
}

#[async_trait::async_trait]
impl <C: Send + 'static> Operation<C> for MoreLobbyAuth {
    type User = crate::UserTy;

    async fn handle_async(&self, params: polariton::operation::ParameterTable<C>, user: &Self::User) -> polariton::operation::OperationResponse<C> {
        let params_dict = params.to_dict();
        if let Some(Typed::Str(auth_payload)) = params_dict.get(&Self::AUTH_PAYLOAD_KEY) {
            //let mut write_lock = user.write().unwrap();
            if user.update_with_auth(&auth_payload.string).await {
                if user.user().unwrap().is_banned() {
                    return polariton::operation::OperationResponse {
                        code: Self::op_code(),
                        return_code: oj_rc_core::data::error_codes::WebServicesError::Banned as i16,
                        message: polariton::operation::Typed::Null,
                        params: polariton::operation::ParameterTable::with_capacity(0),
                    }
                } else {
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
        }
        polariton::operation::OperationResponse {
            code: Self::op_code(),
            return_code: 120,
            message: polariton::operation::Typed::Null,
            params: std::collections::HashMap::new().into(),
        }
    }

    fn handle(&self, _params: polariton::operation::ParameterTable<C>, _user: &Self::User) -> polariton::operation::OperationResponse<C> {
        unreachable!()
    }
}

impl OperationCode for MoreLobbyAuth {
    fn op_code() -> u8 {
        230
    }
}
