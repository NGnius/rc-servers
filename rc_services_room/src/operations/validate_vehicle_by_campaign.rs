use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

use crate::data::vehicle_validation::*;

const VALIDATE_CAMPAIGN_ROBOT_CODE: u8 = 59;

const CAMPAIGN_ID_PARAM_KEY: u8 = 22;
const VALIDATE_ROBOT_RESULT_PARAM_KEY: u8 = 111;

pub(super) struct CampaignVehicleValidator {
    campaign_map: std::sync::Arc<std::collections::HashMap<String, crate::vehicle_validators::InitedVehicleValidator>>,
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for CampaignVehicleValidator {
    type User = crate::UserTy;
    const CODE: u8 = VALIDATE_CAMPAIGN_ROBOT_CODE;

    async fn handle(&self, params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let mut params = params.to_dict();
        if let Some(Typed::Str(campaign_id)) = params.get(&CAMPAIGN_ID_PARAM_KEY) {
            log::debug!("Got campaign id {}", campaign_id.string);
            if let Some(campaign_validator) = self.campaign_map.get(&campaign_id.string) {
                let user_info = user.user()?;
                let vehicle_data = user_info.selected_vehicle_data().await?;
                let result_code = ValidateMachineResult::from_plugin(campaign_validator.validate(
                    &vehicle_data.robot_data,
                    &vehicle_data.colour_data,
                ));
                params.insert(VALIDATE_ROBOT_RESULT_PARAM_KEY, Typed::Int(result_code as _));
            } else {
                // bad state, assume user is doing something sketchy
                log::warn!("Failed to find vehicle validator for campaign {}", campaign_id.string);
                params.insert(VALIDATE_ROBOT_RESULT_PARAM_KEY, Typed::Int(ValidateMachineResult::Sanctioned as _));
            }
        } else {
            log::warn!("No campaign ID provided for campaign vehicle validator");
        }
        Ok(params.into())
    }
}

pub(super) fn validate_campaign_robot_provider<C: Send + 'static>(validators: &crate::vehicle_validators::InitedVehicleValidators) -> SimpleOpImpl<C, crate::UserTy, CampaignVehicleValidator> {
    SimpleOpImpl::new(CampaignVehicleValidator {
        campaign_map: validators.campaigns.clone(),
    })
}
