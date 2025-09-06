use crate::matches::CustomGameLogic;

struct PlayerTracker {
    connected: tokio::sync::Mutex<std::collections::HashMap<u8, std::collections::HashSet<u8>>>, // team -> set of player_id
    in_point: tokio::sync::RwLock<std::collections::HashMap<u8, std::sync::atomic::AtomicU16>>, // player_id -> in point state (if val > u8::MAX then not in a point)
    respawning: tokio::sync::RwLock<std::collections::HashMap<u8, std::sync::atomic::AtomicI64>>, // player_id -> time when they'll spawn (time since unix epoch)
}

impl PlayerTracker {
    fn new() -> Self {
        Self {
            connected: tokio::sync::Mutex::new(std::collections::HashMap::new()),
            in_point: tokio::sync::RwLock::new(std::collections::HashMap::new()),
            respawning: tokio::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    async fn team(&self, player_id: u8) -> Option<u8> {
        for (team, players) in self.connected.lock().await.iter() {
            if players.contains(&player_id) {
                return Some(*team);
            }
        }
        None
    }

    async fn swap_is_in_point(&self, player_id: u8, point: Option<u8>) -> Option<u8> {
        self.in_point.read().await.get(&player_id).and_then(|x| {
            let old_point = x.swap(point.map(|x| x as u16).unwrap_or(u16::MAX), std::sync::atomic::Ordering::Relaxed);
            if old_point > u8::MAX as u16 {
                None
            } else {
                Some(old_point as u8)
            }
        })
    }

    async fn track_player(&self, player: &oj_rc_core::persist::user::PlayerDescriptor) {
        let mut conn_lock = self.connected.lock().await;
        if let Some(team) = conn_lock.get_mut(&(player.team as u8)) {
            team.insert(player.player_id);
        } else {
            let mut new_team = std::collections::HashSet::new();
            new_team.insert(player.player_id);
            conn_lock.insert(player.team as u8, new_team);
        }
        self.in_point.write().await.insert(player.player_id, std::sync::atomic::AtomicU16::new(u16::MAX));
        self.respawning.write().await.insert(player.player_id, std::sync::atomic::AtomicI64::new(i64::MIN));
    }

    async fn disconnect_player(&self, player_id: u8) {
        let mut conn_lock = self.connected.lock().await;
        for team_members in conn_lock.values_mut() {
            team_members.remove(&player_id);
        }
    }

    async fn single_remaining_team(&self) -> Option<u8> {
        let conn_lock = self.connected.lock().await;
        let mut first_remaining_team = None;
        for (team, team_members) in conn_lock.iter() {
            if !team_members.is_empty() {
                if first_remaining_team.is_some() {
                    return None;
                } else {
                    first_remaining_team = Some(*team)
                }
            }
        }
        first_remaining_team
    }
}

struct PointInfo {
    team: std::sync::atomic::AtomicI8,
    on_point: tokio::sync::RwLock<std::collections::HashMap<u8, std::sync::atomic::AtomicU8>>,
    capture: atomic_float::AtomicF32,
    percent_per_second: f32,
}

impl PointInfo {
    fn new(percent_per_second: f32) -> Self {
        Self {
            team: std::sync::atomic::AtomicI8::new(-1),
            on_point: tokio::sync::RwLock::new([
                (0, std::sync::atomic::AtomicU8::new(0)),
                (1, std::sync::atomic::AtomicU8::new(0)),
            ].into_iter().collect()),
            capture: atomic_float::AtomicF32::new(0.0),
            percent_per_second,
        }
    }

    /*async fn friendlies_on_point(&self, team: u8) -> u8 {
        if let Some(counter) = self.on_point.read().await.get(&team) {
            counter.load(std::sync::atomic::Ordering::SeqCst)
        } else {
            0
        }
    }*/

    async fn enemies_on_point(&self, team: u8) -> u8 {
        let mut total = 0;
        for (iter_team, counter) in self.on_point.read().await.iter() {
            if team == *iter_team { continue; }
            total += counter.load(std::sync::atomic::Ordering::SeqCst);
        }
        total
    }

    async fn owners_on_point(&self) -> u8 {
        let team = self.team.load(std::sync::atomic::Ordering::SeqCst);
        if team < 0 {
            0
        } else if let Some(counter) = self.on_point.read().await.get(&(team as u8)) {
            counter.load(std::sync::atomic::Ordering::SeqCst)
        } else {
            0
        }
    }

    async fn stealers_on_point(&self) -> u8 {
        let owning_team = self.team.load(std::sync::atomic::Ordering::SeqCst);
        let mut total = 0;
        if owning_team < 0 {
            for counter in self.on_point.read().await.values() {
                total += counter.load(std::sync::atomic::Ordering::SeqCst);
            }
        } else {
            for (team, counter) in self.on_point.read().await.iter() {
                if (owning_team as u8) == *team { continue; }
                total += counter.load(std::sync::atomic::Ordering::SeqCst);
            }
        }
        total
    }

    async fn stealers_team(&self) -> Option<u8> {
        let owning_team = self.team.load(std::sync::atomic::Ordering::SeqCst);
        let mut stealing_team = None;
        if owning_team < 0 {
            for (team, counter) in self.on_point.read().await.iter() {
                let count = counter.load(std::sync::atomic::Ordering::SeqCst);
                if count != 0 {
                    if stealing_team.is_some() {
                        return None;
                    } else {
                        stealing_team = Some(*team);
                    }
                }
            }
        } else {
            let owning_team = owning_team as u8;
            for (team, counter) in self.on_point.read().await.iter() {
                if owning_team == *team { continue; }
                let count = counter.load(std::sync::atomic::Ordering::SeqCst);
                if count != 0 {
                    if stealing_team.is_some() {
                        return None;
                    } else {
                        stealing_team = Some(*team);
                    }
                }
            }
        }
        stealing_team
    }
}

struct PointTracker {
    points: Vec<PointInfo>,
    last_tick: std::sync::atomic::AtomicI64,
}

struct PointTickInfo {
    owned: std::collections::HashMap<u8, u8>, // team -> capture point count
    captured_firsts: std::collections::HashSet<u8>, // team
    lost_lasts: std::collections::HashSet<u8>, // team
    delta: i64,
}

impl PointTracker {
    const TICK_MS: i64 = 50;

    fn new(points: impl Iterator<Item=f32>) -> Self {
        Self {
            points: points.map(PointInfo::new).collect(),
            last_tick: std::sync::atomic::AtomicI64::new(i64::MIN),
        }
    }

    async fn on_enter(&self, generic: &crate::matches::GenericGamemodeEngine<BattleArenaLogic>, point_i: u8, _player_id: u8, player_team: i8) {
        if player_team < 0 {
            return;
        }
        let player_team_u8 = player_team as u8;
        if let Some(point) = self.points.get(point_i as usize) {
            let point_team = point.team.load(std::sync::atomic::Ordering::SeqCst);
            if !point.on_point.read().await.contains_key(&player_team_u8) {
                point.on_point.write().await.insert(player_team_u8, std::sync::atomic::AtomicU8::new(0));
            }
            if point_team == player_team {
                let old_friendlies = point.on_point.read().await[&player_team_u8].fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let current_enemies = point.enemies_on_point(player_team_u8).await;
                if current_enemies != 0 && old_friendlies == 0 {
                    generic.broadcast(
                        rlnl::event_code::NetworkEvent::CapturePointNotification,
                        literustlib::packet::Property::ReliableOrdered,
                        &rlnl::events::ingame::CapturePointNotification {
                            notification: rlnl::types::CapturePointNotificationType::CaptureLocked,
                            id: point_i,
                            defending_team: point_team,
                            attacking_team: player_team,
                        },
                        true,
                    ).await;
                }
            } else {
                let old_enemies = point.on_point.read().await[&player_team_u8].fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let current_contesters = point.enemies_on_point(player_team_u8).await;
                if old_enemies == 0 {
                    generic.broadcast(
                        rlnl::event_code::NetworkEvent::CapturePointNotification,
                        literustlib::packet::Property::ReliableOrdered,
                        &rlnl::events::ingame::CapturePointNotification {
                            notification: rlnl::types::CapturePointNotificationType::CaptureStarted,
                            id: point_i,
                            defending_team: point_team,
                            attacking_team: player_team,
                        },
                        true,
                    ).await;
                    if current_contesters != 0 {
                        generic.broadcast(
                            rlnl::event_code::NetworkEvent::CapturePointNotification,
                            literustlib::packet::Property::ReliableOrdered,
                            &rlnl::events::ingame::CapturePointNotification {
                                notification: rlnl::types::CapturePointNotificationType::CaptureLocked,
                                id: point_i,
                                defending_team: point_team,
                                attacking_team: player_team,
                            },
                            true,
                        ).await;
                    }
                }
            }
        }
    }

    async fn on_exit(&self, generic: &crate::matches::GenericGamemodeEngine<BattleArenaLogic>, point_i: u8, _player_id: u8, player_team: i8, max_progress: f32) {
        if player_team < 0 {
            return;
        }
        let player_team_u8 = player_team as u8;
        if let Some(point) = self.points.get(point_i as usize) {
            let point_team = point.team.load(std::sync::atomic::Ordering::SeqCst);
            if !point.on_point.read().await.contains_key(&player_team_u8) {
                point.on_point.write().await.insert(player_team_u8, std::sync::atomic::AtomicU8::new(0));
            }
            if point_team == player_team {
                let old_friendlies = point.on_point.read().await[&player_team_u8].fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                let current_enemies = point.enemies_on_point(player_team_u8).await;
                if old_friendlies == 0 {
                    // something is out of sync, let's just ignore it and try to undo any underflow
                    log::warn!("Team {} players on point {} counting error", player_team, point_i);
                    point.on_point.read().await[&player_team_u8].store(0, std::sync::atomic::Ordering::SeqCst);
                } else if old_friendlies == 1 && current_enemies != 0 {
                    generic.broadcast(
                        rlnl::event_code::NetworkEvent::CapturePointNotification,
                        literustlib::packet::Property::ReliableOrdered,
                        &rlnl::events::ingame::CapturePointNotification {
                            notification: rlnl::types::CapturePointNotificationType::CaptureUnlocked,
                            id: point_i,
                            defending_team: point_team,
                            attacking_team: player_team,
                        },
                        true,
                    ).await;
                }
            } else {
                let old_enemies = point.on_point.read().await[&player_team_u8].fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                //let current_friendlies = point.friendlies.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                if old_enemies == 0 {
                    // something is out of sync, let's just ignore it and try to undo any underflow
                    log::warn!("Team {} players on point {} counting error", player_team, point_i);
                    point.on_point.read().await[&player_team_u8].store(0, std::sync::atomic::Ordering::SeqCst);
                } else if old_enemies == 1 {
                    //log::info!("Enemy has left the capture point");
                    generic.broadcast(
                        rlnl::event_code::NetworkEvent::CapturePointNotification,
                        literustlib::packet::Property::ReliableOrdered,
                        &rlnl::events::ingame::CapturePointNotification {
                            notification: rlnl::types::CapturePointNotificationType::CaptureStoppedNoAttackers,
                            id: point_i,
                            defending_team: point_team,
                            attacking_team: player_team,
                        },
                        true,
                    ).await;
                    let progress_now = point.capture.load(std::sync::atomic::Ordering::SeqCst).floor();
                    point.capture.store(progress_now, std::sync::atomic::Ordering::SeqCst);
                    let data = rlnl::events::ingame::TeamBaseState {
                        base_team_or_mining_point_index: point_i,
                        current_progress: rlnl::types::ByteFloat::from(progress_now),
                        max_progress: rlnl::types::ByteFloat::from(max_progress),
                    };
                    generic.broadcast(
                        rlnl::event_code::NetworkEvent::CapturePointProgress,
                        literustlib::packet::Property::ReliableOrdered,
                        &data,
                        true
                    ).await;
                }
            }
        }
    }

    async fn tick(&self, generic: &crate::matches::GenericGamemodeEngine<BattleArenaLogic>, max_progress: f32) -> Option<PointTickInfo> {
        let now = chrono::Utc::now().timestamp_millis();
        let last_tick = self.last_tick.load(std::sync::atomic::Ordering::SeqCst);
        let delta = if last_tick == i64::MIN {
            // first tick
            self.last_tick.store(now, std::sync::atomic::Ordering::SeqCst);
            1
        } else {
            let delta = (now - last_tick) / Self::TICK_MS;
            if delta == 0 { return None; }
            self.last_tick.store(last_tick + (delta * Self::TICK_MS), std::sync::atomic::Ordering::SeqCst);
            delta
        };
        let mut owned_points = std::collections::HashMap::with_capacity(2);
        let mut captured_firsts = std::collections::HashSet::new();
        let mut lost_lasts = std::collections::HashSet::new();
        for (i, cap_point) in self.points.iter().enumerate() {
            let point_owner = cap_point.team.load(std::sync::atomic::Ordering::SeqCst);
            if point_owner >= 0 {
                let point_owner = point_owner as u8;
                if let Some(count) = owned_points.get_mut(&point_owner) {
                    *count += 1;
                } else {
                    owned_points.insert(point_owner, 1);
                }
            }
            let friendlies = cap_point.owners_on_point().await;
            let enemies = cap_point.stealers_on_point().await;
            if friendlies != 0 { continue; }
            if enemies == 0 { continue; }
            let stealing_team = cap_point.stealers_team().await;
            if stealing_team.is_none() { continue; }
            let stealing_team = stealing_team.unwrap();
            //log::info!("Team {} is capturing point {} (# of players: {})", stealing_team, i, enemies);
            let to_add = (delta as f32) * (Self::TICK_MS as f32) * cap_point.percent_per_second * max_progress / (100.0 * 1000.0);
            let pre_add = cap_point.capture.fetch_add(to_add, std::sync::atomic::Ordering::SeqCst);
            let post_add = pre_add + to_add;
            if post_add >= max_progress {
                let new_team = stealing_team as i8;
                log::info!("Point {} was captured by team {} in game {}", i, new_team, generic.game_guid());
                cap_point.capture.store(0.0, std::sync::atomic::Ordering::SeqCst);
                cap_point.team.store(new_team, std::sync::atomic::Ordering::SeqCst);
                if owned_points.get(&(new_team as u8)).copied().unwrap_or(0) == 0 {
                    captured_firsts.insert(new_team as u8);
                }
                if point_owner >= 0 && *owned_points.get(&(point_owner as u8)).unwrap() == 1 {
                    lost_lasts.insert(point_owner as u8);
                }
                generic.broadcast(
                    rlnl::event_code::NetworkEvent::CapturePointNotification,
                    literustlib::packet::Property::ReliableOrdered,
                    &rlnl::events::ingame::CapturePointNotification {
                        notification: rlnl::types::CapturePointNotificationType::CaptureCompleted,
                        id: i as u8,
                        defending_team: point_owner,
                        attacking_team: new_team,
                    },
                    true,
                ).await;
            }
            let progress_now = cap_point.capture.load(std::sync::atomic::Ordering::SeqCst);
            let data = rlnl::events::ingame::TeamBaseState {
                base_team_or_mining_point_index: i as u8,
                current_progress: rlnl::types::ByteFloat::from(progress_now),
                max_progress: rlnl::types::ByteFloat::from(max_progress),
            };
            generic.broadcast(
                rlnl::event_code::NetworkEvent::CapturePointProgress,
                literustlib::packet::Property::ReliableOrdered,
                &data,
                true
            ).await;
        }
        Some(PointTickInfo {
            owned: owned_points,
            captured_firsts,
            lost_lasts,
            delta,
        })
    }
}

struct BaseTracker {
    bases: std::collections::HashMap<u8, BaseInfo>,
}

impl BaseTracker {
    fn new<'a>(bases_iter: impl std::iter::Iterator<Item=&'a u8>, crystals: &[oj_rc_core::cubes::CubeLocationInfo]) -> Self {
        let mut bases = std::collections::HashMap::new();
        for base_id in bases_iter {
            bases.insert(*base_id, BaseInfo::new(crystals));
        }
        Self {
            bases,
        }
    }
}

