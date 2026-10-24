const SLEEP_PERIOD: std::time::Duration = std::time::Duration::from_millis(250);

type PlayerInfo = (super::generic::UserSender, std::sync::Arc<super::generic::UserState>);

pub struct MatchTimerHandle {
    handle: tokio::task::JoinHandle<()>,
    new_player_tx: tokio::sync::mpsc::Sender<PlayerInfo>,
}

impl MatchTimerHandle {
    #[inline]
    pub fn abort(&self) {
        self.handle.abort();
    }

    pub async fn add_player(&self, player: PlayerInfo) {
        if let Err(_e) = self.new_player_tx.send(player).await {
            log::error!("Failed to add new player to timer task (send failure)");
        }
    }

    pub fn spawn(players: Vec<PlayerInfo>, game_start: chrono::DateTime<chrono::Utc>, game_end: chrono::DateTime<chrono::Utc>, start_packets: Vec<super::RlnlPacket>, end_packets: Vec<super::RlnlPacket>) -> Self {
        match_time_syncer(players, game_start, game_end, start_packets, end_packets)
    }
}

fn match_time_syncer(players: Vec<PlayerInfo>, game_start: chrono::DateTime<chrono::Utc>, game_end: chrono::DateTime<chrono::Utc>, extra_packets: Vec<super::RlnlPacket>, end_packets: Vec<super::RlnlPacket>) -> MatchTimerHandle {
    let (tx, rx) = tokio::sync::mpsc::channel(4);
    let handle = tokio::spawn(do_match_timer_async(players, game_start, game_end, extra_packets, end_packets, rx));
    MatchTimerHandle {
        handle,
        new_player_tx: tx,
    }
}

pub fn time_to_game_end_payload(game_end: chrono::DateTime<chrono::Utc>) -> rlnl::events::GameTime {
    let now = chrono::Utc::now();
    let time_until_end = game_end.signed_duration_since(now);
    let time_until_end_f32 = (time_until_end.num_milliseconds().clamp(0, i64::MAX) as f32) / 1000.0;
    rlnl::events::GameTime(time_until_end_f32)
}

async fn do_match_timer_async(mut players: Vec<PlayerInfo>, game_start: chrono::DateTime<chrono::Utc>, game_end: chrono::DateTime<chrono::Utc>, extra_packets: Vec<super::RlnlPacket>, end_packets: Vec<super::RlnlPacket>, mut new_player_rx: tokio::sync::mpsc::Receiver<PlayerInfo>) {
    let now = chrono::Utc::now();
    let time_until_start_ms = game_start.signed_duration_since(now).num_milliseconds().clamp(0, i64::MAX) + SLEEP_PERIOD.as_millis() as i64;
    tokio::time::sleep(std::time::Duration::from_millis(time_until_start_ms as u64)).await;
    let extras_count = extra_packets.len();
    for packet in extra_packets {
        for player in players.iter() {
            let mode = super::generic::ConnectionMode::from_u8(player.1.mode.load(std::sync::atomic::Ordering::Relaxed));
            if matches!(mode, super::generic::ConnectionMode::InGame) {
                let sender = player.0.rlnl();
                crate::events::log_lnl_send_failure(sender.send_data(
                    &*packet.data,
                    packet.event,
                    packet.property,
                    &player.0.connection
                ).await);
            }
        }
    }
    if extras_count != 0 {
        log::info!("Broadcast {} custom packets for game start", extras_count);
    }
    'timer_loop: loop {
        if let Ok(new_player) = new_player_rx.try_recv() {
            players.push(new_player);
        }
        let payload = time_to_game_end_payload(game_end);
        'player_loop: for player in players.iter() {
            if !player.0.connection.is_connected() { continue 'player_loop; }
            let sender = player.0.rlnl();
            if let Err(e) = sender.send_data(
                &payload,
                rlnl::event_code::NetworkEvent::CurrentGameTime,
                literustlib::packet::Property::Unreliable,
                &player.0.connection)
            .await {
                log::error!("Failed to send CurrentGameTime event to a user: {}", e);
            }
        }
        if payload.0 > f32::EPSILON {
            tokio::time::sleep(SLEEP_PERIOD).await;
        } else {
            break 'timer_loop;
        }
    }
    for packet in end_packets {
        for player in players.iter() {
            let mode = super::generic::ConnectionMode::from_u8(player.1.mode.load(std::sync::atomic::Ordering::Relaxed));
            if !matches!(mode, super::generic::ConnectionMode::Disconnected) {
                let sender = player.0.rlnl();
                crate::events::log_lnl_send_failure(sender.send_data(
                    &*packet.data,
                    packet.event,
                    packet.property,
                    &player.0.connection
                ).await);
            }
        }
    }
    log::debug!("Game timer (a)sync thread has completed");
}
