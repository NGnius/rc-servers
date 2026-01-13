use polariton::operation::{Typed, Dict};
use polariton::serdes::TypePrefix;

pub struct TechTreeNodeProvider {
    // the order of this probably doesn't matter but for consistency... let's do it this way
    pub(super) tree: indexmap::IndexMap<u32, crate::persist::TechTreeData>,
}

impl TechTreeNodeProvider {
    pub fn tech_tree_nodes<C>(&self, unlocked_cubes: &std::collections::HashSet<u32>) -> Typed<C> {
        let mut seen_cubes = std::collections::HashSet::with_capacity(self.tree.len());
        let mut needed_cubes = std::collections::HashSet::with_capacity(self.tree.len());
        let mut typed_nodes = Vec::new();
        for (cube_id, tree_data) in self.tree.iter() {
            let is_unlocked = unlocked_cubes.contains(cube_id);
            let is_unlockable = tree_data.requires.iter().all(|id| unlocked_cubes.contains(id));
            tree_data.neighbours.iter().for_each(|id| { needed_cubes.insert(*id); });
            seen_cubes.insert(cube_id);
            let node_data = tree_data.to_owned().into_data(*cube_id, is_unlocked, is_unlockable);
            typed_nodes.push(node_data.as_transmissible_key_val());
        }
        for needed_cube_id in needed_cubes {
            if !seen_cubes.contains(&needed_cube_id) {
                log::warn!("Tech tree needs cube {} but it doesn't have tree info", needed_cube_id);
            }
        }
        Typed::Dict(Dict {
            key_ty: TypePrefix::Str,
            val_ty: TypePrefix::HashMap,
            items: typed_nodes,
        })
    }
}
