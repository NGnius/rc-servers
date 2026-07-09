use polariton_server::operations::{Operation, OperationCode};
use polariton::operation::{Typed, ParameterTable};
use oj_rc_factory::VehicleFactoryAdapter;

const CODE: u8 = 87;

const ID_PARAM_KEY: u8 = 94;
const DATA_PARAM_KEY: u8 = 95;

async fn do_handling(params: ParameterTable<()>, _user: &crate::UserTy, factory: &std::sync::Arc<oj_rc_core::factory::Factory>, converter: &std::sync::Arc<oj_rc_core::cubes::CubeConversionParser>) -> Result<ParameterTable, i16> {
    let mut params = params.to_dict();
    if let Some(Typed::Int(id)) = params.remove(&ID_PARAM_KEY) {
        let vehicle = factory.vehicle(id as _).await.map_err(|e| {
            log::error!("Failed to retrieve vehicle {} from factory: {}", id, e);
            oj_rc_core::data::error_codes::WebServicesError::DatabaseError as i16
        })?;
        if let Some(mut vehicle) = vehicle {
            let modernised_vehicle = converter.upgrade_to_modern(
                &mut std::io::Cursor::new(vehicle.0.cube_data),
                &mut std::io::Cursor::new(vehicle.0.colour_data),
            ).map_err(|e| {
                log::error!("Failed to convert vehicle {} from factory: {}", id, e);
                oj_rc_core::data::error_codes::WebServicesError::DatabaseError as i16
            })?;
            vehicle.0.cube_data = modernised_vehicle.cube_data;
            vehicle.0.colour_data = modernised_vehicle.colour_data;
            let vehicle_data = crate::data::crf::ItemData::from(vehicle.0);
            params.insert(DATA_PARAM_KEY, vehicle_data.as_transmissible());
        } else {
            log::warn!("Failed to retrieve non-existent factory vehicle {}", id);
            return Err(oj_rc_core::data::error_codes::WebServicesError::InvalidRobot as i16);
        }
    }
    Ok(params.into())
}

pub struct CrfItemDataProvider {
    factory: std::sync::Arc<oj_rc_core::factory::Factory>,
    convert: std::sync::Arc<oj_rc_core::cubes::CubeConversionParser>,
}

#[async_trait::async_trait]
impl Operation<()> for CrfItemDataProvider {
    type User = crate::UserTy;

    async fn handle_async(&self, params: polariton::operation::ParameterTable<()>, user: &Self::User) -> polariton::operation::OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(do_handling(params, user, &self.factory, &self.convert).await)
    }
}

impl OperationCode for CrfItemDataProvider {
    fn op_code() -> u8 {
        CODE
    }
}

pub(super) fn crf_item_data_provider(factory: &std::sync::Arc<oj_rc_core::factory::Factory>, convert: std::sync::Arc<oj_rc_core::cubes::CubeConversionParser>) -> CrfItemDataProvider {
    CrfItemDataProvider {
        factory: factory.to_owned(),
        convert,
    }
}
