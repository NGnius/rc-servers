use polariton::operation::{ParameterTable, Typed, OperationResponse};

const CODE: u8 = 115;

const INDEX_PARAM_KEY: u8 = 45; // int; in
const CONTROL_TY_PARAM_KEY: u8 = 59; // int (enum); in
const CONTROL_OPTIONS_PARAM_KEY: u8 = 60; // arr of bool (3); in

pub(super) fn garage_slot_controls_provider() -> GarageSlotControlsSaveProvider {
    GarageSlotControlsSaveProvider
}

async fn do_handling(params: ParameterTable<()>, user: &crate::UserTy) -> Result<ParameterTable, i16> {
    let mut params = params.to_dict();
    if let Some(Typed::Int(index)) = params.remove(&INDEX_PARAM_KEY) {
        if let Some(Typed::Int(control_ty)) = params.remove(&CONTROL_TY_PARAM_KEY) {
            if let Some(Typed::Arr(control_options)) = params.remove(&CONTROL_OPTIONS_PARAM_KEY) {
                let mut controls = rc_core::persist::user::ControlData {
                    slot: index,
                    control_ty: rc_core::persist::user::ControlType::from_i32(control_ty)?,
                    vertical_strafing: false,
                    sideways_driving: false,
                    tracks_turn_on_spot: false,
                };
                for (i, val) in control_options.items.iter().enumerate() {
                    if let Typed::Bool(val) = val {
                        match i {
                            0 => controls.vertical_strafing = *val,
                            1 => controls.sideways_driving = *val,
                            2 => controls.tracks_turn_on_spot = *val,
                            _ => log::warn!("Got too many options for setting garage slot controls"),
                        }
                    }
                }
                let user_info = user.user()?;
                user_info.save_slot_controls(controls).await?;
            }
        }
    }
    Ok(params.into())
}

pub struct GarageSlotControlsSaveProvider;

#[async_trait::async_trait]
impl polariton_server::operations::Operation<()> for GarageSlotControlsSaveProvider {
    type User = crate::UserTy;

    async fn handle_async(&self, params: ParameterTable<()>, user: &Self::User) -> OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(do_handling(params, user).await)
    }
}

impl polariton_server::operations::OperationCode for GarageSlotControlsSaveProvider {
    fn op_code() -> u8 {
        CODE
    }
}
