use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed, Arr};

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
                    // TODO refactor this into oj_rc_core in one of the User traits
                    if let Some(transaction) = transactions.get(&sku.string) {
                        let mut is_ok = false;
                        if currency.string == "Robits" && transaction.cost_free == price {
                            let ty = oj_rc_core::persist::user::CurrencyType::Free;
                            let op = oj_rc_core::persist::user::CurrencyOp::AddSub(-price as _);
                            user_info.currency(ty, op).await?;
                            is_ok = true;
                        } else if currency.string == "CosmeticCredits" && transaction.cost_paid == price {
                            let ty = oj_rc_core::persist::user::CurrencyType::Paid;
                            let op = oj_rc_core::persist::user::CurrencyOp::AddSub(-price as _);
                            user_info.currency(ty, op).await?;
                            is_ok = true;
                        }
                        if is_ok {
                            let mut new_cubes = Vec::new();
                            for award in transaction.gives.iter() {
                                match award {
                                    oj_rc_core::persist::config::ShopGain::Cube(x) => {
                                        new_cubes.push(*x);
                                    },
                                    oj_rc_core::persist::config::ShopGain::Experience(xp) => {
                                        user_info.currency(
                                            oj_rc_core::persist::user::CurrencyType::Experience,
                                            oj_rc_core::persist::user::CurrencyOp::AddSub(*xp as _),
                                        ).await?;
                                    },
                                    oj_rc_core::persist::config::ShopGain::FreeCurrency(c) => {
                                        user_info.currency(
                                            oj_rc_core::persist::user::CurrencyType::Free,
                                            oj_rc_core::persist::user::CurrencyOp::AddSub(*c as _),
                                        ).await?;
                                    },
                                    oj_rc_core::persist::config::ShopGain::PaidCurrency(c) => {
                                        user_info.currency(
                                            oj_rc_core::persist::user::CurrencyType::Paid,
                                            oj_rc_core::persist::user::CurrencyOp::AddSub(*c as _),
                                        ).await?;
                                    },
                                    oj_rc_core::persist::config::ShopGain::TechPoints(tp) => {
                                        user_info.currency(
                                            oj_rc_core::persist::user::CurrencyType::TechPoints,
                                            oj_rc_core::persist::user::CurrencyOp::AddSub(*tp as _),
                                        ).await?;
                                    },
                                }
                            }
                            let new_cubes_len = new_cubes.len();
                            if !new_cubes.is_empty() {
                                user_info.unlock_parts(&new_cubes).await?;
                            }

                            let new_cubes_typed: Vec<_> = new_cubes.into_iter().map(|x| Typed::Str(hex::encode((x as i32).to_be_bytes()).into())).collect();
                            //params.insert(NEW_CUBES_PARAM_KEY, Typed::Arr(new_cubes_i32.into()));
                            params.insert(NEW_CUBES_PARAM_KEY, Typed::Arr(Arr {
                                ty: polariton::serdes::TypePrefix::Str,
                                items: new_cubes_typed,
                            }));
                            log::info!("SKU {} purchased (ok? {}) {} ops received, {} cubes unlocked", sku.string, is_ok, transaction.gives.len(), new_cubes_len);
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