struct BaseInfo {
    cube_index: atomic_float::AtomicF32,
    crystals_healths: Vec<std::sync::atomic::AtomicU8>,
}

impl BaseInfo {
    fn new(crystals: &[oj_rc_core::cubes::CubeLocationInfo]) -> Self {
        Self {
            cube_index: atomic_float::AtomicF32::new(0.0),
            crystals_healths: (0..crystals.len())
                .map(|_| std::sync::atomic::AtomicU8::new(0))
                .collect()
        }
    }

    #[inline]
    fn calculate_crystal_health(&self, i: usize, max_health: u32) -> u32 {
        (((self.crystals_healths[i].load(std::sync::atomic::Ordering::Relaxed) as f32) / (u8::MAX as f32))
            * (max_health as f32)).ceil() as u32
    }

    #[inline]
    fn max_index(&self) -> usize {
        self.cube_index.load(std::sync::atomic::Ordering::SeqCst).ceil() as usize
    }

    fn first_damaged(&self, old_index: usize, max_health: u32) -> Option<usize> {
        for i in 0..old_index {
            let health = self.calculate_crystal_health(i, max_health);
            if health != 0 && health != max_health {
                return Some(i);
            }
        }
        None
    }

    /// returns whether crystal exists
    fn damage_crystal_at_pos(&self, pos: rlnl::types::Byte3, damage: i32, crystals: &[oj_rc_core::cubes::CubeLocationInfo], max_health: u32) -> bool {
        if let Some((index, _)) = crystals.iter().enumerate().find(|(_i, crystal)| crystal.x == pos.x && crystal.y == pos.y && crystal.z == pos.z) {
            let max_index = self.max_index();
            if index > max_index { return false; }
            self.damage_crystal(index, damage, max_health);
            true
        } else {
            false
        }
    }

