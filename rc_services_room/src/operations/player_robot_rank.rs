use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const USERNAME_PARAM_KEY: u8 = 30; // in; str
const RANK_PARAM_KEY: u8 = 84; // out; int
const CPU_PARAM_KEY: u8 = 177; // out; int
const COSMETIC_CPU_PARAM_KEY: u8 = 176; // out; int

pub(super) fn player_robot_rank_provider() -> SimpleFunc<79, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, user: &crate::UserTy| {
        let user = user.user()?;
        let mut params = params.to_dict();
        if let Some(Typed::Str(username)) = params.get(&USERNAME_PARAM_KEY) {
            log::debug!("Get robot rank for user {}", username.string);
        }
        let robot = user.slot_by_id(user.selected_garage_slot() as i32)?;
        params.insert(RANK_PARAM_KEY, robot.robot_rank);
        params.insert(CPU_PARAM_KEY, robot.cpu);
        params.insert(COSMETIC_CPU_PARAM_KEY, robot.cosmetic_cpu);
        Ok(params.into())
    })
}
