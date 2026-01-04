use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};
use oj_rc_factory::VehicleFactoryAdapter;

const CODE: u8 = 90;

const SLOT_PARAM_KEY: u8 = 43; // int; in
const COMBAT_PARAM_KEY: u8 = 97; // int; in
const COSMETIC_PARAM_KEY: u8 = 98; // bool; int
//const BUILD_NUMBER_PARAM_KEY: u8 = 99; // str; in

pub(super) struct CrfItemRater {
    factory: std::sync::Arc<oj_rc_core::factory::Factory>,
}

#[async_trait::async_trait]
impl SimpleOperation<()> for CrfItemRater {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable, user: &Self::User) -> Result<ParameterTable, SimpleOpError> {
        if let Some(Typed::Int(slot)) = params.remove(&SLOT_PARAM_KEY) {
            if let Some(Typed::Int(combat_rating)) = params.remove(&COMBAT_PARAM_KEY) {
                if let Some(Typed::Int(cosmetic_rating)) = params.remove(&COSMETIC_PARAM_KEY) {
                    log::warn!("Slot {}", slot);
                    let user_info = user.user()?;
                    if let Some(factory_id) = user_info.rate_vehicle(slot, combat_rating, cosmetic_rating).await? {
                        self.factory.rate_vehicle(factory_id, combat_rating, cosmetic_rating).await.map_err(|e| {
                            log::error!("Failed to rate factory vehicle {}: {}", factory_id, e);
                            SimpleOpError::with_message(
                                oj_rc_core::data::error_codes::WebServicesError::DatabaseError as i16,
                                format!("Failed to rate factory vehicle: {}", e),
                            )
                        })?;
                    }
                }
            }
        }
        Ok(params)
    }
}

pub(super) fn crf_rating_provider(factory: &std::sync::Arc<oj_rc_core::factory::Factory>) -> SimpleOpImpl<(), crate::UserTy, CrfItemRater> {
    SimpleOpImpl::new(CrfItemRater {
        factory: factory.to_owned(),
    })
}
