use polariton::operation::Typed;

pub struct RobotShopConfig {
    pub cpu_ranges: Vec<i32>,
    pub submission_mult: f32,
    pub earnings_mult: f32,
}

impl RobotShopConfig {
     pub fn as_transmissible(&self) -> Typed {
        Typed::HashMap(vec![
            (Typed::Str("robotShopPriceRanges".into()), Typed::IntArr(self.cpu_ranges.clone().into())),
            (Typed::Str("submissionMultiplier".into()), Typed::Float(self.submission_mult)),
            (Typed::Str("earningsMultiplier".into()), Typed::Float(self.earnings_mult)),
        ].into())
     }
}
