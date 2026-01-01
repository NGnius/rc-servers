use polariton_server::operations::SimpleFunc;
use polariton::operation::ParameterTable;

use crate::data::item_shop_bundle::*;

const PARAM_KEY: u8 = 65;

pub(super) fn item_bundle_provider() -> SimpleFunc<188, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(PARAM_KEY, ItemShopBundle::as_transmissible_vec(vec![
            // weekly (top row of 3)
            ItemShopBundle {
                sku: "buy cc 100".to_owned(),
                bundle_name_key: "strRealMoneyStoreName_CosmeticCredits1".to_owned(),
                sprite: "ItemShop_CosmeticCredits".to_owned(),
                is_sprite_full_size: false,
                category: ItemShopCategory::Bundle,
                currency: CurrencyType::Robits,
                price: 100_000,
                discount_time: 0,
                discount_price: 100_000,
                recurrence: ItemShopRecurrence::Weekly,
                owns_required_cube: true,
                is_limited_edition: false,
            },
            ItemShopBundle {
                sku: "buy robopass 1 1".to_owned(),
                bundle_name_key: "strRealMoneyStoreName_RoboPass".to_owned(),
                sprite: "Store_RoboPass_Season2".to_owned(),
                is_sprite_full_size: true,
                category: ItemShopCategory::Bundle,
                currency: CurrencyType::CosmeticCredits,
                price: 10_000_000,
                discount_time: 0,
                discount_price: 10_000_000,
                recurrence: ItemShopRecurrence::Weekly,
                owns_required_cube: false,
                is_limited_edition: true,
            },
            ItemShopBundle {
                sku: "buy robit 100000".to_owned(),
                bundle_name_key: "strRealMoneyStoreName_RobitsBundle2".to_owned(),
                sprite: "ItemShop_Robits".to_owned(),
                is_sprite_full_size: false,
                category: ItemShopCategory::Bundle,
                currency: CurrencyType::CosmeticCredits,
                price: 100,
                discount_time: 0,
                discount_price: 100,
                recurrence: ItemShopRecurrence::Weekly,
                owns_required_cube: true,
                is_limited_edition: false,
            },
            // daily (lower row or 6)
            ItemShopBundle {
                sku: "buy robopass 1 1".to_owned(),
                bundle_name_key: "strRoboPassSeason02".to_owned(),
                sprite: "Store_RoboPass".to_owned(),
                is_sprite_full_size: true,
                category: ItemShopCategory::Bundle,
                currency: CurrencyType::Robits,
                price: 10_000,
                discount_time: (chrono::Utc::now() + std::time::Duration::from_secs(24*60*60)).timestamp(),
                discount_price: 1_000,
                recurrence: ItemShopRecurrence::Daily,
                owns_required_cube: false,
                is_limited_edition: false,
            },
            ItemShopBundle {
                sku: "buy robopass 1 2".to_owned(),
                bundle_name_key: "strRoboPassSeason02".to_owned(),
                sprite: "Store_RoboPass".to_owned(),
                is_sprite_full_size: true,
                category: ItemShopCategory::Cube,
                currency: CurrencyType::Robits,
                price: 10_000,
                discount_time: (chrono::Utc::now() + std::time::Duration::from_secs(24*60*60)).timestamp(),
                discount_price: 2_000,
                recurrence: ItemShopRecurrence::Daily,
                owns_required_cube: false,
                is_limited_edition: false,
            },
            ItemShopBundle {
                sku: "buy robopass 1 3".to_owned(),
                bundle_name_key: "strRoboPassSeason02".to_owned(),
                sprite: "Store_RoboPass".to_owned(),
                is_sprite_full_size: true,
                category: ItemShopCategory::Cube,
                currency: CurrencyType::Robits,
                price: 10_000,
                discount_time: (chrono::Utc::now() + std::time::Duration::from_secs(24*60*60)).timestamp(),
                discount_price: 3_000,
                recurrence: ItemShopRecurrence::Daily,
                owns_required_cube: false,
                is_limited_edition: false,
            },
            ItemShopBundle {
                sku: "buy robopass 1 4".to_owned(),
                bundle_name_key: "strRoboPassSeason02".to_owned(),
                sprite: "Store_RoboPass".to_owned(),
                is_sprite_full_size: true,
                category: ItemShopCategory::Cube,
                currency: CurrencyType::Robits,
                price: 10_000,
                discount_time: (chrono::Utc::now() + std::time::Duration::from_secs(24*60*60)).timestamp(),
                discount_price: 4_000,
                recurrence: ItemShopRecurrence::Daily,
                owns_required_cube: false,
                is_limited_edition: false,
            },
            ItemShopBundle {
                sku: "buy robopass 1 5".to_owned(),
                bundle_name_key: "strRoboPassSeason02".to_owned(),
                sprite: "Store_RoboPass".to_owned(),
                is_sprite_full_size: true,
                category: ItemShopCategory::Cube,
                currency: CurrencyType::Robits,
                price: 10_000,
                discount_time: (chrono::Utc::now() + std::time::Duration::from_secs(24*60*60)).timestamp(),
                discount_price: 5_000,
                recurrence: ItemShopRecurrence::Daily,
                owns_required_cube: false,
                is_limited_edition: false,
            },
            ItemShopBundle {
                sku: "buy robopass 1 6".to_owned(),
                bundle_name_key: "strRoboPassSeason02".to_owned(),
                sprite: "Store_RoboPass".to_owned(),
                is_sprite_full_size: true,
                category: ItemShopCategory::Cube,
                currency: CurrencyType::Robits,
                price: 10_000,
                discount_time: (chrono::Utc::now() + std::time::Duration::from_secs(24*60*60)).timestamp(),
                discount_price: 6_000,
                recurrence: ItemShopRecurrence::Daily,
                owns_required_cube: false,
                is_limited_edition: false,
            },
        ]));
        Ok(params.into())
    })
}

