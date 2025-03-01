use polariton::operation::Typed;

pub struct PrebuiltRobotInfo {
    //pub id: String,
    pub name: String,
    pub class: String,
    pub category: String,
    pub robot_data: Vec<u8>,
    pub colour_data: Vec<u8>,
}

impl PrebuiltRobotInfo {
    pub fn as_transmissible(&self) -> Typed {
        Typed::HashMap(vec![
            //(Typed::Str("[key]".into()), Typed::Str(self.id.clone().into())),
            (Typed::Str("Name".into()), Typed::Str(self.name.clone().into())),
            (Typed::Str("Class".into()), Typed::Str(self.class.clone().into())),
            (Typed::Str("Category".into()), Typed::Str(self.category.clone().into())),
            (Typed::Str("RobotData".into()), Typed::Bytes(self.robot_data.clone().into())),
            (Typed::Str("ColourData".into()), Typed::Bytes(self.colour_data.clone().into())),
        ].into())
    }
}
