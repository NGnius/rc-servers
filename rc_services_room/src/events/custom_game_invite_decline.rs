use polariton::operation::Typed;

#[derive(Clone)]
pub struct CustomGameInviteDecline {
    pub public_id: oj_rc_core::types::PublicId,
}

impl <C: Send + Sync + 'static> polariton_server::events::IntoEvent<C> for CustomGameInviteDecline {
    const CHANNEL: u8 = 0;
    const ENCRYPT: bool = true;
    const RELIABLE: bool = true;

    fn into_event(self) -> polariton::operation::Event<C> {
        use oj_rc_core::types::RawType;
        let mut params = polariton::operation::ParameterTable::with_capacity(1);
        params.insert(190, polariton::operation::Typed::HashMap(vec![
            (Typed::Str("UserName".into()), Typed::Str(self.public_id.as_raw_ref().into()))
        ].into()));
        polariton::operation::Event {
            code: 12,
            params,
        }
    }
}
