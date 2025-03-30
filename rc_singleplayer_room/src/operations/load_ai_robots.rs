use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed};

const PARAM_KEY: u8 = 8;

pub(super) fn tdm_machines_provider() -> SimpleFunc<1, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, user: &crate::UserTy| {
        let ulock = user.user()?;
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, ulock.singleplayer_robots()?);
        let event_tx = user.event_sender();
        let user_bot_data = ulock.slot_by_id(ulock.selected_garage_slot() as _)?;
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(20)).await;
            log::debug!("Sending singleplayer event");
            let mut spawn_params = std::collections::HashMap::with_capacity(4);
            spawn_params.insert(2 /* robot GUID */, Typed::Str("1337_1337".into()));
            spawn_params.insert(3 /* machine model */, user_bot_data.data);
            spawn_params.insert(4 /* robot name */, Typed::Str("RE_robot_spawn_name0".into())); // FIXME
            spawn_params.insert(7 /* color model */, user_bot_data.colour_data);
            event_tx.send(polariton_server::ToSend::Data {
                data: polariton::packet::Data::Event(polariton::operation::Event { code: 3, params: spawn_params.into() }),
                encrypt: true,
                channel: 0,
                reliable: true,
            }).unwrap();
            /*let mut update_params = std::collections::HashMap::with_capacity(1);
            update_params.insert(6 /* ??? */, Typed::Int(5));
            event_tx.send(polariton_server::ToSend::Data {
                data: polariton::packet::Data::Event(polariton::operation::Event { code: 5, params: update_params.into() }),
                encrypt: true,
                channel: 0,
                reliable: true,
            }).unwrap();*/
        });
        Ok(params.into())
    })
}
