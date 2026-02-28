use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::ParameterTable;

const CODE: u8 = 53;

const PARAM_KEY: u8 = 1;

pub(super) struct BattleArenaConfigurer {
    factory: std::sync::Arc<oj_rc_core::factory::Factory>,
    weapon_list: std::sync::Arc<oj_rc_core::cubes::WeaponListParser>,
    cpu_counter: std::sync::Arc<oj_rc_core::cubes::CpuListParser>,
    ba_conf: oj_rc_core::persist::config::BattleArenaResolver,
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for BattleArenaConfigurer {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let user_info = user.user()?;
        //log::warn!("Retrieved ba config");
        let data = self.ba_conf.resolve_typed(user_info.as_ref().as_ref(), self.factory.as_ref(), &self.weapon_list, &self.cpu_counter).await?;
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, data);
        Ok(params.into())
    }
}

pub(super) fn battle_arena_config_provider<C: Send + 'static>(conf: &oj_rc_core::ConfigImpl, factory: &std::sync::Arc<oj_rc_core::factory::Factory>, weapon_list: std::sync::Arc<oj_rc_core::cubes::WeaponListParser>,
    cpu_counter: std::sync::Arc<oj_rc_core::cubes::CpuListParser>) -> SimpleOpImpl<C, crate::UserTy, BattleArenaConfigurer> {
    let ba_conf = <oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::ba_settings(conf);
    SimpleOpImpl::new(BattleArenaConfigurer {
        factory: factory.to_owned(),
        weapon_list,
        cpu_counter,
        ba_conf
    })
}
