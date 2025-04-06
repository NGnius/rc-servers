use polariton_server::operations::{Operation, OperationCode};

pub struct VersionTeller;

impl VersionTeller {
    const VERSION_NUMBER_KEY: u8 = 112;
    const LATEST_VERSION: i32 = 2855;
}

impl <C: Send + 'static> Operation<C> for VersionTeller {
    type User = crate::UserTy;

    fn handle(&self, _: polariton::operation::ParameterTable<C>, _: &Self::User) -> polariton::operation::OperationResponse<C> {
        let mut resp_params = std::collections::HashMap::new();
        resp_params.insert(Self::VERSION_NUMBER_KEY, polariton::operation::Typed::Int(Self::LATEST_VERSION));
        polariton::operation::OperationResponse {
            code: 103,
            return_code: 0,
            message: polariton::operation::Typed::Null,
            params: resp_params.into(),
        }
    }
}

impl OperationCode for VersionTeller {
    fn op_code() -> u8 {
        103
    }
}
