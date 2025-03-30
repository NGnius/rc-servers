use polariton::operation::Typed;
use polariton_server::operations::{Operation, OperationCode};

pub struct MoreLobbyAuth;

impl MoreLobbyAuth {
    const AUTH_PAYLOAD_KEY: u8 = 245;
}

impl <C> Operation<C> for MoreLobbyAuth {
    type State = ();
    type User = crate::UserTy;

    fn handle(&self, params: polariton::operation::ParameterTable<C>, _: &mut Self::State, user: &Self::User) -> polariton::operation::OperationResponse<C> {
        let params_dict = params.to_dict();
        if let Some(Typed::Str(auth_payload)) = params_dict.get(&Self::AUTH_PAYLOAD_KEY) {
            if user.update_with_auth(&auth_payload.string) {
                let mut resp_params = std::collections::HashMap::new();
                resp_params.insert(Self::AUTH_PAYLOAD_KEY, polariton::operation::Typed::Byte(0));
                return polariton::operation::OperationResponse {
                    code: 230,
                    return_code: 0,
                    message: polariton::operation::Typed::Null,
                    params: resp_params.into(),
                }
            }
        }
        polariton::operation::OperationResponse {
            code: 230,
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
