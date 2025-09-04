use polariton::operation::Typed;

pub struct AvatarInfo {
    pub name: String,
    pub use_custom_avatar: bool,
    pub avatar_id: i32,
}

impl AvatarInfo {
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        Typed::HashMap(vec![
            (Typed::Str("name".into()), Typed::Str(self.name.clone().into())),
            (Typed::Str("useCustomAvatar".into()), Typed::Bool(self.use_custom_avatar)),
            (Typed::Str("avatarId".into()), Typed::Int(self.avatar_id)),
        ].into())
    }
}

// TODO pub struct FriendInfo {}
