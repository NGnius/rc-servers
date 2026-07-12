const MAX_HISTORY_LEN: usize = 64;

struct QueueTime {
    time: std::time::Duration,
    players: usize,
}

pub struct QueueTimeTracker {
    average: std::sync::atomic::AtomicI32, // seconds
    history: std::sync::Mutex<std::collections::VecDeque<QueueTime>>,
}

impl QueueTimeTracker {
    pub fn new() -> Self {
        Self {
            average: std::sync::atomic::AtomicI32::new(42),
            history: std::sync::Mutex::new(std::collections::VecDeque::with_capacity(MAX_HISTORY_LEN)),
        }
    }

    fn update_time(&self, players_enqueue_at: impl Iterator<Item = chrono::DateTime<chrono::Utc>>, game_start: chrono::DateTime<chrono::Utc>) {
        let mut total_duration = std::time::Duration::ZERO;
        let mut total_players: usize = 0;
        for player_enqueue_at in players_enqueue_at {
            total_players += 1;
            total_duration += game_start.signed_duration_since(player_enqueue_at).to_std().unwrap_or(std::time::Duration::ZERO);
        }
        let time = total_duration.div_f64(total_players as f64);
        let queue_time = QueueTime {
            time,
            players: total_players,
        };
        let mut lock = self.history.lock().unwrap();
        if lock.len() == MAX_HISTORY_LEN {
            lock.pop_front();
        }
        lock.push_back(queue_time);
        let mut total_time: u128 = 0;
        let mut total_players: u128 = 0;
        for time in lock.iter() {
            total_players += time.players as u128;
            total_time += (time.time.as_secs() as u128) * (time.players as u128);
        }
        let average = total_time / total_players;
        let old_average = self.average.swap(average.try_into().unwrap_or_default(), std::sync::atomic::Ordering::Relaxed);
        log::debug!("Average queue time is now {}s, was {}s", average, old_average);
    }

    pub fn update_time_match_starting_now(&self, players_enqueue_at: impl Iterator<Item = chrono::DateTime<chrono::Utc>>) {
        let now = chrono::Utc::now();
        self.update_time(players_enqueue_at, now);
    }

    pub fn get_average(&self) -> i32 {
        self.average.load(std::sync::atomic::Ordering::Relaxed)
    }
}
