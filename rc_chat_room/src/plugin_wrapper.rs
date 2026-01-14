pub struct PluginWrapper {
    plugins: Vec<std::sync::Arc<Box<dyn oj_rc_plugins::chat::ChatPlugin>>>,
}

impl PluginWrapper {
    pub fn new(plugins: impl IntoIterator<Item=Box<dyn oj_rc_plugins::chat::ChatPlugin>>) -> Self {
        Self {
            plugins: plugins.into_iter().map(std::sync::Arc::new).collect(),
        }
    }

    pub fn set_provider(&self, provider: std::sync::Arc<Box<dyn oj_rc_plugins::chat::ChatProvider>>) {
        for plugin in self.plugins.iter() {
            plugin.set_provider(provider.clone());
        }
    }

    pub async fn on_message(&self, message: &str, channel: &str, username: &str) {
        let mut futures = Vec::with_capacity(self.plugins.len());
        for plugin in self.plugins.iter() {
            let owned_plugin = plugin.to_owned();
            let owned_message = message.to_owned();
            let owned_channel = channel.to_owned();
            let owned_username = username.to_owned();
            let handle = tokio::task::spawn_blocking(move || owned_plugin.on_message(&owned_message, &owned_channel, &owned_username));
            futures.push(handle);
        }
        futures::future::join_all(futures).await;
    }
}

pub struct ProviderWrapper {
    provider: std::sync::Arc<tokio::sync::RwLock<crate::state::chat::ChatSystem>>,
}

impl ProviderWrapper {
    pub fn new(provider: std::sync::Arc<tokio::sync::RwLock<crate::state::chat::ChatSystem>>) -> Self {
        Self {
            provider,
        }
    }

    async fn do_send_message(provider: std::sync::Arc<tokio::sync::RwLock<crate::state::chat::ChatSystem>>, message: String, channel: String, username: String) {
        provider.read().await
            .send_fake_message(message, channel, username).await;
    }
}

impl oj_rc_plugins::chat::ChatProvider for ProviderWrapper {
    fn send_message(&self, message: &str, channel: &str, username: &str) {
        tokio::task::spawn(Self::do_send_message(self.provider.clone(), message.to_owned(), channel.to_owned(), username.to_owned()));
    }
}
