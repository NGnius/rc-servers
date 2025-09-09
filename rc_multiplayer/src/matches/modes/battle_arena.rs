use crate::matches::{modes::trackers::SurrenderGameTracker, CustomGameLogic};

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

    async fn players_on_team(&self, team: u8) -> Vec<u8> {
        if let Some(members) = self.connected.lock().await.get(&team) {
            members.iter().map(|x| *x).collect()
        } else {
            Vec::default()
        }
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
    ticker: super::trackers::TickTracker<{Self::TICK_MS}>,
    last_capture_team: std::sync::atomic::AtomicU8,
    last_capture_time: std::sync::atomic::AtomicI64,
}

struct PointTickInfo {
    owned: std::collections::HashMap<u8, u8>, // team -> capture point count
    captured_firsts: std::collections::HashSet<u8>, // team
    lost_lasts: std::collections::HashSet<u8>, // team
    dominating: Option<u8>,
    delta: u16,
}

impl PointTracker {
    const TICK_MS: i64 = 50;
    const TIME_BEFORE_DOMINANT_S: i64 = 30;

    fn new(points: impl Iterator<Item=f32>) -> Self {
        Self {
            points: points.map(PointInfo::new).collect(),
            ticker: super::trackers::TickTracker::new(),
            last_capture_team: std::sync::atomic::AtomicU8::new(u8::MAX),
            last_capture_time: std::sync::atomic::AtomicI64::new(i64::MIN),
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
        let delta = self.ticker.tick();
        if delta == 0 { return None; }
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
                self.last_capture_team.store(stealing_team, std::sync::atomic::Ordering::Relaxed);
                self.last_capture_time.store(chrono::Utc::now().timestamp(), std::sync::atomic::Ordering::Relaxed);
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
        let last_capture_team = self.last_capture_team.load(std::sync::atomic::Ordering::Relaxed);
        let dominating = if last_capture_team != u8::MAX && self.points.iter().all(|p| p.team.load(std::sync::atomic::Ordering::SeqCst) == (last_capture_team as i8)) {
            let last_capture_time = self.last_capture_time.load(std::sync::atomic::Ordering::Relaxed);
            let now = chrono::Utc::now().timestamp();
            if now > last_capture_time && now - last_capture_time >= Self::TIME_BEFORE_DOMINANT_S {
                Some(last_capture_team)
            } else {
                None
            }
        } else {
            None
        };
        Some(PointTickInfo {
            owned: owned_points,
            captured_firsts,
            lost_lasts,
            dominating,
            delta,
        })
    }
}

struct EqualizerTracker {
    is_disabled: bool,
    start: std::sync::atomic::AtomicI64,
    activated: std::sync::atomic::AtomicBool,
    cancelled: std::sync::atomic::AtomicBool,
    losing_team: std::sync::atomic::AtomicU8,
    winning_team: std::sync::atomic::AtomicU8,
    trigger_index: std::sync::atomic::AtomicU8,
    health: std::sync::atomic::AtomicU64,
}

impl EqualizerTracker {
    fn new(ba_config: &oj_rc_core::data::battle_arena_config::BattleArenaData) -> Self {
        let is_disabled = ba_config.equalizer_health <= 0
                || ba_config.equalizer_model.is_empty()
                //|| ba_config.equalizer_trigger_time_seconds.is_empty()
                || ba_config.equalizer_duration_seconds.is_empty()
                || ba_config.equalizer_duration_seconds.iter().any(|&x| x == 0);
        if is_disabled {
            log::info!("Battle Arena equalizer is disabled by config (model ok? {}, health ok? {}, duration ok? {})",
                       !ba_config.equalizer_model.is_empty(),
                       ba_config.equalizer_health > 0,
                       !ba_config.equalizer_duration_seconds.is_empty() && !ba_config.equalizer_duration_seconds.iter().any(|&x| x == 0)
            );
        }
        Self {
            is_disabled,
            start: std::sync::atomic::AtomicI64::new(i64::MIN),
            activated: std::sync::atomic::AtomicBool::new(false),
            cancelled: std::sync::atomic::AtomicBool::new(false),
            losing_team: std::sync::atomic::AtomicU8::new(u8::MAX),
            winning_team: std::sync::atomic::AtomicU8::new(u8::MAX),
            trigger_index: std::sync::atomic::AtomicU8::new(0),
            health: std::sync::atomic::AtomicU64::new(0),
        }
    }