/*

Allowed sprites:

Emote_Chicken, Emote_CrayWave, Emote_FacePalm, Emote_Heart, Emote_LOL, Emote_ThumbsDown, Emote_ThumbsUp, Explosion, Explosion_BlackHole, Explosion_Feathers_Rainbow, Explosion_Firework, Explosion_Nuclear, Explosion_Toon, Explosion_Warp, ItemShop_2019_Holoflag, ItemShop_6_Years_Holoflag, ItemShop_AbleGamers_Holoflag, ItemShop_Alienware_Holoflag, ItemShop_Altimeter, ItemShop_Balloon, ItemShop_Bubble_Blower, ItemShop_Bundle_Alienware_Mask, ItemShop_Bundle_CatEye, ItemShop_Bundle_Cockpit_Mask, ItemShop_Bundle_Eagle_Mask, ItemShop_Bundle_Exhaust_Stack, ItemShop_Bundle_Flipper, ItemShop_Bundle_Football_Mask, ItemShop_Bundle_Football_Plate, ItemShop_Bundle_Glass_Cubes, ItemShop_Bundle_Honeydew_Mask, ItemShop_Bundle_Jammer, ItemShop_Bundle_Mech_7_Mask, ItemShop_Bundle_Ninja_Mask, ItemShop_Bundle_OverWolf_Mask, ItemShop_Bundle_Radar, ItemShop_Bundle_Receiver, ItemShop_Bundle_Rhino_8_Mask, ItemShop_Bundle_Sabretooth_Mask, ItemShop_Bundle_Scary_Mask, ItemShop_Bundle_Spike, ItemShop_Bundle_Spiked_Plate, ItemShop_Bundle_TRex_Mask, ItemShop_Bundle_Vigilant_Eyes, ItemShop_Bunny_Holoflag, ItemShop_Candy_Cane_Holoflag, ItemShop_ChronoGG_Holoflag, ItemShop_CosmeticCredits, ItemShop_Country_Holoflag_Bundle, ItemShop_Cube_with_C6_Logo, ItemShop_Cube_with_CARBON_Letters, ItemShop_Curse_Holoflag, ItemShop_Cyborg_Eyes, ItemShop_Dev_Supporter_Gold_Holoflag, ItemShop_Dev_Supporter_Holoflag, ItemShop_EasterEgg_Holoflag, ItemShop_Exhaust_Blower, ItemShop_Fairy_Light, ItemShop_Headlamp, ItemShop_Humble_Bundle_Holoflag, ItemShop_Insect_Leg_Huge_Spider, ItemShop_Insect_Leg_Large_Overwolf, ItemShop_Mega_Laser_Carbon_6, ItemShop_Mega_Plasma_Carbon_6, ItemShop_Mothership_Earth, ItemShop_Mothership_Mars, ItemShop_Mothership_Neptune, ItemShop_Mothership_Retro, ItemShop_Neon_Cone, ItemShop_Neon_Corner, ItemShop_Neon_Corner_Round, ItemShop_Neon_Corner_Slope, ItemShop_Neon_Cube, ItemShop_Neon_Cube_Bundle, ItemShop_Neon_Edge, ItemShop_Neon_Edge_Round, ItemShop_Neon_Edge_Slope, ItemShop_Neon_Inner, ItemShop_Neon_Inner_Round, ItemShop_Neon_Inner_Slope, ItemShop_Neon_Pyramid, ItemShop_Nyan_Cray_Holoflag, ItemShop_Overwolf_Holoflag, ItemShop_Pilot_Seat_Bunny, ItemShop_Pilot_Seat_Cray, ItemShop_Pilot_Seat_Gene, ItemShop_Pilot_Seat_Mega, ItemShop_Pilot_Seat_Retro, ItemShop_Pirate_Holoflag, ItemShop_Plasma_Huge_Carbon_6, ItemShop_Present_Large, ItemShop_Present_Small, ItemShop_Retro_Corner, ItemShop_Retro_Cube, ItemShop_Retro_Edge, ItemShop_Retro_Inner, ItemShop_Robits, ItemShop_Robocraft_Holoflag, ItemShop_RoboPass_Season_1_Holoflag, ItemShop_RoboPass_Season_2_Holoflag, ItemShop_Robot_Name_Banner, ItemShop_Rod_Long_Spring, ItemShop_Rod_Short_Spring, ItemShop_Rod_Spring, ItemShop_Rudder_Bat, ItemShop_Rudder_Vampire, ItemShop_Santa_Cray_Holoflag, ItemShop_Snowflake_Holoflag, ItemShop_Speedometer, ItemShop_Sprinter_Leg_Large_Carbon_6, ItemShop_T5_Mortar_Egg, ItemShop_T5_Seeker_Firework_Steam, ItemShop_Thruster_Huge_Carbon_6, ItemShop_Top_Laser_Huge_Carbon_6, ItemShop_Vapor_Trail_Single, ItemShop_Vapor_Trail_Single_Firework, ItemShop_Vapor_Trail_Single_Flower, ItemShop_Vapor_Trail_Single_Snowflake, ItemShop_Vapor_Trail_Twin, ItemShop_Wing_Bat, ItemShop_Wing_Vampire, ItemShop_Yogscast_Holoflag, Spawn, Spawn_BlackHole, Spawn_EasterEgg, Spawn_Lander, Spawn_Lootcrate, Spawn_Present, Spawn_Warp, Store_RoboPass, Store_RoboPass_Season2

*/
