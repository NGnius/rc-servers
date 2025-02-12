use polariton::operation::Dict;
use polariton_server::operations::{Operation, OperationCode};

pub struct NoAnalytics;

impl NoAnalytics {
    const ANALYTICS_DICT_KEY: u8 = 83;
}

impl Operation for NoAnalytics {
    type State = ();
    type User = crate::UserTy;

    fn handle(&self, _: polariton::operation::ParameterTable, _: &mut Self::State, _: &Self::User) -> polariton::operation::OperationResponse {
        let mut resp_params = std::collections::HashMap::new();
        resp_params.insert(Self::ANALYTICS_DICT_KEY, polariton::operation::Typed::Dict(Dict {
            key_ty: 115, // str
            val_ty: 115, // str
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