    async fn tick(&self, generic: &crate::matches::GenericGamemodeEngine<BattleArenaLogic>, bases: &std::collections::HashMap<u8, BaseInfo>, ba_config: &oj_rc_core::data::battle_arena_config::BattleArenaData) {
        if self.is_disabled { return; }
        let trigger_index = self.trigger_index.load(std::sync::atomic::Ordering::Relaxed) as usize;
        let game_start = generic.game_start.load(std::sync::atomic::Ordering::Relaxed);
        if game_start == i64::MIN { return; }
        let mut eq_start = self.start.load(std::sync::atomic::Ordering::Relaxed);
        if eq_start == i64::MIN {
            eq_start = if let Some(trigger_time_s) = ba_config.equalizer_trigger_time_seconds.get(trigger_index) {
                game_start + (*trigger_time_s as i64)
            } else {
                game_start + ((generic.game_duration.as_secs() / 2) as i64)
            };
            self.start.store(eq_start, std::sync::atomic::Ordering::Relaxed);
        }
        let eq_start = eq_start; // no longer mutable
        let now = chrono::Utc::now().timestamp();
        if now >= eq_start {
            let eq_end = if let Some(duration_s) = ba_config.equalizer_duration_seconds.get(trigger_index) {
                eq_start + (*duration_s as i64)
            } else {
                i64::MAX
            };
            let is_activated = self.activated.load(std::sync::atomic::Ordering::Relaxed);
            let is_cancelled = self.cancelled.load(std::sync::atomic::Ordering::Relaxed);
            if is_activated {
                if now > eq_end {
                    // do deactivation
                    log::info!("Equalizer deactivated because the timer ran out");
                    self.activated.store(false, std::sync::atomic::Ordering::Relaxed);
                    self.send_notification(rlnl::types::EqualizerState::Defended, ba_config, eq_start, now, generic).await;
                } else {
                    // check for change in leading team
                    let old_winning_team = self.winning_team.load(std::sync::atomic::Ordering::Relaxed);
                    if let Some((winning_base_id, _)) = Self::winning_base(bases) {
                        if winning_base_id != old_winning_team {
                            // cancel equalizer
                            log::info!("Equalizer cancelled because winning base lost the lead");
                            self.cancelled.store(true, std::sync::atomic::Ordering::Relaxed);
                            self.activated.store(false, std::sync::atomic::Ordering::Relaxed);
                            self.send_notification(rlnl::types::EqualizerState::Lost, ba_config, eq_start, now, generic).await;
                        }
                    }
                }
            } else if now < eq_end && !is_cancelled {
                // do activation
                if let Some(winning_base) = Self::winning_base(bases) {
                    if let Some(losing_base) = Self::losing_base(bases) {
                        self.activated.store(true, std::sync::atomic::Ordering::Relaxed);
                        self.winning_team.store(winning_base.0, std::sync::atomic::Ordering::Relaxed);
                        self.losing_team.store(losing_base.0, std::sync::atomic::Ordering::Relaxed);
                        self.health.store(ba_config.equalizer_health as u64, std::sync::atomic::Ordering::Relaxed);
                        self.send_notification(rlnl::types::EqualizerState::Start, ba_config, eq_start, now, generic).await;
                    } else {
                        log::warn!("No losing team found for game {}", generic.game_guid());
                    }
                } else {
                    log::warn!("No winning team found for game {}", generic.game_guid());
                }
            }
        }
    }

    async fn damage_equalizer(&self, damage: i32, generic: &crate::matches::GenericGamemodeEngine<BattleArenaLogic>, ba_config: &oj_rc_core::data::battle_arena_config::BattleArenaData, bases: &std::collections::HashMap<u8, BaseInfo>, crystals: &[oj_rc_core::cubes::CubeLocationInfo]) {
        if !self.activated.load(std::sync::atomic::Ordering::Relaxed) { return; }
        let damage_u64 = damage as u64;
        let old_health = self.health.fetch_sub(damage as u64, std::sync::atomic::Ordering::Relaxed);
        if old_health < damage_u64 {
            // equalizer is now destroyed
            self.destroy_equalizer(generic, ba_config, bases, crystals).await;
        }
    }

