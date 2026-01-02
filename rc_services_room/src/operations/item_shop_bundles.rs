use polariton_server::operations::{SimpleOpError, SimpleOperation, SimpleOpImpl};
use polariton::operation::ParameterTable;

const CODE: u8 = 188;

const PARAM_KEY: u8 = 65;

pub(super) struct ItemBundleRetriever {
    resolver: oj_rc_core::persist::config::ShopEntriesResolver,
}

#[async_trait::async_trait]
impl SimpleOperation<()> for ItemBundleRetriever {
    type User = crate::UserTy;
    const CODE: u8 = CODE;

    async fn handle(&self, mut params: ParameterTable, user: &Self::User) -> Result<ParameterTable, SimpleOpError> {
        //let mut params = ParameterTable::<C>::with_capacity(2);
        let user_info = user.user()?;
        let entries = self.resolver.resolve_entries(user_info.as_ref().as_ref()).await;
        params.insert(PARAM_KEY, entries);
        Ok(params)
    }
}

pub(super) fn item_bundle_provider(conf: &oj_rc_core::ConfigImpl) -> SimpleOpImpl<(), crate::UserTy, ItemBundleRetriever> {
    SimpleOpImpl::new(ItemBundleRetriever {
        resolver: <oj_rc_core::ConfigImpl as oj_rc_core::ConfigProvider<()>>::shop_entries(conf),
    })
}

/*

Allowed sprites:

Emote_Chicken, Emote_CrayWave, Emote_FacePalm, Emote_Heart, Emote_LOL, Emote_ThumbsDown, Emote_ThumbsUp, Explosion, Explosion_BlackHole, Explosion_Feathers_Rainbow, Explosion_Firework, Explosion_Nuclear, Explosion_Toon, Explosion_Warp, ItemShop_2019_Holoflag, ItemShop_6_Years_Holoflag, ItemShop_AbleGamers_Holoflag, ItemShop_Alienware_Holoflag, ItemShop_Altimeter, ItemShop_Balloon, ItemShop_Bubble_Blower, ItemShop_Bundle_Alienware_Mask, ItemShop_Bundle_CatEye, ItemShop_Bundle_Cockpit_Mask, ItemShop_Bundle_Eagle_Mask, ItemShop_Bundle_Exhaust_Stack, ItemShop_Bundle_Flipper, ItemShop_Bundle_Football_Mask, ItemShop_Bundle_Football_Plate, ItemShop_Bundle_Glass_Cubes, ItemShop_Bundle_Honeydew_Mask, ItemShop_Bundle_Jammer, ItemShop_Bundle_Mech_7_Mask, ItemShop_Bundle_Ninja_Mask, ItemShop_Bundle_OverWolf_Mask, ItemShop_Bundle_Radar, ItemShop_Bundle_Receiver, ItemShop_Bundle_Rhino_8_Mask, ItemShop_Bundle_Sabretooth_Mask, ItemShop_Bundle_Scary_Mask, ItemShop_Bundle_Spike, ItemShop_Bundle_Spiked_Plate, ItemShop_Bundle_TRex_Mask, ItemShop_Bundle_Vigilant_Eyes, ItemShop_Bunny_Holoflag, ItemShop_Candy_Cane_Holoflag, ItemShop_ChronoGG_Holoflag, ItemShop_CosmeticCredits, ItemShop_Country_Holoflag_Bundle, ItemShop_Cube_with_C6_Logo, ItemShop_Cube_with_CARBON_Letters, ItemShop_Curse_Holoflag, ItemShop_Cyborg_Eyes, ItemShop_Dev_Supporter_Gold_Holoflag, ItemShop_Dev_Supporter_Holoflag, ItemShop_EasterEgg_Holoflag, ItemShop_Exhaust_Blower, ItemShop_Fairy_Light, ItemShop_Headlamp, ItemShop_Humble_Bundle_Holoflag, ItemShop_Insect_Leg_Huge_Spider, ItemShop_Insect_Leg_Large_Overwolf, ItemShop_Mega_Laser_Carbon_6, ItemShop_Mega_Plasma_Carbon_6, ItemShop_Mothership_Earth, ItemShop_Mothership_Mars, ItemShop_Mothership_Neptune, ItemShop_Mothership_Retro, ItemShop_Neon_Cone, ItemShop_Neon_Corner, ItemShop_Neon_Corner_Round, ItemShop_Neon_Corner_Slope, ItemShop_Neon_Cube, ItemShop_Neon_Cube_Bundle, ItemShop_Neon_Edge, ItemShop_Neon_Edge_Round, ItemShop_Neon_Edge_Slope, ItemShop_Neon_Inner, ItemShop_Neon_Inner_Round, ItemShop_Neon_Inner_Slope, ItemShop_Neon_Pyramid, ItemShop_Nyan_Cray_Holoflag, ItemShop_Overwolf_Holoflag, ItemShop_Pilot_Seat_Bunny, ItemShop_Pilot_Seat_Cray, ItemShop_Pilot_Seat_Gene, ItemShop_Pilot_Seat_Mega, ItemShop_Pilot_Seat_Retro, ItemShop_Pirate_Holoflag, ItemShop_Plasma_Huge_Carbon_6, ItemShop_Present_Large, ItemShop_Present_Small, ItemShop_Retro_Corner, ItemShop_Retro_Cube, ItemShop_Retro_Edge, ItemShop_Retro_Inner, ItemShop_Robits, ItemShop_Robocraft_Holoflag, ItemShop_RoboPass_Season_1_Holoflag, ItemShop_RoboPass_Season_2_Holoflag, ItemShop_Robot_Name_Banner, ItemShop_Rod_Long_Spring, ItemShop_Rod_Short_Spring, ItemShop_Rod_Spring, ItemShop_Rudder_Bat, ItemShop_Rudder_Vampire, ItemShop_Santa_Cray_Holoflag, ItemShop_Snowflake_Holoflag, ItemShop_Speedometer, ItemShop_Sprinter_Leg_Large_Carbon_6, ItemShop_T5_Mortar_Egg, ItemShop_T5_Seeker_Firework_Steam, ItemShop_Thruster_Huge_Carbon_6, ItemShop_Top_Laser_Huge_Carbon_6, ItemShop_Vapor_Trail_Single, ItemShop_Vapor_Trail_Single_Firework, ItemShop_Vapor_Trail_Single_Flower, ItemShop_Vapor_Trail_Single_Snowflake, ItemShop_Vapor_Trail_Twin, ItemShop_Wing_Bat, ItemShop_Wing_Vampire, ItemShop_Yogscast_Holoflag, Spawn, Spawn_BlackHole, Spawn_EasterEgg, Spawn_Lander, Spawn_Lootcrate, Spawn_Present, Spawn_Warp, Store_RoboPass, Store_RoboPass_Season2

*/
