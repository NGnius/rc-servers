pub struct ClientDisconnecter {
    msg_router: tokio::sync::mpsc::Sender<crate::matches::GameMessage>,
}

pub(super) fn handler(init_ctx: &crate::InitConfig) -> ClientDisconnecter {
    ClientDisconnecter::new(init_ctx)
}

impl ClientDisconnecter {
    fn new(init_ctx: &crate::InitConfig) -> Self {
        Self {
            msg_router: init_ctx.matches_chann.clone(),
        }
    }
}

#[async_trait::async_trait]
impl crate::DisconnectHandler for ClientDisconnecter {
    async fn handle(&self, _peer: &std::sync::Arc<literustlib_server::Connection<crate::PacketData>>, user: &crate::UserData) {
        if let Some(user_info) = user.user().await {
            crate::events::log_channel_send_failure(self.msg_router.send(crate::matches::GameMessage::EndConnection {
                user_id: user_info.user_id(),
                is_unregister: false,
            }).await);
        } else {
            log::error!("Failed to handle disconnect for unknown user");
        }
    }
}
