use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 143;

const RESPONSE_CODE_PARAM_KEY: u8 = 168; // int enum; out
const IDK_PARAM_KEY: u8 = 169; // ???; out (let's send back the game id)

pub(super) struct CustomGameCreator {
    games: std::sync::Arc<crate::custom_game_tracker::CustomGameMesh>,
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for CustomGameCreator {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let user_info = user.user()?;
        let result = self.games.create_game(user_info.public_id()).await;
        match result {
            Err(e) => {
                params.insert(RESPONSE_CODE_PARAM_KEY, Typed::Int(e as _));
            },
            Ok(game_id) => {
                params.insert(RESPONSE_CODE_PARAM_KEY, Typed::Int(crate::data::custom_games::SessionCreateResponseCode::SessionCreated as _));
                params.insert(IDK_PARAM_KEY, Typed::Str(game_id.into()));
            }
        }
        Ok(params)
    }
}

pub(super) fn game_create_provider<C: Send + 'static>(games: &std::sync::Arc<crate::custom_game_tracker::CustomGameMesh>) -> SimpleOpImpl<C, crate::UserTy, CustomGameCreator> {
    SimpleOpImpl::new(CustomGameCreator {
        games: games.to_owned(),
    })
}
