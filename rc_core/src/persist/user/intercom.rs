use serde::{Serialize, Deserialize};

impl super::account_json::UserData {
    async fn listen_on_websocket<D: serde::de::DeserializeOwned>(&self, server_name: &str) -> Result<super::IntercomListener<D>, reqwest_websocket::Error> {
        use reqwest_websocket::RequestBuilderExt;
        let token =  generate_token(format!("{}/{}", server_name, self.account.public_id).as_bytes(), &self.secret);
        let auth_header_val = format!("Internal {}", token);
        let url = format!("{}/intercom/{}/{}", self.intercom, server_name, self.account.public_id);
        log::debug!("Listening on websocket {}", url);
        let websocket = self.http_client.get(url)
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

    async fn post_to_intercom<D: serde::Serialize>(&self, data: &D, server_name: &str, operation: &str) -> Result<(), reqwest::Error> {
        let path = format!("{}/{}/{}", server_name, self.account.public_id, operation);
        let token =  generate_token(path.as_bytes(), &self.secret);
        let auth_header_val = format!("Internal {}", token);
        let url = format!("{}/intercom/{}", self.auth, path);
        log::debug!("Posting intercom message to {}", url);
        self.http_client.post(url)
            .header("Authorization", auth_header_val)
            .json(data)
            .send()
            .await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl super::IntercomUser for super::account_json::UserData {
    async fn save_custom_avatar(&self, image: Vec<u8>) -> Result<(), polariton_server::operations::SimpleOpError> {
        // seems to always be jpg
        let token =  generate_token(self.account.public_id.as_bytes(), &self.secret);
        let auth_header_val = format!("Internal {}", token);
        let url = format!("{}/customavatar/Live/{}", self.cdn, self.account.public_id);
        if let Err(e) = self.http_client.post(url)
            .header("Authorization", auth_header_val)
            .body(image)
            .send()
            .await {
            log::error!("Failed to update custom avatar for {} ({}): {}", self.account.public_id, self.account.id, e);
            return Err((crate::data::error_codes::WebServicesError::UnexpectedError as i16).into());
        }
        Ok(())
    }

    async fn save_factory_thumbnail(&self, factory_id: i32, image: Vec<u8>) -> Result<(), polariton_server::operations::SimpleOpError> {
        let token = generate_token(factory_id.to_string().as_bytes(), &self.secret);
        let auth_header_val = format!("Internal {}", token);
        let url = format!("{}/roboshop/Live/{}", self.cdn, factory_id);
        if let Err(e) = self.http_client.post(url)
            .header("Authorization", auth_header_val)
            .body(image)
            .send()
            .await {
            log::error!("Failed to update factory thumbnail for {} ({}): {}", self.account.public_id, self.account.id, e);
            return Err((crate::data::error_codes::WebServicesError::UnexpectedError as i16).into());
        }
        Ok(())
    }

    async fn webservice_listener(&self) -> Result<super::IntercomListener<IntercomWebServiceUserMessage>, polariton_server::operations::SimpleOpError> {
        self.listen_on_websocket(".oj_services").await
            .map_err(|e| polariton_server::operations::SimpleOpError::with_message(
                crate::data::error_codes::WebServicesError::PlatformFeatureNotAvailable as i16,
                e.to_string()
            ))
    }

    async fn show_dev_message(&self, msg: IntercomDevMessage, to: Vec<String>) {
        let data = IntercomWebServiceMessage {
            public_ids: to,
            data: IntercomWebServiceUserMessage::DevMessage(msg),
            everyone: false,
        };
        if let Err(e) = self.post_to_intercom(&data, ".oj_services", "messages").await {
            log::error!("Failed to send intercom dev message: {}", e);
        }
    }

    async fn enter_maintenance(&self, msg: IntercomMaintenanceMessage, to: Vec<String>) {
        let send_to_everyone = to.is_empty();
        let data = IntercomWebServiceMessage {
            public_ids: to,
            data: IntercomWebServiceUserMessage::Maintenance(msg),
            everyone: send_to_everyone,
        };
        if let Err(e) = self.post_to_intercom(&data, ".oj_services", "messages").await {
            log::error!("Failed to send intercom maintenance mode message: {}", e);
        }
    }

    async fn update_status(&self, server_name: &str, msg: oj_serdes::ServerStatus) {
        if let Err(e) = self.post_to_intercom(&msg, ".status", server_name).await {
            log::error!("Failed to send intercom status message: {}", e);
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IntercomWebServiceMessage {
    pub public_ids: Vec<String>,
    pub data: IntercomWebServiceUserMessage,
    pub everyone: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum IntercomWebServiceUserMessage {
    DevMessage(IntercomDevMessage),
    Maintenance(IntercomMaintenanceMessage),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IntercomDevMessage {
    pub message: String,
    pub duration: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IntercomMaintenanceMessage {
    pub message: String,
}

pub fn generate_token(salt: &[u8], key: &[u8]) -> String {
    use sha2::{Digest, Sha512};
    let mut hasher = Sha512::new();
    hasher.update(salt);
    hasher.update(key);
    let token_bytes = hasher.finalize();
    hex::encode(&token_bytes[..])
}

