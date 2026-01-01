use polariton_server::operations::SimpleFunc;
use polariton::{operation::{Arr, ParameterTable, Typed}, serdes::TypePrefix};

use crate::data::customisation_info::CustomisationData;

const SKINS_KEY: u8 = 228;
const SPAWNS_KEY: u8 = 229;
const DEATHS_KEY: u8 = 230;

const OWNED_SKINS_KEY: u8 = 231;
const OWNED_SPAWNS_KEY: u8 = 232;
const OWNED_DEATHS_KEY: u8 = 233;
const OWNED_EMOTES_KEY: u8 = 76;

fn all_skins() -> Vec<CustomisationData> {
    vec![
        /*CustomisationData { // level ???
            id: "RC_MothershipSkin_Premium_01".to_string(),
            localised_name: "strMothershipSkinPremium".to_string(),
            skin_scene_name: "RC_MothershipSkin_Neptune_01".to_string(),
            simulation_prefab: "TODO_sim_prefab".to_string(),
            preview_image_name: "TODO_preview_img".to_string(),
            is_default: false,
        },*/
        CustomisationData { // level13
            id: "RC_MothershipSkin_Neptune_01".to_string(),
            localised_name: "strMothershipSkinNeptune01".to_string(),
            skin_scene_name: "RC_MothershipSkin_Neptune_01".to_string(),
            simulation_prefab: "TODO_sim_prefab".to_string(),
            preview_image_name: "garagebayskintextures/MothershipSkin_Neptune_BG".to_string(),
            is_default: false,
        },
        CustomisationData { // level 11
            id: "RC_MothershipSkin_Earth_01".to_string(),
            localised_name: "strMothershipSkinEarth01".to_string(),
            skin_scene_name: "RC_MothershipSkin_Earth_01".to_string(),
            simulation_prefab: "TODO_sim_prefab".to_string(),
            preview_image_name: "garagebayskintextures/MothershipSkin_Earth_BG".to_string(),
            is_default: false,
        },
        CustomisationData { // level12
            id: "RC_MothershipSkin_Mars_01".to_string(),
            localised_name: "strMothershipSkinMars01".to_string(),
            skin_scene_name: "RC_MothershipSkin_Mars_01".to_string(),
            simulation_prefab: "TODO_sim_prefab".to_string(),
            preview_image_name: "garagebayskintextures/MothershipSkin_Mars_BG".to_string(),
            is_default: false,
        },
        CustomisationData { // level14
            id: "RC_MothershipSkin_Retro_01".to_string(),
            localised_name: "strMothershipSkinRetro01".to_string(),
            skin_scene_name: "RC_MothershipSkin_Retro_01".to_string(),
            simulation_prefab: "TODO_sim_prefab".to_string(),
            preview_image_name: "garagebayskintextures/MothershipSkin_Retro_BG".to_string(),
            is_default: false,
        },
        CustomisationData { // level2 (the is_default one seems to be special)
            id: "RC_Mothership".to_string(),
            localised_name: "strMothershipSkinDefault".to_string(),
            skin_scene_name: "RC_Mothership".to_string(),
            simulation_prefab: "TODO_sim_prefab".to_string(),
            preview_image_name: "garagebayskintextures/Mothership_Premium_BG".to_string(), // FIXME
            is_default: true,
        },
    ]
}

fn all_spawns() -> Vec<CustomisationData> {
    vec![
        CustomisationData {
            id: "Spawn".to_string(),
            localised_name: "strSpawnEffectDefault".to_string(),
            skin_scene_name: "Splash_Loading_Screen".to_string(),
            simulation_prefab: "Spawn".to_string(),
            preview_image_name: "Spawn".to_string(),
            is_default: true,
        },
        CustomisationData {
            id: "Spawn_BlackHole".to_string(),
            localised_name: "strSpawnFXBlackHole".to_string(),
            skin_scene_name: "Splash_Loading_Screen".to_string(),
            simulation_prefab: "Spawn_BlackHole".to_string(),
            preview_image_name: "Spawn_BlackHole".to_string(),
            is_default: false,
        },
        CustomisationData {
            id: "Spawn_Lander".to_string(),
            localised_name: "strSpawnFXRoyaleLander".to_string(),
            skin_scene_name: "Splash_Loading_Screen".to_string(),
            simulation_prefab: "Spawn_Lander".to_string(),
            preview_image_name: "Spawn_Lander".to_string(),
            is_default: false,
        },
        CustomisationData {
            id: "Spawn_Lootcrate".to_string(),
            localised_name: "strSpawnFXLootCrate".to_string(),
            skin_scene_name: "Splash_Loading_Screen".to_string(),
            simulation_prefab: "Spawn_Lootcrate".to_string(),
            preview_image_name: "Spawn_Lootcrate".to_string(),
            is_default: false,
        },
        CustomisationData {
            id: "Spawn_Warp".to_string(),
            localised_name: "strSpawnFXWarpIn".to_string(),
            skin_scene_name: "Splash_Loading_Screen".to_string(),
            simulation_prefab: "Spawn_Warp".to_string(),
            preview_image_name: "Spawn_Warp".to_string(),
            is_default: false,
        },
        CustomisationData {
            id: "Spawn_Present".to_string(),
            localised_name: "strSpawnFXPresent".to_string(),
            skin_scene_name: "Splash_Loading_Screen".to_string(),
            simulation_prefab: "Spawn_Present".to_string(),
            preview_image_name: "Spawn_Present".to_string(),
            is_default: false,
        },
        CustomisationData {
            id: "Spawn_EasterEgg".to_string(),
            localised_name: "strSpawnFXHatch".to_string(),
            skin_scene_name: "Splash_Loading_Screen".to_string(),
            simulation_prefab: "Spawn_EasterEgg".to_string(),
            preview_image_name: "Spawn_EasterEgg".to_string(),
            is_default: false,
        },
    ]
}

