use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

use crate::data::vehicle_validation::*;

const VALIDATE_LOBBY_ROBOT_CODE: u8 = 102;

const LOBBY_PARAM_KEY: u8 = 134;
const VALIDATE_ROBOT_RESULT_PARAM_KEY: u8 = 111;

pub(super) struct MultiplayerVehicleValidator {
    multiplayer: std::sync::Arc<crate::vehicle_validators::InitedVehicleValidator>,
    custom_game: std::sync::Arc<crate::vehicle_validators::InitedVehicleValidator>,
    singleplayer: std::sync::Arc<crate::vehicle_validators::InitedVehicleValidator>,
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for MultiplayerVehicleValidator {
    type User = crate::UserTy;
    const CODE: u8 = VALIDATE_LOBBY_ROBOT_CODE;

    async fn handle(&self, params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let mut params = params.to_dict();
        if let Some(Typed::Int(lobby_ty)) = params.get(&LOBBY_PARAM_KEY) {
            let lobby_ty = oj_rc_core::data::lobby::LobbyType::from_int(*lobby_ty)?;
            log::debug!("Got lobby type {:?}", lobby_ty);
            let user_info = user.user()?;
            let vehicle_data = user_info.selected_vehicle_data().await?;
            let result_code = match lobby_ty {
                oj_rc_core::data::lobby::LobbyType::None => ValidateMachineResult::Ok,
                oj_rc_core::data::lobby::LobbyType::CustomGame => ValidateMachineResult::from_plugin(self.custom_game.validate(
                    &vehicle_data.robot_data,
                    &vehicle_data.colour_data,
                )),
                oj_rc_core::data::lobby::LobbyType::QuickPlay => ValidateMachineResult::from_plugin(self.multiplayer.validate(
                    &vehicle_data.robot_data,
                    &vehicle_data.colour_data,
                )),
                oj_rc_core::data::lobby::LobbyType::Solo => ValidateMachineResult::from_plugin(self.singleplayer.validate(
                    &vehicle_data.robot_data,
                    &vehicle_data.colour_data,
                )),
            };
            params.insert(VALIDATE_ROBOT_RESULT_PARAM_KEY, Typed::Int(result_code as _));
        } else {
            log::warn!("No lobby type provided for vehicle validator");
        }
        Ok(params.into())
    }
}

pub(super) fn validate_robot_provider<C: Send + 'static>(validators: &crate::vehicle_validators::InitedVehicleValidators) -> SimpleOpImpl<C, crate::UserTy, MultiplayerVehicleValidator> {
    SimpleOpImpl::new(MultiplayerVehicleValidator {
        multiplayer: validators.multiplayer.clone(),
        custom_game: validators.custom_game.clone(),
        singleplayer: validators.singleplayer.clone(),
    })
}
