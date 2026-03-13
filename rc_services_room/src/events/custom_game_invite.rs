//#[derive(Clone)]
pub struct CustomGameInvite {
    pub inviter_public_id: String,
    pub inviter_display_name: String,
    pub session: String,
    pub avatar_id: Option<i32>,
    pub invited_to_team_a: bool,
}

impl <C: Send + Sync + 'static> polariton_server::events::IntoEvent<C> for CustomGameInvite {
    const CHANNEL: u8 = 0;
    const ENCRYPT: bool = true;
    const RELIABLE: bool = true;

    fn into_event(self) -> polariton::operation::Event<C> {
        let mut params = polariton::operation::ParameterTable::with_capacity(1);
        params.insert(172, polariton::operation::Typed::HashMap(vec![
            (polariton::operation::Typed::Str("Inviter".into()), polariton::operation::Typed::Str(self.inviter_public_id.into())),
            (polariton::operation::Typed::Str("DisplayName".into()), polariton::operation::Typed::Str(self.inviter_display_name.into())),
            (polariton::operation::Typed::Str("SessionID".into()), polariton::operation::Typed::Str(self.session.into())),
            (polariton::operation::Typed::Str("UseCustomAvatar".into()), polariton::operation::Typed::Bool(self.avatar_id.is_none())),
            (polariton::operation::Typed::Str("AvatarID".into()), polariton::operation::Typed::Int(self.avatar_id.unwrap_or(0))),
            (polariton::operation::Typed::Str("InvitedToTeamA".into()), polariton::operation::Typed::Bool(self.invited_to_team_a)),
        ].into()));
        polariton::operation::Event {
            code: 7,
            params,
        }
    }
}
