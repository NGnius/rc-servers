use polariton::operation::{Typed, Dict};

pub struct DamageBoostData {
    pub damage_map: Vec<(u32, f32)>, // (cpu, boost)
}

impl DamageBoostData {
    pub fn as_transmissible(&self) -> Typed {
        Typed::Dict(Dict {
            key_ty: 115, // str
            val_ty: 42, // obj
            items: self.damage_map.iter()
                .map(|(cpu, boost)| (Typed::Str(cpu.to_string().into()), Typed::Float(*boost)))
                .collect(),
        })
    }
}
