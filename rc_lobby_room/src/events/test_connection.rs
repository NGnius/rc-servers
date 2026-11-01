const HOST_IP_PARAM_KEY: u8 = 6; // str (ip or domain name)
const HOST_PORT_PARAM_KEY: u8 = 7; // u16 as int
const NETWORK_CONFIG_PARAM_KEY: u8 = 23;

pub struct ConnectTest {
    pub host: String,
    pub port: u16,
    pub network_config: std::sync::Arc<crate::data::network::NetworkConfigData>,
}

impl ConnectTest {
    const CODE: u8 = 2;
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for ConnectTest {
    const CHANNEL: u8 = 0;
    const ENCRYPT: bool = true;
    const RELIABLE: bool = true;

    fn into_event(self) -> polariton::operation::Event<C> {
        polariton::operation::Event {
            code: Self::CODE,
            params: polariton::operation::ParameterTable::from(vec![
                (HOST_IP_PARAM_KEY, polariton::operation::Typed::Str(self.host.clone().into())),
                (HOST_PORT_PARAM_KEY, polariton::operation::Typed::Int(self.port as _)),
                (NETWORK_CONFIG_PARAM_KEY, self.network_config.as_transmissible()),
            ]),
        }
    }
}
