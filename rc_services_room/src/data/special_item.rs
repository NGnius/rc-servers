use polariton::operation::Typed;

pub struct SpecialItem {
    pub name: String,
    pub sprite: String,
    pub size: u32,
}

impl SpecialItem {
    pub fn as_transmissible(&self) -> Typed {
        Typed::HashMap(vec![
            (Typed::Str("name".into()), Typed::Str(self.name.clone().into())),
            (Typed::Str("spriteName".into()), Typed::Str(self.sprite.clone().into())),
            (Typed::Str("mothershipSize".into()), Typed::Int(self.size as i32)),
        ].into())
    }
}
