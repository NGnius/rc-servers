use super::CellPoint;

const GRID_OFFSET: (i8, i8, i8) = (8, 8, 8); // weird hard-coded client thing

#[derive(Clone, Copy)]
pub struct CubeDamageInfo {
    pub location: CellPoint,
    pub max_health: u32,
    pub op: CubeChange,
}

struct CubeDescriptor {
    health: std::sync::atomic::AtomicU32,
    max_health: u32,
}

#[derive(Clone, Copy)]
pub enum CubeChange {
    Damage(u32),
    Destroy,
    Heal(u32),
}

pub struct CubeGraph {
    cubes: std::collections::HashMap<CellPoint, CubeDescriptor>,
    history: std::sync::Arc<tokio::sync::Mutex<std::collections::VecDeque<CubeDamageInfo>>>,
}

impl CubeGraph {
    pub fn with_data(r: &mut dyn std::io::Read, health_map: &std::collections::HashMap<u32, u32>) -> std::io::Result<Self> {
        let parsed_cubes = super::parser::Cube::parse_list(r)?;
        let mut cube_graph = std::collections::HashMap::<CellPoint, CubeDescriptor>::with_capacity(parsed_cubes.len());
        for cube in parsed_cubes {
            let location = CellPoint {
                x: cube.x.saturating_add_signed(GRID_OFFSET.0),
                y: cube.y.saturating_add_signed(GRID_OFFSET.1),
                z: cube.z.saturating_add_signed(GRID_OFFSET.2),
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
            history: std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::VecDeque::default())),
        })
    }

    pub fn empty() -> Self {
        Self {
            cubes: std::collections::HashMap::default(),
            history: std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::VecDeque::default())),
        }
    }

    pub async fn damage_cube(&self, point: &CellPoint, damage: u32) {
        if self.cubes.is_empty() || damage == 0 { return; }
        if let Some(cube) = self.cubes.get(point) {
            let old = cube.health.fetch_sub(damage, std::sync::atomic::Ordering::Relaxed);
            if old > damage { // manual lower bounds clamp
                cube.health.store(0, std::sync::atomic::Ordering::Relaxed);
            }
            self.history.lock().await.push_back(CubeDamageInfo {
                location: *point,
                max_health: cube.max_health,
                op: CubeChange::Damage(damage),
            });
            if damage.is_multiple_of(16) { // this can fire sort of randomly and rarely
                self.clean_history();
            }
        } else {
            log::warn!("Cannot damage cube at point ({}, {}, {}); no cube is there", point.x, point.y, point.z);
        }
    }

    pub async fn destroy_cube(&self, point: &CellPoint) {
        if self.cubes.is_empty() { return; }
        if let Some(cube) = self.cubes.get(point) {
            let old_health = cube.health.swap(0, std::sync::atomic::Ordering::Relaxed);
            if old_health != 0 {
                self.history.lock().await.push_back(CubeDamageInfo {
                    location: *point,
                    max_health: cube.max_health,
                    op: CubeChange::Destroy,
                });
                self.clean_history();
            }
        } else {
            log::warn!("Cannot destroy cube at point ({}, {}, {}); no cube is there", point.x, point.y, point.z);
        }
    }

    pub async fn heal_cube(&self, point: &CellPoint, health: u32) {
        if self.cubes.is_empty() { return; }
        if let Some(cube) = self.cubes.get(point) {
            let old = cube.health.fetch_add(health, std::sync::atomic::Ordering::Relaxed);
            let total = old + health;
            if total > cube.max_health { // manual upper bounds clamp
                cube.health.store(cube.max_health, std::sync::atomic::Ordering::Relaxed);
            }
            self.history.lock().await.push_back(CubeDamageInfo {
                location: *point,
                max_health: cube.max_health,
                op: CubeChange::Heal(health),
            });
            if total >= cube.max_health {
                self.clean_history();
            }
        } else {
            log::warn!("Cannot healh cube at point ({}, {}, {}); no cube is there", point.x, point.y, point.z);
        }
    }

    pub async fn all_changed_cubes(&self) -> Vec<CubeDamageInfo> {
        if self.cubes.is_empty() { return Vec::default(); }
        self.history.lock().await.iter().copied().collect()
    }

    pub async fn full_heal(&self) {
        for dmg in self.cubes.values() {
            dmg.health.store(dmg.max_health, std::sync::atomic::Ordering::Relaxed);
        }
        self.history.lock().await.clear();
    }

    fn clean_history(&self) {
        let history = self.history.clone();
        tokio::task::spawn(async move {
            let mut lock = history.lock().await;
            Self::clean_history_task(&mut lock).await;
        });
    }

    #[cfg(test)]
    async fn clean_history_now(&self) {
        Self::clean_history_task(&mut *self.history.lock().await).await;
    }

    async fn clean_history_task(history: &mut std::collections::VecDeque<CubeDamageInfo>) {
        // these constants feel right but they aren't tested for performance
        const MAX_SIZE: usize = 8 * 4096 / std::mem::size_of::<CubeDamageInfo>(); // 8 standard memory pages
        const MAX_WINDOW: usize = 64;
        const MIN_SIZE: usize = 16;
        // don't worry about small histories
        if history.len() < MIN_SIZE {
            return;
        }
        let original_len = history.len();
        // remove old damage/heal from destroyed cube
        let mut destroyed_cubes = std::collections::HashSet::new();
        let mut i = 0;
        while i < history.len() {
            let actual_i = history.len() - i - 1;
            let entry = &history[actual_i];
            let needs_remove = match entry.op {
                CubeChange::Destroy => {
                    !destroyed_cubes.insert(entry.location)
                },
                CubeChange::Heal(_) | CubeChange::Damage(_) => {
                    destroyed_cubes.contains(&entry.location)
                }
            };
            if !needs_remove {
                i += 1;
            } else {
                history.remove(actual_i);
            }
        }
        destroyed_cubes.clear();

        // merge duplicate damage or heal entries
        let mut i = 0;
        while i < history.len() {
            let entry = &history[i];
            let was_merged = match entry.op {
                CubeChange::Destroy => false,
                CubeChange::Damage(dmg) => {
                    let mut i2 = i + 1;
                    let mut was_merged = false;
                    'inner_dmg_loop: while i2 < history.len() && (i2 - i) < MAX_WINDOW {
                        if history[i2].location != history[i].location {
                            i2 += 1;
                            continue 'inner_dmg_loop;
                        }
                        let entry2 = &mut history[i2];
                        match entry2.op {
                            CubeChange::Damage(dmg2) => {
                                let total = (dmg + dmg2).clamp(0, entry2.max_health);
                                entry2.op = CubeChange::Damage(total);
                                history.remove(i);
                                was_merged = true;
                                break 'inner_dmg_loop;
                            },
                            CubeChange::Destroy => {
                                break 'inner_dmg_loop;
                            },
                            CubeChange::Heal(heal2) => {
                                if heal2 == dmg {
                                    history.remove(i2);
                                } else if heal2 > dmg {
                                    entry2.op = CubeChange::Heal(heal2 - dmg);
                                } else {
                                    // heal2 < dmg
                                    entry2.op = CubeChange::Damage(dmg - heal2);
                                }
                                history.remove(i);
                                was_merged = true;
                                break 'inner_dmg_loop;
                            },
                        }
                    }
                    was_merged
                },
                CubeChange::Heal(heal) => {
                    let mut i2 = i + 1;
                    let mut was_merged = false;
                    'inner_heal_loop: while i2 < history.len() && (i2 - i) < MAX_WINDOW {
                        if history[i2].location != history[i].location {
                            i2 += 1;
                            continue 'inner_heal_loop;
                        }
                        let entry2 = &mut history[i2];
                        match entry2.op {
                            CubeChange::Damage(dmg2) => {
                                if dmg2 == heal {
                                    history.remove(i2);
                                } else if dmg2 > heal {
                                    entry2.op = CubeChange::Damage(dmg2 - heal);
                                } else {
                                    // dmg2 < heal
                                    entry2.op = CubeChange::Heal(heal - dmg2);
                                }
                                history.remove(i);
                                was_merged = true;
                                break 'inner_heal_loop;
                            },
                            CubeChange::Destroy => {
                                break 'inner_heal_loop;
                            },
                            CubeChange::Heal(heal2) => {
                                let total = (heal + heal2).clamp(0, entry2.max_health);
                                entry2.op = CubeChange::Heal(total);
                                history.remove(i);
                                was_merged = true;
                                break 'inner_heal_loop;
                            }
                        }
                    }
                    was_merged
                },
            };
            if !was_merged {
                i += 1;
            }
        }

        // remove old damage/destroy for cubes that are fully healed
        let mut healed_cubes = destroyed_cubes;
        let mut i = 0;
        while i < history.len() {
            let actual_i = history.len() - i - 1;
            let entry = &history[actual_i];
            let needs_remove = match entry.op {
                CubeChange::Heal(heal) => {
                    if heal >= entry.max_health {
                        !healed_cubes.insert(entry.location)
                    } else {
                        false
                    }
                },
                CubeChange::Destroy | CubeChange::Damage(_) => {
                    healed_cubes.contains(&entry.location)
                }
            };
            if !needs_remove {
                i += 1;
            } else {
                history.remove(actual_i);
            }
        }
        healed_cubes.clear();

        // clamp size
        if history.len() > MAX_SIZE {
            log::debug!("Vehicle graph history has grown to {}/{} entries and must drop oldest entries", history.len(), MAX_SIZE);
        }
        while history.len() > MAX_SIZE {
            history.pop_front();
        }
        log::debug!("Reduced vehicle graph history from {} to {}", original_len, history.len());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INTERCEPTOR_DATA: &str = "../test/robot_data_interceptor.bin";
    #[test]
    fn test_cube_graph_gen() {
        let mut robot_data = std::fs::File::open(INTERCEPTOR_DATA).unwrap();
        let graph = CubeGraph::with_data(&mut robot_data, &Default::default()).unwrap();
        assert!(!graph.cubes.is_empty());
        for (loc, cube) in graph.cubes.iter() {
            println!("({}, {}, {}) = {} HP", loc.x, loc.y, loc.z, cube.health.load(std::sync::atomic::Ordering::Relaxed));
        }
    }

    #[tokio::test]
    async fn test_cube_graph_history_cleanup() {
        let mut robot_data = std::fs::File::open(INTERCEPTOR_DATA).unwrap();
        let graph = CubeGraph::with_data(&mut robot_data, &Default::default()).unwrap();
        assert!(!graph.cubes.is_empty());
        for (loc, _cube) in graph.cubes.iter() {
            graph.damage_cube(loc, 1).await;
            graph.heal_cube(loc, 1).await;
            graph.destroy_cube(loc).await;
        }
        graph.clean_history_now().await;
        assert_eq!(graph.all_changed_cubes().await.len(), graph.cubes.len());
    }

    #[tokio::test]
    async fn test_cube_graph_history_cleanup_one() {
        let mut robot_data = std::fs::File::open(INTERCEPTOR_DATA).unwrap();
        let graph = CubeGraph::with_data(&mut robot_data, &Default::default()).unwrap();
        assert!(!graph.cubes.is_empty());
        let first_loc = graph.cubes.keys().next().unwrap();
        for i in 0..128 {
            graph.damage_cube(first_loc, i).await;
        }
        graph.clean_history_now().await;
        assert_eq!(graph.all_changed_cubes().await.len(), 1);
    }
}
