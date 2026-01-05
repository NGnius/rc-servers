 mod parser;

mod weapon_list;
pub use weapon_list::WeaponListParser;

mod cpu_count;
pub use cpu_count::CpuListParser;

mod locations_of;
pub use locations_of::{CubeLocationsParser, CubeLocationInfo};

mod offsetter;
pub use offsetter::OffsetParser;

//pub mod prefabs;

pub struct CubeParsers {
    weapon_list: std::sync::Arc<WeaponListParser>,
    cpu_counter: std::sync::Arc<CpuListParser>,
    locations: std::sync::Arc<CubeLocationsParser>,
    offset: std::sync::Arc<OffsetParser>,
}

impl CubeParsers {
    pub fn new(conf: &crate::ConfigImpl) -> Self {
        let cubes = <crate::ConfigImpl as crate::ConfigProvider<()>>::cubes(conf);
        Self {
            weapon_list: std::sync::Arc::new(WeaponListParser::with_cubes(cubes.values())),
            cpu_counter: std::sync::Arc::new(CpuListParser::with_cubes(cubes.values())),
            locations: std::sync::Arc::new(CubeLocationsParser::with_cubes(cubes.values())),
            offset: std::sync::Arc::new(OffsetParser::with_cubes(cubes.values())),
        }
    }

    pub fn weapon_order(&self) -> std::sync::Arc<WeaponListParser> {
        self.weapon_list.clone()
    }

    pub fn cpu_counter(&self) -> std::sync::Arc<CpuListParser> {
        self.cpu_counter.clone()
    }

    pub fn locations_of(&self) -> std::sync::Arc<CubeLocationsParser> {
        self.locations.clone()
    }

    pub fn offset(&self) -> std::sync::Arc<OffsetParser> {
        self.offset.clone()
    }
}
