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

pub struct PrivateMessage {
    pub sender_name: String,
    pub sender_display_name: String,
    pub text: String,
    pub is_dev: bool,
    pub is_mod: bool,
    pub is_admin: bool,
}

impl PrivateMessage {
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
