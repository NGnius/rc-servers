use polariton::operation::Typed;

pub struct CpuLimitsData {
    pub premium_for_life_cosmetic_gpu: i32,
    pub premium_cosmetic_cpu: i32,
    pub no_premium_cosmetic_cpu: i32,
    pub max_regular_health: i32,
    pub max_megabot_health: i32,
}

impl CpuLimitsData {
    pub fn as_transmissible(&self) -> Typed {
        Typed::HashMap(vec![
                (Typed::Str("PremiumForLifeCosmeticCPU".into()), Typed::Int(self.premium_for_life_cosmetic_gpu)),
                (Typed::Str("PremiumCosmeticCPU".into()), Typed::Int(self.premium_cosmetic_cpu)),
                (Typed::Str("NoPremiumCosmeticCPU".into()), Typed::Int(self.no_premium_cosmetic_cpu)),
                (Typed::Str("MaxRegularHealth".into()), Typed::Int(self.max_regular_health)),
                (Typed::Str("MaxMegabotHealth".into()), Typed::Int(self.max_megabot_health)),
            ].into()
        )
    }
}
