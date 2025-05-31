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
