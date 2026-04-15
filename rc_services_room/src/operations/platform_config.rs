use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::{operation::{Dict, ParameterTable, Typed}, serdes::TypePrefix};

const CODE: u8 = 165;

const PLATFORM_CONFIG_KEY: u8 = 197;

pub(super) fn platform_config_provider<C: Send + 'static>(conf: &oj_rc_core::ConfigImpl) -> SimpleOpImpl<C, crate::UserTy, PlatformConfigProvider> {
    SimpleOpImpl::new(PlatformConfigProvider {
        chat_config: <oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::chat_system_config(conf),
        links: <oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::url_links(conf),
        platform_config: <oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::platform(conf),
    })
}

pub(super) struct PlatformConfigProvider {
    chat_config: oj_rc_core::persist::config::ChatSystemConfig,
    links: oj_rc_core::persist::config::LinksConfig,
    platform_config: oj_rc_core::persist::config::PlatformConfig,
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for PlatformConfigProvider {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let user = user.user()?;
        let mut connected_channels = user.subscribed_channels_strings().await?;
        let client_selected_channel = if connected_channels.is_empty() || connected_channels.contains(&self.chat_config.default_channel) {
            self.chat_config.default_channel.clone()
        } else {
            connected_channels.remove(0)
        };
        let mut params = params.to_dict();
        params.insert(PLATFORM_CONFIG_KEY, Typed::Dict(Dict {
            key_ty: TypePrefix::Any, // obj
            val_ty: TypePrefix::Any, // obj
            items: vec![
                (Typed::Str("BuyPremiumAvailable".into()), Typed::Bool(self.platform_config.enable_buy_premium)),
                (Typed::Str("MainShopButtonAvailable".into()), Typed::Bool(self.platform_config.enable_shop)),
                (Typed::Str("RoboPassButtonAvailable".into()), Typed::Bool(self.platform_config.enable_robopass)),
                (Typed::Str("LanguageSelectionAvailable".into()), Typed::Bool(self.platform_config.enable_select_language)),
                (Typed::Str("AutoJoinPublicChatRoom".into()), Typed::Str(client_selected_channel.into())),
                (Typed::Str("CanCreateChatRooms".into()), Typed::Bool(self.chat_config.can_create_channels)),
                (Typed::Str("CurseVoiceEnabled".into()), Typed::Bool(self.platform_config.enable_voice)),
                (Typed::Str("DeltaDNAEnabled".into()), Typed::Bool(self.platform_config.enable_analytics)),
                (Typed::Str("UseDecimalSystem".into()), Typed::Bool(self.platform_config.enable_standard_units)),
                (Typed::Str("FeedbackURL".into()), Typed::Str(self.links.feedback_url.clone().into())),
                (Typed::Str("SupportURL".into()), Typed::Str(self.links.support_url.clone().into())),
                (Typed::Str("WikiURL".into()), Typed::Str(self.links.wiki_url.clone().into())),
            ],
        }));
        Ok(params.into())
    }
}
