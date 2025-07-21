mod validate_game_guid;
mod loading_progress;
mod all_loading_progress;
mod weapon_select;
mod activate_sync;
mod loading_done;
//mod player_input;

pub async fn handler(init_ctx: &crate::InitConfig) -> crate::handler::LnlEventHandler {
    crate::handler::LnlEventHandler::new(init_ctx.users.clone(), crate::vehicle_motion::handler(init_ctx))
        .add(validate_game_guid::handler(init_ctx))
        .add(loading_progress::handler(init_ctx))
        .add(all_loading_progress::handler(init_ctx))
        .add(weapon_select::handler(init_ctx))
        .add(activate_sync::handler(init_ctx))
        .add(loading_done::handler(init_ctx))
        //.add(player_input::handler(init_ctx))
        .add(crate::handlers::Broadcaster::<
            true,
            {rlnl::event_code::NetworkEvent::OnPlayerInputChanged as i16},
            {rlnl::event_code::NetworkEvent::OnServerReceivedInputChange as i16},
            {literustlib::packet::Property::Unreliable as u8},
            rlnl::events::ingame::MultiPlayerInputChanged,
        >::handler(init_ctx))
        .add(crate::handlers::DatalessBroadcaster::<
            true,
            {rlnl::event_code::NetworkEvent::AlignmentRectifierStarted as i16},
            {rlnl::event_code::NetworkEvent::AlignmentRectifierStarted as i16},
            {literustlib::packet::Property::Unreliable as u8},
        >::handler(init_ctx))
        .add(crate::handlers::Broadcaster::<
            true,
            {rlnl::event_code::NetworkEvent::FireWeaponEffect as i16},
            {rlnl::event_code::NetworkEvent::FireWeaponEffect as i16},
            {literustlib::packet::Property::Unreliable as u8},
            rlnl::events::ingame::WeaponFireEffect,
        >::handler(init_ctx))
        .add(crate::handlers::Broadcaster::<
            true,
            {rlnl::event_code::NetworkEvent::FireMiss as i16},
            {rlnl::event_code::NetworkEvent::FireMiss as i16},
            {literustlib::packet::Property::Unreliable as u8},
            rlnl::events::ingame::FireMiss,
        >::handler(init_ctx))
        .add(crate::handlers::Broadcaster::<
            true,
            {rlnl::event_code::NetworkEvent::EnemySpotted as i16},
            {rlnl::event_code::NetworkEvent::EnemySpotted as i16},
            {literustlib::packet::Property::ReliableOrdered as u8},
            rlnl::events::ingame::SpottingIds,
        >::handler(init_ctx))
        .add(crate::handlers::Broadcaster::<
            true,
            {rlnl::event_code::NetworkEvent::DamageCube as i16},
            {rlnl::event_code::NetworkEvent::DestroyCubesFull as i16},
            {literustlib::packet::Property::ReliableOrdered as u8},
            rlnl::events::ingame::DestroyCubesFull,
        >::handler(init_ctx))
        .add(crate::handlers::Broadcaster::<
            true,
            {rlnl::event_code::NetworkEvent::DamageCubeNoEffect as i16},
            {rlnl::event_code::NetworkEvent::DestroyCubeNoEffect as i16},
            {literustlib::packet::Property::ReliableOrdered as u8},
            rlnl::events::ingame::DestroyCubeNoEffect,
        >::handler(init_ctx))
        .add(crate::handlers::Broadcaster::<
            true,
            {rlnl::event_code::NetworkEvent::DamageCubeEffectOnly as i16},
            {rlnl::event_code::NetworkEvent::DestroyCubeEffectOnly as i16},
            {literustlib::packet::Property::Unreliable as u8},
            rlnl::events::ingame::DestroyCubeEffectOnly,
        >::handler(init_ctx))
}

#[inline]
pub fn log_channel_send_failure<T>(result: Result<(), tokio::sync::mpsc::error::SendError<T>>) {
    if result.is_err() {
        log::error!("Failed to send game message");
    }
}

#[inline]
pub fn log_lnl_send_failure(result: std::io::Result<usize>) {
    if let Err(e) = result {
        log::error!("Failed to send packet: {}", e);
    }
}
