use polariton::operation::Typed;

pub struct CosmeticLimitsData {
    pub others_max_holo_and_trails: u32,
    pub others_max_headlamps: u32,
    pub others_max_cosmetic_items_with_particles: u32,
}

impl CosmeticLimitsData {
    pub fn as_transmissible(&self) -> Typed {
        Typed::HashMap(vec![
                (Typed::Str("OthersMaxNumberHoloAndTrails".into()), Typed::Int(self.others_max_holo_and_trails as i32)),
                (Typed::Str("OthersMaxNumberHeadlamps".into()), Typed::Int(self.others_max_headlamps as i32)),
                (Typed::Str("OthersMaxCosmeticItemsWithParticleSystem".into()), Typed::Int(self.others_max_cosmetic_items_with_particles as i32)),
            ].into()
        )
    }
}
