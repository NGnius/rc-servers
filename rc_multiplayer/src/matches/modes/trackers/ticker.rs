pub struct TickTracker<const TICK_MS: i64 = 50> {
    last_tick: std::sync::atomic::AtomicI64,
}

impl <const TICK_MS: i64> TickTracker<TICK_MS> {
    pub fn new() -> Self {
        Self {
            last_tick: std::sync::atomic::AtomicI64::new(i64::MIN),
        }
    }

    pub fn tick(&self) -> u16 {
        let now = chrono::Utc::now().timestamp_millis();
        let last_tick = self.last_tick.load(std::sync::atomic::Ordering::SeqCst);
        if last_tick == i64::MIN {
            // first tick
            self.last_tick.store(now, std::sync::atomic::Ordering::SeqCst);
            1u16
        } else {
            let delta = (now - last_tick) / TICK_MS;
            if delta == 0 { return 0; }
            self.last_tick.store(last_tick + (delta * TICK_MS), std::sync::atomic::Ordering::SeqCst);
            delta as u16
        }
    }
}
