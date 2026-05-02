use byteserde::prelude::ByteDeserializeSlice;

pub struct VehicleMotionHandler {
    msg_router: tokio::sync::mpsc::Sender<crate::matches::GameMessage>,
}

pub(super) fn handler(init_ctx: &crate::InitConfig) -> VehicleMotionHandler {
    VehicleMotionHandler::new(init_ctx)
}

impl VehicleMotionHandler {
    fn new(init_ctx: &crate::InitConfig) -> Self {
        Self {
            msg_router: init_ctx.matches_chann.clone(),
        }
    }
}

#[async_trait::async_trait]
impl crate::RobotMotionHandler for VehicleMotionHandler {
    async fn handle(&self, data: &bytes::Bytes, user: &crate::UserData) {
        if let Some(user_info) = user.user().await {
            let mut des = byteserde::des_slice::ByteDeserializerSlice::new(data);
            match rlnl::machine_motion::MachineMotion::byte_deserialize(&mut des) {
                Ok(motion_data) => {
                    crate::events::log_channel_send_failure(self.msg_router.send(crate::matches::GameMessage::Motion {
                        user_id: user_info.account_id(),
                        motion: motion_data,
                    }).await);
                },
                Err(e) => {
                    log::error!("Bad deserialization for machine motion, bytes {:?}: {}", &data[..], e);
                }
            }

        } else {
            log::error!("Failed to handle motion unknown user");
        }
    }
}
