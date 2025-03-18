use polariton::operation::Typed;

pub trait ConfigProvider<C> {
    fn cube_list(&self) -> Typed<C>;
    fn movement_list(&self) -> Typed<C>;
    fn weapon_list(&self) -> Typed<C>;
    fn weapon_upgrade_list(&self) -> Typed<C>;
    fn tech_tree_nodes(&self, unlocked_cubes: &std::collections::HashSet<u32>) -> Typed<C>;
    fn ids(&self) -> Vec<u32>;
    fn regen_config(&self) -> Typed<C>;
    fn after_battle_vote_config(&self) -> Typed<C>;
    fn game_mode_config(&self) -> Typed<C>;
}