fn all_deaths() -> Vec<CustomisationData> {
    vec![
        CustomisationData {
            id: "Explosion".to_string(),
            localised_name: "strDeathEffectDefault".to_string(),
            skin_scene_name: "Splash_Loading_Screen".to_string(),
            simulation_prefab: "Explosion".to_string(),
            preview_image_name: "Explosion".to_string(),
            is_default: true,
        },
        CustomisationData {
            id: "Explosion_Toon".to_string(),
            localised_name: "strDeathFXCartoonExplosion".to_string(),
            skin_scene_name: "Splash_Loading_Screen".to_string(),
            simulation_prefab: "Explosion_Toon".to_string(),
            preview_image_name: "Explosion_Toon".to_string(),
            is_default: false,
        },
        CustomisationData {
            id: "Explosion_Feathers_Rainbow".to_string(),
            localised_name: "strDeathFXFeatherExplosion".to_string(),
            skin_scene_name: "Splash_Loading_Screen".to_string(),
            simulation_prefab: "Explosion_Feathers_Rainbow".to_string(),
            preview_image_name: "Explosion_Feathers_Rainbow".to_string(),
            is_default: false,
        },
        CustomisationData {
            id: "Explosion_Nuclear".to_string(),
            localised_name: "strDeathFXNuclearExplosion".to_string(),
            skin_scene_name: "Splash_Loading_Screen".to_string(),
            simulation_prefab: "Explosion_Nuclear".to_string(),
            preview_image_name: "Explosion_Nuclear".to_string(),
            is_default: false,
        },
        CustomisationData {
            id: "Explosion_Warp".to_string(),
            localised_name: "strDeathFXEmergencyWarp".to_string(),
            skin_scene_name: "Splash_Loading_Screen".to_string(),
            simulation_prefab: "Explosion_Warp".to_string(),
            preview_image_name: "Explosion_Warp".to_string(),
            is_default: false,
        },
        CustomisationData {
            id: "Explosion_BlackHole".to_string(),
            localised_name: "strDeathFXBlackHole".to_string(),
            skin_scene_name: "Splash_Loading_Screen".to_string(),
            simulation_prefab: "Explosion_BlackHole".to_string(),
            preview_image_name: "Explosion_BlackHole".to_string(),
            is_default: false,
        },
        CustomisationData {
            id: "Explosion_Firework".to_string(),
            localised_name: "strDeathFXFireworkExplosion".to_string(),
            skin_scene_name: "Splash_Loading_Screen".to_string(),
            simulation_prefab: "Explosion_Firework".to_string(),
            preview_image_name: "Explosion_Firework".to_string(),
            is_default: false,
        },
    ]
}

pub(super) fn all_customisations_provider() -> SimpleFunc<216, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(SKINS_KEY, Typed::Arr(Arr {
            ty: TypePrefix::HashMap, // hashtable
            items: all_skins().into_iter().map(|x| x.as_transmissible()).collect(),
        }));
        params.insert(SPAWNS_KEY, Typed::Arr(Arr {
            ty: TypePrefix::HashMap, // hashtable
            items: all_spawns().into_iter().map(|x| x.as_transmissible()).collect(),
        }));
        params.insert(DEATHS_KEY, Typed::Arr(Arr {
            ty: TypePrefix::HashMap, // hashtable
            items: all_deaths().into_iter().map(|x| x.as_transmissible()).collect(),
        }));

        params.insert(OWNED_SKINS_KEY, Typed::StrArr(all_skins().into_iter().map(|x| x.id.into()).collect::<Vec<_>>().into()));
        params.insert(OWNED_SPAWNS_KEY, Typed::StrArr(all_spawns().into_iter().map(|x| x.id.into()).collect::<Vec<_>>().into()));
        params.insert(OWNED_DEATHS_KEY, Typed::StrArr(all_deaths().into_iter().map(|x| x.id.into()).collect::<Vec<_>>().into()));
        params.insert(OWNED_EMOTES_KEY, Typed::StrArr(vec![].into()));
        Ok(params.into())
    })
}
