use polariton::operation::Typed;

#[derive(Clone)]
pub struct CustomGameConfigRefresh {
    pub field: String,
    pub value: String,
}

impl <C: Send + Sync + 'static> polariton_server::events::IntoEvent<C> for CustomGameConfigRefresh {
    const CHANNEL: u8 = 0;
    const ENCRYPT: bool = true;
    const RELIABLE: bool = true;

    fn into_event(self) -> polariton::operation::Event<C> {
        let mut params = polariton::operation::ParameterTable::with_capacity(1);
        params.insert(181, polariton::operation::Typed::HashMap(vec![
            (Typed::Str("Field".into()), Typed::Str(self.field.into())),
            (Typed::Str("Value".into()), Typed::Str(self.value.into())),
        ].into()));
        polariton::operation::Event {
            code: 9,
            params,
        }
    }
}