    /// returns whether crystal was destroyed
    fn damage_crystal(&self, i: usize, damage: i32, max_health: u32) -> bool {
        let damage_byte = (((damage as f32) / (max_health as f32)) * (u8::MAX as f32)).ceil() as u8;
        let old_health = self.crystals_healths[i].fetch_sub(damage_byte, std::sync::atomic::Ordering::Relaxed);
        log::debug!("Crystal {} damaged (was {}, now {}, delta {})", i, old_health, old_health.saturating_sub(damage_byte), damage_byte);
        if old_health < damage_byte {
            // guarantee underflow behaviour
            self.crystals_healths[i].store(0, std::sync::atomic::Ordering::Relaxed);
        }
        old_health != 0 && old_health >= damage_byte
    }

    /// returns whether crystal exists
    fn destroy_crystal_at_pos(&self, pos: rlnl::types::Byte3, crystals: &[oj_rc_core::cubes::CubeLocationInfo]) -> bool {
        if let Some((index, _)) = crystals.iter().enumerate().find(|(_i, crystal)| crystal.x == pos.x && crystal.y == pos.y && crystal.z == pos.z) {
            let max_index = self.max_index();
            if index > max_index { return false; }
            self.destroy_crystal(index);
            true
        } else {
            false
        }
    }

