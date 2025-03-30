use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct TechTreeData {
    pub position_x: i32,
    pub position_y: i32,
    pub tech_points: u32,
    pub neighbours: Vec<u32>, // cube IDs
    pub requires: Vec<u32>,
}

impl TechTreeData {
    pub fn into_data(self, self_id: u32, self_is_unlocked: bool, self_is_unlockable: bool) -> crate::data::tech_tree::TechTreeNode {
        crate::data::tech_tree::TechTreeNode {
            main_cube_id: self_id as i32,
            position_x: self.position_x,
            position_y: self.position_y,
            is_unlocked: self_is_unlocked,
            is_unlockable: self_is_unlockable,
            tech_points: self.tech_points,
            neighbours: self.neighbours.into_iter().map(|x| x as i32).collect(),
        }
    }
}
