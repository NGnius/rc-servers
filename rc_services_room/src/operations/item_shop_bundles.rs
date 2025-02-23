use polariton_server::operations::SimpleFunc;
use polariton::operation::ParameterTable;

use crate::data::item_shop_bundle::*;

const PARAM_KEY: u8 = 65;

pub(super) fn item_bundle_provider() -> SimpleFunc<188, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, ItemShopBundle::as_transmissible_vec(vec![
            ItemShopBundle {
                sku: "12345".to_owned(),
                bundle_name_key: "RE_todo_item_shop_bundle_name_key".to_owned(),
                sprite: "RE_todo_item_shop_sprite_name".to_owned(),
                is_sprite_full_size: true,
                category: ItemShopCategory::Cube,
                currency: CurrencyType::Robits,
                price: 10_000,
                discount_time: 1,
                discount_price: 5_000,
                recurrence: ItemShopRecurrence::Daily,
                owns_required_cube: true,
                is_limited_edition: true,
            }
        ]));
        Ok(params.into())
    })
}