    fn destroy_crystal(&self, i: usize) {
        self.crystals_healths[i].store(0, std::sync::atomic::Ordering::Relaxed);
    }

    fn generate_full_base_heal(&self, base_id: u8, max_health: u32, crystals: &[oj_rc_core::cubes::CubeLocationInfo]) -> rlnl::events::HealedCubes {
        let max_index = self.max_index();
        let target_crystals: Vec<rlnl::types::HitCubeInfo> = (0..crystals.len())
            .take_while(|&i| i <= max_index)
            .filter(|&i| self.calculate_crystal_health(i, max_health) != 0)
            .map(|i| {
                let target_loc = &crystals[i];
                rlnl::types::HitCubeInfo {
                    pos: rlnl::types::Byte3 { x: target_loc.x, y: target_loc.y, z: target_loc.z, },
                    damage: max_health as i32,
                }
            })
            .collect();
        rlnl::events::HealedCubes {
            healed_machine: base_id as u16,
            type_performing_healing: rlnl::types::TargetType::TeamBase,
            target_type: rlnl::types::TargetType::TeamBase,
            num_healed_cubes: target_crystals.len() as _,
            hit_cubes: target_crystals,
        }
    }
}

enum WinMode {
    BaseFull,
    OutOfTime,
    OutOfPlayers,
}

pub struct BattleArenaLogic {
    game_duration: std::time::Duration,
    game_end: std::sync::atomic::AtomicI64,
    respawn_full_heal_duration: f32,
    respawn_heal_duration: f32,
    timer_task: tokio::sync::Mutex<Option<tokio::task::JoinHandle<()>>>,
    player_tracking: PlayerTracker,
    capture_tracking: PointTracker,
    base_tracking: BaseTracker,
    //cube_parser: std::sync::Arc<oj_rc_core::cubes::CubeLocationsParser>,
    crystals: Vec<oj_rc_core::cubes::CubeLocationInfo>,
    config: oj_rc_core::data::battle_arena_config::BattleArenaData,
}

impl BattleArenaLogic {
    const CRYSTAL_ID: u32 = 3950293873;
    const CLASP_ID: u32 = 606866102;

    pub fn new(config: &oj_rc_core::data::game_mode::GameModeConfig, map: &oj_rc_core::persist::config::MapConfig, parsers: &oj_rc_core::cubes::CubeParsers, ba_config: oj_rc_core::data::battle_arena_config::BattleArenaData) -> Self {
        let dur = std::time::Duration::from_secs((config.game_time_minutes as u64) * 60);
        let fake_end = (chrono::Utc::now() + dur).timestamp();
        let cube_parser = parsers.locations_of();
        let crystals = cube_parser.locations_of_by_distance_to_first(&mut std::io::Cursor::new(&ba_config.base_machine_map), Self::CRYSTAL_ID, Self::CLASP_ID);
        Self {
            game_duration: dur,
            respawn_full_heal_duration: config.respawn_full_heal_duration,
            respawn_heal_duration: config.respawn_heal_duration,
            game_end: std::sync::atomic::AtomicI64::new(fake_end),
            timer_task: tokio::sync::Mutex::new(None),
            player_tracking: PlayerTracker::new(),
            capture_tracking: PointTracker::new(map.capture_points.iter().map(|(_, speed)| *speed)),
            base_tracking: BaseTracker::new(map.bases.keys(), &crystals),
            //cube_parser,
            //ba_base: teambase,
            //ba_equalizer: equalizer,
            crystals,
            config: ba_config,
        }
    }

    async fn abort_timer_sync(&self) {
        let mut lock = self.timer_task.lock().await;
        if let Some(timer_t) = &*lock {
            timer_t.abort();
            log::debug!("Aborted battle arena match timer task");
        }
        *lock = None;
    }

    fn sphere_to_capture_point(sphere: &oj_rc_core::persist::config::Sphere, max_progress: f32) -> rlnl::types::CapturePoint {
        rlnl::types::CapturePoint {
            pos: rlnl::types::PosQuatPair {
                pos: (sphere.center.x, sphere.center.y, sphere.center.z).into(),
                rot: (0.0, 0.0, 0.0, 0.0).into(),
            },
            team: -1,
            progress: 0.0.into(),
            max_progress: max_progress.into(),
        }
    }

    fn default_capture_point(max_progress: f32) -> rlnl::types::CapturePoint {
        rlnl::types::CapturePoint {
            pos: rlnl::types::PosQuatPair {
                pos: (0.0, 0.0, 0.0).into(),
                rot: (0.0, 0.0, 0.0, 0.0).into(),
            },
            team: -1,
            progress: 0.0.into(),
            max_progress: max_progress.into(),
        }
    }

    async fn check_if_match_time_is_done(&self, generic: &crate::matches::GenericGamemodeEngine<Self>) -> bool {
        if self.game_end.load(std::sync::atomic::Ordering::Relaxed) <= chrono::Utc::now().timestamp() {
            // find winning team
            let mut winning_team = None;
            for (base, tracking) in self.base_tracking.bases.iter() {
                if let Some((_, winning_score)) = winning_team {
                    let score = tracking.cube_index.load(std::sync::atomic::Ordering::SeqCst);
                    if score > winning_score {
                        winning_team = Some((*base, score));
                    }
                } else {
                    winning_team = Some((*base, tracking.cube_index.load(std::sync::atomic::Ordering::SeqCst)));
                }
            }
            let winners = if let Some((winning_team, _)) = winning_team {
                winning_team
            } else {
                u8::MAX
            };
            // game is done, hooray
            self.do_win(winners, WinMode::OutOfTime, generic).await;
            true
        } else {
            false
        }
    }

    async fn do_win(&self, winning_team: u8, ty: WinMode, generic: &crate::matches::GenericGamemodeEngine<Self>) {
        generic.game_done();
        let end_reason = match ty {
            WinMode::BaseFull
            | WinMode::OutOfPlayers => rlnl::types::GameEndReason::BaseDestroyed,
            WinMode::OutOfTime => rlnl::types::GameEndReason::TimeOut,
        };
        let payload = rlnl::events::ingame::GameLoseWin {
            winning_team,
            end_reason,
        };
        for player in generic.users.read().await.values() {
            let is_winner = player.descriptor.team == winning_team as i32;
            let net_event = match ty {
                WinMode::BaseFull
                | WinMode::OutOfPlayers => {
                    if is_winner { rlnl::event_code::NetworkEvent::GameWonBaseDestroyed } else { rlnl::event_code::NetworkEvent::GameLostBaseDestroyed }
                },
                WinMode::OutOfTime => {
                    if is_winner { rlnl::event_code::NetworkEvent::GameWon } else { rlnl::event_code::NetworkEvent::GameLost }
                }
            };
            crate::events::log_lnl_send_failure(
                player.connection.rlnl()
                    .send_data(
                        &payload,
                        net_event,
                        literustlib::packet::Property::ReliableOrdered,
                        &player.connection.connection,
                    ).await
            );
        }
    }

