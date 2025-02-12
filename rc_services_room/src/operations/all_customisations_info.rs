use polariton_server::operations::SimpleFunc;
use polariton::operation::{ParameterTable, Typed, Arr};

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
            ty: 104, // hashtable
            items: vec![
                CustomisationData {
                    id: "skin0".to_string(),
                    localised_name: "Default".to_string(),
                    skin_scene_name: "TODO_skin".to_string(),
                    simulation_prefab: "TODO_sim_prefab".to_string(),
                    preview_image_name: "TODO_preview_img".to_string(),
                    is_default: true,
                }.as_transmissible(),
            ],
        }));
        params.insert(SPAWNS_KEY, Typed::Arr(Arr {
            ty: 104, // hashtable
            items: vec![
                CustomisationData {
                    id: "spawn0".to_string(),
                    localised_name: "Default".to_string(),
                    skin_scene_name: "TODO_skin".to_string(),
                    simulation_prefab: "TODO_sim_prefab".to_string(),
                    preview_image_name: "TODO_preview_img".to_string(),
                    is_default: true,
                }.as_transmissible(),
            ],
        }));
        params.insert(DEATHS_KEY, Typed::Arr(Arr {
            ty: 104, // hashtable
            items: vec![
                CustomisationData {
                    id: "death0".to_string(),
                    localised_name: "Default".to_string(),
                    skin_scene_name: "TODO_skin".to_string(),
                    simulation_prefab: "TODO_sim_prefab".to_string(),
                    preview_image_name: "TODO_preview_img".to_string(),
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
