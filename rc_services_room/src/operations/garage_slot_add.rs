use polariton::operation::{ParameterTable, Typed, OperationResponse};

const CODE: u8 = 38;

const SLOT_REUSE_PARAM_KEY: u8 = 43; // int; in
const SLOT_PARAM_KEY: u8 = 45; // uint
const UUID_0_PARAM_KEY: u8 = 40; // str of uint
const UUID_1_PARAM_KEY: u8 = 41; // str of uint
const NAME_PARAM_KEY: u8 = 42; // str
const DEFAULT_BAY_CPU_PARAM_KEY: u8 = 8; // int
const MASTERY_LEVEL_PARAM_KEY: u8 = 18; // int

pub(super) fn garage_slot_add_provider() -> SlotCreator {
    SlotCreator
}

async fn do_add(params: ParameterTable<()>, user: &crate::UserTy) -> Result<ParameterTable, i16> {
    let mut params = params.to_dict();
    let user_info = user.user()?;
    let reset_slot = if let Some(Typed::Int(reuse_garage_slot)) = params.get(&SLOT_REUSE_PARAM_KEY) {
        // reset existing garage slot
        Some(*reuse_garage_slot)
    } else {
        // create new garage slot
        None
    };
    let new_slot = user_info.new_slot(reset_slot).await?;
    params.insert(SLOT_PARAM_KEY, new_slot.slot);
    params.insert(UUID_0_PARAM_KEY, new_slot.uuid_0);
    params.insert(UUID_1_PARAM_KEY, new_slot.uuid_1);
    params.insert(MASTERY_LEVEL_PARAM_KEY, new_slot.mastery_level);
    params.insert(NAME_PARAM_KEY, new_slot.name);
    params.insert(DEFAULT_BAY_CPU_PARAM_KEY, new_slot.bay_cpu);
    Ok(params.into())
}

pub struct SlotCreator;

#[async_trait::async_trait]
impl polariton_server::operations::Operation<()> for SlotCreator {
    type User = crate::UserTy;

    async fn handle_async(&self, params: ParameterTable<()>, user: &Self::User) -> OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(do_add(params, user).await)
    }
}

impl polariton_server::operations::OperationCode for SlotCreator {
    fn op_code() -> u8 {
        CODE
    }
}
