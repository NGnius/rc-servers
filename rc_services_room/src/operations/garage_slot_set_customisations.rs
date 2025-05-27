use polariton::operation::{ParameterTable, Typed, OperationResponse};

const CODE: u8 = 217;

const UUID_PARAM_KEY: u8 = 54; // str; in
const BAY_SKIN_PARAM_KEY: u8 = 234; // str; in
const SPAWN_SKIN_PARAM_KEY: u8 = 235; // str; in
const DEATH_SKIN_PARAM_KEY: u8 = 236; // str; in

pub(super) fn garage_slot_customisation_provider() -> GarageSlotCustomisationSaveProvider {
    GarageSlotCustomisationSaveProvider
}

async fn do_handling(params: ParameterTable<()>, user: &crate::UserTy) -> Result<ParameterTable, i16> {
    let mut params = params.to_dict();
    if let Some(Typed::Str(uuid)) = params.remove(&UUID_PARAM_KEY) {
        if let Some(Typed::Str(bay_skin)) = params.remove(&BAY_SKIN_PARAM_KEY) {
            if let Some(Typed::Str(spawn)) = params.remove(&SPAWN_SKIN_PARAM_KEY) {
                if let Some(Typed::Str(death)) = params.remove(&DEATH_SKIN_PARAM_KEY) {
                    let user_info = user.user()?;
                    user_info.save_slot_customisations(rc_core::persist::user::CustomisationData {
                        uuid: uuid.string,
                        bay: bay_skin.string,
                        spawn: spawn.string,
                        death: death.string,
                    }).await?;
                }
            }
        }
    }
    Ok(params.into())
}

pub struct GarageSlotCustomisationSaveProvider;

#[async_trait::async_trait]
impl polariton_server::operations::Operation<()> for GarageSlotCustomisationSaveProvider {
    type User = crate::UserTy;

    async fn handle_async(&self, params: ParameterTable<()>, user: &Self::User) -> OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(do_handling(params, user).await)
    }
}

impl polariton_server::operations::OperationCode for GarageSlotCustomisationSaveProvider {
    fn op_code() -> u8 {
        CODE
    }
}
