use polariton_server::operations::{Operation, OperationCode};
use polariton::operation::{Typed, ParameterTable};
use rc_factory::VehicleFactoryAdapter;

const CODE: u8 = 87;

const ID_PARAM_KEY: u8 = 94;
const DATA_PARAM_KEY: u8 = 95;

async fn do_handling(params: ParameterTable<()>, _user: &crate::UserTy, factory: &std::sync::Arc<rc_core::factory::Factory>) -> Result<ParameterTable, i16> {
    let mut params = params.to_dict();
    if let Some(Typed::Int(id)) = params.remove(&ID_PARAM_KEY) {
        let vehicle = factory.vehicle(id as _).await.map_err(|e| {
            log::error!("Failed to retrieve vehicle {} from factory: {}", id, e);
            rc_core::data::error_codes::WebServicesError::DatabaseError as i16
        })?;
        if let Some(vehicle) = vehicle {
            let vehicle_data = crate::data::crf::ItemData::from(vehicle);
            params.insert(DATA_PARAM_KEY, vehicle_data.as_transmissible());
        } else {
            log::warn!("Failed to retrieve non-existent factory vehicle {}", id);
            return Err(rc_core::data::error_codes::WebServicesError::InvalidRobot as i16);
        }
    }
    Ok(params.into())
}

pub struct CrfItemDataProvider {
    factory: std::sync::Arc<rc_core::factory::Factory>,
}

#[async_trait::async_trait]
impl Operation<()> for CrfItemDataProvider {
    type User = crate::UserTy;

    async fn handle_async(&self, params: polariton::operation::ParameterTable<()>, user: &Self::User) -> polariton::operation::OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(do_handling(params, user, &self.factory).await)
    }
}

impl OperationCode for CrfItemDataProvider {
    fn op_code() -> u8 {
        CODE
    }
}

pub(super) fn crf_item_data_provider(factory: &std::sync::Arc<rc_core::factory::Factory>) -> CrfItemDataProvider {
    CrfItemDataProvider {
        factory: factory.to_owned(),
    }
}
