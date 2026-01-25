const TIMEOUT_GRANULARITY: std::time::Duration = std::time::Duration::from_millis(100);

pub struct WithTimeoutAction<TA: (Fn() -> bool) + Send + 'static> {
    on_timeout: TA,
}

pub struct WithTimeoutAndCancelAction<TA: (Fn() -> bool) + Send + Sync + 'static, CA: (FnMut() -> bool) + Send + 'static> {
    // returns true if successful (return false to continuously re-run it)
    on_timeout: std::sync::Arc<TA>,
    // returns true if cancelled
    is_cancelled: CA,
}


pub struct Timeout<T> {
    inner: T,
    duration: std::time::Duration,
}

impl Timeout<()> {
    #[must_use]
    pub fn new(duration: std::time::Duration) -> Self {
        Self {
            inner: (),
            duration,
        }
    }

    #[must_use]
    pub fn on_timeout<TA: (Fn() -> bool) + Send + 'static>(self, action: TA) -> Timeout<WithTimeoutAction<TA>> {
        Timeout {
            inner: WithTimeoutAction { on_timeout: action },
            duration: self.duration,
        }
    }
}

impl <TA: (Fn() -> bool) + Send + Sync + 'static> Timeout<WithTimeoutAction<TA>> {
    #[must_use]
    pub fn with_cancel_check<CA: (FnMut() -> bool) + Send + 'static>(self, action: CA) -> Timeout<WithTimeoutAndCancelAction<TA, CA>> {
        Timeout {
            inner: WithTimeoutAndCancelAction {
                on_timeout: std::sync::Arc::new(self.inner.on_timeout),
                is_cancelled: action,
            },
            duration: self.duration,
        }
    }

    /*async fn main_task(mut self2: Self) {
        tokio::time::sleep(self2.duration).await;
        while !(self2.inner.on_timeout)() {
            tokio::time::sleep(TIMEOUT_GRANULARITY).await;
        }

    }

    pub async fn start(self) -> tokio::task::JoinHandle<()> {
        tokio::task::spawn(Self::main_task(self))
    }*/
}

impl <TA: (Fn() -> bool) + Send + Sync + 'static, CA: (FnMut() -> bool) + Send + 'static> Timeout<WithTimeoutAndCancelAction<TA, CA>> {
    async fn main_task(mut self, deadline: chrono::DateTime<chrono::Utc>) {
        loop {
            if (self.inner.is_cancelled)() { return; }
            let now = chrono::Utc::now();
            if now >= deadline {
                let on_timeout_clone = self.inner.on_timeout.clone();
                if tokio::task::spawn_blocking(move || on_timeout_clone() ).await.unwrap_or(true) {
                    return;
                }
            }
            tokio::time::sleep(TIMEOUT_GRANULARITY).await;
        }
    }

    pub async fn start(self) -> tokio::task::JoinHandle<()> {
        let deadline = chrono::Utc::now() + self.duration;
        tokio::task::spawn(self.main_task(deadline))
    }
}