    async fn do_destruct_tasks(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, player_id: u8) {
        if let Some(player_team) = self.player_tracking.team(player_id).await {
            let was_in_point = self.player_tracking.swap_is_in_point(player_id, None).await;
            if let Some(was_in_point) = was_in_point {
                self.capture_tracking.on_exit(generic, was_in_point, player_id, player_team as i8, self.config.num_segments as f32).await;
            }
        }
    }

    async fn do_respawn_tasks(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, player_id: u8) {
        log::info!("Handling respawn player {} in game {}", player_id, generic.game_guid());
        let respawn_time = std::time::Duration::from_secs(self.config.respawn_time_seconds as u64);
        let now = chrono::Utc::now();
        let respawn_timestamp = now + respawn_time;
        if let Some(player_respawn) = self.player_tracking.respawning.read().await.get(&player_id) {
            player_respawn.store(respawn_timestamp.timestamp(), std::sync::atomic::Ordering::Relaxed);
        }
        let respawn_payload = rlnl::events::ingame::RespawnTime {
            owner: player_id,
            waiting_time: self.config.respawn_time_seconds as i16,
        };
        generic.broadcast(
            rlnl::event_code::NetworkEvent::SetRespawnWaitingTime,
            literustlib::packet::Property::ReliableOrdered,
            &respawn_payload,
            true
        ).await;
        if let Some(player_team) = self.player_tracking.team(player_id).await {
            let spawn_point = if let Some(team_spawns) = generic.map_config.spawns.get(&player_team) {
                if team_spawns.is_empty() {
                    oj_rc_core::persist::config::Point {
                        x: 10.0 * (player_id as f32),
                        y: 100.0,
                        z: 10.0 * (player_team as f32) + 10.0,
                    }
                } else {
                    let index = (player_id as usize) % team_spawns.len();
                    team_spawns[index].clone()
                }
            } else {
                oj_rc_core::persist::config::Point {
                    x: 10.0 * (player_id as f32),
                    y: 100.0,
                    z: 10.0 * (player_team as f32) + 10.0,
                }
            };
        let connections = generic.users.read().await.values().map(|player_info| player_info.connection.clone()).collect();
        tokio::task::spawn(Self::respawn_player_after(respawn_timestamp, connections, spawn_point, player_id));
        } else {
            log::error!("Player {} cannot respawn because they are not in a team!?", player_id);
        }
    }

    async fn respawn_player_after(after: chrono::DateTime<chrono::Utc>, players: Vec<crate::matches::generic::UserSender>, spawn: oj_rc_core::persist::config::Point, player_id: u8) {
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
    }

    async fn do_team_base_stealing(&self, cube_damage: &rlnl::events::ingame::DestroyCubeNoEffect, generic: &crate::matches::GenericGamemodeEngine<Self>) {
        let base_id = cube_damage.hit_machine_id as u8;
        let mut total_destroyed = 0;
        let mut valid_cubes = Vec::with_capacity(cube_damage.hit_cubes.len());
        if let Some(base) = self.base_tracking.bases.get(&base_id) {
            for hit_cube in cube_damage.hit_cubes.iter() {
                if let Some(damage) = hit_cube.status.damage {
                    if base.damage_crystal_at_pos(hit_cube.loc, damage, &self.crystals, self.config.protonium_health as u32) {
                        valid_cubes.push(hit_cube.to_owned());
                    }
                } else if matches!(hit_cube.status.ty, rlnl::types::CubeHistoryEventType::Destroy) {
                    if base.destroy_crystal_at_pos(hit_cube.loc, &self.crystals) {
                        valid_cubes.push(hit_cube.to_owned());
                        total_destroyed += 1;
                    } else {
                        log::warn!("Did not destroy cube with destroy status");
                    }
                }
            }
            generic.broadcast(
                rlnl::event_code::NetworkEvent::DestroyCubeNoEffect,
                literustlib::packet::Property::ReliableOrdered,
                &rlnl::events::ingame::DestroyCubeNoEffect {
                    shooting_machine_id: cube_damage.shooting_machine_id,
                    hit_machine_id: cube_damage.hit_machine_id,
                    target_type: rlnl::types::TargetType::TeamBase,
                    num_hits: valid_cubes.len() as _,
                    hit_cubes: valid_cubes,
                },
                true,
            ).await;
            generic.broadcast(
                rlnl::event_code::NetworkEvent::SyncTeamBaseCubes,
                literustlib::packet::Property::ReliableOrdered,
                &base.generate_full_base_heal(base_id, self.config.protonium_health as u32, &self.crystals),
                true,
            ).await;
        }
        if total_destroyed != 0 {
            if let Some(team_id) = self.player_tracking.team(cube_damage.shooting_machine_id as u8).await {
                if let Some(base) = self.base_tracking.bases.get(&team_id) {
                    //log::info!("Player {} stole {} crystals", cube_damage.shooting_machine_id, total_destroyed);
                    base.cube_index.fetch_add(total_destroyed as f32, std::sync::atomic::Ordering::SeqCst);
                    generic.broadcast(
                        rlnl::event_code::NetworkEvent::SyncTeamBaseCubes,
                        literustlib::packet::Property::ReliableOrdered,
                        &base.generate_full_base_heal(team_id, self.config.protonium_health as u32, &self.crystals),
                        true,
                    ).await;
                }
            } else {
                log::warn!("Machine {} has no team", cube_damage.shooting_machine_id);
            }
        }
    }
}

#[async_trait::async_trait]
impl CustomGameLogic for BattleArenaLogic {
    async fn on_player_join(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>, player: &crate::matches::generic::UserConnection, _others: &[oj_rc_core::persist::user::PlayerDescriptor]) -> bool {
        log::info!("Player {} joined", player.descriptor.player_id);
        self.player_tracking.track_player(&player.descriptor).await;
        true
    }

