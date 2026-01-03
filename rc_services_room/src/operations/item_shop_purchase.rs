use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 189;

const SKU_PARAM_KEY: u8 = 32; // str; in
const CURRENCY_PARAM_KEY: u8 = 53; // str; in
const PRICE_PARAM_KEY: u8 = 65; // int; in
const NEW_CUBES_PARAM_KEY: u8 = 72; // arr of str; out

pub(super) struct ItemBundleBuyer {
    resolver: oj_rc_core::persist::config::ShopEntriesResolver,
}

#[async_trait::async_trait]
impl SimpleOperation<()> for ItemBundleBuyer {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable, user: &Self::User) -> Result<ParameterTable, SimpleOpError> {
        if let Some(Typed::Str(sku)) = params.remove(&SKU_PARAM_KEY) {
            if let Some(Typed::Str(currency)) = params.remove(&CURRENCY_PARAM_KEY) {
                if let Some(Typed::Int(price)) = params.remove(&PRICE_PARAM_KEY) {
                    let user_info = user.user()?;
                    let transactions = self.resolver.resolve_transactions().await;
                    if let Some(transaction) = transactions.get(&sku.string) {
                        let is_ok = (currency.string == "Robits" && transaction.cost_free == price)
                            || (currency.string == "CosmeticCredits" && transaction.cost_paid == price);
                        if is_ok {
                            let purchase_result = user_info.apply_purchase(transaction).await?;
                            let new_cubes = polariton::operation::Typed::Arr(polariton::operation::Arr {
                                ty: polariton::serdes::TypePrefix::Str,
                                items: purchase_result.cube_awards.into_keys()
                                    .map(|k| polariton::operation::Typed::Str(k.into()))
                                    .collect(),
                            });
                            params.insert(NEW_CUBES_PARAM_KEY, new_cubes);
                            log::debug!("SKU {} purchased (ok? {}) {} ops", sku.string, is_ok, transaction.gives.len());
                        }
                    }
                }
            }
        }
        Ok(params)
    }
}

pub(super) fn item_purchase_provider(conf: &oj_rc_core::ConfigImpl) -> SimpleOpImpl<(), crate::UserTy, ItemBundleBuyer> {
    SimpleOpImpl::new(ItemBundleBuyer {
        resolver: <oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::shop_entries(conf),
    })
}
