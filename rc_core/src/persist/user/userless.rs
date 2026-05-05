use reqwest_websocket::Upgrade;

impl super::AccountProvider {
    async fn listen_on_websocket<D: serde::de::DeserializeOwned>(&self, server_name: &str) -> Result<super::IntercomListener<D>, reqwest_websocket::Error> {
        //use reqwest_websocket::RequestBuilderExt;
        let token =  super::generate_intercom_token(format!("state/{}", server_name).as_bytes(), &self.secret);
        let auth_header_val = format!("Internal {}", token);
        let url = format!("{}/intercom/userless/{}", self.intercom, server_name);
        log::debug!("Listening on websocket {}", url);
        let websocket = self.intercom_http_client.get(url)
            .header("Authorization", auth_header_val)
            .upgrade()
            .send()
            .await?
            .into_websocket()
            .await?;
        Ok(super::IntercomListener {
            websocket,
            _d: Default::default(),
        })
    }
}

#[async_trait::async_trait]
impl super::Userless for super::AccountProvider {
    async fn lobby_state_listener(&self) -> Result<super::IntercomListener<super::intercom::IntercomLobbyStateMessage>, reqwest_websocket::Error> {
        self.listen_on_websocket(".oj_lobby").await
    }
}
