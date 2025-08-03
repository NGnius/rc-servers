mod validate_game_guid;
mod loading_progress;
mod all_loading_progress;
mod weapon_select;
mod activate_sync;
mod loading_done;
//mod player_input;
mod spot_player;
mod kill_player;
mod client_unregister;
mod flipper_start;
mod self_destruct_elimination;
mod map_ping;
mod kill_bonus;
mod assist_bonus;
mod damage_bonus;
mod heal_bonus;

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
            rlnl::events::ingame::PlayerIdAndInputData,
        >::handler(init_ctx))
        .add(flipper_start::handler(init_ctx))
        /*.add(crate::handlers::Broadcaster::<
            true,
            {rlnl::event_code::NetworkEvent::AlignmentRectifierStarted as i16},
            {rlnl::event_code::NetworkEvent::AlignmentRectifierStarted as i16},
            {literustlib::packet::Property::Unreliable as u8},
            rlnl::events::ingame::PlayerId,
        >::handler(init_ctx))*/
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
            {rlnl::event_code::NetworkEvent::MultipleFireMisses as i16},
            {rlnl::event_code::NetworkEvent::MultipleFireMisses as i16},
            {literustlib::packet::Property::Unreliable as u8},
            rlnl::events::ingame::MultipleFireMisses,
        >::handler(init_ctx))
        .add(spot_player::handler(init_ctx))
        .add(crate::handlers::Broadcaster::<
            false,
            {rlnl::event_code::NetworkEvent::DamageCube as i16},
            {rlnl::event_code::NetworkEvent::DestroyCubesFull as i16},
            {literustlib::packet::Property::ReliableOrdered as u8},
            rlnl::events::ingame::DestroyCubesFull,
        >::handler(init_ctx))
        .add(crate::handlers::Broadcaster::<
            false,
            {rlnl::event_code::NetworkEvent::DamageCubeNoEffect as i16},
            {rlnl::event_code::NetworkEvent::DestroyCubeNoEffect as i16},
            {literustlib::packet::Property::ReliableOrdered as u8},
            rlnl::events::ingame::DestroyCubeNoEffect,
        >::handler(init_ctx))
        .add(crate::handlers::Broadcaster::<
            false,
            {rlnl::event_code::NetworkEvent::DamageCubeEffectOnly as i16},
            {rlnl::event_code::NetworkEvent::DestroyCubeEffectOnly as i16},
            {literustlib::packet::Property::Unreliable as u8},
            rlnl::events::ingame::DestroyCubeEffectOnly,
        >::handler(init_ctx))
        .add(damage_bonus::handler(init_ctx))
        .add(heal_bonus::handler(init_ctx))
        .add(crate::handlers::Broadcaster::<
            false,
            {rlnl::event_code::NetworkEvent::HealSelf as i16},
            {rlnl::event_code::NetworkEvent::HealSelfResponse as i16},
            {literustlib::packet::Property::ReliableOrdered as u8},
            rlnl::events::HealedCubes,
        >::handler(init_ctx))
        .add(kill_bonus::handler(init_ctx))
        .add(assist_bonus::handler(init_ctx))
        .add(kill_player::handler(init_ctx))
        .add(crate::handlers::Broadcaster::<
            true,
            {rlnl::event_code::NetworkEvent::EnergyModuleActivated as i16},
            {rlnl::event_code::NetworkEvent::EnergyModuleActivated as i16},
            {literustlib::packet::Property::ReliableOrdered as u8},
            rlnl::events::ingame::PlayerId,
        >::handler(init_ctx))
        .add(crate::handlers::Broadcaster::<
            true,
            {rlnl::event_code::NetworkEvent::RadarModuleActivated as i16},
            {rlnl::event_code::NetworkEvent::RemoteRadarModuleActivated as i16},
            {literustlib::packet::Property::ReliableOrdered as u8},
            rlnl::events::ingame::PlayerId,
        >::handler(init_ctx))
        .add(crate::handlers::Broadcaster::<
            true,
            {rlnl::event_code::NetworkEvent::BroadcastActivateTeleportEffect as i16},
            {rlnl::event_code::NetworkEvent::ActivateTeleportEffect as i16},
            {literustlib::packet::Property::ReliableOrdered as u8},
            rlnl::events::ingame::TeleportActivateEffect,
        >::handler(init_ctx))
        .add(crate::handlers::Broadcaster::<
            true,
            {rlnl::event_code::NetworkEvent::BroadcastSpawnEmpLocator as i16},
            {rlnl::event_code::NetworkEvent::SpawnEmpLocator as i16},
            {literustlib::packet::Property::ReliableOrdered as u8},
            rlnl::events::ingame::SpawnEmpLocator,
        >::handler(init_ctx))
        .add(crate::handlers::Broadcaster::<
            true,
            {rlnl::event_code::NetworkEvent::BroadcastSpawnEmpMachineEffect as i16},
            {rlnl::event_code::NetworkEvent::SpawnEmpMachineEffect as i16},
            {literustlib::packet::Property::ReliableOrdered as u8},
            rlnl::events::ingame::NetworkStunnedMachineEffect,
        >::handler(init_ctx))
        .add(crate::handlers::Broadcaster::<
            true,
            {rlnl::event_code::NetworkEvent::ShieldSpawned as i16},
            {rlnl::event_code::NetworkEvent::SpawnShield as i16},
            {literustlib::packet::Property::ReliableOrdered as u8},
            rlnl::events::ingame::ShieldModuleEvent,
        >::handler(init_ctx))
        .add(crate::handlers::Broadcaster::<
            true,
            {rlnl::event_code::NetworkEvent::Taunt as i16},
            {rlnl::event_code::NetworkEvent::Taunt as i16},
            {literustlib::packet::Property::ReliableOrdered as u8},
            rlnl::events::ingame::Taunt,
        >::handler(init_ctx))
        .add(crate::handlers::Broadcaster::<
            true,
            {rlnl::event_code::NetworkEvent::CosmeticAction as i16},
            {rlnl::event_code::NetworkEvent::CosmeticAction as i16},
            {literustlib::packet::Property::ReliableOrdered as u8},
            rlnl::events::ingame::CosmeticAction,
        >::handler(init_ctx))
        .add(client_unregister::handler(init_ctx))
        .add(crate::handlers::Broadcaster::<
            true,
            {rlnl::event_code::NetworkEvent::BroadcastInvisible as i16},
            {rlnl::event_code::NetworkEvent::MakeInvisible as i16},
            {literustlib::packet::Property::ReliableOrdered as u8},
            rlnl::events::ingame::PlayerId,
        >::handler(init_ctx))
        .add(crate::handlers::Broadcaster::<
            true,
            {rlnl::event_code::NetworkEvent::BroadcastVisible as i16},
            {rlnl::event_code::NetworkEvent::MakeVisible as i16},
            {literustlib::packet::Property::ReliableOrdered as u8},
            rlnl::events::ingame::PlayerId,
        >::handler(init_ctx))
        .add(crate::handlers::Broadcaster::<
            true,
            {rlnl::event_code::NetworkEvent::UpdateVotingAfterBattle as i16},
            {rlnl::event_code::NetworkEvent::UpdateVotingAfterBattle as i16},
            {literustlib::packet::Property::ReliableOrdered as u8},
            rlnl::events::ingame::UpdateVotingAfterBattle,
        >::handler(init_ctx))
        .add(self_destruct_elimination::handler(init_ctx))
        .add(map_ping::handler(init_ctx))
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

