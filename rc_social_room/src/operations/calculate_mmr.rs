use polariton_server::operations::Immediate;
use polariton::operation::Typed;

const CODE: u8 = 60;
const MMR_PARAM_KEY: u8 = 77; // f64

pub(super) fn mmr_provider<C: Clone + Send + Sync>() -> Immediate<CODE, crate::UserTy, C> {
    Immediate::new(|| {
        let mut params = std::collections::HashMap::with_capacity(1);
        params.insert(MMR_PARAM_KEY, Typed::Double(1.0));
        params.into()
    })
}
