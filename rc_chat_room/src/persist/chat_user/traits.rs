use polariton::operation::Typed;

pub trait ChatUser {
    fn subscribed_channels(&self) -> Typed<()>;
    fn subscribed_channels_strings(&self) -> Vec<String>;
    fn add_subscribed_channel(&self, channel: String, channel_ty: crate::data::channel::ChatChannelType) -> Typed<()>;
    fn remove_subscribed_channel(&self, channel: String, channel_ty: crate::data::channel::ChatChannelType) -> bool;
}