mod _broadcast_impls {
    use crate::Broadcastable;

    impl Broadcastable for rlnl::events::ingame::PlayerIdAndInputData {}
    impl Broadcastable for rlnl::events::ingame::WeaponFireEffect {}
    impl Broadcastable for rlnl::events::ingame::FireMiss {}
    impl Broadcastable for rlnl::events::ingame::MultipleFireMisses {}
    impl Broadcastable for rlnl::events::ingame::DestroyCubesFull {}
    impl Broadcastable for rlnl::events::ingame::DestroyCubeNoEffect {}
    impl Broadcastable for rlnl::events::ingame::DestroyCubeEffectOnly {}
    impl Broadcastable for rlnl::events::HealedCubes {}
    impl Broadcastable for rlnl::events::ingame::PlayerId {}
    impl Broadcastable for rlnl::events::ingame::SpawnEmpLocator {}
    impl Broadcastable for rlnl::events::ingame::NetworkStunnedMachineEffect {}
    impl Broadcastable for rlnl::events::ingame::ShieldModuleEvent {}
    impl Broadcastable for rlnl::events::ingame::Taunt {}
    impl Broadcastable for rlnl::events::ingame::CosmeticAction {}
    impl Broadcastable for rlnl::events::ingame::UpdateVotingAfterBattle {}
    impl Broadcastable for rlnl::events::ingame::TeleportActivateEffect {}
    impl Broadcastable for rlnl::events::ingame::MapPing {}

    impl Broadcastable for rlnl::events::sync::UpdateGameModeSettings {}
    impl Broadcastable for rlnl::events::GameTime {}
    impl Broadcastable for rlnl::events::ingame::TeamBaseState {}
}
