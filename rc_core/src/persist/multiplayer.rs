use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MultiplayerConfig {
    pub players_per_game: usize,
    pub enabled: bool,
    #[serde(default = "default_net_conf")]
    pub network: NetworkConf,
    #[serde(default = "default_fake_users")]
    pub fakes: Vec<FakePlayerConf>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetworkConf {
    pub network_channel_ty: String,
    pub max_sent_message_queue_size: u16,
    pub is_acks_long: bool,
    pub network_drop_threshold: u8,
    pub packet_size: u16,
    pub max_combined_reliable_message_count: u16,
    pub max_combined_reliable_message_size: u16,
    pub min_update_timeout: u16,
    pub max_delay: i64,
    pub overflow_threshold: u8,
    pub max_packet_size: u16,
    pub resend_delay_base: f64,
    pub resend_delay_rtt_mult: f64,
    pub network_peer_update_interval: i32,
    pub max_delay_for_disconnect_ms: i32,
}

pub(super) fn default_net_conf() -> NetworkConf {
    NetworkConf {
        network_channel_ty: "3113".to_owned(),
        max_sent_message_queue_size: 64,
        is_acks_long: true,
        network_drop_threshold: 80,
        packet_size: 1200,
        max_combined_reliable_message_count: 20,
        max_combined_reliable_message_size: 200,
        min_update_timeout: 1,
        max_delay: 1,
        overflow_threshold: 10,
        max_packet_size: 5888,
        resend_delay_base: 0.1,
        resend_delay_rtt_mult: 0.5,
        network_peer_update_interval: 1,
        max_delay_for_disconnect_ms: 1000,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FakePlayerConf {
    pub public_id: String,
    pub display_name: String,
    pub team: u8,
    #[serde(flatten)]
    pub implementation: ClientEmulation,
}

pub(super) fn default_fake_users() -> Vec<FakePlayerConf> {
    //Vec::default()
    vec![
        FakePlayerConf {
            public_id: "ServerExperiment01".to_owned(),
            display_name: "Server".to_owned(),
            team: 1,
            implementation: ClientEmulation::Experimental,
        }
    ]
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "impl")]
pub enum ClientEmulation {
    Experimental,
}

impl ClientEmulation {
    pub(super) fn to_config(self) -> super::config::ClientEmulator {
        match self {
            Self::Experimental => super::config::ClientEmulator::Experiment,
        }
    }
}
