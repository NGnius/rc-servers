use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::{ParameterTable, Typed};

const CODE: u8 = 184;

const CUBE_PARAM_KEY: u8 = 211; // str (hex); in
const NODES_PARAM_KEY: u8 = 210;

pub(super) struct TechTreeUnlocker {
    cost_map: std::collections::HashMap<String, u32>,
    nodes: oj_rc_core::persist::config::TechTreeNodeProvider,
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for TechTreeUnlocker {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let mut params = params.to_dict();
        if let Some(Typed::Str(cube_hex)) = params.remove(&CUBE_PARAM_KEY) {
            if let Some(cost) = self.cost_map.get(&cube_hex.string) {
                let user_info = user.user()?;
                let bytes = hex::decode(cube_hex.string).unwrap();
                user_info.currency_debit(oj_rc_core::persist::user::CurrencyType::TechPoints, *cost as u64).await?;
                user_info.unlock_parts(&[u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])]).await?;
                let unlocked = user_info.unlocked_parts().await;
                let nodes = self.nodes.tech_tree_nodes(&unlocked.into_iter().collect());
                params.insert(NODES_PARAM_KEY, nodes);
            }
        }
        Ok(params.into())
    }
}

pub(super) fn tech_tree_cube_unlock_provider<C: Send + 'static>(cubes: &oj_rc_core::ConfigImpl) -> SimpleOpImpl<C, crate::UserTy, TechTreeUnlocker> {
    SimpleOpImpl::new(TechTreeUnlocker {
        cost_map: <oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::tech_tree_costs(cubes),
        nodes: <oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::tech_tree_nodes(cubes),
    })
}
