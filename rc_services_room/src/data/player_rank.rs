use polariton::operation::{Typed, Dict, Arr};
use polariton::serdes::TypePrefix;

pub struct PlayerRankStaticInfo {
    pub sub_rank_thresholds: Vec<i32>,
}

impl PlayerRankStaticInfo {
    pub fn as_transmissible(&self) -> Typed {
        Typed::Dict(Dict {
            key_ty: TypePrefix::Str,
            val_ty: TypePrefix::Any,
            items: vec![
                (Typed::Str("subRankCount".into()), Typed::Int(self.sub_rank_thresholds.len() as i32)),
                (Typed::Str("subRankThresholds".into()), Typed::Arr(Arr {
                    ty: TypePrefix::Int, // int
                    items: self.sub_rank_thresholds.iter().map(|x| Typed::Int(*x)).collect(),
                })),
            ],
        })
    }
}
