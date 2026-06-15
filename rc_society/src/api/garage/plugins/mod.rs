mod factory;
pub use factory::FactoryLike;

mod bobocraft;
pub use bobocraft::RexLike;

mod classic;
pub use classic::FifteenLike;

#[derive(Default)]
pub struct ImportPlugins {
    map: std::collections::HashMap<String, Box<dyn oj_rc_plugins::vehicle_import::VehicleImportPlugin>>,
}

impl ImportPlugins {
    pub fn standard(
        assets_path: impl AsRef<std::path::Path>,
        parsers: &oj_rc_core::cubes::CubeParsers,
    ) -> Self {
        let mut map = std::collections::HashMap::with_capacity(3);
        map.insert("rcbup".to_owned(), Box::new(FactoryLike::new(parsers.converter())) as _);
        map.insert("bobo".to_owned(), Box::new(RexLike::new(parsers.converter())) as _);
        let image_path = assets_path.as_ref().join("rc_export.png");
        log::debug!("Loading PNG for RC15 export steganography from {}", image_path.display());
        let image_data = std::fs::read(image_path).expect("Failed to read rc_export.png file required for exporting RC15 vehicles");
        map.insert("rc15".to_owned(), Box::new(FifteenLike::new(parsers.converter(), image_data, parsers.cpu_counter())) as _);
        Self {
            map,
        }
    }

    pub fn plugin_names(&self) -> impl std::iter::Iterator<Item=&'_ String> {
        let mut to_sort = self.map.keys().collect::<Vec<_>>();
        to_sort.sort_by_key(|x| x.to_lowercase());
        to_sort.into_iter()
    }

    pub fn file_ext(&self, name: &str) -> Result<&'static str, oj_rc_plugins::vehicle_import::VehicleImportErrorCode> {
        if let Some(plugin) = self.map.get(name) {
            Ok(plugin.file_ext())
        } else {
            Err(oj_rc_plugins::vehicle_import::VehicleImportErrorCode::Unsupported)
        }
    }

    pub fn import_by_name(&self, name: &str, upload: &[u8]) -> Result<oj_rc_plugins::vehicle_import::VehicleImportData, oj_rc_plugins::vehicle_import::VehicleImportErrorCode> {
        if let Some(plugin) = self.map.get(name) {
            plugin.import(upload)
        } else {
            Err(oj_rc_plugins::vehicle_import::VehicleImportErrorCode::Unsupported)
        }
    }

    pub fn export_by_name(&self, name: &str, data: &oj_rc_plugins::vehicle_import::VehicleImportData) -> Result<Vec<u8>, oj_rc_plugins::vehicle_import::VehicleImportErrorCode> {
        if let Some(plugin) = self.map.get(name) {
            plugin.export(data)
        } else {
            Err(oj_rc_plugins::vehicle_import::VehicleImportErrorCode::Unsupported)
        }
    }
}
