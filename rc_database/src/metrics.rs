pub(crate) struct MetricsState {
    total_calls: u64,
    total_duration_ns: u128,
    total_failed: u64,
    fastest: StatementInfo,
    slowest: StatementInfo,
}

#[derive(Debug)]
pub struct DatabaseMetrics {
    pub avg_query_duration: std::time::Duration,
    pub fail_queries: u64,
    pub success_queries: u64,
    pub slowest: StatementInfo,
    pub fastest: StatementInfo,
}

impl core::fmt::Display for DatabaseMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "avg: {}s success: {}, failed: {} worst: {} ({}s), best: {} ({}s)",
            self.avg_query_duration.as_secs_f32(),
            self.success_queries,
            self.fail_queries,
            self.slowest.statement, self.slowest.elapsed.as_secs_f32(),
            self.fastest.statement, self.fastest.elapsed.as_secs_f32(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct StatementInfo {
    elapsed: std::time::Duration,
    statement: String,
}

impl StatementInfo {
    fn fastest() -> Self {
        Self {
            elapsed: std::time::Duration::ZERO,
            statement: "".to_owned(),
        }
    }

    fn slowest() -> Self {
        Self {
            elapsed: std::time::Duration::MAX,
            statement: "".to_owned(),
        }
    }
}

impl MetricsState {
    pub(crate) fn new() -> Self {
        Self {
            total_calls: 0,
            total_duration_ns: 0,
            total_failed: 0,
            slowest: StatementInfo::fastest(),
            fastest: StatementInfo::slowest(),
        }
    }

    pub(crate) fn snapshot(&self) -> DatabaseMetrics {
        DatabaseMetrics {
            avg_query_duration: std::time::Duration::from_nanos((self.total_duration_ns / self.total_calls as u128) as u64),
            fail_queries: self.total_failed,
            success_queries: self.total_calls,
            slowest: self.slowest.clone(),
            fastest: self.fastest.clone(),
        }
    }
}

pub(super) fn metrics_cb(state: std::sync::Arc<std::sync::Mutex<MetricsState>>) -> impl Fn(&sea_orm::metric::Info) + Send + Sync + 'static {
    move |info: &sea_orm::metric::Info| {
        let mut lock = state.lock().unwrap();
        lock.total_calls += 1;
        lock.total_duration_ns += info.elapsed.as_nanos();
        if info.failed {
            lock.total_failed += 1;
        }
        if info.elapsed < lock.fastest.elapsed {
            lock.fastest.elapsed = info.elapsed;
            lock.fastest.statement = info.statement.to_string();
        }
        if info.elapsed > lock.slowest.elapsed {
            lock.slowest.elapsed = info.elapsed;
            lock.slowest.statement = info.statement.to_string();
        }
    }
}
