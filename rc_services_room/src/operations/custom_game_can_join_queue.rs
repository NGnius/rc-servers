use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 153;

const RESPONSE_CODE_PARAM_KEY: u8 = 168; // int enum; out

pub(super) struct CustomGameCanJoinQueue {
    games: std::sync::Arc<crate::custom_game_tracker::CustomGameMesh>,
    validator: std::sync::Arc<crate::vehicle_validators::InitedVehicleValidator>,
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for CustomGameCanJoinQueue {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let user_info = user.user()?;
        let my_pub_id = user_info.public_id();
        let resp_code = if let Some(session) = self.games.get_user_game(my_pub_id).await {
            let vehicle = user_info.selected_vehicle_data().await?;
            match self.validator.validate(&vehicle.robot_data, &vehicle.colour_data) {
                oj_rc_plugins::vehicle_validation::ValidationResultCode::Ok => crate::data::custom_games::CheckCanJoinQueueResponseCode::Ok,
                err_code => {
                    log::debug!("Failed to validate user {} vehicle for custom game {}: {:?}", my_pub_id, session.session_id, err_code);
                    crate::data::custom_games::CheckCanJoinQueueResponseCode::Unbalanced
                }
            }
        } else {
            crate::data::custom_games::CheckCanJoinQueueResponseCode::UserNotInSession0
        };
        params.insert(RESPONSE_CODE_PARAM_KEY, Typed::Int(resp_code as _));
        Ok(params)
    }
}

pub(super) fn game_can_queue_provider<C: Send + 'static>(init_ctx: &crate::InitConfig) -> SimpleOpImpl<C, crate::UserTy, CustomGameCanJoinQueue> {
    SimpleOpImpl::new(CustomGameCanJoinQueue {
        games: init_ctx.custom_games.clone(),
        validator: init_ctx.vehicle_validators.custom_game.clone(),
    })
}
