use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ItemShopConfig {
    #[serde(default = "default_items")]
    pub items: Vec<ItemBundle>,
    #[serde(default = "default_codes")]
    pub promo_codes: std::collections::HashMap<String, ItemCode>,
}

impl super::config::SelfValidator for ItemShopConfig {
    type Context = crate::ConfigImpl;
    fn validate(&self, info: &mut super::config::ValidationInfo, ctx: &Self::Context) -> bool {
        let mut is_ok = true;
        let daily_count = self.items.iter().filter(|x| matches!(x.recurrence, Recurrence::Daily)).count();
        if daily_count < 6 {
            info.warn(crate::persist::config::ValidationMessage {
                path: vec!["items".to_owned()],
                message: "Less than 6 daily items in shop so there will be blank slots".to_owned(),
            });
        } else if daily_count > 6 {
            info.warn(crate::persist::config::ValidationMessage {
                path: vec!["items".to_owned()],
                message: "More than 6 daily items in shop so some will never be shown".to_owned(),
            });
        }
        let weekly_count = self.items.iter().filter(|x| matches!(x.recurrence, Recurrence::Weekly)).count();
        if weekly_count < 3 {
            info.warn(crate::persist::config::ValidationMessage {
                path: vec!["items".to_owned()],
                message: "Less than 3 weekly items in shop so there will be blank slots".to_owned(),
            });
        } else if weekly_count > 3 {
            info.warn(crate::persist::config::ValidationMessage {
                path: vec!["items".to_owned()],
                message: "More than 3 weekly items in shop so some will never be shown".to_owned(),
            });
        }
        for (i, item) in self.items.iter().enumerate() {
            is_ok &= item.validate_in(info, ctx, &format!("items[{}]", i));
        }
        for (code_name, item_code) in self.promo_codes.iter() {
            is_ok &= item_code.validate_in(info, ctx, &format!("promo_codes[\"{}\"]", code_name));
        }
        // TODO
        is_ok
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ItemBundle {
    //pub sku: String,
    #[serde(alias = "localized_key")]
    pub localised_key: String,
    pub sprite: String,
    pub is_sprite_full_size: bool,
    pub category: ShopCategory,
    pub currency: Currency,
    pub price: i32,
    #[serde(default)]
    pub discount_until: i64, // seconds since unix epoch
    #[serde(default)]
    pub discount_price: i32,
    pub recurrence: Recurrence,
    //pub owns_required_cube: bool,
    pub is_limited_edition: bool,
    #[serde(default)]
    pub required_cubes: Vec<u32>,
    pub gives: Vec<ItemPurchase>,
}

impl ItemBundle {
    pub fn as_data(&self, sku: String, unlocked_cubes: &[u32]) -> crate::data::item_shop_bundle::ItemShopBundle {
        let mut contains_all = true;
        for req in self.required_cubes.iter() {
            contains_all &= unlocked_cubes.contains(req);
        }
        crate::data::item_shop_bundle::ItemShopBundle {
            sku,
            bundle_name_key: self.localised_key.clone(),
            sprite: self.sprite.clone(),
            is_sprite_full_size: self.is_sprite_full_size,
            category: self.category.into(),
            currency: self.currency.into(),
            price: self.price,
            discount_time: self.discount_until,
            discount_price: self.discount_price,
            recurrence: self.recurrence.into(),
            owns_required_cube: contains_all,
            is_limited_edition: self.is_limited_edition,
        }
    }
}

impl super::config::SelfValidator for ItemBundle {
    type Context = crate::ConfigImpl;
    fn validate(&self, info: &mut super::config::ValidationInfo, ctx: &Self::Context) -> bool {
        for (i, give) in self.gives.iter().enumerate() {
            give.validate_in(info, ctx, &format!("gives[{}]", i));
        }
        true
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum ShopCategory {
    Cube,
    GarageBaySkin,
    Bundle,
    DeathEffect,
    SpawnEffect,
    Emotigram,
}

impl std::convert::From<ShopCategory> for crate::data::item_shop_bundle::ItemShopCategory {
    fn from(value: ShopCategory) -> Self {
        match value {
            ShopCategory::Cube => Self::Cube,
            ShopCategory::GarageBaySkin => Self::GarageBaySkin,
            ShopCategory::Bundle => Self::Bundle,
            ShopCategory::DeathEffect => Self::DeathEffect,
            ShopCategory::SpawnEffect => Self::SpawnEffect,
            ShopCategory::Emotigram => Self::Emotigram,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Recurrence {
    Daily,
    Weekly,
}

impl std::convert::From<Recurrence> for crate::data::item_shop_bundle::ItemShopRecurrence {
    fn from(value: Recurrence) -> Self {
        match value {
            Recurrence::Daily => Self::Daily,
            Recurrence::Weekly => Self::Weekly,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Currency {
    Robits,
    CosmeticCredits,
}

impl std::convert::From<Currency> for crate::data::item_shop_bundle::CurrencyType {
    fn from(value: Currency) -> Self {
        match value {
            Currency::Robits => Self::Robits,
            Currency::CosmeticCredits => Self::CosmeticCredits,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ItemPurchase {
    Cube {
        item_id: u32,
    },
    Experience {
        #[serde(alias = "experience", alias = "exp")]
        xp: i64,
    },
    FreeCurrency {
        #[serde(alias = "robits")]
        free_currency: i64,
    },
    PaidCurrency {
        #[serde(alias = "cc", alias = "cosmetic_credits")]
        paid_currency: i64,
    },
    TechPoints {
        #[serde(alias = "tp")]
        tech_points: i64,
    }
}

impl std::convert::From<ItemPurchase> for crate::persist::config::ShopGain {
    fn from(value: ItemPurchase) -> Self {
        match value {
            ItemPurchase::Cube { item_id } => crate::persist::config::ShopGain::Cube(item_id),
            ItemPurchase::Experience { xp } => crate::persist::config::ShopGain::Experience(xp),
            ItemPurchase::FreeCurrency { free_currency } => crate::persist::config::ShopGain::FreeCurrency(free_currency),
            ItemPurchase::PaidCurrency { paid_currency } => crate::persist::config::ShopGain::PaidCurrency(paid_currency),
            ItemPurchase::TechPoints { tech_points } => crate::persist::config::ShopGain::PaidCurrency(tech_points),
        }
    }
}

impl super::config::SelfValidator for ItemPurchase {
    type Context = crate::ConfigImpl;
    fn validate(&self, info: &mut super::config::ValidationInfo, _ctx: &Self::Context) -> bool {
        if matches!(self, Self::Experience { xp: _ }) {
            info.warn(crate::persist::config::ValidationMessage {
                path: vec!["xp".to_owned()],
                message: "Experience rewards cause (client-side only) UI desync of the XP bar".to_owned(),
            });
        }
        true
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ItemCode {
    #[serde(default)]
    pub message: Option<String>,
    pub bundle_id: Option<String>,
    pub promo_id: Option<String>,
    #[serde(default)]
    pub is_serial: bool,
    #[serde(default)]
    pub is_repeatable: bool,
    #[serde(default)]
    pub value: f32,
    pub gives: Vec<ItemPurchase>,
}

impl super::config::SelfValidator for ItemCode {
    type Context = crate::ConfigImpl;
    fn validate(&self, info: &mut super::config::ValidationInfo, ctx: &Self::Context) -> bool {
        for (i, give) in self.gives.iter().enumerate() {
            give.validate_in(info, ctx, &format!("gives[{}]", i));
        }
        true
    }
}

pub fn default_items() -> Vec<ItemBundle> {
    vec![
        // weekly (top row of 3)
        ItemBundle {
            //sku: "buy cc 100".to_owned(),
            localised_key: "strRealMoneyStoreName_CosmeticCredits1".to_owned(),
            sprite: "ItemShop_CosmeticCredits".to_owned(),
            is_sprite_full_size: false,
            category: ShopCategory::Bundle,
            currency: Currency::Robits,
            price: 100_000,
            discount_until: 0,
            discount_price: 100_000,
            recurrence: Recurrence::Weekly,
            //owns_required_cube: true,
            is_limited_edition: false,
            required_cubes: vec![],
            gives: vec![
                ItemPurchase::PaidCurrency { paid_currency: 100 },
            ],
        },
        ItemBundle {
            //sku: "buy robopass 1 1".to_owned(),
            localised_key: "strRealMoneyStoreName_RoboPass".to_owned(),
            sprite: "Store_RoboPass_Season2".to_owned(),
            is_sprite_full_size: true,
            category: ShopCategory::Bundle,
            currency: Currency::CosmeticCredits,
            price: 10_000_000,
            discount_until: 0,
            discount_price: 10_000_000,
            recurrence: Recurrence::Weekly,
            //owns_required_cube: false,
            is_limited_edition: true,
            required_cubes: vec![],
            gives: vec![],
        },
        ItemBundle {
            //sku: "buy robit 100000".to_owned(),
            localised_key: "strRealMoneyStoreName_RobitsBundle2".to_owned(),
            sprite: "ItemShop_Robits".to_owned(),
            is_sprite_full_size: false,
            category: ShopCategory::Bundle,
            currency: Currency::CosmeticCredits,
            price: 100,
            discount_until: 0,
            discount_price: 100,
            recurrence: Recurrence::Weekly,
            //owns_required_cube: true,
            is_limited_edition: false,
            required_cubes: vec![],
            gives: vec![
                ItemPurchase::FreeCurrency { free_currency: 100_000 },
            ],
        },
        // daily (lower row or 6)
        ItemBundle {
            //sku: "buy robopass 1 1".to_owned(),
            localised_key: "strRoboPassSeason02".to_owned(),
            sprite: "Store_RoboPass".to_owned(),
            is_sprite_full_size: true,
            category: ShopCategory::Bundle,
            currency: Currency::Robits,
            price: 10_000,
            discount_until: (chrono::Utc::now() + std::time::Duration::from_secs(24*60*60)).timestamp(),
            discount_price: 1_000,
            recurrence: Recurrence::Daily,
            //owns_required_cube: false,
            is_limited_edition: false,
            required_cubes: vec![],
            gives: vec![],
        },
        ItemBundle {
            //sku: "buy robopass 1 2".to_owned(),
            localised_key: "strRoboPassSeason02".to_owned(),
            sprite: "Store_RoboPass".to_owned(),
            is_sprite_full_size: true,
            category: ShopCategory::Cube,
            currency: Currency::Robits,
            price: 10_000,
            discount_until: (chrono::Utc::now() + std::time::Duration::from_secs(24*60*60)).timestamp(),
            discount_price: 2_000,
            recurrence: Recurrence::Daily,
            //owns_required_cube: false,
            is_limited_edition: false,
            required_cubes: vec![],
            gives: vec![],
        },
        ItemBundle {
            //sku: "buy robopass 1 3".to_owned(),
            localised_key: "strRoboPassSeason02".to_owned(),
            sprite: "Store_RoboPass".to_owned(),
            is_sprite_full_size: true,
            category: ShopCategory::Cube,
            currency: Currency::Robits,
            price: 10_000,
            discount_until: (chrono::Utc::now() + std::time::Duration::from_secs(24*60*60)).timestamp(),
            discount_price: 3_000,
            recurrence: Recurrence::Daily,
            //owns_required_cube: false,
            is_limited_edition: false,
            required_cubes: vec![],
            gives: vec![],
        },
        ItemBundle {
            //sku: "buy robopass 1 4".to_owned(),
            localised_key: "strRoboPassSeason02".to_owned(),
            sprite: "Store_RoboPass".to_owned(),
            is_sprite_full_size: true,
            category: ShopCategory::Cube,
            currency: Currency::Robits,
            price: 10_000,
            discount_until: (chrono::Utc::now() + std::time::Duration::from_secs(24*60*60)).timestamp(),
            discount_price: 4_000,
            recurrence: Recurrence::Daily,
            //owns_required_cube: false,
            is_limited_edition: false,
            required_cubes: vec![],
            gives: vec![],
        },
        ItemBundle {
            //sku: "buy robopass 1 5".to_owned(),
            localised_key: "strRoboPassSeason02".to_owned(),
            sprite: "Store_RoboPass".to_owned(),
            is_sprite_full_size: true,
            category: ShopCategory::Cube,
            currency: Currency::Robits,
            price: 10_000,
            discount_until: (chrono::Utc::now() + std::time::Duration::from_secs(24*60*60)).timestamp(),
            discount_price: 5_000,
            recurrence: Recurrence::Daily,
            //owns_required_cube: false,
            is_limited_edition: false,
            required_cubes: vec![],
            gives: vec![],
        },
        ItemBundle {
            //sku: "buy robopass 1 6".to_owned(),
            localised_key: "strRoboPassSeason02".to_owned(),
            sprite: "Store_RoboPass".to_owned(),
            is_sprite_full_size: true,
            category: ShopCategory::Cube,
            currency: Currency::Robits,
            price: 10_000,
            discount_until: (chrono::Utc::now() + std::time::Duration::from_secs(24*60*60)).timestamp(),
            discount_price: 6_000,
            recurrence: Recurrence::Daily,
            //owns_required_cube: false,
            is_limited_edition: false,
            required_cubes: vec![],
            gives: vec![],
        },
    ]
}

pub fn default_codes() -> std::collections::HashMap<String, ItemCode> {
    let mut map = std::collections::HashMap::new();
    map.insert("TEST".to_owned(), ItemCode {
        message: Some("Test passed".to_owned()),
        bundle_id: None,
        promo_id: None,
        is_serial: false,
        is_repeatable: true,
        value: 1.5,
        gives: vec![]
    });
    map.insert("TEST-ONCE".to_owned(), ItemCode {
        message: Some("Test passed".to_owned()),
        bundle_id: None,
        promo_id: None,
        is_serial: false,
        is_repeatable: false,
        value: 1.5,
        gives: vec![]
    });
    map.insert("LEVEL10".to_owned(), ItemCode {
        message: Some("Please re-log".to_owned()),
        bundle_id: None,
        promo_id: None,
        is_serial: false,
        is_repeatable: false,
        value: 1.5,
        gives: vec![
            ItemPurchase::Experience {
                xp: 10_000,
            }
        ]
    });
    map
}
