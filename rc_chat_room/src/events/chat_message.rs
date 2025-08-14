pub struct PublicMessage {
    pub sender_name: String,
    pub sender_display_name: String,
    pub text: String,
    pub is_dev: bool,
    pub is_mod: bool,
    pub is_admin: bool,
    pub channel_name: String,
    pub channel_ty: crate::data::channel::ChatChannelType,
}

impl PublicMessage {
    pub const CODE: u8 = 1;

    pub fn as_event_params<C>(&self) -> polariton::operation::ParameterTable<C> {
        let mut params = std::collections::HashMap::with_capacity(8);
        params.insert(5, polariton::operation::Typed::Str(self.sender_name.clone().into()));
        params.insert(30, polariton::operation::Typed::Str(self.sender_display_name.clone().into()));
        params.insert(2, polariton::operation::Typed::Str(self.text.clone().into()));
        params.insert(6, polariton::operation::Typed::Bool(self.is_dev));
        params.insert(12, polariton::operation::Typed::Bool(self.is_mod));
        params.insert(13, polariton::operation::Typed::Bool(self.is_admin));
        params.insert(3, polariton::operation::Typed::Str(self.channel_name.clone().into()));
        params.insert(1, polariton::operation::Typed::Int(self.channel_ty as _));
        params.into()
    }
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for PublicMessage {
    const CHANNEL: u8 = 0;
    const ENCRYPT: bool = true;
    const RELIABLE: bool = true;

    fn into_event(self) -> polariton::operation::Event<C> {
        polariton::operation::Event {
            code: Self::CODE,
            params: self.as_event_params().into(),
        }
    }
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for &PublicMessage {
    const CHANNEL: u8 = 0;
    const ENCRYPT: bool = true;
    const RELIABLE: bool = true;

    fn into_event(self) -> polariton::operation::Event<C> {
        polariton::operation::Event {
            code: PublicMessage::CODE,
            params: self.as_event_params().into(),
        }
    }
}

pub struct PrivateMessage {
    pub sender_name: String,
    pub sender_display_name: String,
    pub text: String,
    pub is_dev: bool,
    pub is_mod: bool,
    pub is_admin: bool,
}

impl PrivateMessage {
    pub const CODE: u8 = 2;

    pub fn as_event_params<C>(&self) -> polariton::operation::ParameterTable<C> {
        let mut params = std::collections::HashMap::with_capacity(6);
        params.insert(5, polariton::operation::Typed::Str(self.sender_name.clone().into()));
        params.insert(30, polariton::operation::Typed::Str(self.sender_display_name.clone().into()));
        params.insert(2, polariton::operation::Typed::Str(self.text.clone().into()));
        params.insert(6, polariton::operation::Typed::Bool(self.is_dev));
        params.insert(12, polariton::operation::Typed::Bool(self.is_mod));
        params.insert(13, polariton::operation::Typed::Bool(self.is_admin));
        params.into()
    }
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for PrivateMessage {
    const CHANNEL: u8 = 0;
    const ENCRYPT: bool = true;
    const RELIABLE: bool = true;

    fn into_event(self) -> polariton::operation::Event<C> {
        polariton::operation::Event {
            code: Self::CODE,
            params: self.as_event_params().into(),
        }
    }
}

impl <C: Send + 'static> polariton_server::events::IntoEvent<C> for &PrivateMessage {
    const CHANNEL: u8 = 0;
    const ENCRYPT: bool = true;
    const RELIABLE: bool = true;

    fn into_event(self) -> polariton::operation::Event<C> {
        polariton::operation::Event {
            code: PrivateMessage::CODE,
            params: self.as_event_params().into(),
        }
    }
}
