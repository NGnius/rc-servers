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

    pub(super) async fn save_clan_avatar(&self, image: Vec<u8>, clan_name: &str) -> Result<(), polariton_server::operations::SimpleOpError> {
        // seems to always be jpg
        let token = generate_token(clan_name.as_bytes(), &self.secret);
        let auth_header_val = format!("Internal {}", token);
        let url = format!("{}/clanavatar/Live/{}", self.cdn, clan_name);
        if let Err(e) = self.http_client.post(url)
            .header("Authorization", auth_header_val)
            .body(image)
            .send()
            .await {
            log::error!("Failed to update clan avatar for {} ({}): {}", self.account.public_id, self.account.id, e);
            return Err((crate::data::error_codes::SocialErrorCode::UnexpectedError as i16).into());
        }
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
        let send_to_everyone = to.is_empty();
        let data = IntercomWebServiceMessage {
            public_ids: to,
            data: IntercomWebServiceUserMessage::DevMessage(msg),
            everyone: send_to_everyone,
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

    async fn update_custom_game(&self, msg: IntercomLobbyCustomGameDataMessage) {
        let data = IntercomLobbyStateMessage::CustomGame(msg);
        if let Err(e) = self.post_to_intercom(&data, ".oj_lobby", "state").await {
            log::error!("Failed to send intercom custom game state message: {}", e);
        }
    }

    async fn update_status(&self, server_name: &str, msg: oj_serdes::ServerStatus) {
        if let Err(e) = self.post_to_intercom(&msg, ".status", server_name).await {
            log::error!("Failed to send intercom status message: {}", e);
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "state")]
pub enum IntercomLobbyStateMessage {
    CustomGame(IntercomLobbyCustomGameDataMessage),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IntercomLobbyCustomGameDataMessage {
    pub session_id: String,
    pub config: IntercomLobbyCustomGameConfig,
    pub users: Vec<IntercomLobbyCustomGameUserData>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CustomGameMode {
    BattleArena,
    TeamDeathmatch,
    Pit,
    SuddenDeath,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum CustomGameVisibility {
    Good,
    Poor,
    Bad,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IntercomLobbyCustomGameConfig {
    pub game_mode: CustomGameMode,
    pub map: String,
    pub map_visibility: CustomGameVisibility,
    pub health_regen: bool,
    pub capture_segment_memory: bool,
    pub base_shields_go_down: bool,
    pub damage_mult: i32,
    pub health_mult: i32,
    pub power_mult: i32,
    pub game_time: i32, // minutes
    pub capture_speed: i32, // seconds
    pub points_kill_streak: bool,
    pub points_total_required: i32,
    pub number_of_kills_to_win: i32,
    pub respawn_time: i32,
    pub core_appear_frequency: i32,
    pub core_health_multiplier: i32,
    pub core_destroy_time: i32,
    pub protonium_harvest: i32,
    pub ceiling_multiplier: i32,
    pub min_cpu: i32,
    pub max_cpu: i32,
}

impl IntercomLobbyCustomGameConfig {
    pub fn as_core(&self) -> super::GameOverrides {
        super::GameOverrides {
            capture_segment_memory: self.capture_segment_memory,
            base_shields_go_down: self.base_shields_go_down,
            damage_mult: self.damage_mult,
            health_mult: self.health_mult,
            power_mult: self.power_mult,
            game_time: self.game_time,
            capture_speed: self.capture_speed,
            points_kill_streak: self.points_kill_streak,
            points_total_required: self.points_total_required,
            number_of_kills_to_win: self.number_of_kills_to_win,
            respawn_time: self.respawn_time,
            core_appear_frequency: self.core_appear_frequency,
            core_health_multiplier: self.core_health_multiplier,
            core_destroy_time: self.core_destroy_time,
            protonium_harvest: self.protonium_harvest,
            ceiling_multiplier: self.ceiling_multiplier,
            min_cpu: self.min_cpu,
            max_cpu: self.max_cpu,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IntercomLobbyCustomGameUserData {
    pub public_id: String,
    pub team: u8,
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

