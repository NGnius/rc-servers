pub(self) mod parser;

mod weapon_list;
pub use weapon_list::WeaponListParser;

pub struct CubeParsers {
    weapon_list: std::sync::Arc<WeaponListParser>,
}

impl CubeParsers {
    pub fn new(conf: &crate::ConfigImpl) -> Self {
        Self {
            weapon_list: std::sync::Arc::new(WeaponListParser::with_cubes(<crate::ConfigImpl as crate::ConfigProvider<()>>::cubes(conf).values())),
        }
    }

    pub fn weapon_order(&self) -> std::sync::Arc<WeaponListParser> {
        self.weapon_list.clone()
    }
}
