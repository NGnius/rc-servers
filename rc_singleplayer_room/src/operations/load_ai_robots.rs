use polariton::operation::{ParameterTable, OperationResponse};

const CODE: u8 = 1;

const PARAM_KEY: u8 = 8;

pub(super) fn tdm_machines_provider(factory: &std::sync::Arc<oj_rc_core::factory::Factory>, weapon_order: std::sync::Arc<oj_rc_core::cubes::WeaponListParser>, conf: &oj_rc_core::ConfigImpl, cpu_counter: std::sync::Arc<oj_rc_core::cubes::CpuListParser>) -> AiRobots {
    AiRobots {
        factory: factory.to_owned(),
        weapon_parser: weapon_order,
        singleplayer_config: <oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::singleplayer_details(conf),
        cpu_parser: cpu_counter,
    }
}

async fn do_handling(params: ParameterTable<()>, user: &crate::UserTy, factory: &oj_rc_core::factory::Factory, weapon_order: &oj_rc_core::cubes::WeaponListParser, singleplayer_conf: &oj_rc_core::persist::config::SingleplayerConfig, cpu_counter: &std::sync::Arc<oj_rc_core::cubes::CpuListParser>) -> Result<ParameterTable<()>, i16> {
    let ulock = user.user()?;
    let mut params = params.to_dict();
    params.insert(PARAM_KEY, ulock.singleplayer_robots(factory, weapon_order, singleplayer_conf, cpu_counter).await?);
    Ok(params.into())
}

pub struct AiRobots {
    factory: std::sync::Arc<oj_rc_core::factory::Factory>,
    weapon_parser: std::sync::Arc<oj_rc_core::cubes::WeaponListParser>,
    singleplayer_config: oj_rc_core::persist::config::SingleplayerConfig,
    cpu_parser: std::sync::Arc<oj_rc_core::cubes::CpuListParser>,
}

#[async_trait::async_trait]
impl polariton_server::operations::Operation<()> for AiRobots {
    type User = crate::UserTy;

    async fn handle_async(&self, params: ParameterTable<()>, user: &Self::User) -> OperationResponse<()> {
        polariton_server::operations::result_to_op_resp::<CODE, ()>(do_handling(params, user, self.factory.as_ref(), self.weapon_parser.as_ref(), &self.singleplayer_config, &self.cpu_parser).await)
    }
}

impl polariton_server::operations::OperationCode for AiRobots {
    fn op_code() -> u8 {
        CODE
    }
}
