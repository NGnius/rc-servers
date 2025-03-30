use polariton::{operation::Typed, serdes::TypePrefix};

pub struct VoteThresholdData {
    pub name: String,
    pub localised_name: String,
    pub color: String,
    pub votes_required: i32,
}

impl VoteThresholdData {
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        Typed::Dict(polariton::operation::Dict {
            key_ty: TypePrefix::Str,
            val_ty: TypePrefix::Any,
            items: vec![
                (Typed::Str("name".into()), Typed::Str(self.name.clone().into())),
                (Typed::Str("localisedName".into()), Typed::Str(self.localised_name.clone().into())),
                (Typed::Str("color".into()), Typed::Str(self.color.clone().into())),
                (Typed::Str("votesRequired".into()), Typed::Int(self.votes_required)),
            ]
        })
    }
}

pub enum Vote {
    BestPlayed = 0,
    BestLooking = 1,
}

impl Vote {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BestPlayed => "BestPlayed",
            Self::BestLooking => "BestLooking",
        }
    }
}
