struct CubeDescriptor {
    connected_to: std::sync::RwLock<std::collections::HashMap<CellPoint, CellPoint>>,
    connections: std::sync::Arc<Vec<Connection>>,
    //references: std::sync::atomic::AtomicU64,
    health: std::sync::atomic::AtomicU32,
}

impl std::clone::Clone for CubeDescriptor {
    fn clone(&self) -> Self {
        Self {
            connected_to: std::sync::RwLock::new(self.connected_to.read().unwrap().to_owned()),
            connections: self.connections.clone(),
            health: std::sync::atomic::AtomicU32::new(self.health.load(std::sync::atomic::Ordering::Relaxed)),
        }
    }
}

#[derive(Copy, Clone)]
struct Connection {
    position: CellPoint,
    direction: AxisDirection,
}

#[repr(u8)]
#[derive(PartialEq, Eq, Copy, Clone)]
enum AxisDirection {
    Up, // +y
    Down, // -y
    Right, // +x
    Left, // -x
    Back, // +z,
    Front, // -z
}

impl AxisDirection {
    #[inline]
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
            Self::Left => Self::Right,
            Self::Back => Self::Front,
            Self::Front => Self::Back,
        }
    }

    /// WARNING: will panic if you give it bad data
    fn from_i8_direction(value: (i8, i8, i8)) -> Self {
        match value {
            (0, 1, 0) => Self::Up,
            (0, -1, 0) => Self::Down,
            (1, 0, 0) => Self::Right,
            (-1, 0, 0) => Self::Left,
            (0, 0, 1) => Self::Back,
            (0, 0, -1) => Self::Front,
            _ => panic!("Invalid axial direction {:?}", value),
        }
    }
}

#[derive(Default)]
pub struct CubeGraph {
    cubes: std::sync::RwLock<std::collections::HashMap<CellPoint, CubeDescriptor>>,
    connects_to: std::sync::RwLock<std::collections::HashMap<CellPoint, Vec<Connection>>>,
    base: std::sync::RwLock<Option<CellPoint>>,
}

impl CubeGraph {
    pub fn with_data(r: &mut dyn std::io::Read, health_map: std::collections::HashMap<u32, u32>, root_id: u32) -> std::io::Result<Self> {
        let parsed_cubes = super::parser::Cube::parse_list(r)?;
        let mut cube_graph = std::collections::HashMap::<CellPoint, CubeDescriptor>::with_capacity(parsed_cubes.len());
        let mut connection_graph = std::collections::HashMap::<CellPoint, Vec<Connection>>::with_capacity(parsed_cubes.len());
        let mut root_location = None;
        for cube in parsed_cubes {
            let location = CellPoint {
                x: cube.x,
                y: cube.y,
                z: cube.z,
            };
            if cube.id == root_id {
                root_location = Some(location);
            }
            let conns = super::CUBE_CONNECTIONS.iter().find(|x| x.id == cube.id).unwrap_or(&super::DEFAULT_CONNECTION);
            let rot = super::CUBE_ROTATIONS[(cube.orientation & 0b01111111) as usize];
            let unit_rot = rot.normalize().unwrap();
            let is_destroyed = (cube.orientation & 0b10000000) != 0;
            if is_destroyed && cube.id != root_id { continue; }
            let cube_health = if is_destroyed {
                0
            } else if let Some(health) = health_map.get(&cube.id) {
                *health
            } else {
                log::warn!("No health data for cube id {} in CubeGraph at ({}, {}, {})", cube.id, location.x, location.y, location.z);
                1
            };
            let mut abs_connections = Vec::with_capacity(conns.connections.len());
            for conn in conns.connections.iter() {
                let conn = Self::calculate_connection(
                    (location.x, location.y, location.z),
                    conn,
                    &unit_rot,
                );
                abs_connections.push(conn);
                let connects_to = conn.position.connects_to(conn.direction);
                let graph_pointer = Connection {
                    position: location,
                    direction: conn.direction,
                };
                if let Some(connects_to_point_list) = connection_graph.get_mut(&connects_to) {
                    connects_to_point_list.push(graph_pointer);
                } else {
                    connection_graph.insert(connects_to, vec![graph_pointer]);
                }
            }
            cube_graph.insert(location, CubeDescriptor {
                connected_to: std::sync::RwLock::new(std::collections::HashMap::with_capacity(abs_connections.len())),
                connections: std::sync::Arc::new(abs_connections),
                //references: std::sync::atomic::AtomicU64::new(0),
                health: std::sync::atomic::AtomicU32::new(cube_health),
            });
        }

        if let Some(root) = root_location {
            let this = Self {
                cubes: std::sync::RwLock::new(cube_graph),
                connects_to: std::sync::RwLock::new(connection_graph),
                base: std::sync::RwLock::new(None),
            };
            this.rebase_on(&root);
            Ok(this)
        } else {
            Err(std::io::Error::other("Graph root cube not found"))
        }
    }

