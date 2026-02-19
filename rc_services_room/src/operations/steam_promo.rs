use polariton_server::operations::Immediate;
use polariton::operation::Typed;

const CODE: u8 = 52;
const PROMO_LIST_PARAM_KEY: u8 = 63; // list of str
const STEAM_PROMOS_PARAM_KEY: u8 = 121; // string (json)
const CUBES_AWARDED_PARAM_KEY: u8 = 128; // string (json)
const PASS_AWARDED_PARAM_KEY: u8 = 4; // bool

pub(super) fn steam_promos_provider() -> Immediate<CODE, crate::UserTy> {
    Immediate::new(|| {
        let mut params = std::collections::HashMap::with_capacity(4);
        params.insert(PROMO_LIST_PARAM_KEY, Typed::Arr(polariton::operation::Arr {
            ty: polariton::serdes::TypePrefix::Str,
            custom_ty: None,
            items: Vec::default(),
        }));
        params.insert(STEAM_PROMOS_PARAM_KEY, Typed::Str("{}".into()));
        params.insert(CUBES_AWARDED_PARAM_KEY, Typed::Str("{}".into()));
        params.insert(PASS_AWARDED_PARAM_KEY, Typed::Bool(false));
        params.into()
    })
}
