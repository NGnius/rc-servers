const HOST_IP_PARAM_KEY: u8 = 6; // seems to actually be an IP address or domain name
const HOST_PORT_PARAM_KEY: u8 = 7;
const MAP_NAME_PARAM_KEY: u8 = 8;
const GAME_MODE_PARAM_KEY: u8 = 1;
const GAME_GUID_PARAM_KEY: u8 = 40;
const IS_RANKED_PARAM_KEY: u8 = 25;
const IS_CUSTOM_GAME_PARAM_KEY: u8 = 27;
const MAP_VISIBILITY_PARAM_KEY: u8 = 28;
const IS_AUTO_HEAL_PARAM_KEY: u8 = 42;
const PLAYER_DATA_PARAM_KEY: u8 = 5;
const NETWORK_CONFIG_PARAM_KEY: u8 = 23;

pub struct BattleEnter {
    pub host: String,
    pub port: u16,
    pub map: String,
    pub mode: oj_rc_core::data::game_mode::GameMode,
    pub guid: String,
    pub is_ranked: bool,
    pub is_custom: bool,
    pub visibility: Option<oj_rc_core::data::game_mode::MapVisibility>, // ?
    pub auto_heal: bool,
    pub player_datas: Vec<oj_rc_core::data::player_data::PlayerData>,
    pub network_config: crate::data::network::NetworkConfigData,
}

impl BattleEnter {
    const CODE: u8 = 5;

    fn as_transmissible<C>(&self) -> Vec<(u8, polariton::operation::Typed<C>)> {
        let player_datas = polariton::operation::Typed::Arr(polariton::operation::Arr {
            ty: polariton::serdes::TypePrefix::HashMap,
            custom_ty: None,
            items: self.player_datas.iter().map(|x| x.as_transmissible()).collect()
        });
        let mut vec = Vec::with_capacity(11);
        vec.push((HOST_IP_PARAM_KEY, polariton::operation::Typed::Str(self.host.clone().into())));
        vec.push((HOST_PORT_PARAM_KEY, polariton::operation::Typed::Int(self.port as _)));
        vec.push((MAP_NAME_PARAM_KEY, polariton::operation::Typed::Str(self.map.clone().into())));
        vec.push((GAME_MODE_PARAM_KEY, polariton::operation::Typed::Int(self.mode as i32)));
        vec.push((GAME_GUID_PARAM_KEY, polariton::operation::Typed::Str(self.guid.clone().into())));
        vec.push((IS_RANKED_PARAM_KEY, polariton::operation::Typed::Bool(self.is_ranked)));
        vec.push((IS_CUSTOM_GAME_PARAM_KEY, polariton::operation::Typed::Bool(self.is_custom)));
        if let Some(visibility) = self.visibility {
            vec.push((MAP_VISIBILITY_PARAM_KEY, polariton::operation::Typed::Int(visibility as i32)));
        }
        vec.push((IS_AUTO_HEAL_PARAM_KEY, polariton::operation::Typed::Bool(self.auto_heal)));
        vec.push((PLAYER_DATA_PARAM_KEY, player_datas));
        vec.push((NETWORK_CONFIG_PARAM_KEY, self.network_config.as_transmissible()));
        vec
    }
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for BattleEnter {
    const CHANNEL: u8 = 0;
    const ENCRYPT: bool = true;
    const RELIABLE: bool = true;

    fn into_event(self) -> polariton::operation::Event<C> {
        polariton::operation::Event {
            code: Self::CODE,
            params: self.as_transmissible().into(),
        }
    }
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for &BattleEnter {
    const CHANNEL: u8 = 0;
    const ENCRYPT: bool = true;
    const RELIABLE: bool = true;

    fn into_event(self) -> polariton::operation::Event<C> {
        polariton::operation::Event {
            code: BattleEnter::CODE,
            params: self.as_transmissible().into(),
        }
    }
}