    /*fn reset(&self) {
        for cube in self.cubes.values() {
            cube.references.store(0, std::sync::atomic::Ordering::Relaxed);
        }
    }*/

    fn rebase_on(&self, point: &CellPoint) {
        let cubes = self.cubes.read().unwrap();
        let connects_to = self.connects_to.read().unwrap();
        if cubes.is_empty() { return; }
        if !cubes.contains_key(point) {
            log::warn!("Cannot rebase CubeGraph at point ({}, {}, {}); no cube is there", point.x, point.y, point.z);
            return;
        }
        if cubes.len() == 1 {
            // trivial case, base is the only cube (no need to connect anything)
            *self.base.write().unwrap() = Some(*point);
            return;
        }
        let mut seen = std::collections::HashSet::with_capacity(cubes.len());
        seen.insert(*point);
        let mut to_be_processed = std::collections::HashSet::new();
        to_be_processed.insert(*point);
        while !to_be_processed.is_empty() {
            let to_be_processed_now = to_be_processed.clone();
            to_be_processed.clear();
            for loc in to_be_processed_now.iter() {
                let cube = cubes.get(loc).unwrap();
                //let src_ref_count = cube.references.load(std::sync::atomic::Ordering::Relaxed);
                for conn in cube.connections.iter() {
                    if cube.connected_to.read().unwrap().contains_key(&conn.position) { continue; } // already connected
                    let opposite_direction = conn.direction.opposite();
                    if let Some(other_conns) = connects_to.get(&conn.position) {
                        for other_conn in other_conns.iter() {
                            if other_conn.direction != opposite_direction { continue; }
                            let other_cube = cubes.get(&other_conn.position).unwrap();
                            cube.connected_to.write().unwrap().insert(conn.position, other_conn.position);
                            let connected_to = conn.position.connects_to(conn.direction);
                            other_cube.connected_to.write().unwrap().insert(connected_to, *loc);
                            //cubes.get(&other_conn.position).unwrap().references.fetch_add(src_ref_count, std::sync::atomic::Ordering::Relaxed);
                            if seen.insert(other_conn.position) {
                                to_be_processed.insert(other_conn.position);
                            }
                        }
                    }
                }
            }
        }
        *self.base.write().unwrap() = Some(*point);
    }

