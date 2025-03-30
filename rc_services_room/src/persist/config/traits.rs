use polariton::operation::Typed;

pub trait ConfigProvider<C> {
    fn cube_list(&self) -> Typed<C>;
    fn movement_list(&self) -> Typed<C>;
    fn weapon_list(&self) -> Typed<C>;
    fn weapon_upgrade_list(&self) -> Typed<C>;
    fn weapon_keys(&self) -> Typed<C>;
    fn tech_tree_nodes(&self, unlocked_cubes: &std::collections::HashSet<u32>) -> Typed<C>;
    fn ids(&self) -> Vec<u32>;
    fn regen_config(&self) -> Typed<C>;
    fn after_battle_vote_config(&self) -> Typed<C>;
    fn game_mode_config(&self) -> Typed<C>;
    fn campaigns_parameters(&self) -> Typed<C>;
    fn campaign_waves(&self) -> Typed<C>;
    fn campaign_version(&self) -> Typed<C>;
    fn campaign_details(&self) -> CompleteCampaignProvider;
}

pub struct CompleteCampaignProvider {
    map: std::collections::HashMap<String, std::collections::HashMap<i32, crate::data::campaign::CampaignWavesDifficultyData>>,
}

impl CompleteCampaignProvider {
    pub fn new(map: std::collections::HashMap<String, std::collections::HashMap<i32, crate::data::campaign::CampaignWavesDifficultyData>>) -> Self {
        Self { map }
    }

    pub fn get<C>(&self, id: &str, difficulty: &i32) -> Result<Typed<C>, i16> {
        if let Some(campaign) = self.map.get(id) {
            if let Some(details) = campaign.get(difficulty) {
                Ok(details.as_transmissible())
            } else {
                log::warn!("Couldn't find difficulty {} in campaign `{}`", difficulty, id);
                Err(crate::data::error_codes::WebServicesError::DatabaseError as i16)
            }
        } else {
            log::warn!("Couldn't find campaign {} (ignoring difficulty {})", id, difficulty);
            Err(crate::data::error_codes::WebServicesError::DatabaseError as i16)
        }
    }
}
