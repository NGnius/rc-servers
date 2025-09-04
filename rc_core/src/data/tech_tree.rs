use polariton::{operation::{Arr, Typed}, serdes::TypePrefix};

pub struct TechTreeNode {
    pub main_cube_id: i32, // hex
    pub position_x: i32,
    pub position_y: i32,
    pub is_unlocked: bool,
    pub is_unlockable: bool,
    pub tech_points: u32,
    pub neighbours: Vec<i32>, // cube IDs, hex
}

impl TechTreeNode {
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        Typed::HashMap(vec![
            (Typed::Str("mainCubeId".into()), Typed::Str(hex::encode(self.main_cube_id.to_be_bytes()).into())),
            (Typed::Str("positionX".into()), Typed::Int(self.position_x)),
            (Typed::Str("positionY".into()), Typed::Int(self.position_y)),
            (Typed::Str("isUnlocked".into()), Typed::Bool(self.is_unlocked)),
            (Typed::Str("isUnlockable".into()), Typed::Bool(self.is_unlockable)),
            (Typed::Str("tp".into()), Typed::Int(self.tech_points as i32)),
            (Typed::Str("neighbours".into()), Typed::Arr(Arr {
                ty: TypePrefix::Str, // str
                items: self.neighbours.iter().map(|cube_id| Typed::Str(hex::encode(cube_id.to_be_bytes()).into())).collect(),
            })),
        ].into())
    }

    pub fn as_transmissible_key_val<C>(&self) -> (Typed<C>, Typed<C>) {
        (Typed::Str(hex::encode(self.main_cube_id.to_be_bytes()).into()), self.as_transmissible())
    }
}
