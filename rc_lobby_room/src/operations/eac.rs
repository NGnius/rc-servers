use polariton_server::operations::{Operation, OperationCode};

pub struct EacChallengeIgnorer;

impl <C: Send + 'static> Operation<C> for EacChallengeIgnorer {
    type User = crate::UserTy;

    fn handle(&self, params: polariton::operation::ParameterTable<C>, _: &Self::User) -> polariton::operation::OperationResponse<C> {
        polariton::operation::OperationResponse {
            code: 5, // skip the challenge (hopefully)
            return_code: 0,
            message: polariton::operation::Typed::Null,
            params,
        }
    }
}

impl OperationCode for EacChallengeIgnorer {
    fn op_code() -> u8 {
        4
    }
}
