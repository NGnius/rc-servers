use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 4;

const IS_SUCCESS_PARAM_KEY: u8 = 20; // bool; in

pub(super) struct ConnectionTestFinished {
    queue_handler: std::sync::Arc<crate::QueueHandler>,
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for ConnectionTestFinished {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let user = user.user()?;
        if let Some(Typed::Bool(is_success)) = params.remove(&IS_SUCCESS_PARAM_KEY) {
            if !is_success {
                user.show_dev_message(
                    oj_rc_core::persist::user::intercom::IntercomDevMessage {
                        message: "WARNING: Multiplayer match connection test failed".to_owned(),
                        duration: 30,
                    },
                    vec![user.public_id().to_owned()],
                ).await;
                log::warn!("Multiplayer connection test failed for {}", user.public_id());
            } else {
                log::info!("Multiplayer connection test succeeded for {}", user.public_id());
            }
        }
        self.queue_handler.complete_connection_test(user).await;
        Ok(params)
    }
}

pub(super) fn connect_test_finished_provider<C: Send + 'static>(queue_handler: &std::sync::Arc<crate::QueueHandler>) -> SimpleOpImpl<C, crate::UserTy, ConnectionTestFinished> {
    SimpleOpImpl::new(ConnectionTestFinished {
        queue_handler: queue_handler.to_owned(),
    })
}