    pub fn add_cube(&self, point: &CellPoint, cube_id: u32, health: u32, extras: u8) {
        let conns = super::CUBE_CONNECTIONS.iter().find(|x| x.id == cube_id).unwrap_or(&super::DEFAULT_CONNECTION);
        let rot = super::CUBE_ROTATIONS[(extras & 0b01111111) as usize];
        let unit_rot = rot.normalize().unwrap();
        //let is_destroyed = health == 0 || (extras & 0b10000000) != 0;
        let mut abs_connections = Vec::with_capacity(conns.connections.len());
        //let mut ref_count = 0;
        let mut connects_to_lock = self.connects_to.write().unwrap();
        let mut connected_to = std::collections::HashMap::with_capacity(conns.connections.len());
        for conn in conns.connections.iter() {
            let conn = Self::calculate_connection(
                (point.x, point.y, point.z),
                conn,
                &unit_rot,
            );
            abs_connections.push(conn);
            let connects_to = conn.position.connects_to(conn.direction);
            let opposite_direction = conn.direction.opposite();
            let graph_pointer = Connection {
                position: *point,
                direction: conn.direction,
            };
            if let Some(connects_to_point_list) = connects_to_lock.get_mut(&connects_to) {
                connects_to_point_list.push(graph_pointer);
            } else {
                connects_to_lock.insert(connects_to, vec![graph_pointer]);
            }
            if let Some(connects_to_point_list) = connects_to_lock.get_mut(&conn.position) {
                let cubes_lock = self.cubes.read().unwrap();
                for other_cube_loc in connects_to_point_list.iter() {
                    if other_cube_loc.direction != opposite_direction { continue; }
                    let other_cube = cubes_lock.get(&other_cube_loc.position).unwrap();
                    //ref_count += other_cube.references.load(std::sync::atomic::Ordering::Relaxed);
                    other_cube.connected_to.write().unwrap().insert(connects_to, *point);
                    connected_to.insert(conn.position, other_cube_loc.position);
                }
            }
        }
        drop(connects_to_lock);
        self.cubes.write().unwrap().insert(*point, CubeDescriptor {
            connected_to: std::sync::RwLock::new(connected_to),
            connections: std::sync::Arc::new(abs_connections),
            //references: std::sync::atomic::AtomicU64::new(ref_count),
            health: std::sync::atomic::AtomicU32::new(health),
        });
    }

    pub fn remove_cube(&self, point: &CellPoint) -> std::collections::HashSet<CellPoint> {
        self.remove_cubes_and_disconnects(&[*point])
    }

    fn remove_cube_only(&self, point: &CellPoint, cubes: &mut std::collections::HashMap<CellPoint, CubeDescriptor>) {
        if let Some(cube) = cubes.remove(point) {
            let mut connects_to = self.connects_to.write().unwrap();
            let connected_to = cube.connected_to.read().unwrap();
            for conn in cube.connections.iter() {
                let my_connects_to = conn.position.connects_to(conn.direction);
                connects_to.get_mut(&my_connects_to).unwrap().retain(|x| !(&x.position == point && x.direction == conn.direction));
            }
            for other_conn in connected_to.values() {
                let other_cube = cubes.get(other_conn).unwrap();
                other_cube.connected_to.write().unwrap().retain(|_, conn| conn != point);
            }
        } else {
            log::warn!("Cannot remove cube at point ({}, {}, {}); no cube is there", point.x, point.y, point.z);
        }
    }

    fn remove_cubes_and_disconnects(&self, points: &[CellPoint]) -> std::collections::HashSet<CellPoint> {
        let mut cubes = self.cubes.write().unwrap();
        for to_remove in points.iter() {
            self.remove_cube_only(to_remove, &mut cubes);
        }
        let mut chunks = ChunkTracker::with_capacity(6);
        for (point, cube) in cubes.iter() {
            chunks.add_cube(*point, cube.connected_to.read().unwrap().values());
        }
        if let Some(root) = *self.base.read().unwrap() {
            let disconnected_cubes = chunks.cubes_not_in_chunk(root);
            for to_remove in disconnected_cubes.iter() {
                if !cubes.contains_key(to_remove) { continue; }
                self.remove_cube_only(to_remove, &mut cubes);
            }
            disconnected_cubes
        } else {
            log::warn!("Cannot calculate disconnections without base cube of CubeGraph");
            Default::default()
        }
    }

    pub fn damage_cube(&self, point: &CellPoint, damage: u32) {
        let cubes = self.cubes.read().unwrap();
        if let Some(cube) = cubes.get(point) {
            cube.health.fetch_sub(damage, std::sync::atomic::Ordering::Relaxed);
        } else {
            log::warn!("Cannot damage cube at point ({}, {}, {}); no cube is there", point.x, point.y, point.z);
        }
    }

