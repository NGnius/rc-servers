mod no_op;
#[allow(unused_imports)]
pub use no_op::NoOpLogic;

mod elimination;
pub use elimination::EliminationLogic;

mod battle_arena;
pub use battle_arena::BattleArenaLogic;

mod pit;
pub use pit::PitLogic;

mod team_death_match;
pub use team_death_match::TeamDeathMatchLogic;

pub(super) mod trackers;

async fn respawn_player_after(after: chrono::DateTime<chrono::Utc>, players: Vec<crate::matches::generic::UserSender>, spawn: oj_rc_core::persist::config::Point, player_id: u8, alive_flag: std::sync::Arc<std::sync::atomic::AtomicBool>) {
    let sleep_dur = after.signed_duration_since(chrono::Utc::now()).to_std().expect("Respawn duration too long to sleep");
    tokio::time::sleep(sleep_dur).await;
    let spawn_payload = rlnl::events::sync::SpawnPoint {
        pos: rlnl::types::PosQuatPair {
            pos: rlnl::types::CompressedVec3::from((spawn.x, spawn.y, spawn.z)),
            rot: rlnl::types::CompressedQuat { x: 0, y: 0, z: 0 },
        },
        owner: player_id,
    };
    log::debug!("Respawning player {} after {}ms", player_id, sleep_dur.as_millis());
    for player in players {
        if !player.connection.is_connected() { continue; }
        crate::events::log_lnl_send_failure(player.rlnl().send_data(
            &spawn_payload,
            rlnl::event_code::NetworkEvent::FreeRespawnPoint,
            literustlib::packet::Property::ReliableOrdered,
            &player.connection,
        ).await);
    }
    alive_flag.store(true, std::sync::atomic::Ordering::Relaxed);
}