    async fn destroy_equalizer(&self, generic: &crate::matches::GenericGamemodeEngine<BattleArenaLogic>, ba_config: &oj_rc_core::data::battle_arena_config::BattleArenaData, bases: &std::collections::HashMap<u8, BaseInfo>, crystals: &[oj_rc_core::cubes::CubeLocationInfo]) {
        if !self.activated.swap(false, std::sync::atomic::Ordering::Relaxed) { return; }
        self.cancelled.store(true, std::sync::atomic::Ordering::Relaxed);
        let eq_start = self.start.load(std::sync::atomic::Ordering::Relaxed);
        let now = chrono::Utc::now().timestamp();
        self.send_notification(rlnl::types::EqualizerState::Destroyed, ba_config, eq_start, now, generic).await;
        self.health.store(0, std::sync::atomic::Ordering::Relaxed);
        // give health to losing team
        let winning_team = self.winning_team.load(std::sync::atomic::Ordering::Relaxed);
        let losing_team = self.losing_team.load(std::sync::atomic::Ordering::Relaxed);
        if let Some(losing_base) = bases.get(&losing_team) {
            if let Some(winning_base) = bases.get(&winning_team) {
                let old_index = losing_base.max_index();
                losing_base.cube_index.store(winning_base.cube_index.load(std::sync::atomic::Ordering::Relaxed), std::sync::atomic::Ordering::Relaxed);
                let new_index = losing_base.max_index();
                for i in old_index..new_index {
                    losing_base.crystals_healths[i].store(u8::MAX, std::sync::atomic::Ordering::Relaxed);
                }
                let base_heal = losing_base.generate_full_base_heal(losing_team, ba_config.protonium_health as u32, crystals);
                generic.broadcast(
                    rlnl::event_code::NetworkEvent::SyncTeamBaseCubes,
                    literustlib::packet::Property::ReliableOrdered,
                    &base_heal,
                    true
                ).await;
            } else {
                log::warn!("Winning base {} no longer exists for game {}", winning_team, generic.game_guid());
            }
        } else {
            log::warn!("Losing base {} no longer exists for game {}", losing_team, generic.game_guid());
        }
    }

    async fn send_notification(&self, variant: rlnl::types::EqualizerState, ba_config: &oj_rc_core::data::battle_arena_config::BattleArenaData, start: i64, now: i64, generic: &crate::matches::GenericGamemodeEngine<BattleArenaLogic>) {
        if let Some(notif) = self.generate_notification(variant, ba_config, start, now) {
            generic.broadcast(
                rlnl::event_code::NetworkEvent::EqualizerNotification,
                literustlib::packet::Property::ReliableOrdered,
                &notif,
                true,
            ).await;
        }
    }

    fn generate_notification(&self, variant: rlnl::types::EqualizerState, ba_config: &oj_rc_core::data::battle_arena_config::BattleArenaData, start: i64, now: i64) -> Option<rlnl::events::sync::EqualizerNotification> {
        let trigger_index = self.trigger_index.load(std::sync::atomic::Ordering::Relaxed) as usize;
        if let Some(duration_s) = ba_config.equalizer_duration_seconds.get(trigger_index) {
            let end = start + (*duration_s as i64);
            //let losing_team = self.losing_team.load(std::sync::atomic::Ordering::Relaxed);
            let winning_team = self.winning_team.load(std::sync::atomic::Ordering::Relaxed);
            let current_health = self.health.load(std::sync::atomic::Ordering::Relaxed);
            let time_remaining = match variant {
                rlnl::types::EqualizerState::Start => *duration_s as i16,
                rlnl::types::EqualizerState::Lost
                | rlnl::types::EqualizerState::Destroyed => {
                    if now >= start {
                        if now < end {
                            ((*duration_s as i64) - (now - start)) as i16
                        } else {
                            0
                        }
                    } else {
                        *duration_s as i16
                    }
                },
                rlnl::types::EqualizerState::Defended => 0,
            };
            Some(rlnl::events::sync::EqualizerNotification {
                notification: variant,
                //team_id: losing_team as i16,
                team_id: winning_team as i16,
                time: time_remaining,
                max_health: ba_config.equalizer_health as i32,
                health: current_health as i32,
            })
        } else {
            None
        }
    }

