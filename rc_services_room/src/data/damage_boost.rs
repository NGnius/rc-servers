use polariton::operation::{Typed, Dict};
use polariton::serdes::TypePrefix;

pub struct DamageBoostData {
    pub damage_map: Vec<(u32, f32)>, // (cpu, boost)
}

impl DamageBoostData {
    pub fn as_transmissible(&self) -> Typed {
        Typed::Dict(Dict {
            key_ty: TypePrefix::Str,
            val_ty: TypePrefix::Any,
            items: self.damage_map.iter()
                .map(|(cpu, boost)| (Typed::Str(cpu.to_string().into()), Typed::Float(*boost)))
                .collect(),
        })
    }
}
