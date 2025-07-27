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
    pub percent_per_second: f32,
}

const DEFAULT_PERCENT_PER_SECOND: f32 = 2.5;
const DEFAULT_BASE_RADIUS: f32 = 20.0;

pub(super) fn default_map() -> std::collections::HashMap<super::combat::GameMap, MapConfig> {
    let mut map = std::collections::HashMap::with_capacity(9);
    //let coords_t0 = corner_to_center((6.60, 4.09, 20.3), 10.0);
    //let coords_t1 = corner_to_center((364.60, 10.63, 372.20), 10.0);
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
                x: 6.60,
                y: 4.09,
                z: 20.3,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_PERCENT_PER_SECOND,
            },
            CaptureBase {
                team: 1,
                x: 364.60,
                y: 10.63,
                z: 372.20,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_PERCENT_PER_SECOND,
            },
        ],
    });
    map.insert(super::combat::GameMap::Mars2, MapConfig {
        spawn_points: vec![],
        bases: vec![
            CaptureBase {
                team: 0,
                x: 230.520,
                y: 21.140,
                z: 187.560,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_PERCENT_PER_SECOND,
            },
            CaptureBase {
                team: 1,
                x: 661.680,
                y: 21.230,
                z: 620.280,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_PERCENT_PER_SECOND,
            },
        ],
    });
    map.insert(super::combat::GameMap::Mars3, MapConfig {
        spawn_points: vec![],
        bases: vec![
            CaptureBase {
                team: 0,
                x: 50.880,
                y: 32.340,
                z: -309.720,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_PERCENT_PER_SECOND,
            },
            CaptureBase {
                team: 1,
                x: 50.280,
                y: 32.320,
                z: 207.360,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_PERCENT_PER_SECOND,
            },
        ],
    });
    map.insert(super::combat::GameMap::Neptune1, MapConfig {
        spawn_points: vec![],
        bases: vec![
            CaptureBase {
                team: 0,
                x: -97.718,
                y: 1.580,
                z: -82.150,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_PERCENT_PER_SECOND,
            },
            CaptureBase {
                team: 1,
                x: -453.065,
                y: -0.110,
                z: 292.956,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_PERCENT_PER_SECOND,
            },
        ],
    });
    map.insert(super::combat::GameMap::Neptune2, MapConfig {
        spawn_points: vec![],
        bases: vec![
            CaptureBase {
                team: 0,
                x: 195.972,
                y: 18.370,
                z: -181.488,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_PERCENT_PER_SECOND,
            },
            CaptureBase {
                team: 1,
                x: -182.556,
                y: 18.390,
                z: 196.416,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_PERCENT_PER_SECOND,
            },
        ],
    });
    map.insert(super::combat::GameMap::Neptune3, MapConfig {
        spawn_points: vec![],
        bases: vec![
            CaptureBase {
                team: 0,
                x: 143.880,
                y: 42.270,
                z: -151.440,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_PERCENT_PER_SECOND,
            },
            CaptureBase {
                team: 1,
                x: -149.520,
                y: 42.290,
                z: 147.240,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_PERCENT_PER_SECOND,
            },
        ],
    });
    map.insert(super::combat::GameMap::Earth1, MapConfig {
        spawn_points: vec![],
        bases: vec![
            CaptureBase {
                team: 0,
                x: -43.824,
                y: 2.700,
                z: -240.888,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_PERCENT_PER_SECOND,
            },
            CaptureBase {
                team: 1,
                x: -52.356,
                y: 2.770,
                z: 243.600,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_PERCENT_PER_SECOND,
            },
        ],
    });
    map.insert(super::combat::GameMap::Earth2, MapConfig {
        spawn_points: vec![],
        bases: vec![
            CaptureBase {
                team: 0,
                x: -256.680,
                y: 13.900,
                z: -253.920,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_PERCENT_PER_SECOND,
            },
            CaptureBase {
                team: 1,
                x: 258.000,
                y: 13.900,
                z: 259.200,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_PERCENT_PER_SECOND,
            },
        ],
    });
    map
}

/*const fn corner_to_center(corner: (f32, f32, f32), radius: f32) -> (f32, f32, f32) {
    (corner.0 + radius, corner.1, corner.2 + radius)
}*/
