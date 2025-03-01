use polariton::operation::{Typed, Dict, Arr};

pub struct PlayerRankStaticInfo {
    pub sub_rank_thresholds: Vec<i32>,
}

impl PlayerRankStaticInfo {
    pub fn as_transmissible(&self) -> Typed {
        Typed::Dict(Dict {
            key_ty: 115, // str
            val_ty: 42, // obj
            items: vec![
                (Typed::Str("subRankCount".into()), Typed::Int(self.sub_rank_thresholds.len() as i32)),
                (Typed::Str("subRankThresholds".into()), Typed::Arr(Arr {
                    ty: 105, // int
                    items: self.sub_rank_thresholds.iter().map(|x| Typed::Int(*x)).collect(),
                })),
            ],
        })
    }
}
