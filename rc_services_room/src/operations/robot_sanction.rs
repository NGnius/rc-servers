use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Arr};

const ROBOT_ID_PARAM_KEY: u8 = 54; // str (in)
const SANCTION_JSONS_PARAM_KEY: u8 = 102; // str arr (out; list of jsons)

pub(super) fn robot_sanction_provider() -> SimpleFunc<174, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        if let Some(Typed::Str(s)) = params.get(&ROBOT_ID_PARAM_KEY) {
            log::debug!("Got sanction check for robot {}", s.string);
        }
        params.insert(SANCTION_JSONS_PARAM_KEY, Typed::Arr(Arr {
            ty: 115, // str
            items: Vec::default(),
        }));
        Ok(params.into())
    })
}
