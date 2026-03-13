#[derive(Clone)]
pub struct CustomGameRefresh {
    pub session: String,
}

impl <C: Send + Sync + 'static> polariton_server::events::IntoEvent<C> for CustomGameRefresh {
    const CHANNEL: u8 = 0;
    const ENCRYPT: bool = true;
    const RELIABLE: bool = true;

    fn into_event(self) -> polariton::operation::Event<C> {
        let mut params = polariton::operation::ParameterTable::with_capacity(1);
        params.insert(172, polariton::operation::Typed::Str(self.session.into()));
        polariton::operation::Event {
            code: 8,
            params,
        }
    }
}
