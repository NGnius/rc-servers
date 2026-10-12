use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::ParameterTable;

const CODE: u8 = 8;

pub(super) struct ReconnectGameProvider {
    config: Option<oj_rc_core::persist::config::ReconnectConfig>,
    network: crate::data::network::NetworkConfigData,
    hostname: std::sync::Arc<String>,
    hostport: u16,
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for ReconnectGameProvider {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        if let Some(reconfig) = &self.config {
            let user = user.user()?;
            let game_opt = user.reconnect_info(&reconfig.allowed_modes).await?;
            if let Some(game) = game_opt {
                log::debug!("User {} can reconnect to game {} with players {:?}", user.account_id(), game.guid, game.player_datas);
                let battle_enter = crate::events::battle_enter::BattleEnter {
                    host: self.hostname.to_string(),
                    port: self.hostport,
                    map: game.map,
                    mode: game.mode,
                    guid: game.guid,
                    is_ranked: game.is_ranked,
                    is_custom: game.is_custom,
                    visibility: Some(game.visibility),
                    auto_heal: game.auto_heal,
                    player_datas: game.player_datas,
                    network_config: self.network.clone(),
                };
                Ok(battle_enter.as_transmissible().into())
            } else {
                Ok(params)
            }
        } else {
            Ok(params)
        }
    }
}

pub(super) fn reconnect_provider<C: Send + 'static>(config: &oj_rc_core::persist::config::ConfigImpl, queue_handler: &std::sync::Arc<crate::QueueHandler>) -> SimpleOpImpl<C, crate::UserTy, ReconnectGameProvider> {
    let (hostname, hostport) = queue_handler.redirect();
    SimpleOpImpl::new(ReconnectGameProvider {
        config: <oj_rc_core::persist::config::ConfigImpl as oj_rc_core::ConfigProvider<()>>::reconnect(config),
        network: crate::data::network::NetworkConfigData::from_conf(oj_rc_core::ConfigProvider::<()>::network_config(config)),
        hostname,
        hostport,
    })
}
