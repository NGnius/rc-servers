use polariton_server::operations::{Operation, OperationCode};

pub struct EacChallengeIgnorer;

impl <C> Operation<C> for EacChallengeIgnorer {
    type State = ();
    type User = crate::UserTy;

    fn handle(&self, params: polariton::operation::ParameterTable<C>, _: &mut Self::State, _: &Self::User) -> polariton::operation::OperationResponse<C> {
        polariton::operation::OperationResponse {
            code: 161, // skip the challenge (hopefully)
            return_code: 0,
            message: polariton::operation::Typed::Null,
            params,
        }
    }
}

impl OperationCode for EacChallengeIgnorer {
    fn op_code() -> u8 {
        160
    }
}
