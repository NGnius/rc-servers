use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 149;

const FIELD_PARAM_KEY: u8 = 179; // str; in
const VALUE_PARAM_KEY: u8 = 180; // any; in
const RESPONSE_CODE_PARAM_KEY: u8 = 168; // int enum; out

pub(super) struct CustomGameConfigChanger {
    games: std::sync::Arc<crate::custom_game_tracker::CustomGameMesh>,
    mesh: std::sync::Arc<crate::user_service::UserMesh>,
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for CustomGameConfigChanger {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        if let Some(Typed::Str(field_name)) = params.remove(&FIELD_PARAM_KEY) {
            if let Some(Typed::Str(value)) = params.remove(&VALUE_PARAM_KEY) {
                let user_info = user.user()?;
                let my_pub_id = user_info.public_id();
                let (resp_code, session_opt) = self.games.set_config_field(my_pub_id, &field_name.string, &value.string).await;
                if let Some(session) = session_opt {
                    let event = crate::events::CustomGameConfigRefresh {
                        field: field_name.string.clone(),
                        value: value.string.clone(),
                    };
                    let session_members_iter = session.users.iter()
                        .filter(|mem| !mem.is_invited && mem.public_id != my_pub_id)
                        .map(|mem| &mem.public_id as &str);
                    self.mesh.broadcast_event_to(session_members_iter, event).await;
                }
                params.insert(RESPONSE_CODE_PARAM_KEY, Typed::Int(resp_code as _));
            }
        } else {
            log::warn!("Missing custom game field name parameter string");
        }
        Ok(params)
    }
}

pub(super) fn game_adjust_provider<C: Send + 'static>(init_ctx: &crate::InitConfig) -> SimpleOpImpl<C, crate::UserTy, CustomGameConfigChanger> {
    SimpleOpImpl::new(CustomGameConfigChanger {
        games: init_ctx.custom_games.clone(),
        mesh: init_ctx.user_mesh.clone(),
    })
}
