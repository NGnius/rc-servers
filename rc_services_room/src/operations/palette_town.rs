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
            Colour::write_many(vec![
                // Red
                Colour {
                    index: 0,
                    diffuse: ColourValue { r: 255, g: 0, b: 0, a: u8::MAX },
                    specular: ColourValue { r: 255, g: 0, b: 0, a: u8::MAX },
                    overlay: ColourValue { r: 255, g: 0, b: 0, a: u8::MAX },
                    premium: false,
                },
                // Blue
                Colour {
                    index: 1,
                    diffuse: ColourValue { r: 0, g: 0, b: 255, a: u8::MAX },
                    specular: ColourValue { r: 0, g: 0, b: 255, a: u8::MAX },
                    overlay: ColourValue { r: 0, g: 0, b: 255, a: u8::MAX },
                    premium: false,
                },
                // Green
                Colour {
                    index: 2,
                    diffuse: ColourValue { r: 0, g: 255, b: 0, a: u8::MAX },
                    specular: ColourValue { r: 0, g: 255, b: 0, a: u8::MAX },
                    overlay: ColourValue { r: 0, g: 255, b: 0, a: u8::MAX },
                    premium: false,
                },
                // Black
                Colour {
                    index: 3,
                    diffuse: ColourValue { r: 0, g: 0, b: 0, a: u8::MAX },
                    specular: ColourValue { r: 0, g: 0, b: 0, a: u8::MAX },
                    overlay: ColourValue { r: 0, g: 0, b: 0, a: u8::MAX },
                    premium: false,
                },
                // White
                Colour {
                    index: 4,
                    diffuse: ColourValue { r: 255, g: 255, b: 255, a: u8::MAX },
                    specular: ColourValue { r: 255, g: 255, b: 255, a: u8::MAX },
                    overlay: ColourValue { r: 255, g: 255, b: 255, a: u8::MAX },
                    premium: false,
                },
            ].as_slice(), &mut buf).unwrap_or_default();
            buf.into()
        }));
        params.insert(ORDER_KEY, Typed::Bytes(vec![0u8, 1, 2, 3, 4].into()));
        Ok(params.into())
    })
}