    async fn on_player_end(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, player: &crate::matches::generic::UserConnection) -> bool {
        let player_id = player.descriptor.player_id;
        self.do_destruct_tasks(generic, player_id).await;
        self.player_tracking.disconnect_player(player_id).await;
        if let Some(winning_team) = self.player_tracking.single_remaining_team().await {
            self.do_win(winning_team, WinMode::OutOfPlayers, generic).await;
        }
        true
    }

    async fn on_vehicle_destroyed(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, _killer: u8, victim: u8) -> bool {
        self.do_destruct_tasks(generic, victim).await;
        self.do_respawn_tasks(generic, victim).await;
        true
    }

    async fn on_vehicle_self_destruct(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, user: u8, _is_classic: bool) -> bool {
        self.do_destruct_tasks(generic, user).await;
        self.do_respawn_tasks(generic, user).await;
        true
    }

    async fn extra_sync_events(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, _player: &crate::matches::generic::UserConnection) -> Vec<crate::matches::RlnlPacket> {
        vec![
            Some(crate::matches::RlnlPacket {
                event: rlnl::event_code::NetworkEvent::GameModeSettings,
                property: literustlib::packet::Property::ReliableOrdered,
                data: Box::new(rlnl::events::sync::UpdateGameModeSettings {
                    respawn_heal_duration: self.respawn_heal_duration,
                    respawn_full_heal_duration: self.respawn_full_heal_duration,
                }),
            }),
            // TeamBase
            if generic.map_config.bases.is_empty() {
                None
            } else {
                Some(crate::matches::RlnlPacket {
                    event: rlnl::event_code::NetworkEvent::TeamBase,
                    property: literustlib::packet::Property::ReliableOrdered,
                    data: Box::new(rlnl::events::sync::GetTeamBase {
                        base_1: rlnl::types::PosQuatPair {
                            pos: generic.map_config.bases.get(&0).map(|(s, _)| (s.center.x, s.center.y, s.center.z)).unwrap_or((0.0, 0.0, 0.0)).into(),
                            rot: (0.0, 0.0, 0.0, 0.0).into(),
                        },
                        base_2: rlnl::types::PosQuatPair {
                            pos: generic.map_config.bases.get(&1).map(|(s, _)| (s.center.x, s.center.y, s.center.z)).unwrap_or((0.0, 0.0, 0.0)).into(),
                            rot: (0.0, 0.0, 0.0, 0.0).into(),
                        },
                        protonium_cube_health: self.config.protonium_health as i32,
                    }),
                })
            },
            // RegisterCapturePoints
            if generic.map_config.capture_points.is_empty() {
                None
            } else {
                Some(crate::matches::RlnlPacket {
                    event: rlnl::event_code::NetworkEvent::RegisterCapturePoints,
                    property: literustlib::packet::Property::ReliableOrdered,
                    data: Box::new(rlnl::events::sync::GetCapturePoints {
                        points: [
                            generic.map_config.capture_points.first().map(|(s, _)| Self::sphere_to_capture_point(s, self.config.num_segments as f32)).unwrap_or_else(|| Self::default_capture_point(self.config.num_segments as f32)),
                            generic.map_config.capture_points.get(1).map(|(s, _)| Self::sphere_to_capture_point(s, self.config.num_segments as f32)).unwrap_or_else(|| Self::default_capture_point(self.config.num_segments as f32)),
                            generic.map_config.capture_points.get(2).map(|(s, _)| Self::sphere_to_capture_point(s, self.config.num_segments as f32)).unwrap_or_else(|| Self::default_capture_point(self.config.num_segments as f32)),
                        ]
                    }),
                })
            },
            // RegisterEqualizer
            Some(crate::matches::RlnlPacket {
                event: rlnl::event_code::NetworkEvent::RegisterEqualizer,
                property: literustlib::packet::Property::ReliableOrdered,
                data: Box::new(rlnl::events::sync::GetEqualizer {
                    pos: rlnl::types::PosQuatPair {
                        pos: (0.0, 0.0, 0.0).into(),
                        rot: (0.0, 0.0, 0.0, 0.0).into(),
                    },
                    total_health: 42,
                }),
            }),
            // SetShieldState
            /*if generic.map_config.bases.contains_key(&0) {
                Some(crate::matches::RlnlPacket {
                    event: rlnl::event_code::NetworkEvent::SetShieldState,
                    property: literustlib::packet::Property::ReliableOrdered,
                    data: Box::new(rlnl::events::sync::FusionShieldState {
                        team_id: 0,
                        full_power: 0,
                    }),
                })
            } else {
                None
            },
            if generic.map_config.bases.contains_key(&1) {
                Some(crate::matches::RlnlPacket {
                    event: rlnl::event_code::NetworkEvent::SetShieldState,
                    property: literustlib::packet::Property::ReliableOrdered,
                    data: Box::new(rlnl::events::sync::FusionShieldState {
                        team_id: 1,
                        full_power: 0,
                    }),
                })
            } else {
                None
            },*/
            // CurrentGameTime
            Some(crate::matches::RlnlPacket {
                event: rlnl::event_code::NetworkEvent::CurrentGameTime,
                property: literustlib::packet::Property::ReliableOrdered,
                data: Box::new(rlnl::events::GameTime(self.game_duration.as_millis() as f32 / 1000.0)),
            }),
            // SyncTeamBaseCubes
            // TODO ???
            /*crate::matches::RlnlPacket {
                event: rlnl::event_code::NetworkEvent::SyncTeamBaseCubes,
                property: literustlib::packet::Property::ReliableOrdered,
                data: Box::new(rlnl::events::HealedCubes {
                    healed_machine: 0,
                    type_performing_healing: rlnl::types::TargetType::TeamBase,
                    target_type: rlnl::types::TargetType::TeamBase,
                    num_healed_cubes: 1,
                    hit_cubes: vec![
                        rlnl::types::HitCubeInfo {
                            pos: rlnl::types::Byte3 { x: 0, y: 0, z: 0, },
                            damage: 1,
                        }
                    ],
                }),
            },*/
            /*Some(
                crate::matches::RlnlPacket {
                    event: rlnl::event_code::NetworkEvent::SyncTeamBaseCubes,
                    property: literustlib::packet::Property::ReliableOrdered,
                    data: Box::new(rlnl::events::HealedCubes {
                        healed_machine: 0,
                        type_performing_healing: rlnl::types::TargetType::TeamBase,
                        target_type: rlnl::types::TargetType::TeamBase,
                        num_healed_cubes: oj_rc_core::cubes::prefabs::CRYSTAL_COUNT as _,
                        hit_cubes: oj_rc_core::cubes::prefabs::team_base_ba_crystals(oj_rc_core::cubes::prefabs::CRYSTAL_COUNT)
                            .into_iter()
                            //.chain(vec![oj_rc_core::cubes::prefabs::team_base_ba_location()].into_iter())
                            .map(|loc| {
                                //log::info!("Doing sync-time base heal for cube at ({}, {}, {})", loc.0, loc.1, loc.2);
                                rlnl::types::HitCubeInfo {
                                    pos: rlnl::types::Byte3 { x: loc.0, y: loc.1, z: loc.2, },
                                    damage: Self::CRYSTAL_HEALTH,
                                }
                            })
                            .collect(),
                    }),
                }
            ),*/
            // SyncEqualizerNotification
            /*crate::matches::RlnlPacket {
                event: rlnl::event_code::NetworkEvent::SyncEqualizerNotification,
                property: literustlib::packet::Property::ReliableOrdered,
                data: Box::new(rlnl::events::sync::EqualizerNotification {
                    notification: rlnl::types::EqualizerState::Lost,
                    team_id: 0,
                    time: 0,
                    max_health: 42,
                    health: 7,
                }),
            },
            crate::matches::RlnlPacket {
                event: rlnl::event_code::NetworkEvent::SyncEqualizerNotification,
                property: literustlib::packet::Property::ReliableOrdered,
                data: Box::new(rlnl::events::sync::EqualizerNotification {
                    notification: rlnl::types::EqualizerState::Lost,
                    team_id: 1,
                    time: 0,
                    max_health: 42,
                    health: 7,
                }),
            },*/
        ].into_iter().flatten().collect()
    }