    #[inline]
    fn calculate_connection(location: (u8, u8, u8), relative_conn: &crate::cubes::connections::CubeConnection, unit_rot: &num_quaternion::UnitQuaternion<f32>) -> Connection {
        let rotated_pos = unit_rot.rotate_vector([
            relative_conn.position.0 as f32,
            relative_conn.position.1 as f32,
            relative_conn.position.2 as f32
        ]);
        let rotated_dir = unit_rot.rotate_vector([
            relative_conn.direction.0 as f32,
            relative_conn.direction.1 as f32,
            relative_conn.direction.2 as f32
        ]);
        let abs_point = CellPoint {
            x: location.0.saturating_add_signed(rotated_pos[0] as i8),
            y: location.1.saturating_add_signed(rotated_pos[1] as i8),
            z: location.2.saturating_add_signed(rotated_pos[2] as i8),
        };
        let direction = AxisDirection::from_i8_direction((rotated_dir[0] as i8, rotated_dir[1] as i8, rotated_dir[2] as i8));
        Connection {
            position: abs_point,
            direction,
        }
    }
}

impl std::clone::Clone for CubeGraph {
    fn clone(&self) -> Self {
        Self {
            cubes: std::sync::RwLock::new(self.cubes.read().unwrap().to_owned()),
            connects_to: std::sync::RwLock::new(self.connects_to.read().unwrap().to_owned()),
            base: std::sync::RwLock::new(self.base.read().unwrap().to_owned()),
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct CellPoint {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl CellPoint {
    #[inline]
    fn connects_to(&self, direction: AxisDirection) -> Self {
        match direction {
            AxisDirection::Up => Self {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            AxisDirection::Down => Self {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            AxisDirection::Right => Self {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            AxisDirection::Left => Self {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            AxisDirection::Back => Self {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
            AxisDirection::Front => Self {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
        }
    }
}

impl std::convert::From<(u8, u8, u8)> for CellPoint {
    fn from(value: (u8, u8, u8)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

struct ChunkTracker {
    chunks: Vec<std::collections::HashSet<CellPoint>>,
}

impl ChunkTracker {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            chunks: Vec::with_capacity(capacity),
        }
    }

    fn add_cube<'a>(&mut self, new_cube: CellPoint, connects_to: impl std::iter::Iterator<Item=&'a CellPoint>) {
        let connections: Vec<_> = connects_to.copied().collect();
        let mut found_in_chunks = Vec::with_capacity(self.chunks.len());
        for (i, chunk) in self.chunks.iter_mut().enumerate() {
            for connected_to in connections.iter() {
                if chunk.contains(connected_to) && !found_in_chunks.contains(&i) {
                    chunk.insert(new_cube);
                    found_in_chunks.push(i);
                }
            }
        }
        if found_in_chunks.is_empty() {
            let mut new_chunk = std::collections::HashSet::<CellPoint>::new();
            new_chunk.insert(new_cube);
            for connected_to in connections.iter() {
                new_chunk.insert(*connected_to);
            }
            self.chunks.push(new_chunk);
        } else {
            self.merge_chunks(found_in_chunks, &connections);
        }
    }

    fn merge_chunks(&mut self, mut chunks_to_merge: Vec<usize>, connections: &[CellPoint]) {
        if chunks_to_merge.is_empty() || chunks_to_merge.len() == 1 { return; }
        // by sorting and then reversing the order it is guaranteed that
        // self.chunks.swap_remove(chunks_to_merge[i]) will not affect the index of other chunks
        chunks_to_merge.sort();
        chunks_to_merge.reverse();
        let mut super_chunk = self.chunks.swap_remove(chunks_to_merge[0]);
        for chunk_i in chunks_to_merge[1..].iter() {
            let to_merge = self.chunks.swap_remove(*chunk_i);
            for point in to_merge {
                super_chunk.insert(point);
            }
        }
        for connected_to in connections.iter() {
            super_chunk.insert(*connected_to);
        }
        self.chunks.push(super_chunk);
    }

    fn cubes_not_in_chunk(mut self, point: CellPoint) -> std::collections::HashSet<CellPoint> {
        let chunk_indices: Vec<_> = self.chunks.iter().enumerate()
            .filter(|(_, chunk)| !chunk.contains(&point))
            .map(|(i, _)| i)
            .collect();
        self.merge_chunks(chunk_indices, &[]);
        if self.chunks[0].contains(&point) {
            if self.chunks.len() == 1 {
                Default::default()
            } else {
                self.chunks.swap_remove(1)
            }
        } else {
            self.chunks.swap_remove(0)
        }
    }
}
