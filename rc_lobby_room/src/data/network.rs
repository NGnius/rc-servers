#[derive(Clone)]
pub struct NetworkConfigData {
    pub network_channel_ty: String,
    pub max_sent_message_queue_size: u16,
    pub is_acks_long: bool,
    pub network_drop_threshold: u8,
    pub packet_size: u16,
    pub max_combined_reliable_message_count: u16,
    pub max_combined_reliable_message_size: u16,
    pub min_update_timeout: u16, // stored as a u32 for some reason
    pub max_delay: i64,
    pub overflow_threshold: u8,
    pub max_packet_size: u16,
    pub resend_delay_base: f64,
    pub resend_delay_rtt_mult: f64,
    pub network_peer_update_interval: i32,
    pub max_delay_for_disconnect: i32, // milliseconds
}

impl NetworkConfigData {
    pub fn as_transmissible<C>(&self) -> polariton::operation::Typed<C> {
        polariton::operation::Typed::HashMap(vec![
            (polariton::operation::Typed::Str("NetworkChannelTypes".into()), polariton::operation::Typed::Str(self.network_channel_ty.clone().into())),
            (polariton::operation::Typed::Str("MaxSentMessageQueueSize".into()), polariton::operation::Typed::Long(self.max_sent_message_queue_size as _)),
            (polariton::operation::Typed::Str("IsAcksLong".into()), polariton::operation::Typed::Long(self.is_acks_long as _)),
            (polariton::operation::Typed::Str("NetworkDropThreshold".into()), polariton::operation::Typed::Long(self.network_drop_threshold as _)),
            (polariton::operation::Typed::Str("PacketSize".into()), polariton::operation::Typed::Long(self.packet_size as _)),
            (polariton::operation::Typed::Str("MaxCombinedReliableMessageCount".into()), polariton::operation::Typed::Long(self.max_combined_reliable_message_count as _)),
            (polariton::operation::Typed::Str("MaxCombinedReliableMessageSize".into()), polariton::operation::Typed::Long(self.max_combined_reliable_message_size as _)),
            (polariton::operation::Typed::Str("MinUpdateTimeout".into()), polariton::operation::Typed::Long(self.min_update_timeout as _)),
            (polariton::operation::Typed::Str("MaxDelay".into()), polariton::operation::Typed::Long(self.max_delay)),
            (polariton::operation::Typed::Str("OverflowThreshold".into()), polariton::operation::Typed::Long(self.overflow_threshold as _)),
            (polariton::operation::Typed::Str("MaxPacketSize".into()), polariton::operation::Typed::Long(self.max_packet_size as _)),
            (polariton::operation::Typed::Str("ResendDelayBase".into()), polariton::operation::Typed::Double(self.resend_delay_base)),
            (polariton::operation::Typed::Str("ResendDelayRttMult".into()), polariton::operation::Typed::Double(self.resend_delay_rtt_mult)),
            (polariton::operation::Typed::Str("NetworkPeerUpdateInterval".into()), polariton::operation::Typed::Long(self.network_peer_update_interval as _)),
            (polariton::operation::Typed::Str("MaxMillisecondsDelayForBeingDisconnected".into()), polariton::operation::Typed::Long(self.max_delay_for_disconnect as _)),
        ].into())
    }

    pub fn from_conf(conf: oj_rc_core::persist::NetworkConf) -> Self {
        Self {
            network_channel_ty: conf.network_channel_ty,
            max_sent_message_queue_size: conf.max_sent_message_queue_size,
            is_acks_long: conf.is_acks_long,
            network_drop_threshold: conf.network_drop_threshold,
            packet_size: conf.packet_size,
            max_combined_reliable_message_count: conf.max_combined_reliable_message_count,
            max_combined_reliable_message_size: conf.max_combined_reliable_message_size,
            min_update_timeout: conf.min_update_timeout,
            max_delay: conf.max_delay,
            overflow_threshold: conf.overflow_threshold,
            max_packet_size: conf.max_packet_size,
            resend_delay_base: conf.resend_delay_base,
            resend_delay_rtt_mult: conf.resend_delay_rtt_mult,
            network_peer_update_interval: conf.network_peer_update_interval,
            max_delay_for_disconnect: conf.max_delay_for_disconnect_ms,
        }
    }
}