    fn winning_base(bases: &std::collections::HashMap<u8, BaseInfo>) -> Option<(u8, f32)> {
        let mut max = None;
        for (id, info) in bases.iter() {
            if let Some((_, charge)) = max {
                let new_charge = info.base_charge();
                if new_charge > charge {
                    max = Some((*id, new_charge));
                }
            } else {
                max = Some((*id, info.base_charge()));
            }
        }
        max
    }

    fn losing_base(bases: &std::collections::HashMap<u8, BaseInfo>) -> Option<(u8, f32)> {
        let mut min = None;
        for (id, info) in bases.iter() {
            if let Some((_, charge)) = min {
                let new_charge = info.base_charge();
                if new_charge < charge {
                    min = Some((*id, new_charge));
                }
            } else {
                min = Some((*id, info.base_charge()));
            }
        }
        min
    }
}

struct BaseTracker {
    bases: std::collections::HashMap<u8, BaseInfo>,
    equalizer: EqualizerTracker,
    dominating: std::sync::atomic::AtomicBool,
}

struct BaseTickInfo {
    win: Option<(u8, WinMode)>,
}

impl BaseTracker {
    const DOMINATING_MULT: f32 = 10.0;

    fn new<'a>(bases_iter: impl std::iter::Iterator<Item=&'a u8>, crystals: &[oj_rc_core::cubes::CubeLocationInfo], ba_config: &oj_rc_core::data::battle_arena_config::BattleArenaData) -> Self {
        let mut bases = std::collections::HashMap::new();
        for base_id in bases_iter {
            bases.insert(*base_id, BaseInfo::new(crystals));
        }
        Self {
            bases,
            equalizer: EqualizerTracker::new(ba_config),
            dominating: std::sync::atomic::AtomicBool::new(false),
        }
    }

