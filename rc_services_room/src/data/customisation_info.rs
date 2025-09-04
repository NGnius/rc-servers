use polariton::operation::Typed;

pub struct CustomisationData {
    pub id: String,
    pub localised_name: String,
    pub skin_scene_name: String,
    pub simulation_prefab: String,
    pub preview_image_name: String,
    pub is_default: bool,
}

impl CustomisationData {
    pub fn as_transmissible(&self) -> Typed {
        Typed::HashMap(vec![
            (Typed::Str("id".into()), Typed::Str(self.id.clone().into())),
            (Typed::Str("localisedName".into()), Typed::Str(self.localised_name.clone().into())),
            (Typed::Str("skinsceneName".into()), Typed::Str(self.skin_scene_name.clone().into())),
            (Typed::Str("simulationPrefab".into()), Typed::Str(self.simulation_prefab.clone().into())),
            (Typed::Str("previewImageName".into()), Typed::Str(self.preview_image_name.clone().into())),
            (Typed::Str("isDefault".into()), Typed::Bool(self.is_default)),
        ].into())
    }
}
