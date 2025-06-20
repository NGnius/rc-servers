pub struct BattleFound;

impl BattleFound {
    const CODE: u8 = 3;
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for BattleFound {
    const CHANNEL: u8 = 0;
    const ENCRYPT: bool = true;
    const RELIABLE: bool = true;

    fn into_event(self) -> polariton::operation::Event<C> {
        polariton::operation::Event {
            code: Self::CODE,
            params: polariton::operation::ParameterTable::with_capacity(0),
        }
    }
}
