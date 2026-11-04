use polariton_server::operations::Immediate;
use polariton::operation::{ParameterTable, Typed};

const PARAM_KEY: u8 = 1;

pub(super) fn garage_colour_combo_provider() -> Immediate<37, crate::UserTy> {
    Immediate::new(|| {
        let mut params = ParameterTable::with_capacity(2);
        // max colours is 3 per inner array, exceeding it crashes the Robot Picker UI!
        params.insert(PARAM_KEY, Typed::Str("[[0,1,4],[0,1,2]]".into())); // why tf is this a JSON in a string??? (list of lists of bytes)
        params
    })
}
