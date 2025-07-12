pub fn match_countdown(players: Vec<super::generic::UserSender>, game_start: chrono::DateTime<chrono::Utc>) {
    tokio::spawn(do_match_countdown_async(players, game_start));
}

pub fn time_to_game_start_payload(game_start: chrono::DateTime<chrono::Utc>) -> rlnl::events::GameTime {
    let now = chrono::Utc::now();
    let time_until_start = game_start.signed_duration_since(now);
    let time_until_start_f32 = (time_until_start.abs().num_milliseconds() as f32) / 1000.0;
    rlnl::events::GameTime(time_until_start_f32)
}

async fn do_match_countdown_async(players: Vec<super::generic::UserSender>, game_start: chrono::DateTime<chrono::Utc>) {
    let now = chrono::Utc::now();
    let time_until_start = game_start.signed_duration_since(now);

    let payload = time_to_game_start_payload(game_start);
    for player in players.iter() {
        let sender = player.rlnl();
        if let Err(e) = sender.send_data(
            &payload,
            rlnl::event_code::NetworkEvent::TimeToGameStart,
            literustlib::packet::Property::ReliableOrdered,
            &player.connection)
        .await {
            log::error!("Failed to send TimeToGameStart to a user: {}", e);
        }
    }

    tokio::time::sleep(time_until_start.to_std().unwrap_or_default()).await;
    log::debug!("Sending starting game event");
    let payload = rlnl::events::ingame::GameStart {
        is_reconnecting: 0,
    };
    for player in players {
        let sender = player.rlnl();
        if let Err(e) = sender.send_data(
            &payload,
            rlnl::event_code::NetworkEvent::GameStarted,
            literustlib::packet::Property::ReliableOrdered,
            &player.connection)
        .await {
            log::error!("Failed to send GameStarted event to a user: {}", e);
        }
    }
}
