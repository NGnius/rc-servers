use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};
use oj_rc_factory::VehicleFactoryAdapter;

const CODE: u8 = 91;

const ID_PARAM_KEY: u8 = 94; // int; in
const EARNINGS_PARAM_KEY: u8 = 96;

pub(super) struct FactoryRemoveVehicle {
    factory: std::sync::Arc<oj_rc_core::factory::Factory>,
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for FactoryRemoveVehicle {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        if let Some(Typed::Int(factory_id)) = params.remove(&ID_PARAM_KEY) {
            let user_info = user.user()?;
            let user_id = user_info.account_id();
            self.factory.remove_vehicle(factory_id, user_id).await.map_err(|e| {
                log::error!("Failed to remove vehicle {} by user {} from factory: {}", factory_id, user_id, e);
                SimpleOpError::with_message(
                    oj_rc_core::data::error_codes::WebServicesError::DatabaseError as i16,
                    format!("Failed to remove vehicle (for offset) from factory: {}", e)
                )
            })?;
        }
        // TODO Factory earnings
        params.insert(EARNINGS_PARAM_KEY, Typed::HashMap(vec![
            (Typed::Str("buyCount".into()), Typed::Int(0)),
            (Typed::Str("earnings".into()), Typed::Int(0)),
        ].into()));
        Ok(params)
    }
}

pub(super) fn factory_remove_provider<C: Send + 'static>(factory: &std::sync::Arc<oj_rc_core::factory::Factory>) -> SimpleOpImpl<C, crate::UserTy, FactoryRemoveVehicle> {
    SimpleOpImpl::new(FactoryRemoveVehicle {
        factory: factory.to_owned()
    })
}
