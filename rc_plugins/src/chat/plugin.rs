pub trait ChatPlugin: crate::Plugin {
    fn set_provider(&self, provider: std::sync::Arc<Box<dyn ChatProvider>>);
    fn on_message(&self, message: &str, channel: &str, username: &str);
}

pub trait ChatProvider: Send + Sync {
    fn send_message(&self, message: &str, channel: &str, username: &str);
}