    async fn tick(&self, tick_info: &PointTickInfo, crystals: &[oj_rc_core::cubes::CubeLocationInfo], generic: &crate::matches::GenericGamemodeEngine<BattleArenaLogic>, capture_points_count: usize, crystal_health: u32, ba_config: &oj_rc_core::data::battle_arena_config::BattleArenaData) -> BaseTickInfo {
        let multiplier = if let Some(dominant_team) = tick_info.dominating {
            if !self.dominating.swap(true, std::sync::atomic::Ordering::Relaxed) {
                log::info!("Team {} is now dominating game {}", dominant_team, generic.game_guid());
                // FIXME this doesn't seem to trigger the announcement client-side
                generic.broadcast(
                    rlnl::event_code::NetworkEvent::CapturePointNotification,
                    literustlib::packet::Property::ReliableOrdered,
                    &rlnl::events::ingame::CapturePointNotification {
                        notification: rlnl::types::CapturePointNotificationType::Dominating,
                        id: dominant_team,
                        defending_team: dominant_team as i8,
                        attacking_team: (((dominant_team as usize) + 1) % generic.map_config.bases.len()) as i8
                    },
                    true
                ).await;
            }
            Self::DOMINATING_MULT
        } else {
            if self.dominating.swap(false, std::sync::atomic::Ordering::Relaxed) {
                log::info!("No longer dominating game {}", generic.game_guid());
            }
            1.0
        };
        for (base_id, tracked_base) in self.bases.iter() {
                //log::info!("Healing base {}", base_id);
                if let Some(owned_points) = tick_info.owned.get(base_id) {
                    let one_tick = (crystals.len() as f32)
                    * ((PointTracker::TICK_MS as f32) / (generic.game_duration.as_millis() as f32))
                    * ((self.bases.len() as f32) / (capture_points_count as f32));
                    let increment = tick_info.delta as f32 * (*owned_points as f32) * one_tick * multiplier;
                    let old_float_index = tracked_base.cube_index.fetch_add(increment, std::sync::atomic::Ordering::SeqCst);
                    let new_float_index = old_float_index + increment;
                    let old_index = (old_float_index.ceil() as usize).clamp(0, crystals.len());
                    let new_index = (new_float_index.ceil() as usize).clamp(0, crystals.len());
                    if new_index != old_index {
                        log::debug!("Base {} increment passed a crystal index barrier", base_id);
                        let first_damaged = tracked_base.first_damaged(old_index, crystal_health);
                        let payload = if new_index - old_index == 1 && first_damaged.is_some() {
                            // undo cube_index update
                            tracked_base.cube_index.fetch_sub(increment, std::sync::atomic::Ordering::SeqCst);
                            log::debug!("Skipping increment in favour of healing damaged/destroyed cube");
                            let first_damaged = first_damaged.unwrap();
                            let healing = crystal_health - tracked_base.calculate_crystal_health(first_damaged, crystal_health);
                            tracked_base.crystals_healths[first_damaged].store(u8::MAX, std::sync::atomic::Ordering::Relaxed);
                            let target_crystal = &crystals[first_damaged];
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
                            let target_crystals = &crystals[old_index..new_index];
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
                                        damage: crystal_health as i32,
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

                        if new_index == crystals.len() {
                            // team base is charged to 100%
                            //self.do_win(*base_id, WinMode::BaseFull, generic).await;
                            return BaseTickInfo {
                                win: Some((*base_id, WinMode::BaseFull)),
                            };
                        }
                    }
                }
            }
            self.equalizer.tick(generic, &self.bases, ba_config).await;
            BaseTickInfo {
                win: None,
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

    // total base health, out of 1
    fn base_charge(&self) -> f32 {
        let mut total_health: usize = 0;
        for crystal in self.crystals_healths.iter() {
            let health = crystal.load(std::sync::atomic::Ordering::Relaxed);
            total_health += health as usize;
        }
        (total_health as f32) / ((self.crystals_healths.len() * u8::MAX as usize) as f32)
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
    Surrender,
}

pub struct BattleArenaLogic {
    game_end: std::sync::atomic::AtomicI64,
    respawn_full_heal_duration: f32,
    respawn_heal_duration: f32,
    timer_task: tokio::sync::Mutex<Option<tokio::task::JoinHandle<()>>>,
    player_tracking: PlayerTracker,
    capture_tracking: PointTracker,
    base_tracking: BaseTracker,
    surrender_tracking: super::trackers::SurrenderGameTracker,
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
            respawn_full_heal_duration: config.respawn_full_heal_duration,
            respawn_heal_duration: config.respawn_heal_duration,
            game_end: std::sync::atomic::AtomicI64::new(fake_end),
            timer_task: tokio::sync::Mutex::new(None),
            player_tracking: PlayerTracker::new(),
            capture_tracking: PointTracker::new(map.capture_points.iter().map(|(_, speed)| *speed)),
            surrender_tracking: super::trackers::SurrenderGameTracker::new(),
            base_tracking: BaseTracker::new(map.bases.keys(), &crystals, &ba_config),
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
            WinMode::BaseFull => rlnl::types::GameEndReason::BaseDestroyed,
            WinMode::OutOfPlayers => rlnl::types::GameEndReason::OneTeamRemaining,
            WinMode::Surrender => rlnl::types::GameEndReason::Surrendered,
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
                | WinMode::OutOfPlayers
                | WinMode::Surrender => {
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

    async fn do_team_base_stealing(&self, cube_damage: &rlnl::events::ingame::DestroyCubeNoEffect, generic: &crate::matches::GenericGamemodeEngine<Self>, actual_damage_data: impl FnOnce(Vec<rlnl::types::CubeState>) -> crate::matches::RlnlPacket) {
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
            /*generic.broadcast(
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
            ).await;*/
            let actual_damage_data = actual_damage_data(valid_cubes);
            generic.broadcast(
                actual_damage_data.event,
                actual_damage_data.property,
                actual_damage_data.data.as_ref(),
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

    async fn do_equalizer_damage(&self, cube_damage: &rlnl::events::ingame::DestroyCubeNoEffect, generic: &crate::matches::GenericGamemodeEngine<Self>) {
        for hit_cube in cube_damage.hit_cubes.iter() {
            if let Some(damage) = hit_cube.status.damage {
                self.base_tracking.equalizer.damage_equalizer(damage, generic, &self.config, &self.base_tracking.bases, &self.crystals).await;
            } else if matches!(hit_cube.status.ty, rlnl::types::CubeHistoryEventType::Destroy) {
                self.base_tracking.equalizer.destroy_equalizer(generic, &self.config, &self.base_tracking.bases, &self.crystals).await;
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
                        pos: (generic.map_config.equalizer.x, generic.map_config.equalizer.y, generic.map_config.equalizer.z).into(),
                        rot: (0.0, 0.0, 0.0, 0.0).into(),
                    },
                    total_health: self.config.equalizer_health as i32,
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
                data: Box::new(rlnl::events::GameTime(generic.game_duration.as_millis() as f32 / 1000.0)),
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
        let game_end = game_start + generic.game_duration;
        let extra_packets = Vec::default();
        let new_timer_task = crate::matches::timer::match_time_syncer(senders, game_start, game_end, extra_packets, Vec::default());
        let mut timer_lock = self.timer_task.lock().await;
        if let Some(timer_t) = &*timer_lock { // this is quite unlikely (i.e. impossible), but I've done it for completeness
            log::warn!("Aborting an existing timer task for battle arena mode suggests an assumption was wrong");
            timer_t.abort();
        }
        *timer_lock = Some(new_timer_task);
        self.game_end.store(game_end.timestamp(), std::sync::atomic::Ordering::Relaxed);
        generic.broadcast(
            rlnl::event_code::NetworkEvent::SetSurrenderTimes,
            literustlib::packet::Property::ReliableOrdered,
            &SurrenderGameTracker::surrender_times(),
            false,
        ).await;
        true
    }

    async fn on_game_completed(&self, _generic: &crate::matches::GenericGamemodeEngine<Self>) -> bool {
        self.abort_timer_sync().await;
        true
    }

    async fn on_broadcast(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, _user_id: i32, _event_out: rlnl::event_code::NetworkEvent, event_in: rlnl::event_code::NetworkEvent, _property: literustlib::packet::Property, data: &Option<Box<dyn crate::Broadcastable>>, _skip_user: bool) -> bool {
        match (event_in, data) {
            (rlnl::event_code::NetworkEvent::DamageCube, Some(data)) => {
                let maybe_cube_dmg = <dyn core::any::Any>::downcast_ref::<rlnl::events::ingame::DestroyCubesFull>(data.as_ref());
                if let Some(cube_damage) = maybe_cube_dmg {
                    let pseudo = rlnl::events::ingame::DestroyCubeNoEffect {
                        shooting_machine_id: cube_damage.shooting_machine_id,
                        hit_machine_id: cube_damage.hit_machine_id,
                        target_type: cube_damage.target_type,
                        num_hits: cube_damage.num_hit_cubes,
                        hit_cubes: cube_damage.hit_cubes.clone(),
                    };
                    match cube_damage.target_type {
                        rlnl::types::TargetType::TeamBase => {
                            let actual_damage_data = |cubes: Vec<rlnl::types::CubeState>| {
                                let mut cube_dmg = cube_damage.to_owned();
                                cube_dmg.num_hit_cubes = cubes.len() as _;
                                cube_dmg.hit_cubes = cubes;
                                crate::matches::RlnlPacket {
                                    event: rlnl::event_code::NetworkEvent::DestroyCubesFull,
                                    property: literustlib::packet::Property::ReliableOrdered,
                                    data: Box::new(cube_dmg),
                                }
                            };
                            self.do_team_base_stealing(&pseudo, generic, actual_damage_data).await;
                            false
                        },
                        rlnl::types::TargetType::EqualizerCrystal => {
                            self.do_equalizer_damage(&pseudo, generic).await;
                            true
                        }
                        _ => {
                            //log::info!("Got DamageCube with target_type {:?}", cube_damage.target_type);
                            true
                        },
                    }
                } else {
                    log::warn!("Got DamageCube event with bad serialization type");
                    true
                }
            },
            (rlnl::event_code::NetworkEvent::DamageCubeNoEffect, Some(data)) => {
                let maybe_cube_dmg = <dyn core::any::Any>::downcast_ref::<rlnl::events::ingame::DestroyCubeNoEffect>(data.as_ref());
                if let Some(cube_damage) = maybe_cube_dmg {
                    match cube_damage.target_type {
                        rlnl::types::TargetType::TeamBase => {
                            let actual_damage_data = |cubes: Vec<rlnl::types::CubeState>| {
                                let mut cube_dmg = cube_damage.to_owned();
                                cube_dmg.num_hits = cubes.len() as _;
                                cube_dmg.hit_cubes = cubes;
                                crate::matches::RlnlPacket {
                                    event: rlnl::event_code::NetworkEvent::DestroyCubeNoEffect,
                                    property: literustlib::packet::Property::ReliableOrdered,
                                    data: Box::new(cube_dmg.to_owned()),
                                }
                            };
                            self.do_team_base_stealing(cube_damage, generic, &actual_damage_data).await;
                            false
                        },
                        rlnl::types::TargetType::EqualizerCrystal => {
                            self.do_equalizer_damage(cube_damage, generic).await;
                            true
                        }
                        _ => {
                            //log::info!("Got DamageCubeNoEffect with target_type {:?}", cube_damage.target_type);
                            true
                        },
                    }
                } else {
                    log::warn!("Got DamageCubeNoEffect event with bad serialization type");
                    true
                }
            },
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
            for team in tick_info.captured_firsts.iter() {
                generic.broadcast(
                    rlnl::event_code::NetworkEvent::SetShieldState,
                    literustlib::packet::Property::ReliableOrdered,
                    &rlnl::events::sync::FusionShieldState {
                        team_id: *team as i8,
                        full_power: 1,
                    },
                    true,
                ).await;
            }
            for team in tick_info.lost_lasts.iter() {
                generic.broadcast(
                    rlnl::event_code::NetworkEvent::SetShieldState,
                    literustlib::packet::Property::ReliableOrdered,
                    &rlnl::events::sync::FusionShieldState {
                        team_id: *team as i8,
                        full_power: 0,
                    },
                    true,
                ).await;
            }

            // do base charge tick
            let base_tick_info = self.base_tracking.tick(&tick_info, &self.crystals, generic, self.capture_tracking.points.len(), self.config.protonium_health as u32, &self.config).await;
            if let Some((winning_team, win_mode)) = base_tick_info.win {
                self.do_win(winning_team, win_mode, generic).await;
            }

            // handle surrender vote tick
            self.surrender_tracking.tick(generic).await;
        }
        true
    }

    async fn on_custom(&self, generic: &crate::matches::GenericGamemodeEngine<Self>, user_id: i32, event: rlnl::event_code::NetworkEvent, property: literustlib::packet::Property, data: Box<dyn crate::Broadcastable>) {
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
                let maybe_init_surr = <dyn core::any::Any>::downcast_ref::<rlnl::events::ingame::InitiateSurrender>(data.as_ref());
                if let Some(init_surr) = maybe_init_surr {
                    if let Some(team) = self.player_tracking.team(init_surr.player_id).await {
                        let team_members = self.player_tracking.players_on_team(team).await;
                        let result = self.surrender_tracking.request_new(team, init_surr.player_id, team_members.into_iter(), generic).await;
                        if matches!(result, super::trackers::SurrenderVoteResult::Succeeded) {
                            let winning_team = if team == 0 { 1 } else { 0 };
                            self.do_win(winning_team, WinMode::Surrender, generic).await;
                        }
                    }
                } else {
                    log::warn!("Bad SurrenderRequest data");
                }
            },
            (rlnl::event_code::NetworkEvent::SurrenderVoteCast, literustlib::packet::Property::ReliableOrdered) => {
                if let Some(player_id) = generic.user_key_by_user_id(user_id).await {
                    if let Some(team) = self.player_tracking.team(player_id).await {
                        let maybe_vote = <dyn core::any::Any>::downcast_ref::<rlnl::events::ingame::SurrenderVoteCast>(data.as_ref());
                        if let Some(vote) = maybe_vote {
                            let result = self.surrender_tracking.vote(team, player_id, vote.surrender != 0, generic).await;
                            if matches!(result, super::trackers::SurrenderVoteResult::Succeeded) {
                                let winning_team = if team == 0 { 1 } else { 0 };
                                self.do_win(winning_team, WinMode::Surrender, generic).await;
                            }
                        }
                    }
                }

            },
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
