use std::io::Cursor;

use libfj::robocraft::{FactoryRobotGetInfo, FactoryInfo};

pub struct FactoryLike {
    convert: std::sync::Arc<oj_rc_core::cubes::CubeConversionParser>,
}

impl FactoryLike {
    pub fn new(convert: std::sync::Arc<oj_rc_core::cubes::CubeConversionParser>,) -> Self {
        Self {
            convert,
        }
    }

    fn try_guess_json_format(upload: &[u8]) -> Option<FactoryRobotGetInfo> {
        let reader = Cursor::new(upload);
        match serde_json::from_reader::<_, FactoryRobotGetInfo>(reader) {
            Ok(data) => Some(data),
            Err(e) => {
                let reader = Cursor::new(upload);
                match serde_json::from_reader::<_, FactoryInfo<FactoryRobotGetInfo>>(reader) {
                    Ok(data) => Some(data.response),
                    Err(e2) => {
                        log::error!("Failed to parse json for factory-like vehicle import: (1){} (2){}", e, e2);
                        None
                    }
                }
            }
        }
    }
}

impl oj_rc_plugins::vehicle_import::VehicleImportPlugin for FactoryLike {
    fn file_ext(&self) -> &'static str {
        "rcbup"
    }

    fn import(&self, upload: &[u8]) -> Result<oj_rc_plugins::vehicle_import::VehicleImportData, oj_rc_plugins::vehicle_import::VehicleImportErrorCode> {
        if let Some(upload_data) = Self::try_guess_json_format(upload) {
            use base64::Engine;
            let cube_data = base64::engine::general_purpose::STANDARD.decode(&upload_data.cube_data).map_err(|e| {
                log::error!("Bad base64 encoding of cube data: {}", e);
                oj_rc_plugins::vehicle_import::VehicleImportErrorCode::Invalid
            })?;
            let colour_data = base64::engine::general_purpose::STANDARD.decode(&upload_data.cube_data).map_err(|e| {
                log::error!("Bad base64 encoding of colour data: {}", e);
                oj_rc_plugins::vehicle_import::VehicleImportErrorCode::Invalid
            })?;
            let upgraded = self.convert.upgrade_to_modern(
                &mut std::io::Cursor::new(&cube_data),
                &mut std::io::Cursor::new(&colour_data),
            ).map_err(|e| {
                log::warn!("Failed to upgrade rcbup import data: {}", e);
                oj_rc_plugins::vehicle_import::VehicleImportErrorCode::Unsupported
            })?;
            Ok(oj_rc_plugins::vehicle_import::VehicleImportData {
                cube_data: upgraded.cube_data,
                colour_data: upgraded.colour_data,
                vehicle_name: Some(upload_data.item_name),
                vehicle_author: Some(upload_data.added_by_display_name),
            })
        } else {
            Err(oj_rc_plugins::vehicle_import::VehicleImportErrorCode::Invalid)
        }
    }

    fn export(&self, data: &oj_rc_plugins::vehicle_import::VehicleImportData) -> Result<Vec<u8>, oj_rc_plugins::vehicle_import::VehicleImportErrorCode> {
        use base64::Engine;
        let json_data = FactoryRobotGetInfo {
            item_id: 0,
            item_name: data.vehicle_name.clone().unwrap_or_default(),
            item_description: String::default(),
            thumbnail: String::default(),
            added_by: data.vehicle_author.clone().unwrap_or_default(),
            added_by_display_name: data.vehicle_author.clone().unwrap_or_default(),
            added_date: "1970-01-01T00:00:01".to_owned(),
            expiry_date: "2100-01-01T00:00:01".to_owned(),
            rent_count: 0,
            buy_count: 0,
            cpu: 0, // TODO
            total_robot_ranking: 0, // TODO
            buyable: true,
            removed_date: None,
            ban_date: None,
            featured: false,
            banner_message: None,
            combat_rating: 5.0,
            cosmetic_rating: 5.0,
            cube_data: base64::engine::general_purpose::STANDARD.encode(&data.cube_data),
            colour_data: base64::engine::general_purpose::STANDARD.encode(&data.colour_data),
            cube_amounts: String::default(),
        };
        let output = serde_json::to_vec_pretty(&json_data).map_err(|e| {
            log::error!("Failed to convert vehicle `{:?}` to JSON: {}", json_data.item_name, e);
            oj_rc_plugins::vehicle_import::VehicleImportErrorCode::Invalid
        })?;
        Ok(output)
    }
}

impl oj_rc_plugins::Plugin for FactoryLike {}
