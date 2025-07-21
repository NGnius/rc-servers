use polariton::operation::{ParameterTable, Typed, OperationResponse};
use oj_rc_factory::VehicleFactoryAdapter;

const CODE: u8 = 166;

const SLOT_PARAM_KEY: u8 = 43; // in; int
//const FREE_CURRENCY_COST_PARAM_KEY: u8 = 5; // in; int
//const PREMIUM_CURRENCY_COST_PARAM_KEY: u8 = 6; // in; int
const FACTORY_ID_PARAM_KEY: u8 = 94; // in; int


async fn do_handling(params: ParameterTable<()>, user: &crate::UserTy, factory: &std::sync::Arc<oj_rc_core::factory::Factory>, weapon_order: &std::sync::Arc<oj_rc_core::cubes::WeaponListParser>, cpu_counter: &std::sync::Arc<oj_rc_core::cubes::CpuListParser>,) -> Result<ParameterTable, i16> {
    let mut params = params.to_dict();
    let user_info = user.user()?;
    let slot = if let Some(Typed::Int(slot)) = params.remove(&SLOT_PARAM_KEY) {
        user_info.new_slot(Some(slot)).await?;
        slot
    } else {
        user_info.new_slot(None).await?.slot_i
    };
    if let Some(Typed::Int(factory_id)) = params.remove(&FACTORY_ID_PARAM_KEY) {
        // TODO charge for robot?
        let vehicle = factory.vehicle(factory_id as _).await.map_err(|e| {
            log::error!("Failed to retrieve vehicle {} (for copy-construct) from factory: {}", factory_id, e);
            oj_rc_core::data::error_codes::WebServicesError::DatabaseError as i16
        })?;
        if let Some((vehicle_to_copy, vehicle_meta)) = vehicle {
            // parse cube data for weapon order
            let mut cursor = std::io::Cursor::new(&vehicle_to_copy.cube_data);
            let weapons = weapon_order.guess_weapons(&mut cursor);
            // save to database
            let to_save = oj_rc_core::persist::user::VehicleData {
                name: Some(vehicle_meta.name),
                slot,
                robot_data: vehicle_to_copy.cube_data,
                colour_data: vehicle_to_copy.colour_data,
                weapon_order: weapons,
                crf_id: Some(factory_id),
            };
            user_info.save_slot(to_save, cpu_counter).await?;
        } else {
            log::warn!("Failed to retrieve (for copy-construct) non-existent factory vehicle {}", factory_id);
            return Err(oj_rc_core::data::error_codes::WebServicesError::DatabaseError as i16);
        }

    }

    Ok(params.into())
}

pub struct CrfItemPurchaseProvider {
    factory: std::sync::Arc<oj_rc_core::factory::Factory>,
    weapon_order: std::sync::Arc<oj_rc_core::cubes::WeaponListParser>,
    cpu_counter: std::sync::Arc<oj_rc_core::cubes::CpuListParser>,
}

#[async_trait::async_trait]
impl polariton_server::operations::Operation<()> for CrfItemPurchaseProvider {
    type User = crate::UserTy;

    async fn handle_async(&self, params: ParameterTable<()>, user: &Self::User) -> OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(do_handling(params, user, &self.factory, &self.weapon_order, &self.cpu_counter).await)
    }
}

impl polariton_server::operations::OperationCode for CrfItemPurchaseProvider {
    fn op_code() -> u8 {
        CODE
    }
}


pub(super) fn crf_copy_to_bay_provider(factory: &std::sync::Arc<oj_rc_core::factory::Factory>, weapon_order: std::sync::Arc<oj_rc_core::cubes::WeaponListParser>, cpu_counter: std::sync::Arc<oj_rc_core::cubes::CpuListParser>) -> CrfItemPurchaseProvider {
    CrfItemPurchaseProvider {
        factory: factory.to_owned(),
        weapon_order,
        cpu_counter
    }
}
