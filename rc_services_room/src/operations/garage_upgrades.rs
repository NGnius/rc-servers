use polariton::operation::{ParameterTable, OperationResponse};

const CODE: u8 = 1;

const PARAM_KEY: u8 = 1;

pub(super) fn garage_upgrades_provider(conf: &oj_rc_core::ConfigImpl) -> GarageUpgradesProvider {
    GarageUpgradesProvider {
        upgrades: <oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::garage_upgrades(conf),
    }
}

pub struct GarageUpgradesProvider {
    upgrades: oj_rc_core::persist::config::GarageUpgrades,
}

impl GarageUpgradesProvider {
    async fn do_handling(&self, params: ParameterTable) -> Result<ParameterTable, i16> {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, self.upgrades.slot_upgrades());
        Ok(params.into())
    }
}

#[async_trait::async_trait]
impl polariton_server::operations::Operation<()> for GarageUpgradesProvider {
    type User = crate::UserTy;

    async fn handle_async(&self, params: ParameterTable<()>, _user: &Self::User) -> OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(self.do_handling(params).await)
    }
}

impl polariton_server::operations::OperationCode for GarageUpgradesProvider {
    fn op_code() -> u8 {
        CODE
    }
}
