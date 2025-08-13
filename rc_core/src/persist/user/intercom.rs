#[async_trait::async_trait]
impl super::IntercomUser for super::account_json::UserData {
    async fn save_custom_avatar(&self, image: Vec<u8>) -> Result<(), polariton_server::operations::SimpleOpError> {
        // seems to always be jpg
        let token =  generate_token(self.account.public_id.as_bytes(), &self.secret);
        let auth_header_val = format!("Internal {}", token);
        let url = format!("{}/customavatar/Live/{}", self.cdn, self.account.public_id);
        if let Err(e) = reqwest::Client::new().post(url)
            .header("Authorization", auth_header_val)
            .body(image)
            .send()
            .await {
            log::error!("Failed to update custom avatar for {} ({}): {}", self.account.public_id, self.account.id, e);
            return Err((crate::data::error_codes::WebServicesError::UnexpectedError as i16).into());
        }
        Ok(())
    }
}

pub fn generate_token(salt: &[u8], key: &[u8]) -> String {
    use sha2::{Digest, Sha512};
    let mut hasher = Sha512::new();
    hasher.update(salt);
    hasher.update(key);
    let token_bytes = hasher.finalize();
    hex::encode(&token_bytes[..])
}

