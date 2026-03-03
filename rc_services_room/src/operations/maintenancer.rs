use std::collections::HashMap;

use polariton_server::operations::{Operation, OperationCode};

pub struct MaintenanceModeTeller {
    message: Option<String>,
}

impl <C: Send + 'static> Operation<C> for MaintenanceModeTeller {
    type User = crate::UserTy;

    fn handle(&self, _: polariton::operation::ParameterTable<C>, _: &Self::User) -> polariton::operation::OperationResponse<C> {
        let mut resp_params = HashMap::new();
        resp_params.insert(20 /* is in maintenance mode? */, polariton::operation::Typed::Bool(self.message.is_some()));
        let message = self.message.clone().unwrap_or_default();
        resp_params.insert(19 /* maintenace mode message */, polariton::operation::Typed::Str(message.into()));
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

pub fn maintenace_teller(conf: &oj_rc_core::ConfigImpl) -> MaintenanceModeTeller {
    MaintenanceModeTeller {
        message: <oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::server_config(conf).maintenance_message,
    }
}
