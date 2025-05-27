use polariton::operation::{ParameterTable, Typed, OperationResponse};

const CODE: u8 = 218;

const UUID_KEY: u8 = 54; // str; in
const BAY_SKIN_KEY: u8 = 234;
const SPAWN_EFFECT_KEY: u8 = 235;
const DEATH_EFFECT_KEY: u8 = 236;

pub(super) fn bay_customisations_provider() -> GarageSlotCustomisationProvider {
    GarageSlotCustomisationProvider
}

async fn do_handling(params: ParameterTable<()>, user: &crate::UserTy) -> Result<ParameterTable, i16> {
    let mut params = params.to_dict();
    if let Some(Typed::Str(uuid)) = params.remove(&UUID_KEY) {
        let user_info = user.user()?;
        let customs = user_info.get_slot_customisations(&uuid.string).await?;
        params.insert(BAY_SKIN_KEY, customs.bay);
        params.insert(SPAWN_EFFECT_KEY, customs.spawn);
        params.insert(DEATH_EFFECT_KEY, customs.death);
    }

    Ok(params.into())
}

pub(super) struct GarageSlotCustomisationProvider;

#[async_trait::async_trait]
impl polariton_server::operations::Operation<()> for GarageSlotCustomisationProvider {
    type User = crate::UserTy;

    async fn handle_async(&self, params: ParameterTable<()>, user: &Self::User) -> OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(do_handling(params, user).await)
    }
}

impl polariton_server::operations::OperationCode for GarageSlotCustomisationProvider {
    fn op_code() -> u8 {
        CODE
    }
}
