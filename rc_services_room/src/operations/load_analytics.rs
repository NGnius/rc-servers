use polariton::{operation::Dict, serdes::TypePrefix};
use polariton_server::operations::{Operation, OperationCode};

pub struct NoAnalytics;

impl NoAnalytics {
    const ANALYTICS_DICT_KEY: u8 = 83;
}

impl <C> Operation<C> for NoAnalytics {
    type State = ();
    type User = crate::UserTy;

    fn handle(&self, _: polariton::operation::ParameterTable<C>, _: &mut Self::State, _: &Self::User) -> polariton::operation::OperationResponse<C> {
        let mut resp_params = std::collections::HashMap::new();
        resp_params.insert(Self::ANALYTICS_DICT_KEY, polariton::operation::Typed::Dict(Dict {
            key_ty: TypePrefix::Str, // str
            val_ty: TypePrefix::Str, // str
            items: Vec::new(),
        }));
        polariton::operation::OperationResponse {
            code: 70,
            return_code: 0,
            message: polariton::operation::Typed::Null,
            params: resp_params.into(),
        }
    }
}

impl OperationCode for NoAnalytics {
    fn op_code() -> u8 {
        70
    }
}
