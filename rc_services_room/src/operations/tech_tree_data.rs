use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::ParameterTable;

const CODE: u8 = 183;

const NODES_PARAM_KEY: u8 = 210;

/*pub(super) fn tech_tree_layout_provider(cubes: &oj_rc_core::ConfigImpl) -> Immediate<183, crate::UserTy> {
    Immediate::new(|| {
        let mut params = std::collections::HashMap::with_capacity(2);
        params.insert(NODES_PARAM_KEY, cubes.tech_tree_nodes(&vec![
            227205318,
            227917916,
            1931676396,
        ].into_iter().collect()));
        /*params.insert(NODES_PARAM_KEY, Typed::Dict(Dict {
            key_ty: TypePrefix::Str, // str
            val_ty: TypePrefix::HashMap, // hashmap
            items: vec![
                TechTreeNode {
                    main_cube_id: 227205318, // default cube id
                    position_x: 0,
                    position_y: 0,
                    is_unlocked: true,
                    is_unlockable: false,
                    tech_points: 1,
                    neighbours: Vec::default(),
                }.as_transmissible_key_val(),
            ],
        }));*/
        params.into()
    })
}*/

pub(super) struct TechTreeNoder {
    nodes: oj_rc_core::persist::config::TechTreeNodeProvider,
}

#[async_trait::async_trait]
impl <C: Send + 'static> SimpleOperation<C> for TechTreeNoder {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, params: ParameterTable<C>, user: &Self::User) -> Result<ParameterTable<C>, SimpleOpError> {
        let mut params = params.to_dict();
        let user_info = user.user()?;
        let unlocked = user_info.unlocked_parts().await;
        let nodes = self.nodes.tech_tree_nodes(&unlocked.into_iter().collect());
        params.insert(NODES_PARAM_KEY, nodes);
        Ok(params.into())
    }
}

pub(super) fn tech_tree_layout_provider<C: Send + 'static>(cubes: &oj_rc_core::ConfigImpl) -> SimpleOpImpl<C, crate::UserTy, TechTreeNoder> {
    SimpleOpImpl::new(TechTreeNoder {
        nodes: <oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::tech_tree_nodes(cubes),
    })
}