    async fn on_countdown_start(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, game_start: chrono::DateTime<chrono::Utc>) -> bool {
        let read_lock = generic.users.read().await;
        let mut senders = Vec::with_capacity(read_lock.len());
        for conn in read_lock.values() {
            senders.push((conn.connection.clone(), conn.state.clone()));
        }
        drop(read_lock);
        let game_end = game_start + self.game_duration;
        let extra_packets = Vec::default();
        let new_timer_task = crate::matches::timer::match_time_syncer(senders, game_start, game_end, extra_packets, Vec::default());
        let mut timer_lock = self.timer_task.lock().await;
        if let Some(timer_t) = &*timer_lock { // this is quite unlikely (i.e. impossible), but I've done it for completeness
            log::warn!("Aborting an existing timer task for battle arena mode suggests an assumption was wrong");
            timer_t.abort();
        }
        *timer_lock = Some(new_timer_task);
        self.game_end.store(game_end.timestamp(), std::sync::atomic::Ordering::Relaxed);
        true
    }

    async fn on_game_completed(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>) -> bool {
        self.abort_timer_sync().await;
        true
    }

    async fn on_broadcast(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, _user_id: i32, _event_out: rlnl::event_code::NetworkEvent, event_in: rlnl::event_code::NetworkEvent, _property: literustlib::packet::Property, data: &Option<Box<dyn crate::Broadcastable>>, _skip_user: bool) -> bool {
        match (event_in, data) {
            (rlnl::event_code::NetworkEvent::DamageCubeNoEffect, Some(data)) => {
                let maybe_cube_dmg = <dyn core::any::Any>::downcast_ref::<rlnl::events::ingame::DestroyCubeNoEffect>(data.as_ref());
                if let Some(cube_damage) = maybe_cube_dmg {
                    if matches!(cube_damage.target_type, rlnl::types::TargetType::TeamBase) {
                        self.do_team_base_stealing(cube_damage, generic).await;
                        false
                    } else {
                        //log::info!("Got non-base DamageCubeNoEffect {:?}", cube_damage.target_type);
                        true
                    }
                } else {
                    log::warn!("Got DamageCubeNoEffect event with bad serialization type");
                    true
                }
            },
            /*(rlnl::event_code::NetworkEvent::DamageCubeEffectOnly, Some(data)) => {
                let maybe_cube_dmg = <dyn core::any::Any>::downcast_ref::<rlnl::events::ingame::DestroyCubeEffectOnly>(data.as_ref());
                if let Some(cube_damage) = maybe_cube_dmg {
                    if matches!(cube_damage.target_type, rlnl::types::TargetType::TeamBase) {
                        log::info!("Got team base DamageCubeEffectOnly for pos ({}, {}, {})", cube_damage.hit_cube.x, cube_damage.hit_cube.y, cube_damage.hit_cube.z);
                        false
                    } else {
                        log::info!("Got non-base DamageCubeEffectOnly {:?}", cube_damage.target_type);
                        true
                    }
                } else {
                    log::warn!("Got DamageCubeEffectOnly event with bad serialization type");
                    true
                }
            },*/
            _ => true
        }
    }

