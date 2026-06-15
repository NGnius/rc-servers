use oj_convert::{Bobocraft, BobocraftBlock};

pub struct RexLike {
    convert: std::sync::Arc<oj_rc_core::cubes::CubeConversionParser>,
}

impl RexLike {
    pub fn new(convert: std::sync::Arc<oj_rc_core::cubes::CubeConversionParser>) -> Self {
        Self {
            convert,
        }
    }
}

impl oj_rc_plugins::vehicle_import::VehicleImportPlugin for RexLike {
    fn file_ext(&self) -> &'static str {
        "bobo"
    }

    fn import(&self, upload: &[u8]) -> Result<oj_rc_plugins::vehicle_import::VehicleImportData, oj_rc_plugins::vehicle_import::VehicleImportErrorCode> {
        let data = Bobocraft::parse(upload).map_err(|e| {
            log::error!("Failed to parse bobocraft import: {}", e);
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
        let colours = data.cubes.iter()
            .map(|b| oj_rc_core::cubes::Colour {
                colour: b.color.unwrap_or(0),
                x: b.x,
                y: b.y,
                z: b.z,
            }).collect();
        let colour_data = oj_rc_core::cubes::Colour::dump_list(colours)
            .map_err(|e| {
                log::error!("Failed to write colour data: {}", e);
                oj_rc_plugins::vehicle_import::VehicleImportErrorCode::Invalid
            })?;
        let upgraded = self.convert.upgrade_to_modern(
            &mut std::io::Cursor::new(&cube_data),
            &mut std::io::Cursor::new(&colour_data),
        ).map_err(|e| {
            log::warn!("Failed to upgrade bobo import data: {}", e);
            oj_rc_plugins::vehicle_import::VehicleImportErrorCode::Unsupported
        })?;
        Ok(oj_rc_plugins::vehicle_import::VehicleImportData {
            cube_data: upgraded.cube_data,
            colour_data: upgraded.colour_data,
            vehicle_name: Some(data.item_name),
            vehicle_author: Some(data.added_by_display_name),
        })
    }

    fn export(&self, data: &oj_rc_plugins::vehicle_import::VehicleImportData) -> Result<Vec<u8>, oj_rc_plugins::vehicle_import::VehicleImportErrorCode> {
        let cubes = oj_rc_core::cubes::Cube::parse_list(&mut std::io::Cursor::new(&data.cube_data))
            .map_err(|e| {
                log::error!("Failed to read cube data: {}", e);
                oj_rc_plugins::vehicle_import::VehicleImportErrorCode::Invalid
            })?;
        let colours = oj_rc_core::cubes::Colour::parse_list(&mut std::io::Cursor::new(&data.colour_data))
            .map_err(|e| {
                log::error!("Failed to read colour data: {}", e);
                oj_rc_plugins::vehicle_import::VehicleImportErrorCode::Invalid
            })?;
        if colours.len() != cubes.len() {
            log::error!("Failed to parse the same amount of cubes and colours (aborting)");
            return Err(oj_rc_plugins::vehicle_import::VehicleImportErrorCode::Invalid);
        }
        let mut placements = Vec::with_capacity(colours.len());
        for i in 0..cubes.len() {
            let colour = &colours[i];
            let cube = &cubes[i];
            // assumption: cube and colours are in the same order
            let placement = BobocraftBlock {
                cube_id: cube.id,
                x: cube.x,
                y: cube.y,
                z: cube.z,
                orientation: cube.orientation,
                color: if (cube.x, cube.y, cube.z) == (colour.x, colour.y, colour.z) { Some(colour.colour) } else { None },
            };
            placements.push(placement);
        }
        let bobo = Bobocraft {
            item_id: 0,
            item_name: data.vehicle_name.clone().unwrap_or_else(|| "Exported vehicle".to_owned()),
            item_description: format!("Exported by OpenJam {} {}", env!("CARGO_CRATE_NAME"), env!("CARGO_PKG_VERSION")),
            thumbnail: String::default(),
            added_by: data.vehicle_author.clone().unwrap_or_default(),
            added_by_display_name: data.vehicle_author.clone().unwrap_or_default(),
            added_date: chrono::Utc::now().naive_utc(),
            expiry_date: chrono::Utc::now().naive_utc(),
            cpu: 0, // TODO?
            total_robot_ranking: None, // TODO?
            buyable: true,
            buy_count: 0,
            unknown_count: 0,
            combat_rating: 5.0,
            cosmetic_rating: 5.0,
            featured: false,
            banner_message: None,
            offset: (0, 0, 0),
            cubes: placements,
        };
        bobo.dump()
            .map_err(|e| {
                log::error!("Failed to write bobocraft format: {}", e);
                oj_rc_plugins::vehicle_import::VehicleImportErrorCode::Invalid
            })
    }
}

impl oj_rc_plugins::Plugin for RexLike {}
