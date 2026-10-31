use super::CellPoint;

pub struct CubeDamageInfo {
    pub location: CellPoint,
    pub max_health: u32,
    pub current_health: u32,
}

struct CubeDescriptor {
    health: std::sync::atomic::AtomicU32,
    max_health: u32,
}

pub struct CubeGraph {
    cubes: std::collections::HashMap<CellPoint, CubeDescriptor>,
}

impl CubeGraph {
    pub fn with_data(r: &mut dyn std::io::Read, health_map: &std::collections::HashMap<u32, u32>) -> std::io::Result<Self> {
        let parsed_cubes = super::parser::Cube::parse_list(r)?;
        let mut cube_graph = std::collections::HashMap::<CellPoint, CubeDescriptor>::with_capacity(parsed_cubes.len());
        for cube in parsed_cubes {
            let location = CellPoint {
                x: cube.x,
                y: cube.y,
                z: cube.z,
            };
            let is_destroyed = (cube.orientation & 0b10000000) != 0;
            let cube_health = if is_destroyed {
                0
            } else if let Some(health) = health_map.get(&cube.id) {
                *health
            } else {
                log::warn!("No health data for cube id {} in CubeGraph at ({}, {}, {})", cube.id, location.x, location.y, location.z);
                1
            };
            cube_graph.insert(location, CubeDescriptor {
                health: std::sync::atomic::AtomicU32::new(cube_health),
                max_health: health_map.get(&cube.id).copied().unwrap_or(1),
            });
        }
        Ok(Self {
            cubes: cube_graph,
        })
    }

    pub fn empty() -> Self {
        Self {
            cubes: std::collections::HashMap::default(),
        }
    }

    pub fn damage_cube(&self, point: &CellPoint, damage: u32) {
        if self.cubes.is_empty() { return; }
        if let Some(cube) = self.cubes.get(point) {
            cube.health.fetch_sub(damage, std::sync::atomic::Ordering::Relaxed);
        } else {
            log::warn!("Cannot damage cube at point ({}, {}, {}); no cube is there", point.x, point.y, point.z);
        }
    }

    pub fn destroy_cube(&self, point: &CellPoint) {
        if self.cubes.is_empty() { return; }
        if let Some(cube) = self.cubes.get(point) {
            cube.health.store(0, std::sync::atomic::Ordering::Relaxed);
        } else {
            log::warn!("Cannot destroy cube at point ({}, {}, {}); no cube is there", point.x, point.y, point.z);
        }
    }

    pub fn heal_cube(&self, point: &CellPoint, health: u32) {
        if self.cubes.is_empty() { return; }
        if let Some(cube) = self.cubes.get(point) {
            cube.health.fetch_add(health, std::sync::atomic::Ordering::Relaxed);
        } else {
            log::warn!("Cannot healh cube at point ({}, {}, {}); no cube is there", point.x, point.y, point.z);
        }
    }

    /*pub fn health_at(&self, point: &CellPoint) -> Option<CubeDamageInfo> {
        if let Some(cube) = self.cubes.get(point) {
            let health = cube.health.load(std::sync::atomic::Ordering::Relaxed);
            let is_full_health = if let Some(max_health) = self.max_health.get(&cube.cube_id) {
                health >= *max_health
            } else {
                health >= 1
            };
            Some(CubeDamageInfo {
                health,
                is_full_health,
            })
        } else {
            log::warn!("No cube health at point ({}, {}, {}); no cube is there", point.x, point.y, point.z);
            None
        }
    }*/

    pub fn all_changed_cubes(&self) -> Vec<CubeDamageInfo> {
        if self.cubes.is_empty() { return Vec::default(); }
        let mut damaged_cubes = Vec::new();
        for (loc, dmg) in self.cubes.iter() {
            let current_health = dmg.health.load(std::sync::atomic::Ordering::Relaxed);
            if dmg.max_health != current_health {
                damaged_cubes.push(CubeDamageInfo {
                    location: loc.to_owned(),
                    max_health: dmg.max_health,
                    current_health,
                });
            }
        }
        damaged_cubes
    }

    pub fn full_heal(&self) {
        for dmg in self.cubes.values() {
            dmg.health.store(dmg.max_health, std::sync::atomic::Ordering::Relaxed);
        }
    }
}
