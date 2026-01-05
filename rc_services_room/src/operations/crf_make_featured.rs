use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};
use oj_rc_factory::VehicleFactoryAdapter;

const CODE: u8 = 99;

const ID_PARAM_KEY: u8 = 94;

pub(super) struct FactorySetFeatured {
    factory: std::sync::Arc<oj_rc_core::factory::Factory>,
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for FactorySetFeatured {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        if let Some(Typed::Int(factory_id)) = params.remove(&ID_PARAM_KEY) {
            let user_info = user.user()?;
            if user_info.is_dev() {
                self.factory.set_featured(factory_id, true).await.map_err(|e| {
                    log::error!("Failed to make featured vehicle {} (for offset) from factory: {}", factory_id, e);
                    SimpleOpError::with_message(
                        oj_rc_core::data::error_codes::WebServicesError::DatabaseError as i16,
                        format!("Failed to make featured vehicle (for offset) from factory: {}", e)
                    )
                })?;
            }
        }
        Ok(ParameterTable::with_capacity(1))
    }
}

pub(super) fn factory_featured_provider<C: Send + 'static>(factory: &std::sync::Arc<oj_rc_core::factory::Factory>) -> SimpleOpImpl<C, crate::UserTy, FactorySetFeatured> {
    SimpleOpImpl::new(FactorySetFeatured {
        factory: factory.to_owned()
    })
}
