use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const LOBBY_PARAM_KEY: u8 = 134;
const VALIDATE_ROBOT_RESULT_PARAM_KEY: u8 = 111;

// possible codes
#[allow(dead_code)]
#[repr(u8)]
enum ValidateMachineResult {
    Invalid = 0,
    Ok = 1,
    NoWeapon = 2,
    NoMovement = 3,
    Sanctioned = 4,
}

pub(super) fn validate_robot_provider() -> SimpleFunc<102, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _user: &crate::UserTy| {
        let mut params = params.to_dict();
        if let Some(Typed::Int(lobby_ty)) = params.get(&LOBBY_PARAM_KEY) {
            log::info!("Got lobby type {} ({:?})", lobby_ty, crate::data::lobby::LobbyType::from_int(*lobby_ty));
        }
        // let lock = user.read().unwrap();
        // let user_info = lock.user()?;
        // TODO actually validate the vehicle
        params.insert(VALIDATE_ROBOT_RESULT_PARAM_KEY, Typed::Int(ValidateMachineResult::Ok as _));
        Ok(params.into())
    })
}
