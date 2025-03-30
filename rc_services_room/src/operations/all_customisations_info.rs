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

pub(super) fn all_customisations_provider() -> SimpleFunc<216, crate::UserTy, impl (Fn(ParameterTable, &crate::UserTy) -> Result<ParameterTable, i16>) + Sync + Sync> {
    SimpleFunc::new(|params, _| {
        let mut params = params.to_dict();
        params.insert(SKINS_KEY, Typed::Arr(Arr {
            ty: TypePrefix::HashMap, // hashtable
            items: vec![
                CustomisationData {
                    id: "RC_MothershipSkin_Neptune_01".to_string(),
                    localised_name: "strNeptune".to_string(),
                    skin_scene_name: "RC_MothershipSkin_Neptune_01".to_string(),
                    simulation_prefab: "TODO_sim_prefab".to_string(),
                    preview_image_name: "TODO_preview_img".to_string(),
                    is_default: true,
                }.as_transmissible(),
            ],
        }));
        params.insert(SPAWNS_KEY, Typed::Arr(Arr {
            ty: TypePrefix::HashMap, // hashtable
            items: vec![
                // TODO set these up with the correct values (IDs are correct)
                CustomisationData {
                    id: "Spawn".to_string(),
                    localised_name: "strSpawnFXWarpIn".to_string(),
                    skin_scene_name: "Splash_Loading_Screen".to_string(),
                    simulation_prefab: "Spawn".to_string(),
                    preview_image_name: "RC_Splash_Screen_01_Japanese".to_string(),
                    is_default: true,
                }.as_transmissible(),
                CustomisationData {
                    id: "Spawn_BlackHole".to_string(),
                    localised_name: "strSpawnFXWarpIn".to_string(),
                    skin_scene_name: "Splash_Loading_Screen".to_string(),
                    simulation_prefab: "Spawn_BlackHole".to_string(),
                    preview_image_name: "RC_Splash_Screen_01_Japanese".to_string(),
                    is_default: true,
                }.as_transmissible(),
                CustomisationData {
                    id: "Spawn_Lander".to_string(),
                    localised_name: "strSpawnFXWarpIn".to_string(),
                    skin_scene_name: "Splash_Loading_Screen".to_string(),
                    simulation_prefab: "Spawn_Lander".to_string(),
                    preview_image_name: "RC_Splash_Screen_01_Japanese".to_string(),
                    is_default: true,
                }.as_transmissible(),
                CustomisationData {
                    id: "Spawn_Lootcrate".to_string(),
                    localised_name: "strSpawnFXWarpIn".to_string(),
                    skin_scene_name: "Splash_Loading_Screen".to_string(),
                    simulation_prefab: "Spawn_Lootcrate".to_string(),
                    preview_image_name: "RC_Splash_Screen_01_Japanese".to_string(),
                    is_default: true,
                }.as_transmissible(),
                CustomisationData {
                    id: "Spawn_Warp".to_string(),
                    localised_name: "strSpawnFXWarpIn".to_string(),
                    skin_scene_name: "Splash_Loading_Screen".to_string(),
                    simulation_prefab: "Spawn_Warp".to_string(),
                    preview_image_name: "RC_Splash_Screen_01_Japanese".to_string(),
                    is_default: true,
                }.as_transmissible(),
                CustomisationData {
                    id: "Spawn_Present".to_string(),
                    localised_name: "strSpawnFXWarpIn".to_string(),
                    skin_scene_name: "Splash_Loading_Screen".to_string(),
                    simulation_prefab: "Spawn_Present".to_string(),
                    preview_image_name: "RC_Splash_Screen_01_Japanese".to_string(),
                    is_default: true,
                }.as_transmissible(),
                CustomisationData {
                    id: "Spawn_EasterEgg".to_string(),
                    localised_name: "strSpawnFXWarpIn".to_string(),
                    skin_scene_name: "Splash_Loading_Screen".to_string(),
                    simulation_prefab: "Spawn_EasterEgg".to_string(),
                    preview_image_name: "RC_Splash_Screen_01_Japanese".to_string(),
                    is_default: true,
                }.as_transmissible(),
            ],
        }));
        params.insert(DEATHS_KEY, Typed::Arr(Arr {
            ty: TypePrefix::HashMap, // hashtable
            items: vec![
                // TODO set these up with the correct values (IDs are correct)
                CustomisationData {
                    id: "Explosion".to_string(),
                    localised_name: "strDeathFXEmergencyWarp".to_string(),
                    skin_scene_name: "Splash_Loading_Screen".to_string(),
                    simulation_prefab: "Explosion".to_string(),
                    preview_image_name: "RC_Splash_Screen_01_Japanese".to_string(),
                    is_default: true,
                }.as_transmissible(),
                CustomisationData {
                    id: "Explosion_Toon".to_string(),
                    localised_name: "strDeathFXEmergencyWarp".to_string(),
                    skin_scene_name: "Splash_Loading_Screen".to_string(),
                    simulation_prefab: "Explosion_Toon".to_string(),
                    preview_image_name: "RC_Splash_Screen_01_Japanese".to_string(),
                    is_default: true,
                }.as_transmissible(),
                CustomisationData {
                    id: "Explosion_Feathers_Rainbow".to_string(),
                    localised_name: "strDeathFXEmergencyWarp".to_string(),
                    skin_scene_name: "Splash_Loading_Screen".to_string(),
                    simulation_prefab: "Explosion_Feathers_Rainbow".to_string(),
                    preview_image_name: "RC_Splash_Screen_01_Japanese".to_string(),
                    is_default: true,
                }.as_transmissible(),
                CustomisationData {
                    id: "Explosion_Nuclear".to_string(),
                    localised_name: "strDeathFXEmergencyWarp".to_string(),
                    skin_scene_name: "Splash_Loading_Screen".to_string(),
                    simulation_prefab: "Explosion_Nuclear".to_string(),
                    preview_image_name: "RC_Splash_Screen_01_Japanese".to_string(),
                    is_default: true,
                }.as_transmissible(),
                CustomisationData {
                    id: "Explosion_Warp".to_string(),
                    localised_name: "strDeathFXEmergencyWarp".to_string(),
                    skin_scene_name: "Splash_Loading_Screen".to_string(),
                    simulation_prefab: "Explosion_Warp".to_string(),
                    preview_image_name: "RC_Splash_Screen_01_Japanese".to_string(),
                    is_default: true,
                }.as_transmissible(),
                CustomisationData {
                    id: "Explosion_BlackHole".to_string(),
                    localised_name: "strDeathFXEmergencyWarp".to_string(),
                    skin_scene_name: "Splash_Loading_Screen".to_string(),
                    simulation_prefab: "Explosion_BlackHole".to_string(),
                    preview_image_name: "RC_Splash_Screen_01_Japanese".to_string(),
                    is_default: true,
                }.as_transmissible(),
                CustomisationData {
                    id: "Explosion_Firework".to_string(),
                    localised_name: "strDeathFXEmergencyWarp".to_string(),
                    skin_scene_name: "Splash_Loading_Screen".to_string(),
                    simulation_prefab: "Explosion_Firework".to_string(),
                    preview_image_name: "RC_Splash_Screen_01_Japanese".to_string(),
                    is_default: true,
                }.as_transmissible(),
            ],
        }));

        params.insert(OWNED_SKINS_KEY, Typed::StrArr(vec![].into()));
        params.insert(OWNED_SPAWNS_KEY, Typed::StrArr(vec![].into()));
        params.insert(OWNED_DEATHS_KEY, Typed::StrArr(vec![].into()));
        params.insert(OWNED_EMOTES_KEY, Typed::StrArr(vec![].into()));
        Ok(params.into())
    })
}
