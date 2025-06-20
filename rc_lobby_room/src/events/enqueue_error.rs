const DATA_PARAM_KEY: u8 = 21;
const TEXT_PARAM_KEY: u8 = 22;

pub struct QueueJoinError {
    pub code: i16,
    pub text: String,
}

impl QueueJoinError {
    const CODE: u8 = 1;
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for QueueJoinError {
    const CHANNEL: u8 = 0;
    const ENCRYPT: bool = true;
    const RELIABLE: bool = true;

    fn into_event(self) -> polariton::operation::Event<C> {
        polariton::operation::Event {
            code: Self::CODE,
            params: vec![
                (DATA_PARAM_KEY, polariton::operation::Typed::Short(self.code)),
                (TEXT_PARAM_KEY, polariton::operation::Typed::Str(self.text.into())),
            ].into(),
        }
    }
}
