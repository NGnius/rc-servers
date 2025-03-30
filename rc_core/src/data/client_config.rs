use polariton::operation::Typed;

pub struct GameplaySettings {
    pub show_tutorial_after_date: String,
    pub health_threshold: f32, // percent
    pub microbot_sphere: f32, // radius
    pub misfire_angle: f32, // degrees?
    pub shield_dps: i32,
    pub shield_hps: u32,
    pub request_review_level: u32,
    pub critical_ratio: f32,
    pub cross_promo_image: String, // url
    pub cross_promo_link: String, // url
}

impl GameplaySettings {
    pub fn as_transmissible<C>(&self) -> Typed<C> {
        Typed::HashMap(vec![
            // TODO
            (Typed::Str("showTutorialAfterDate".into()), Typed::Str(self.show_tutorial_after_date.clone().into())),
            (Typed::Str("healthTresholdPercent".into()), Typed::Float(self.health_threshold)),
            (Typed::Str("microbotSphereRadius".into()), Typed::Float(self.microbot_sphere)),
            (Typed::Str("smartRotationMisfireAngle".into()), Typed::Float(self.misfire_angle)),
            (Typed::Str("fusionShieldDPS".into()), Typed::Int(self.shield_dps)),
            (Typed::Str("fusionShieldHPS".into()), Typed::Int(self.shield_hps as i32)),
            (Typed::Str("requestReviewAtLevel".into()), Typed::Int(self.request_review_level as i32)),
            (Typed::Str("criticalRatio".into()), Typed::Float(self.critical_ratio)),
            (Typed::Str("crossPromotionAdImageUrl".into()), Typed::Str(self.cross_promo_image.clone().into())),
            (Typed::Str("crossPromotionAdLinkUrl".into()), Typed::Str(self.cross_promo_link.clone().into())),
        ].into())
    }
}
