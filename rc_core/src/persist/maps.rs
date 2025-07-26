use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MapsConfig {
    #[serde(default = "default_map")]
    pub map: std::collections::HashMap<super::combat::GameMap, MapConfig>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MapConfig {
    pub spawn_points: Vec<SpawnPoint>,
    pub bases: Vec<CaptureBase>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpawnPoint {
    pub team: u8,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CaptureBase {
    pub team: u8,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub radius: f32,
}

pub(super) fn default_map() -> std::collections::HashMap<super::combat::GameMap, MapConfig> {
    let mut map = std::collections::HashMap::with_capacity(9);
    let coords_t0 = corner_to_center((6.60, 4.09, 20.3), 10.0);
    let coords_t1 = corner_to_center((364.60, 10.63, 372.20), 10.0);
    map.insert(super::combat::GameMap::Mars1, MapConfig {
        spawn_points: vec![
            // team 0
            SpawnPoint {
                team: 0,
                x: 32.07,
                y: 1.73,
                z: 49.75,
            },
            SpawnPoint {
                team: 0,
                x: 39.20,
                y: 1.73,
                z: 40.21,
            },
            SpawnPoint {
                team: 0,
                x: 21.14,
                y: 1.73,
                z: 44.16,
            },
            SpawnPoint {
                team: 0,
                x: 32.50,
                y: 1.73,
                z: 30.66,
            },
            SpawnPoint {
                team: 0,
                x: 31.1,
                y: 1.73,
                z: 6.80,
            },
            SpawnPoint {
                team: 0,
                x: 43.40,
                y: 1.73,
                z: 8.60,
            },
            SpawnPoint {
                team: 0,
                x: 36.00,
                y: 1.73,
                z: 18.70,
            },
            SpawnPoint {
                team: 0,
                x: 3.00,
                y: 1.73,
                z: 57.4,
            },
            SpawnPoint {
                team: 0,
                x: 9.90,
                y: 1.73,
                z: 47.90,
            },
            SpawnPoint {
                team: 0,
                x: -2.40,
                y: 1.73,
                z: 46.40,
            },
            // team 1
            SpawnPoint {
                team: 1,
                x: 346.09,
                y: 8.10,
                z: 339.18,
            },
            SpawnPoint {
                team: 1,
                x: 337.10,
                y: 8.10,
                z: 346.80,
            },
            SpawnPoint {
                team: 1,
                x: 356.10,
                y: 8.10,
                z: 344.90,
            },
            SpawnPoint {
                team: 1,
                x: 340.10,
                y: 8.10,
                z: 358.20,
            },
            SpawnPoint {
                team: 1,
                x: 327.15,
                y: 8.10,
                z: 381.87,
            },
            SpawnPoint {
                team: 1,
                x: 339.10,
                y: 8.10,
                z: 383.60,
            },
            SpawnPoint {
                team: 1,
                x: 334.80,
                y: 8.10,
                z: 372.40,
            },
            SpawnPoint {
                team: 1,
                x: 382.50,
                y: 8.10,
                z: 335.10,
            },
            SpawnPoint {
                team: 1,
                x: 373.10,
                y: 8.10,
                z: 342.40,
            },
            SpawnPoint {
                team: 1,
                x: 384.50,
                y: 8.10,
                z: 346.80,
            },
        ],
        bases: vec![
            CaptureBase {
                team: 0,
                x: coords_t0.0,
                y: coords_t0.1,
                z: coords_t0.2,
                radius: 10.0,
            },
            CaptureBase {
                team: 1,
                x: coords_t1.0,
                y: coords_t1.1,
                z: coords_t1.2,
                radius: 10.0,
            },
        ],
    });
    map.insert(super::combat::GameMap::Mars2, MapConfig {
        spawn_points: vec![],
        bases: vec![],
    });
    map.insert(super::combat::GameMap::Mars3, MapConfig {
        spawn_points: vec![],
        bases: vec![],
    });
    map.insert(super::combat::GameMap::Neptune1, MapConfig {
        spawn_points: vec![],
        bases: vec![],
    });
    map.insert(super::combat::GameMap::Neptune2, MapConfig {
        spawn_points: vec![],
        bases: vec![],
    });
    map.insert(super::combat::GameMap::Neptune3, MapConfig {
        spawn_points: vec![],
        bases: vec![],
    });
    map.insert(super::combat::GameMap::Earth1, MapConfig {
        spawn_points: vec![],
        bases: vec![],
    });
    map.insert(super::combat::GameMap::Earth2, MapConfig {
        spawn_points: vec![],
        bases: vec![],
    });
    map
}

const fn corner_to_center(corner: (f32, f32, f32), radius: f32) -> (f32, f32, f32) {
    (corner.0 + radius, corner.1, corner.2 + radius)
}
