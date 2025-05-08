use polariton::operation::{ParameterTable, Typed, OperationResponse};

const USERNAME_PARAM_KEY: u8 = 30; // in; str
const RANK_PARAM_KEY: u8 = 84; // out; int
const CPU_PARAM_KEY: u8 = 177; // out; int
const COSMETIC_CPU_PARAM_KEY: u8 = 176; // out; int

pub(super) fn player_robot_rank_provider() -> PlayerRank {
    PlayerRank
}

async fn do_handling(params: ParameterTable<()>, user: &crate::UserTy) -> Result<ParameterTable, i16> {
    let user = user.user()?;
    let mut params = params.to_dict();
    if let Some(Typed::Str(username)) = params.get(&USERNAME_PARAM_KEY) {
        log::debug!("Get robot rank for user {}", username.string);
    }
    let robot = user.slot_by_id(user.selected_garage().await.1 as i32).await?;
    params.insert(RANK_PARAM_KEY, robot.robot_rank);
    params.insert(CPU_PARAM_KEY, robot.cpu);
    params.insert(COSMETIC_CPU_PARAM_KEY, robot.cosmetic_cpu);
    Ok(params.into())
}

pub struct PlayerRank;

#[async_trait::async_trait]
impl polariton_server::operations::Operation<()> for PlayerRank {
    type User = crate::UserTy;

    async fn handle_async(&self, params: ParameterTable<()>, user: &Self::User) -> OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<79, ()>(do_handling(params, user).await)
    }
}

impl polariton_server::operations::OperationCode for PlayerRank {
    fn op_code() -> u8 {
        79
    }
}