    async fn on_motion(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, motion: &rlnl::machine_motion::MachineMotion, location: (f32, f32, f32)) -> bool {
        let game_start = generic.game_start.load(std::sync::atomic::Ordering::Relaxed);
        if generic.game_start.load(std::sync::atomic::Ordering::Relaxed) == -1 || chrono::Utc::now().timestamp() < game_start {
            // game is not in progress, ignore motion event
            log::debug!("Ignoring early motion event from player {}", motion.player_id);
            return true;
        }
        if generic.is_game_done() {
            self.abort_timer_sync().await;
            return true;
        }
        if self.check_if_match_time_is_done(generic).await {
            return true;
        }
        if generic.map_config.capture_points.is_empty() {
            return true; // don't bother trying to track whether players are in capture points since there are none
        }
        if let Some(player_team) = self.player_tracking.team(motion.player_id).await {
            let mut now_in_point = None;
            for (point_i, point) in generic.map_config.capture_points.iter().enumerate() {
                if crate::matches::GenericGamemodeEngine::<Self>::is_in(&location, &point.0) {
                    now_in_point = Some(point_i as u8);
                    break;
                }
            }
            let was_in_point = self.player_tracking.swap_is_in_point(motion.player_id, now_in_point).await;
            if was_in_point != now_in_point {
                //log::info!("Player {}'s occupied capture point changed from {:?} to {:?}", motion.player_id, was_in_point, now_in_point);
                if let Some(now_in_point) = now_in_point {
                    self.capture_tracking.on_enter(generic, now_in_point, motion.player_id, player_team as i8).await;
                }
                if let Some(was_in_point) = was_in_point {
                    self.capture_tracking.on_exit(generic, was_in_point, motion.player_id, player_team as i8, self.config.num_segments as f32).await;
                }
            }
        }
        if let Some(tick_info) = self.capture_tracking.tick(generic, self.config.num_segments as f32).await {
            // handle shield (de)activation
            for team in tick_info.captured_firsts {
                generic.broadcast(
                    rlnl::event_code::NetworkEvent::SetShieldState,
                    literustlib::packet::Property::ReliableOrdered,
                    &rlnl::events::sync::FusionShieldState {
                        team_id: team as i8,
                        full_power: 1,
                    },
                    true,
                ).await;
            }
            for team in tick_info.lost_lasts {
                generic.broadcast(
                    rlnl::event_code::NetworkEvent::SetShieldState,
                    literustlib::packet::Property::ReliableOrdered,
                    &rlnl::events::sync::FusionShieldState {
                        team_id: team as i8,
                        full_power: 0,
                    },
                    true,
                ).await;
            }

            // do base charge tick
            for base_id in generic.map_config.bases.keys() {
                //log::info!("Healing base {}", base_id);
                if let Some(owned_points) = tick_info.owned.get(base_id) {
                    if let Some(tracked_base) = self.base_tracking.bases.get(base_id) {
                        let one_tick = (self.crystals.len() as f32)
                        * ((PointTracker::TICK_MS as f32) / (self.game_duration.as_millis() as f32))
                        * ((self.base_tracking.bases.len() as f32) / (self.capture_tracking.points.len() as f32));
                        let increment = tick_info.delta as f32 * (*owned_points as f32) * one_tick;
                        let old_float_index = tracked_base.cube_index.fetch_add(increment, std::sync::atomic::Ordering::SeqCst);
                        let new_float_index = old_float_index + increment;
                        let old_index = (old_float_index.ceil() as usize).clamp(0, self.crystals.len());
                        let new_index = (new_float_index.ceil() as usize).clamp(0, self.crystals.len());
                        if new_index != old_index {
                            log::debug!("Base {} increment passed a crystal index barrier", base_id);
                            let first_damaged = tracked_base.first_damaged(old_index, self.config.protonium_health as u32);
                            let payload = if new_index - old_index == 1 && first_damaged.is_some() {
                                // undo cube_index update
                                tracked_base.cube_index.fetch_sub(increment, std::sync::atomic::Ordering::SeqCst);
                                log::debug!("Skipping increment in favour of healing damaged/destroyed cube");
                                let first_damaged = first_damaged.unwrap();
                                let healing = self.config.protonium_health as u32 - tracked_base.calculate_crystal_health(first_damaged, self.config.protonium_health as u32);
                                tracked_base.crystals_healths[first_damaged].store(u8::MAX, std::sync::atomic::Ordering::Relaxed);
                                let target_crystal = &self.crystals[first_damaged];
                                rlnl::events::HealedCubes {
                                    healed_machine: *base_id as u16,
                                    type_performing_healing: rlnl::types::TargetType::TeamBase,
                                    target_type: rlnl::types::TargetType::TeamBase,
                                    num_healed_cubes: 1,
                                    hit_cubes: vec![
                                        rlnl::types::HitCubeInfo {
                                            pos: rlnl::types::Byte3 { x: target_crystal.x, y: target_crystal.y, z: target_crystal.z, },
                                            damage: healing as i32,
                                        }
                                    ],
                                }
                            } else {
                                let target_crystals = &self.crystals[old_index..new_index];
                                for crystal_i in old_index..new_index {
                                    tracked_base.crystals_healths[crystal_i].store(u8::MAX, std::sync::atomic::Ordering::Relaxed);
                                }
                                rlnl::events::HealedCubes {
                                    healed_machine: *base_id as u16,
                                    type_performing_healing: rlnl::types::TargetType::TeamBase,
                                    target_type: rlnl::types::TargetType::TeamBase,
                                    num_healed_cubes: target_crystals.len() as _,
                                    hit_cubes: target_crystals
                                        .iter()
                                        .map(|loc| rlnl::types::HitCubeInfo {
                                            pos: rlnl::types::Byte3 { x: loc.x, y: loc.y, z: loc.z, },
                                            damage: self.config.protonium_health as i32,
                                        })
                                        .collect(),
                                }
                            };

                            generic.broadcast(
                                rlnl::event_code::NetworkEvent::SyncTeamBaseCubes,
                                literustlib::packet::Property::ReliableOrdered,
                                &payload,
                                true
                            ).await;

                            if new_index == self.crystals.len() {
                                // team base is charged to 100%
                                self.do_win(*base_id, WinMode::BaseFull, generic).await;
                            }
                        }
                    }
                }
            }
        }
        true
    }

    async fn on_custom(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, _user_id: i32, event: rlnl::event_code::NetworkEvent, property: literustlib::packet::Property, data: Box<dyn crate::Broadcastable>) {
        match (event, property) {
            (rlnl::event_code::NetworkEvent::SendDamagedByEnemyShield, literustlib::packet::Property::ReliableOrdered) => {
                generic.broadcast(
                    rlnl::event_code::NetworkEvent::DamagedByEnemyShield,
                    literustlib::packet::Property::ReliableOrdered,
                    &*data,
                    true,
                ).await;
            },
            (rlnl::event_code::NetworkEvent::SurrenderRequest, literustlib::packet::Property::ReliableOrdered) => {
                // TODO
                log::warn!("Ignoring SurrenderRequest because it's not implemented (yet)");
            }
            (rlnl::event_code::NetworkEvent::AwardTeamBaseProtoniumDestroyedRequest, literustlib::packet::Property::ReliableOrdered) => {
                let maybe_crystal_bonus = <dyn core::any::Any>::downcast_ref::<rlnl::events::ingame::AwardProtoniumDestroyedCubes>(data.as_ref());
                if let Some(crystal_destroyed) = maybe_crystal_bonus {
                    if let Some(generic_player) = generic.users.read().await.get(&crystal_destroyed.player_id) {
                        generic_player.counters.crystals.fetch_add(crystal_destroyed.destroyed_cubes as u32, std::sync::atomic::Ordering::Relaxed);
                        let data = generic_player.counters.get_generic_packet(crystal_destroyed.player_id, rlnl::types::IngameStatId::DestroyedProtoniumCubes, Some(crystal_destroyed.destroyed_cubes as _));
                        generic.broadcast(
                            rlnl::event_code::NetworkEvent::UpdateGameStats,
                            literustlib::packet::Property::ReliableOrdered,
                            &data,
                            true,
                        ).await;
                    }
                }
            }
            _ => {}
        }
    }
}
