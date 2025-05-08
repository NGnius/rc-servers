use polariton::operation::{ParameterTable, Typed, OperationResponse};

const CODE: u8 = 33;

// const USERNAME_PARAM_KEY: u8 = 30; // str
const SLOT_PARAM_KEY: u8 = 31; // int
const DATA_PARAM_KEY: u8 = 33; // byte arr

pub(super) fn garage_machine_colour_provider() -> MachineColour {
    MachineColour
}

async fn do_handling(params: ParameterTable<()>, user: &crate::UserTy) -> Result<ParameterTable, i16> {
    let mut params = params.to_dict();
    let user_info = user.user()?;
    if let Some(Typed::Int(garage_slot)) = params.get(&SLOT_PARAM_KEY) {
        log::debug!("Got machine colour request for slot {:?}", garage_slot);
        let machine = user_info.slot_by_id(*garage_slot).await?;
        params.insert(DATA_PARAM_KEY, machine.colour_data);
    } else {
        params.insert(SLOT_PARAM_KEY, Typed::Int(user_info.selected_garage().await.1 as _));
    }

    Ok(params.into())
}

pub struct MachineColour;

#[async_trait::async_trait]
impl polariton_server::operations::Operation<()> for MachineColour {
    type User = crate::UserTy;

    async fn handle_async(&self, params: ParameterTable<()>, user: &Self::User) -> OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(do_handling(params, user).await)
    }
}

impl polariton_server::operations::OperationCode for MachineColour {
    fn op_code() -> u8 {
        CODE
    }
}
