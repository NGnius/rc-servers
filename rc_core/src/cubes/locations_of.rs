pub struct CubeLocationsParser;

pub struct CubeLocationInfo {
    pub x: u8,
    pub y: u8,
    pub z: u8,
    extras: u8,
}

impl CubeLocationInfo {
    pub fn orientation(&self) -> u8 {
        self.extras & 0x7F
    }

    pub fn is_destroyed(&self) -> bool {
        self.extras & 0x80 != 0
    }
}

impl CubeLocationsParser {
    pub fn with_cubes<'a, I: std::iter::Iterator<Item=&'a crate::persist::Cube>>(_iter: I) -> Self {
        Self
    }

    pub fn locations_of(&self, r: &mut dyn std::io::Read, cube_id: u32) -> Vec<CubeLocationInfo> {
        match super::parser::Cube::parse_list(r) {
            Ok(cubes) => {
                cubes.into_iter()
                    .filter(|x| x.id == cube_id)
                    .map(|cube| CubeLocationInfo {
                        x: cube.x,
                        y: cube.y,
                        z: cube.z,
                        extras: cube.orientation,
                    })
                    .collect()
            }
            Err(e) => {
                log::error!("Failed to parse cube data to find cube locations: {}", e);
                Vec::default()
            }
        }
    }

    fn locations_sorted_by_distance_from_point(cubes: &[super::parser::Cube], point: (u8, u8, u8), locations_of_id: u32) -> Vec<CubeLocationInfo> {
        let target_x = point.0 as f32;
        let target_y = point.1 as f32;
        let target_z = point.2 as f32;
        let mut relevant_cubes: Vec<(f32, CubeLocationInfo)> = cubes.iter()
            .filter(|x| x.id == locations_of_id)
            .map(|cube| {
                let distance = (
                    (cube.x as f32 - target_x).powi(2)
                    + (cube.y as f32 - target_y).powi(2)
                    + (cube.z as f32 - target_z).powi(2)
                ).sqrt();
                (distance, CubeLocationInfo {
                    x: cube.x,
                    y: cube.y,
                    z: cube.z,
                    extras: cube.orientation,
                })
            })
            .collect();
        relevant_cubes.sort_by_key(|(distance, _)| (distance * 1_000_000.0) as i64);
        relevant_cubes.into_iter()
            .map(|(_, cube)| cube)
            .collect()
    }

    pub fn locations_of_by_distance_to_first(&self, r: &mut dyn std::io::Read, locations_of_id: u32, distance_to_id: u32) -> Vec<CubeLocationInfo> {
        match super::parser::Cube::parse_list(r) {
            Ok(cubes) => {
                if let Some(target) = cubes.iter().find(|x| x.id == distance_to_id) {
                    Self::locations_sorted_by_distance_from_point(
                        &cubes,
                        (target.x, target.y, target.z),
                        locations_of_id,
                    )
                } else {
                    log::warn!("No cube with id {} to calculate distance", distance_to_id);
                    cubes.into_iter()
                        .filter(|x| x.id == locations_of_id)
                        .map(|cube| CubeLocationInfo {
                            x: cube.x,
                            y: cube.y,
                            z: cube.z,
                            extras: cube.orientation,
                        })
                        .collect()
                }

            }
            Err(e) => {
                log::error!("Failed to parse cube data to find cube locations: {}", e);
                Vec::default()
            }
        }
    }

    pub fn locations_of_by_distance_from(&self, r: &mut dyn std::io::Read, locations_of_id: u32, from: (u8, u8, u8)) -> Vec<CubeLocationInfo> {
        match super::parser::Cube::parse_list(r) {
            Ok(cubes) => {
                Self::locations_sorted_by_distance_from_point(
                    &cubes,
                    from,
                    locations_of_id,
                )
            }
            Err(e) => {
                log::error!("Failed to parse cube data to find cube locations: {}", e);
                Vec::default()
            }
        }
    }
}
