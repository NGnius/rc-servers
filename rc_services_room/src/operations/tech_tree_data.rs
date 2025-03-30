use polariton_server::operations::Immediate;
use rc_core::ConfigProvider;

const PARAM_KEY: u8 = 210;

pub(super) fn tech_tree_layout_provider(cubes: &rc_core::ConfigImpl) -> Immediate<183, crate::UserTy> {
    Immediate::new(|| {
        let mut params = std::collections::HashMap::with_capacity(2);
        params.insert(PARAM_KEY, cubes.tech_tree_nodes(&vec![
            227205318,
            227917916,
            1931676396,
        ].into_iter().collect()));
        /*params.insert(PARAM_KEY, Typed::Dict(Dict {
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
}
