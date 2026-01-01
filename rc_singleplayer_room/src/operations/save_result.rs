use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const GAME_RESULT_PARAM_KEY: u8 = 9; // bytes; in
const LONG_PLAY_PARAM_KEY: u8 = 12; // float; in
const GUID_PARAM_KEY: u8 = 18; // string; in

const CODE: u8 = 2;

pub(super) struct GameResultSaver;

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for GameResultSaver {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        if let Some(Typed::Bytes(game_result)) = params.remove(&GAME_RESULT_PARAM_KEY) {
            let mut result_cursor = std::io::Cursor::new(game_result.vec);
            let game_result = oj_rc_core::data::game_result::GameResult::parse(&mut result_cursor)
                .map_err(|e| SimpleOpError::with_message(
                    oj_rc_core::data::error_codes::SingleplayerErrorCode::UnexpectedError as i16,
                    format!("Bad game result serialization: {}", e),
                ))?;
            if let Some(Typed::Float(_long_play)) = params.remove(&LONG_PLAY_PARAM_KEY) {
                if let Some(Typed::Str(game_guid)) = params.remove(&GUID_PARAM_KEY) {
                    let user_info = user.user()?;
                    user_info.save_game_result(&game_guid.string, game_result).await?;
                }
            }
        }
        Ok(ParameterTable::with_capacity(1)) // parameter-less response
    }
}

pub(super) fn save_result_provider<C: Send + 'static>() -> SimpleOpImpl<C, crate::UserTy, GameResultSaver> {
    SimpleOpImpl::new(GameResultSaver)
}

