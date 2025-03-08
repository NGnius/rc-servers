use std::collections::HashMap;

use polariton_server::operations::{Operation, OperationCode};

pub struct MaintenanceModeTeller;

impl <C> Operation<C> for MaintenanceModeTeller {
    type State = ();
    type User = crate::UserTy;

    fn handle(&self, _: polariton::operation::ParameterTable<C>, _: &mut Self::State, _: &Self::User) -> polariton::operation::OperationResponse<C> {
        let mut resp_params = HashMap::new();
        resp_params.insert(20 /* is in maintenance mode? */, polariton::operation::Typed::Bool(false.into()));
        resp_params.insert(19 /* maintenace mode message */, polariton::operation::Typed::Str("OpenJam's servers are currently undergoing maintenance".into()));
        polariton::operation::OperationResponse {
            code: 20,
            return_code: 0,
            message: polariton::operation::Typed::Null,
            params: resp_params.into(),
        }
    }
}

impl OperationCode for MaintenanceModeTeller {
    fn op_code() -> u8 {
        20
    }
}
