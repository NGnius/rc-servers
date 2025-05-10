use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

use crate::data::palette::*;

const PALETTE_KEY: u8 = 34;
const ORDER_KEY: u8 = 149;

pub(super) fn kanto() -> SimpleFunc<31, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PALETTE_KEY, Typed::Bytes({
            let mut buf = Vec::new();
            Colour::write_many(Colour::default_many(), &mut buf).unwrap_or_default();
            buf.into()
        }));
        params.insert(ORDER_KEY, Typed::Bytes(Colour::default_many().iter().map(|x| x.index).collect::<Vec<_>>().into()));
        Ok(params.into())
    })
}
