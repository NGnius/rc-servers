use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 46;

const SLOT_PARAM_KEY: u8 = 45; // int; in
const NAME_PARAM_KEY: u8 = 42; // str; in

pub(super) struct GarageSlotRenamer;

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for GarageSlotRenamer {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let mut params = params.to_dict();
        if let Some(Typed::Int(slot)) = params.remove(&SLOT_PARAM_KEY) {
            if let Some(Typed::Str(name)) = params.remove(&NAME_PARAM_KEY) {
                let user_info = user.user()?;
                user_info.set_slot_name(slot, name.string).await?;
            }
        }
        Ok(params.into())
    }
}

pub(super) fn garage_slot_rename_provider<C: Send + 'static>() -> SimpleOpImpl<C, crate::UserTy, GarageSlotRenamer> {
    SimpleOpImpl::new(GarageSlotRenamer)
}
