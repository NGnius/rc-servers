use super::{CLASP_ID, CRYSTAL_ID};

pub struct CubeLocationsParser;

#[derive(Clone)]
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

    fn locations_sorted_by_distance_from_point_reward_inlines(cubes: &[super::parser::Cube], point: (u8, u8, u8), locations_of_id: u32, inline_div: f32) -> Vec<CubeLocationInfo> {
        let target_x = point.0 as f32;
        let target_y = point.1 as f32;
        let target_z = point.2 as f32;
        let mut relevant_cubes: Vec<(f32, CubeLocationInfo)> = cubes.iter()
            .filter(|x| x.id == locations_of_id)
            .map(|cube| {
                let is_inline = cube.x.wrapping_sub(point.0) == 0 || cube.z.wrapping_sub(point.2) == 0;
                let distance = (
                    (cube.x as f32 - target_x).powi(2)
                    + (cube.y as f32 - target_y).powi(2)
                    + (cube.z as f32 - target_z).powi(2)
                ).sqrt();
                let distance = if is_inline { distance / inline_div } else { distance };
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

    pub fn locations_of_by_distance_from_center(&self, r: &mut dyn std::io::Read, locations_of_id: u32) -> Vec<CubeLocationInfo> {
        match super::parser::Cube::parse_list(r) {
            Ok(cubes) => {
                let mut total = 0;
                let mut total_x = 0;
                let mut total_y = 0;
                let mut total_z = 0;
                for cube in cubes.iter() {
                    if cube.id == locations_of_id {
                        total += 1;
                        total_x += cube.x as usize;
                        total_y += cube.y as usize;
                        total_z += cube.z as usize;
                    }
                }
                let center = (total_x / total, total_y / total, total_z / total);
                let center = (center.0 as u8, center.1 as u8, center.2 as u8);
                Self::locations_sorted_by_distance_from_point_reward_inlines(
                    &cubes,
                    center,
                    locations_of_id,
                    4.0,
                )
            }
            Err(e) => {
                log::error!("Failed to parse cube data to find cube locations: {}", e);
                Vec::default()
            }
        }
    }

    pub fn locations_of_reactor_sort(&self, r: &mut dyn std::io::Read) -> Vec<CubeLocationInfo> {
        self.locations_of_reactor_sort_custom(r, 32, 2)
    }

    /// Generate sorted crystal cube list using custom thresholds
    ///
    /// `bail_after_iters` is the maximum loops to attempt before giving up trying to sort the vehicle's cubes.
    /// Decreasing this guarantees a faster function return but increases the risk of the result being incomplete.
    /// In the game client, an incomplete result will cause the match to end at a base charge lower than 100%.
    ///
    /// `deterministic_after_iters` is the maxiumum initial loops to allow for random connection traversal.
    /// Decreasing this makes the crystal order more consistent and makes the function return faster.
    /// Increasing this makes the crystal order more random and interesting but usually requires more iterations (slower).
    ///
    /// Usually, this algorithm takes `deterministic_after_iters + 2` iterations to complete.
    pub fn locations_of_reactor_sort_custom(&self, r: &mut dyn std::io::Read, bail_after_iters: usize, deterministic_after_iters: usize) -> Vec<CubeLocationInfo> {
        use super::{CUBE_CONNECTIONS, CUBE_ROTATIONS};
        match super::parser::Cube::parse_list(r) {
            Ok(cubes) => {
                // get connection data
                let location_cube_connections = if let Some(conn) = CUBE_CONNECTIONS.iter().find(|x| x.id == CRYSTAL_ID) {
                    conn
                } else {
                    CUBE_CONNECTIONS.iter().find(|x| x.id == 0).unwrap()
                };
                let connected_to_connections = if let Some(conn) = CUBE_CONNECTIONS.iter().find(|x| x.id == CLASP_ID) {
                    conn
                } else {
                    CUBE_CONNECTIONS.iter().find(|x| x.id == 0).unwrap()
                };
                // calculate connect target connection points
                let mut abs_connected_to = None;
                for cube in cubes.iter() {
                    if cube.id == CLASP_ID {
                        let rot_index = cube.orientation & 0b01111111;
                        let rot = &CUBE_ROTATIONS[rot_index as usize];
                        log::trace!("Calculating target connection points");
                        abs_connected_to = Some(calculate_absolute_connections(connected_to_connections, rot, (cube.x, cube.y, cube.z)));
                        break;
                    }
                }
                let mut available_faces = if let Some(abs_connected_to) = abs_connected_to {
                    abs_connected_to
                } else {
                    log::error!("Failed to find cube with id {} to calculate connections to", CLASP_ID);
                    return Vec::default();
                };
                // find cubes connected to target (or to another relevant cube connected to the target)
                let mut to_be_sorted = std::collections::HashMap::new();
                for (i, cube) in cubes.iter().enumerate() {
                    if cube.id == CRYSTAL_ID {
                        let rot_index = cube.orientation & 0b01111111;
                        let rot = &CUBE_ROTATIONS[rot_index as usize];
                        log::trace!("Calculating location {} connection points", i);
                        let calc_conns = calculate_absolute_connections(location_cube_connections, rot, (cube.x, cube.y, cube.z));
                        to_be_sorted.insert(i, calc_conns);
                    }
                }
                let mut sorted = Vec::with_capacity(to_be_sorted.len() + 1);
                let mut iteration = 0;
                let mut random = rand::rng();
                let mut to_be_released = Vec::new();
                while !to_be_sorted.is_empty() && iteration < bail_after_iters {
                    for (cube_i, calc_conns) in to_be_sorted.iter() {
                        let connection = is_sharing_connection(calc_conns, &available_faces)
                            .and_then(|(cube_face_i, face_available_i)| {
                                if face_available_i < connected_to_connections.connections.len() && sorted.len() < connected_to_connections.connections.len() {
                                    // prioritize finding all target connections first
                                    Some((cube_face_i, face_available_i))
                                } else if iteration < deterministic_after_iters || (face_available_i >= connected_to_connections.connections.len() && sorted.len() < connected_to_connections.connections.len()) {
                                    use rand::Rng;
                                    let random_face = random.random_range(0..calc_conns.len());
                                    if cube_face_i >= random_face {
                                        Some((cube_face_i, face_available_i))
                                    } else {
                                        None
                                    }
                                } else {
                                    Some((cube_face_i, face_available_i))
                                }
                            });
                        if connection.is_some() {
                            to_be_released.push(*cube_i);
                            let cube = &cubes[*cube_i];
                            sorted.push(CubeLocationInfo {
                                x: cube.x,
                                y: cube.y,
                                z: cube.z,
                                extras: cube.orientation,
                            });
                        }
                    }
                    for to_release in to_be_released.iter() {
                        if let Some(mut new_faces) = to_be_sorted.remove(to_release) {
                            available_faces.append(&mut new_faces);
                        }
                    }
                    // guarantee this will not endlessly try to find just target connections
                    // (this moves on to considering already-connected locations_of_id cubes)
                    log::debug!("Cube connection iteration {} found {} new connections", iteration, to_be_released.len());
                    to_be_released.clear();
                    iteration += 1;
                }
                if to_be_sorted.is_empty() {
                    log::debug!("Solved cube connections order in {} iterations", iteration);
                } else {
                    log::warn!("Failed to solve cube connections order after {} iterations (returning incomplete list)", iteration);
                }
                sorted
            }
            Err(e) => {
                log::error!("Failed to parse cube data to find cube locations by connections: {}", e);
                Vec::default()
            }
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct AbsoluteConnection {
    direction: (i8, i8, i8),
    position: (u8, u8, u8),
}

fn calculate_absolute_connections(conns: &super::connections::CubeConnections, rot: &num_quaternion::Quaternion<f32>, pos: (u8, u8, u8)) -> Vec<AbsoluteConnection> {
    let unit_rot = rot.normalize().unwrap();
    let mut abs_conns = Vec::with_capacity(conns.connections.len());
    for conn in conns.connections.iter() {
        let rotated_pos = unit_rot.rotate_vector([
            conn.position.0 as f32,
            conn.position.1 as f32,
            conn.position.2 as f32
        ]);
        let rotated_dir = unit_rot.rotate_vector([
            conn.direction.0 as f32,
            conn.direction.1 as f32,
            conn.direction.2 as f32
        ]);
        let new_conn = AbsoluteConnection {
            direction: (rotated_dir[0] as i8, rotated_dir[1] as i8, rotated_dir[2] as i8),
            position: (
                pos.0.saturating_add_signed(rotated_pos[0] as i8).saturating_add_signed(rotated_dir[0].clamp(0.0, 1.0) as i8),
                pos.1.saturating_add_signed(rotated_pos[1] as i8).saturating_add_signed(rotated_dir[1].clamp(0.0, 1.0) as i8),
                pos.2.saturating_add_signed(rotated_pos[2] as i8).saturating_add_signed(rotated_dir[2].clamp(0.0, 1.0) as i8),
            ),
        };
        log::trace!("absolute connection point {:?}", new_conn.position);
        abs_conns.push(new_conn);
    }

    abs_conns
}

fn is_sharing_connection(a: &[AbsoluteConnection], b: &[AbsoluteConnection]) -> Option<(usize, usize)> {
    for (i_a, conn_a) in a.iter().enumerate() {
        for (i_b, conn_b) in b.iter().enumerate() {
            if conn_b.position == conn_a.position
                && conn_b.direction.0 == -conn_a.direction.0
                && conn_b.direction.1 == -conn_a.direction.1
                && conn_b.direction.2 == -conn_a.direction.2 {
                return Some((i_a, i_b));
            }
        }
    }
    None
}
