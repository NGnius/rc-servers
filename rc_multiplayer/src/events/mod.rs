mod validate_game_guid;

pub async fn handler(init_ctx: &crate::InitConfig) -> crate::handler::LnlEventHandler {
    crate::handler::LnlEventHandler::new()
        .add(validate_game_guid::handler(init_ctx))
}
