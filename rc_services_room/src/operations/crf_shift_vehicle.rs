use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};
use oj_rc_factory::VehicleFactoryAdapter;

const CODE: u8 = 97;

const ID_PARAM_KEY: u8 = 45; // int; in
const OFFSET_X_PARAM_KEY: u8 = 104; // int; in
const OFFSET_Z_PARAM_KEY: u8 = 105; // int; in
const EXPECTED_FIRST_X_PARAM_KEY: u8 = 106; // int; in
const EXPECTED_FIRST_Y_PARAM_KEY: u8 = 107; // int; in
const EXPECTED_FIRST_Z_PARAM_KEY: u8 = 108; // int; in

pub(super) struct FactoryVehicleOffsetApplier {
    factory: std::sync::Arc<oj_rc_core::factory::Factory>,
    offsetter: std::sync::Arc<oj_rc_core::cubes::OffsetParser>,
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for FactoryVehicleOffsetApplier {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        if let Some(Typed::Int(factory_id)) = params.remove(&ID_PARAM_KEY) {
            if let Some(Typed::Int(offset_x)) = params.remove(&OFFSET_X_PARAM_KEY) {
                if let Some(Typed::Int(offset_z)) = params.remove(&OFFSET_Z_PARAM_KEY) {
                    if let Some(Typed::Int(expected_first_x)) = params.remove(&EXPECTED_FIRST_X_PARAM_KEY) {
                        if let Some(Typed::Int(expected_first_y)) = params.remove(&EXPECTED_FIRST_Y_PARAM_KEY) {
                            if let Some(Typed::Int(expected_first_z)) = params.remove(&EXPECTED_FIRST_Z_PARAM_KEY) {
                                let _ = user.user()?; // double-check they're authenticated
                                log::debug!("Factory vehicle {} has first expected cube at (x:{}, y:{}, z:{}) offset (x:{}, z:{})", factory_id, expected_first_x, expected_first_y, expected_first_z, offset_x, offset_z);
                                let vehicle_opt = self.factory.vehicle(factory_id).await.map_err(|e| {
                                    log::error!("Failed to retrieve vehicle {} (for offset) from factory: {}", factory_id, e);
                                    SimpleOpError::with_message(
                                        oj_rc_core::data::error_codes::WebServicesError::DatabaseError as i16,
                                        format!("Failed to retrieve vehicle (for offset) from factory: {}", e)
                                    )
                                })?;
                                if let Some((mut vehicle, _)) = vehicle_opt {
                                    self.offsetter.offset_inplace_by(&mut vehicle.cube_data, &mut vehicle.colour_data, (offset_x as _, 0, offset_z as _));
                                    self.factory.update_vehicle(factory_id, Some(vehicle.cube_data), Some(vehicle.colour_data)).await
                                    .map_err(|e| {
                                        log::error!("Failed to update factory vehicle {} (for offset): {}", factory_id, e);
                                        SimpleOpError::with_message(
                                            oj_rc_core::data::error_codes::WebServicesError::DatabaseError as i16,
                                            format!("Failed to retrieve factory vehicle (for offset): {}", e)
                                        )
                                    })?;
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(ParameterTable::with_capacity(1))
    }
}

pub(super) fn factory_offset_provider<C: Send + 'static>(factory: &std::sync::Arc<oj_rc_core::factory::Factory>, offsetter:  std::sync::Arc<oj_rc_core::cubes::OffsetParser>) -> SimpleOpImpl<C, crate::UserTy, FactoryVehicleOffsetApplier> {
    SimpleOpImpl::new(FactoryVehicleOffsetApplier {
        factory: factory.to_owned(),
        offsetter,
    })
}
