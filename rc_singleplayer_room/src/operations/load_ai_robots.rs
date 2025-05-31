use polariton::operation::{ParameterTable, OperationResponse};

const CODE: u8 = 1;

const PARAM_KEY: u8 = 8;

pub(super) fn tdm_machines_provider(factory: &std::sync::Arc<rc_core::factory::Factory>, weapon_order: std::sync::Arc<rc_core::cubes::WeaponListParser>) -> AiRobots {
    AiRobots {
        factory: factory.to_owned(),
        weapon_parser: weapon_order,
    }
}

async fn do_handling(params: ParameterTable<()>, user: &crate::UserTy, factory: &rc_core::factory::Factory, weapon_order: &rc_core::cubes::WeaponListParser) -> Result<ParameterTable<()>, i16> {
    let ulock = user.user()?;
    let mut params = params.to_dict();
    params.insert(PARAM_KEY, ulock.singleplayer_robots(factory, weapon_order).await?);
    /*let event_tx = user.event_sender();
    let user_bot_data = ulock.slot_by_id(ulock.selected_garage().await.1 as _).await?;
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(20)).await;
        log::debug!("Sending singleplayer event");
        let mut spawn_params = std::collections::HashMap::with_capacity(4);
        spawn_params.insert(2 /* robot GUID */, user_bot_data.uuid);
        spawn_params.insert(3 /* machine model */, user_bot_data.data);
        spawn_params.insert(4 /* robot name */, Typed::Str("Robot123".into())); // FIXME
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
    });*/
    Ok(params.into())
}

pub struct AiRobots {
    factory: std::sync::Arc<rc_core::factory::Factory>,
    weapon_parser: std::sync::Arc<rc_core::cubes::WeaponListParser>,
}

#[async_trait::async_trait]
impl polariton_server::operations::Operation<()> for AiRobots {
    type User = crate::UserTy;

    async fn handle_async(&self, params: ParameterTable<()>, user: &Self::User) -> OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(do_handling(params, user, self.factory.as_ref(), self.weapon_parser.as_ref()).await)
    }
}

impl polariton_server::operations::OperationCode for AiRobots {
    fn op_code() -> u8 {
        CODE
    }
}
