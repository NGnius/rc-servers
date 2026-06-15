use oj_convert::{Classic, ClassicBlock};

pub struct FifteenLike {
    convert: std::sync::Arc<oj_rc_core::cubes::CubeConversionParser>,
    base_image: std::sync::Arc<Vec<u8>>,
    cpu_counter: std::sync::Arc<oj_rc_core::cubes::CpuListParser>,
}

impl FifteenLike {
    pub fn new(
        convert: std::sync::Arc<oj_rc_core::cubes::CubeConversionParser>,
        base_image: Vec<u8>,
        cpu_counter: std::sync::Arc<oj_rc_core::cubes::CpuListParser>,
    ) -> Self {
        Self {
            convert,
            base_image: std::sync::Arc::new(base_image),
            cpu_counter,
        }
    }
}

impl oj_rc_plugins::vehicle_import::VehicleImportPlugin for FifteenLike {
    fn file_ext(&self) -> &'static str {
        "png"
    }

    fn import(&self, upload: &[u8]) -> Result<oj_rc_plugins::vehicle_import::VehicleImportData, oj_rc_plugins::vehicle_import::VehicleImportErrorCode> {
        let data = Classic::parse(upload).map_err(|e| {
            log::error!("Failed to parse RC15 import: {}", e);
            oj_rc_plugins::vehicle_import::VehicleImportErrorCode::Invalid
        })?;
        let cubes = data.cubes.iter()
            .map(|b| oj_rc_core::cubes::Cube {
                id: b.cube_id,
                x: b.x,
                y: b.y,
                z: b.z,
                orientation: b.orientation,
            }).collect();
        let cube_data = oj_rc_core::cubes::Cube::dump_list(cubes)
            .map_err(|e| {
                log::error!("Failed to write cube data: {}", e);
                oj_rc_plugins::vehicle_import::VehicleImportErrorCode::Invalid
            })?;
        let converted = self.convert.convert_to_modern(
            &mut std::io::Cursor::new(&cube_data),
        ).map_err(|e| {
            log::warn!("Failed to modernize RC15 import data: {}", e);
            oj_rc_plugins::vehicle_import::VehicleImportErrorCode::Unsupported
        })?;
        Ok(oj_rc_plugins::vehicle_import::VehicleImportData {
            cube_data: converted.cube_data,
            colour_data: converted.colour_data,
            vehicle_name: Some(data.item_name),
            vehicle_author: data.item_author,
        })
    }

    fn export(&self, data: &oj_rc_plugins::vehicle_import::VehicleImportData) -> Result<Vec<u8>, oj_rc_plugins::vehicle_import::VehicleImportErrorCode> {
        let cpu_count = self.cpu_counter.calculate_cpu(&mut std::io::Cursor::new(&data.cube_data)).total;
        let classified = self.convert.convert_to_classic(
            &mut std::io::Cursor::new(&data.cube_data),
            &mut std::io::Cursor::new(&data.colour_data),
        ).map_err(|e| {
            log::warn!("Failed to classic-ify RC15 export data: {}", e);
            oj_rc_plugins::vehicle_import::VehicleImportErrorCode::Unsupported
        })?;
        let cubes = oj_rc_core::cubes::Cube::parse_list(&mut std::io::Cursor::new(&classified))
            .map_err(|e| {
                log::error!("Failed to read cube data: {}", e);
                oj_rc_plugins::vehicle_import::VehicleImportErrorCode::Invalid
            })?;
        let mut placements = Vec::with_capacity(cubes.len());
        for cube in cubes.iter() {
            let placement = ClassicBlock {
                cube_id: cube.id,
                x: cube.x,
                y: cube.y,
                z: cube.z,
                orientation: cube.orientation,
            };
            placements.push(placement);
        }
        let classic = Classic {
            item_author: data.vehicle_author.clone(),
            game_version: Some(1), // once told me
            export_time: Some(chrono::Utc::now().to_rfc3339()),
            item_name: data.vehicle_name.clone().unwrap_or_else(|| "Exported vehicle".to_owned()),
            item_description: Some(format!("Exported by OpenJam {} {}", env!("CARGO_CRATE_NAME"), env!("CARGO_PKG_VERSION"))),
            item_tier: 1, // TODO
            item_cpu: cpu_count,
            cubes: placements,
        };
        classic.dump(&self.base_image)
            .map_err(|e| {
                log::error!("Failed to dump the RC15 PNG data: {}", e);
                oj_rc_plugins::vehicle_import::VehicleImportErrorCode::Unsupported
            })
    }
}

impl oj_rc_plugins::Plugin for FifteenLike {}
