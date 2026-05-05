use rand::RngExt;
use byteserde::ser_heap::ByteSerializeHeap;

pub struct ExperimentalPlayer {
    me: oj_rc_core::persist::user::PlayerDescriptor,
    is_complete: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl ExperimentalPlayer {
    pub fn new(me: oj_rc_core::persist::user::PlayerDescriptor) -> Self {
        Self {
            me,
            is_complete: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }
}

#[async_trait::async_trait]
impl super::FakeUser for ExperimentalPlayer {
    async fn on_init(&self, _descriptors: &[oj_rc_core::persist::user::PlayerDescriptor], _player_id: u8) {

    }

    async fn on_ready(&self, real_players: &std::collections::HashMap<u8, crate::matches::generic::UserSender>) {
        let movement_rx = real_players.values().map(|x| x.to_owned()).collect();
        let is_complete = self.is_complete.clone();
        let player_id = self.me.player_id;
        tokio::task::spawn(erratic_behaviour(movement_rx, is_complete, player_id));
    }

    async fn on_end(&self) {
        self.is_complete.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

fn random_map_coord(rng: &mut rand::rngs::ThreadRng) -> f32 {
    const CONVERSION_FACTOR: f32 = 300.0 / (i16::MAX as f32);
    let int = rng.random::<i16>();
    (int as f32) * CONVERSION_FACTOR
}

fn random_height_coord(rng: &mut rand::rngs::ThreadRng) -> f32 {
    const CONVERSION_FACTOR: f32 = 50.0 / (i16::MAX as f32);
    let int = rng.random::<i16>();
    ((int as f32) * CONVERSION_FACTOR) + 10.0
}

async fn erratic_behaviour(send_to: Vec<crate::matches::generic::UserSender>, is_complete: std::sync::Arc<std::sync::atomic::AtomicBool>, player_id: u8) {
    const SLEEP_PERIOD: std::time::Duration = std::time::Duration::from_secs(1);
    let mut fake_timestamp = 42.0;
    while !is_complete.load(std::sync::atomic::Ordering::Relaxed) {
        let (pos_x, pos_y, pos_z) = {
            let mut rng = rand::rng();
            let pos_x: f32 = random_map_coord(&mut rng);
            let pos_y: f32 = random_height_coord(&mut rng);
            let pos_z: f32 = random_map_coord(&mut rng);
            (pos_x, pos_y, pos_z)
        };
        let motion = rlnl::machine_motion::MachineMotion {
            last_sent_seconds_a: 1.0,
            last_sent_seconds_b: 2.0,
            timestamp: fake_timestamp,
            player_id,
            target_point: rlnl::types::CompressedVec3::from((50.0, 50.0, 50.0)),
            rb_state: rlnl::machine_motion::RigidBodyState {
                rb_pos_rot: rlnl::types::PosQuatPair {
                    pos: rlnl::types::CompressedVec3::from((pos_x, pos_y, pos_z)),
                    rot: rlnl::types::CompressedQuat { x: 0, y: 0, z: 0 },
                },
                angular_velocity: rlnl::types::CompressedVec3::from((0.0, 0.0, 0.0)),
                center_of_mass: rlnl::types::CompressedVec3::from((1.0, 1.0, 1.0)),
            },
        };
        let mut ser = byteserde::ser_heap::ByteSerializerHeap::default();
        if let Err(e) = motion.byte_serialize_heap(&mut ser) {
            log::error!("Failed to serialize motion data from experimental fake player: {}", e);
        } else {
            let motion_data = bytes::Bytes::copy_from_slice(ser.as_slice());
            for conn in send_to.iter() {
                send_motion_data_to(conn, motion_data.clone()).await;
            }
            log::debug!("Moved experimental bot to ({}, {}, {})", pos_x, pos_y, pos_z);
        }
        fake_timestamp += 1.0;
        tokio::time::sleep(SLEEP_PERIOD).await;
    }
    is_complete.store(false, std::sync::atomic::Ordering::Relaxed);
}

async fn send_motion_data_to(to: &crate::matches::generic::UserSender, motion: bytes::Bytes) {
    crate::events::log_lnl_send_failure(to.sender.send_data(crate::handler::EventData {
        message_ty: crate::data::MessageType::RobotMotion,
        variant: 0,
        data_size: motion.len() as _,
        data: motion,
    }, literustlib::packet::Property::Unreliable, &to.connection).await);
}
