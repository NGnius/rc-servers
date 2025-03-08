use polariton::{operation::{Dict, Typed}, serdes::TypePrefix};

pub struct PremiumEffects {
    pub factor: PremiumFactor,
    pub multiplayer: PremiumMultiplayer,
}

impl PremiumEffects {
    pub fn as_transmissible(&self) -> Typed {
        Typed::Dict(Dict {
            key_ty: TypePrefix::Str, // str
            val_ty: TypePrefix::HashMap, // hashtable
            items: vec![
                (Typed::Str("PremiumFactor".into()), self.factor.as_transmissible()),
                (Typed::Str("TieredMultiplayer".into()), self.multiplayer.as_transmissible()),
            ]
        })
    }
}

pub struct PremiumFactor {
    pub factor: i32, // percent
    pub party_bonus: i32, // percent
}

impl PremiumFactor {
    fn as_transmissible(&self) -> Typed {
        Typed::HashMap(vec![
            (Typed::Str("Factor".into()), Typed::Int(self.factor)),
            (Typed::Str("PartyBonusPercentagePerPlayer".into()), Typed::Int(self.party_bonus)),
        ].into())
    }
}

pub struct PremiumMultiplayer {
    pub tier_multiplier: f64,
}

impl PremiumMultiplayer {
    fn as_transmissible(&self) -> Typed {
        Typed::HashMap(vec![
            (Typed::Str("BonusPerTierMultiplier".into()), Typed::Double(self.tier_multiplier)),
        ].into())
    }
}
