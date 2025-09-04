use polariton::operation::{Typed, Dict};
use polariton::serdes::TypePrefix;

pub struct PlayerRoboPassSeasonInfo {
    pub delta_xp_to_show: i32,
    pub grade: i32,
    pub has_deluxe: bool,
    pub progress_in_grade: f32,
    pub xp_from_start: i32,
}

impl PlayerRoboPassSeasonInfo {
    pub fn as_transmissible(&self) -> Typed {
        Typed::Dict(Dict {
            key_ty: TypePrefix::Str,
            val_ty: TypePrefix::Any,
            items: vec![
                (Typed::Str("deltaXpToShow".into()), Typed::Int(self.delta_xp_to_show)),
                (Typed::Str("grade".into()), Typed::Int(self.grade)),
                (Typed::Str("hasDeluxe".into()), Typed::Bool(self.has_deluxe)),
                (Typed::Str("progressInGrade".into()), Typed::Float(self.progress_in_grade)),
                (Typed::Str("xpFromSeasonStart".into()), Typed::Int(self.xp_from_start)),
            ],
        })
    }
}
