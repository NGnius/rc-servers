use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 69; // nice

const ORIGINAL_SLOT_PARAM_KEY: u8 = 43; // int; in
const REUSE_SLOT_PARAM_KEY: u8 = 8; // int (optional); in
const COPY_STR_PARAM_KEY: u8 = 213; // str; in

pub(super) struct GarageSlotCopier;

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for GarageSlotCopier {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let mut params = params.to_dict();
        if let Some(Typed::Int(slot)) = params.remove(&ORIGINAL_SLOT_PARAM_KEY) {
            if let Some(Typed::Str(copy_str)) = params.remove(&COPY_STR_PARAM_KEY) {
                let user_info = user.user()?;
                if let Some(Typed::Int(reuse_slot)) = params.remove(&REUSE_SLOT_PARAM_KEY) {
                    user_info.copy_slot(slot, Some(reuse_slot), &copy_str.string).await?;
                } else {
                    user_info.copy_slot(slot, None, &copy_str.string).await?;
                }
            }
        }
        Ok(params.into())
    }
}

pub(super) fn garage_slot_copy_provider<C: Send + 'static>() -> SimpleOpImpl<C, crate::UserTy, GarageSlotCopier> {
    SimpleOpImpl::new(GarageSlotCopier)
}
