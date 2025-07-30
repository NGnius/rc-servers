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
    map.insert(super::combat::GameMap::Earth1, MapConfig { // level3
        spawn_points: vec![
            // team 0
            SpawnPoint {
                team: 0,
                x: -226.255,
                y: -6.475,
                z: -99.293,
            },
            SpawnPoint {
                team: 0,
                x: -190.662,
                y: -6.083,
                z: -85.001,
            },
            SpawnPoint {
                team: 0,
                x: -176.454,
                y: -6.308,
                z: -95.670,
            },
            SpawnPoint {
                team: 0,
                x: -171.310,
                y: -6.534,
                z: -113.252,
            },
            SpawnPoint {
                team: 0,
                x: -228.559,
                y: -6.724,
                z: -114.155,
            },
            SpawnPoint {
                team: 0,
                x: -174.672,
                y: -6.653,
                z: -80.095,
            },
            SpawnPoint {
                team: 0,
                x: -241.176,
                y: -6.653,
                z: -104.069,
            },
            SpawnPoint {
                team: 0,
                x: -175.586,
                y: -6.653,
                z: -129.159,
            },
            SpawnPoint {
                team: 0,
                x:-222.358,
                y: -6.653,
                z: -130.716,
            },
            SpawnPoint {
                team: 0,
                x: -189.260,
                y: -6.653,
                z: -139.542,
            },
            // team 1
            SpawnPoint {
                team: 1,
                x: -183.190,
                y: -6.300,
                z: 358.657,
            },
            SpawnPoint {
                team: 1,
                x: -229.629,
                y: -7.259,
                z: 352.230,
            },
            SpawnPoint {
                team: 1,
                x: -178.319,
                y: -6.415,
                z: 377.784,
            },
            SpawnPoint {
                team: 1,
                x: -193.644,
                y: -6.300,
                z: 347.371,
            },
            SpawnPoint {
                team: 1,
                x: -186.017,
                y: -5.857,
                z: 392.741,
            },
            SpawnPoint {
                team: 1,
                x: -237.030,
                y: -6.202,
                z: 366.760,
            },
            SpawnPoint {
                team: 1,
                x: -179.863,
                y: -6.300,
                z: 344.639,
            },
            SpawnPoint {
                team: 1,
                x: -168.506,
                y: -6.439,
                z: 389.723,
            },
            SpawnPoint {
                team: 1,
                x: -244.407,
                y: -6.154,
                z: 353.739,
            },
            SpawnPoint {
                team: 1,
                x: -235.521,
                y: -6.159,
                z: -383.094,
            },
        ],
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
    map.insert(super::combat::GameMap::Earth2, MapConfig { // level4
        spawn_points: vec![
            // team 0
            SpawnPoint {
                team: 0,
                x: 225.119,
                y: -70.621,
                z: -243.719,
            },
            SpawnPoint {
                team: 0,
                x: 238.440,
                y: -71.040,
                z: -230.640,
            },
            SpawnPoint {
                team: 0,
                x: 255.238,
                y: -70.644,
                z: -225.356,
            },
            SpawnPoint {
                team: 0,
                x: 220.075,
                y: -71.426,
                z: -262.486,
            },
            SpawnPoint {
                team: 0,
                x: 272.108,
                y: -71.222,
                z: -231.268,
            },
            SpawnPoint {
                team: 0,
                x: 227.268,
                y: -72.461,
                z: -278.220,
            },
            SpawnPoint {
                team: 0,
                x: 222.239,
                y: -71.984,
                z: -226.676,
            },
            SpawnPoint {
                team: 0,
                x: 243.700,
                y: -70.474,
                z: -285.800,
            },
            SpawnPoint {
                team: 0,
                x: 282.100,
                y: -72.200,
                z: -261.200,
            },
            SpawnPoint {
                team: 0,
                x: 284.372,
                y: -71.781,
                z: -243.522,
            },
            SpawnPoint {
                team: 0,
                x: 283.800,
                y: -71.222,
                z: -278.700,
            },
            SpawnPoint {
                team: 0,
                x: 255.400,
                y: -71.426,
                z: -292.200,
            },
            SpawnPoint {
                team: 0,
                x: 291.800,
                y: -70.644,
                z: -253.800,
            },
            SpawnPoint {
                team: 0,
                x: 268.700,
                y: -71.040,
                z: -289.900,
            },
            SpawnPoint {
                team: 0,
                x: 212.800,
                y: -70.621,
                z: -249.600,
            },
            // team 1
            SpawnPoint {
                team: 1,
                x: -232.320,
                y: -70.143,
                z: 247.800,
            },
            SpawnPoint {
                team: 1,
                x: -246.719,
                y: -70.488,
                z: 231.837,
            },
            SpawnPoint {
                team: 1,
                x: -231.120,
                y: -70.868,
                z: 266.880,
            },
            SpawnPoint {
                team: 1,
                x: -265.437,
                y: -70.994,
                z: 227.632,
            },
            SpawnPoint {
                team: 1,
                x: -238.680,
                y: -70.822,
                z: 280.680,
            },
            SpawnPoint {
                team: 1,
                x: -280.197,
                y: -70.480,
                z: 236.397,
            },
            SpawnPoint {
                team: 1,
                x: -279.238,
                y: -70.681,
                z: 219.475,
            },
            SpawnPoint {
                team: 1,
                x: -230.638,
                y: -70.933,
                z: 230.638,
            },
            SpawnPoint {
                team: 1,
                x: -222.358,
                y: -71.183,
                z: 278.399,
            },
            SpawnPoint {
                team: 1,
                x: -289.680,
                y: -71.445,
                z: 250.320,
            },
            SpawnPoint {
                team: 1,
                x: -279.600,
                y: -70.480,
                z: 277.700,
            },
            SpawnPoint {
                team: 1,
                x: -269.200,
                y: -70.681,
                z: 292.100,
            },
            SpawnPoint {
                team: 1,
                x: -255.400,
                y: -70.933,
                z: 286.600,
            },
            SpawnPoint {
                team: 1,
                x: -221.700,
                y: -71.183,
                z: 257.500,
            },
            SpawnPoint {
                team: 1,
                x: -292.500,
                y: -71.445,
                z: 266.000,
            },
        ],
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
    map
}

/*const fn corner_to_center(corner: (f32, f32, f32), radius: f32) -> (f32, f32, f32) {
    (corner.0 + radius, corner.1, corner.2 + radius)
}*/
