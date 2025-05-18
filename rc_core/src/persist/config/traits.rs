use polariton::operation::Typed;

#[async_trait::async_trait]
pub trait ConfigProvider<C: Clone> {
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
    fn client_config(&self) -> Typed<C>;
    fn login_messages(&self) -> DevMessageProvider<C>;
    fn public_channels(&self) -> Typed<C>;
    fn server_config(&self) -> ServerConfig;
    fn garage_upgrades(&self) -> GarageUpgrades;
    async fn factory(&self) -> Result<crate::factory::Factory, Box<dyn std::error::Error + 'static>>;
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

pub struct DevMessageProvider<C: Clone> {
    messages: Vec<TypedDevMessage<C>>,
}

impl <C: Clone> DevMessageProvider<C> {
    pub fn new(messages: Vec<(String, i32)>) -> Self {
        Self {
            messages: messages.into_iter().map(|(msg, time)| {
                let bytes: Vec<u8> = msg.as_bytes().into();
                TypedDevMessage {
                    message: Typed::Bytes(bytes.into()),
                    display_time: Typed::Int(time),
                }
            }
            ).collect(),
        }
    }

    pub fn get(&self, index: usize) -> TypedDevMessage<C> {
        // TODO maybe make this less obtuse -- it works for random, but isn't really obvious for anything else
        if self.messages.is_empty() {
            TypedDevMessage {
                message: Typed::Bytes(Vec::default().into()),
                display_time: Typed::Int(-1),
            }
        } else if self.messages.len() == 1 {
            self.messages[0].clone()
        } else {
            let actual_index = index % self.messages.len(); // guarantees index is within allowed range of messages
            self.messages[actual_index].clone()
        }
    }
}

#[derive(Clone, Debug)]
pub struct TypedDevMessage<C> {
    pub message: Typed<C>,
    pub display_time: Typed<C>,
}

pub struct ServerConfig {
    pub database: String,
    pub auto_signup: bool,
}

#[derive(Clone, Debug)]
pub struct GarageUpgrades {
    pub increments: Vec<GarageUpgradeIncrement>,
}

#[derive(Clone, Debug)]
pub struct GarageUpgradeIncrement {
    pub cpu: u32,
    pub cost: u32,
}

impl GarageUpgrades {
    pub fn slot_upgrades<C>(&self) -> Typed<C> {
        Typed::HashMap(vec![
            (Typed::Str("cpuIncreaseCost".into()), Typed::Dict(polariton::operation::Dict {
                key_ty: polariton::serdes::TypePrefix::Int, // int
                val_ty: polariton::serdes::TypePrefix::Int, // int
                // (CPU limit, upgrade cost)
                items: self.increments.iter().map(|inc| (Typed::Int(inc.cpu as _), Typed::Int(inc.cost as _))).collect(),
            }))
        ].into())
    }
}
