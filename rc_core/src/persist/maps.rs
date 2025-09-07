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
    pub capture_points: Vec<CapturePoint>,
    pub equalizer: Point,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpawnPoint {
    pub team: u8,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl SpawnPoint {
    const fn offset(mut self, x: f32, y: f32, z: f32) -> Self {
        self.x += x;
        self.y += y;
        self.z += z;
        self
    }

    const fn scale(mut self, scale: f32) -> Self {
        self.x *= scale;
        self.y *= scale;
        self.z *= scale;
        self
    }

    fn rotated(mut self, rot: num_quaternion::Quaternion<f32>) -> Self {
        let unit_rot = rot.normalize().expect("Bad rotation quaternion for SpawnPoint");
        let rotated = unit_rot.rotate_vector([self.x, self.y, self.z]);
        self.x = rotated[0];
        self.y = rotated[1];
        self.z = rotated[2];
        self
    }
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

impl CaptureBase {
    const fn offset(mut self, x: f32, y: f32, z: f32) -> Self {
        self.x += x;
        self.y += y;
        self.z += z;
        self
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CapturePoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub radius: f32,
    pub percent_per_second: f32,
}

impl CapturePoint {
    const fn offset(mut self, x: f32, y: f32, z: f32) -> Self {
        self.x += x;
        self.y += y;
        self.z += z;
        self
    }

    fn rotated(mut self, rot: num_quaternion::Quaternion<f32>) -> Self {
        let unit_rot = rot.normalize().expect("Bad rotation quaternion for CapturePoint");
        let rotated = unit_rot.rotate_vector([self.x, self.y, self.z]);
        self.x = rotated[0];
        self.y = rotated[1];
        self.z = rotated[2];
        self
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    #[allow(dead_code)]
    const fn offset(mut self, x: f32, y: f32, z: f32) -> Self {
        self.x += x;
        self.y += y;
        self.z += z;
        self
    }

    #[allow(dead_code)]
    fn rotated(mut self, rot: num_quaternion::Quaternion<f32>) -> Self {
        let unit_rot = rot.normalize().expect("Bad rotation quaternion for Point");
        let rotated = unit_rot.rotate_vector([self.x, self.y, self.z]);
        self.x = rotated[0];
        self.y = rotated[1];
        self.z = rotated[2];
        self
    }
}

const DEFAULT_BASE_PERCENT_PER_SECOND: f32 = 2.5;
const DEFAULT_BASE_RADIUS: f32 = 20.0;
const DEFAULT_CAPTURE_PERCENT_PER_SECOND: f32 = DEFAULT_BASE_PERCENT_PER_SECOND * 1.5;
const DEFAULT_CAPTURE_RADIUS: f32 = 14.0;

#[allow(clippy::approx_constant)]
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
                y: -6.475 + 10.0,
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
        ].into_iter().map(|x| x.offset(156.360, 9.983, -129.000)).collect(),
        bases: vec![
            CaptureBase {
                team: 0,
                x: -43.824,
                y: 2.700,
                z: -240.888,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_BASE_PERCENT_PER_SECOND,
            },
            CaptureBase {
                team: 1,
                x: -52.356,
                y: 2.770,
                z: 243.600,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_BASE_PERCENT_PER_SECOND,
            },
        ],
        capture_points: vec![
            CapturePoint {
                x: 177.240,
                y: -0.264,
                z: 141.540,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            },
            CapturePoint {
                x: -9.480,
                y: 30.396,
                z: -164.676,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            },
            CapturePoint {
                x: -197.400,
                y: -0.228,
                z: 142.452,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            }
        ].into_iter().map(|p| p.rotated(num_quaternion::Quaternion {
            w: 0.707107,
            x: 0.0,
            y: 0.707107,
            z: 0.0,
        }).offset(13.320, 0.0, -9.84)).collect(),
        equalizer: Point {
            x: 1.404,
            y: 14.3,
            z: -0.588,
        },
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
        ].into_iter().map(|x| x.rotated(num_quaternion::Quaternion {
            x: 0.0,
            y: 0.707107,
            z: 0.0,
            w: 0.707107,
        }).offset(0.0, 86.718, 0.0)).collect(),
        bases: vec![
            CaptureBase {
                team: 0,
                x: -256.680,
                y: 13.900,
                z: -253.920,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_BASE_PERCENT_PER_SECOND,
            },
            CaptureBase {
                team: 1,
                x: 258.000,
                y: 13.900,
                z: 259.200,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_BASE_PERCENT_PER_SECOND,
            },
        ],
        capture_points: vec![], // TODO
        equalizer: Point { // TODO
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
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
                percent_per_second: DEFAULT_BASE_PERCENT_PER_SECOND,
            },
            CaptureBase {
                team: 1,
                x: 364.60,
                y: 10.63,
                z: 372.20,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_BASE_PERCENT_PER_SECOND,
            },
        ],
        capture_points: vec![], // TODO
        equalizer: Point { // TODO
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    });
    map.insert(super::combat::GameMap::Mars2, MapConfig {
        spawn_points: vec![
            // team 0
            SpawnPoint {
                team: 0,
                x: -198.924,
                y: -14.583,
                z: -201.221,
            },
            SpawnPoint {
                team: 0,
                x: -181.922,
                y: -14.038,
                z: -213.642,
            },
            SpawnPoint {
                team: 0,
                x: -183.744,
                y: -13.899,
                z: -196.812,
            },
            SpawnPoint {
                team: 0,
                x: -237.587,
                y: -12.163,
                z: -213.035,
            },
            SpawnPoint {
                team: 0,
                x: -222.539,
                y: -12.982,
                z: -207.438,
            },
            SpawnPoint {
                team: 0,
                x: -232.544,
                y: -12.942,
                z: -229.363,
            },
            SpawnPoint {
                team: 0,
                x: -165.911,
                y: -12.618,
                z: -243.646,
            },
            SpawnPoint {
                team: 0,
                x: -177.804,
                y: -13.899,
                z: -231.290,
            },
            SpawnPoint {
                team: 0,
                x: -186.688,
                y: -13.650,
                z: -251.367,
            },
            SpawnPoint {
                team: 0,
                x: -219.357,
                y: -13.650,
                z: -252.358,
            },
            // team 1
            SpawnPoint {
                team: 1,
                x: 202.422,
                y: -14.396,
                z: 199.954,
            },
            SpawnPoint {
                team: 1,
                x: 214.672,
                y: -13.189,
                z: 181.421,
            },
            SpawnPoint {
                team: 1,
                x: 252.463,
                y: -14.752,
                z: 221.509,
            },
            SpawnPoint {
                team: 1,
                x: 237.125,
                y: -13.703,
                z: 179.705,
            },
            SpawnPoint {
                team: 1,
                x: 255.156,
                y: -14.128,
                z: 176.603,
            },
            SpawnPoint {
                team: 1,
                x: 255.433,
                y: -10.939,
                z: 197.208,
            },
            SpawnPoint {
                team: 1,
                x: 217.628,
                y: -11.689,
                z: 241.903,
            },
            SpawnPoint {
                team: 1,
                x: 234.023,
                y: -8.302,
                z: 233.468,
            },
            SpawnPoint {
                team: 1,
                x: 207.966,
                y: -13.330,
                z: 226.684,
            },
            SpawnPoint {
                team: 1,
                x: 197.538,
                y: -13.869,
                z: 184.945,
            },
        ].into_iter().map(|x| x.offset(0.0, 36.0796, 0.0)).collect(),
        bases: vec![
            CaptureBase {
                team: 0,
                x: 230.520,
                y: 21.140,
                z: 187.560,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_BASE_PERCENT_PER_SECOND,
            },
            CaptureBase {
                team: 1,
                x: 661.680,
                y: 21.230,
                z: 620.280,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_BASE_PERCENT_PER_SECOND,
            },
        ].into_iter().map(|x| x.offset(-434.640, 0.0, -414.720)).collect(),
        capture_points: vec![], // TODO
        equalizer: Point { // TODO
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    });
    map.insert(super::combat::GameMap::Mars3, MapConfig {
        spawn_points: vec![
            // team 0
            SpawnPoint {
                team: 0,
                x: 86.581,
                y: 36.306,
                z: -234.321,
            },
            SpawnPoint {
                team: 0,
                x: 73.656,
                y: 38.254,
                z: -248.292,
            },
            SpawnPoint {
                team: 0,
                x: 103.320,
                y: 38.896,
                z: -232.269,
            },
            SpawnPoint {
                team: 0,
                x: 121.271,
                y: 38.397,
                z: -244.241,
            },
            SpawnPoint {
                team: 0,
                x: 129.254,
                y: 37.3032,
                z: -261.716,
            },
            SpawnPoint {
                team: 0,
                x: 60.802,
                y: 36.021,
                z: -256.596,
            },
            SpawnPoint {
                team: 0,
                x: 73.300,
                y: 37.185,
                z: -267.537,
            },
            SpawnPoint {
                team: 0,
                x: 138.259,
                y: 35.640,
                z: -248.434,
            },
            SpawnPoint {
                team: 0,
                x: 90.537,
                y: 36.971,
                z: -218.770,
            },
            SpawnPoint {
                team: 0,
                x: 82.328,
                y: 38.016,
                z: -279.655,
            },
            // team 1
            SpawnPoint {
                team: 1,
                x: 72.801,
                y: 38.373,
                z: 251.072,
            },
            SpawnPoint {
                team: 1,
                x: 85.298,
                y: 38.302,
                z: 237.315,
            },
            SpawnPoint {
                team: 1,
                x: 105.043,
                y: 38.397,
                z: 234.796,
            },
            SpawnPoint {
                team: 1,
                x: 121.877,
                y: 38.005,
                z: 246.166,
            },
            SpawnPoint {
                team: 1,
                x: 128.399,
                y: 38.254,
                z: 264.461,
            },
            SpawnPoint {
                team: 1,
                x: 92.902,
                y: 37.874,
                z: 222.394,
            },
            SpawnPoint {
                team: 1,
                x: 69.902,
                y: 37.304,
                z: 269.355,
            },
            SpawnPoint {
                team: 1,
                x: 57.143,
                y: 37.767,
                z: 256.845,
            },
            SpawnPoint {
                team: 1,
                x: 139.115,
                y: 36.745,
                z: 249.598,
            },
            SpawnPoint {
                team: 1,
                x: 84.027,
                y: 39.537,
                z: 283.955,
            },
        ].into_iter().map(|x| x.offset(0.0, 4.1933, 0.0)).collect(),
        bases: vec![
            CaptureBase {
                team: 0,
                x: 50.880,
                y: 32.340,
                z: -309.720,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_BASE_PERCENT_PER_SECOND,
            },
            CaptureBase {
                team: 1,
                x: 50.280,
                y: 32.320,
                z: 207.360,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_BASE_PERCENT_PER_SECOND,
            },
        ].into_iter().map(|x| x.offset(49.608, 0.0, 52.493)).collect(),
        capture_points: vec![], // TODO
        equalizer: Point { // TODO
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    });
    map.insert(super::combat::GameMap::Neptune1, MapConfig {
        spawn_points: vec![
            // team 0
            SpawnPoint {
                team: 0,
                x: -2.100,
                y: -0.669,
                z: -42.500,
            },
            SpawnPoint {
                team: 0,
                x: -6.000,
                y: -2.493,
                z: -27.600,
            },
            SpawnPoint {
                team: 0,
                x: -7.600,
                y: -1.200,
                z: -61.900,
            },
            SpawnPoint {
                team: 0,
                x: 9.300,
                y: -1.598,
                z: -29.800,
            },
            SpawnPoint {
                team: 0,
                x: 24.500,
                y: -2.361,
                z: -22.900,
            },
            SpawnPoint {
                team: 0,
                x: -15.500,
                y: -2.818,
                z: -75.900,
            },
            SpawnPoint {
                team: 0,
                x: -2.400,
                y: -3.722,
                z: -83.100,
            },
            SpawnPoint {
                team: 0,
                x: 39.200,
                y: -3.310,
                z: -13.900,
            },
            SpawnPoint {
                team: 0,
                x: 46.000,
                y: -4.140,
                z: -27.300,
            },
            SpawnPoint {
                team: 0,
                x: 57.800,
                y: -3.741,
                z: -35.700,
            },
            // team 1
            SpawnPoint {
                team: 1,
                x: -378.600,
                y: -5.531,
                z: 379.400,
            },
            SpawnPoint {
                team: 1,
                x: -393.800,
                y: -5.262,
                z: 382.200,
            },
            SpawnPoint {
                team: 1,
                x: -381.300,
                y: -4.766,
                z: 394.500,
            },
            SpawnPoint {
                team: 1,
                x: -377.400,
                y: -5.798,
                z: 418.700,
            },
            SpawnPoint {
                team: 1,
                x: -367.400,
                y: -5.523,
                z: 407.300,
            },
            SpawnPoint {
                team: 1,
                x: -425.100,
                y: -5.567,
                z: 363.000,
            },
            SpawnPoint {
                team: 1,
                x: -412.400,
                y: -4.945,
                z: 373.200,
            },
            SpawnPoint {
                team: 1,
                x: -430.900,
                y: -5.413,
                z: 377.600,
            },
            SpawnPoint {
                team: 1,
                x: -372.500,
                y: -5.051,
                z: 434.100,
            },
            SpawnPoint {
                team: 1,
                x: -388.300,
                y: -4.076,
                z: 435.700,
            },
        ].into_iter().map(|x| x.offset(283.600, 4.592, -24.100).scale(0.8)).collect(),
        bases: vec![
            CaptureBase {
                team: 0,
                x: -97.718,
                y: 1.580,
                z: -82.150,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_BASE_PERCENT_PER_SECOND,
            },
            CaptureBase {
                team: 1,
                x: -453.065,
                y: -0.110,
                z: 292.956,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_BASE_PERCENT_PER_SECOND,
            },
        ].into_iter().map(|x| x.offset(405.542, 0.0, 10.668)).collect(),
        capture_points: vec![], // TODO
        equalizer: Point { // TODO
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    });
    map.insert(super::combat::GameMap::Neptune2, MapConfig {
        spawn_points: vec![
            // team 0
            SpawnPoint {
                team: 0,
                x: -211.622,
                y: -84.836,
                z: -146.150,
            },
            SpawnPoint {
                team: 0,
                x: -218.613,
                y: -84.447,
                z: -160.734,
            },
            SpawnPoint {
                team: 0,
                x: -188.728,
                y: -84.449,
                z: -137.079,
            },
            SpawnPoint {
                team: 0,
                x: -214.104,
                y: -85.091,
                z: -180.661,
            },
            SpawnPoint {
                team: 0,
                x: -166.816,
                y: -84.447,
                z: -156.735,
            },
            SpawnPoint {
                team: 0,
                x: -203.100,
                y: -84.447,
                z: -191.041,
            },
            SpawnPoint {
                team: 0,
                x: -171.769,
                y: -83.701,
                z: -139.698,
            },
            SpawnPoint {
                team: 0,
                x: -226.649,
                y: -84.441,
                z: -184.727,
            },
            SpawnPoint {
                team: 0,
                x: -178.042,
                y: -84.407,
                z: -126.921,
            },
            SpawnPoint {
                team: 0,
                x: -563.460,
                y: -84.447,
                z: 185.603,
            },
            // team 1
            SpawnPoint {
                team: 1,
                x: -563.460,
                y: -84.447,
                z: 185.603,
            },
            SpawnPoint {
                team: 1,
                x: -545.107,
                y: -84.501,
                z: 225.604,
            },
            SpawnPoint {
                team: 1,
                x: -589.174,
                y: -85.324,
                z: 192.699,
            },
            SpawnPoint {
                team: 1,
                x: -596.323,
                y: -84.447,
                z: 222.468,
            },
            SpawnPoint {
                team: 1,
                x: -549.553,
                y: -84.667,
                z: 193.565,
            },
            SpawnPoint {
                team: 1,
                x: -596.682,
                y: -85.404,
                z: 205.550,
            },
            SpawnPoint {
                team: 1,
                x: -551.084,
                y: -85.303,
                z: 178.306,
            },
            SpawnPoint {
                team: 1,
                x: -530.545,
                y: -85.757,
                z: 221.274,
            },
            SpawnPoint {
                team: 1,
                x: -603.821,
                y: -83.181,
                z: 190.830,
            },
            SpawnPoint {
                team: 1,
                x: -539.510,
                y: -84.908,
                z: 208.982,
            },
        ].into_iter().map(|x| x.offset(387.552, 105.420, -16.572)).collect(),
        bases: vec![
            CaptureBase {
                team: 0,
                x: 195.972,
                y: 18.370,
                z: -181.488,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_BASE_PERCENT_PER_SECOND,
            },
            CaptureBase {
                team: 1,
                x: -182.556,
                y: 18.390,
                z: 196.416,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_BASE_PERCENT_PER_SECOND,
            },
        ],
        capture_points: vec![], // TODO
        equalizer: Point { // TODO
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    });
    map.insert(super::combat::GameMap::Neptune3, MapConfig {
        spawn_points: vec![
            // team 0
            SpawnPoint {
                team: 0,
                x: 112.560,
                y: 41.0682,
                z: -177.072,
            },
            SpawnPoint {
                team: 0,
                x: 172.716,
                y: 42.917,
                z: -143.796,
            },
            SpawnPoint {
                team: 0,
                x: 165.696,
                y: 43.337,
                z: -166.644,
            },
            SpawnPoint {
                team: 0,
                x: 174.360,
                y: 41.7932,
                z: -124.560,
            },
            SpawnPoint {
                team: 0,
                x: 115.944,
                y: 41.0492,
                z: -125.136,
            },
            SpawnPoint {
                team: 0,
                x: 121.488,
                y: 42.8753,
                z: -139.692,
            },
            SpawnPoint {
                team: 0,
                x: 161.700,
                y: 42.803,
                z: -130.848,
            },
            SpawnPoint {
                team: 0,
                x: 128.760,
                y: 41.9734,
                z: -175.392,
            },
            SpawnPoint {
                team: 0,
                x: 118.152,
                y: 43.127,
                z: -161.844,
            },
            SpawnPoint {
                team: 0,
                x: 131.712,
                y: 43.1972,
                z: -127.668,
            },
            // team 1
            SpawnPoint {
                team: 1,
                x: -141.600,
                y: 43.090,
                z: 122.496,
            },
            SpawnPoint {
                team: 1,
                x: -158.124,
                y: 43.122,
                z: 121.536,
            },
            SpawnPoint {
                team: 1,
                x: -126.492,
                y: 42.652,
                z: 166.764,
            },
            SpawnPoint {
                team: 1,
                x: -128.352,
                y: 42.895,
                z: 132.768,
            },
            SpawnPoint {
                team: 1,
                x: -171.864,
                y: 43.1253,
                z: 130.596,
            },
            SpawnPoint {
                team: 1,
                x: -140.580,
                y: 42.8625,
                z: 176.520,
            },
            SpawnPoint {
                team: 1,
                x: -120.744,
                y: 42.014,
                z: 179.496,
            },
            SpawnPoint {
                team: 1,
                x: -171.708,
                y: 43.7184,
                z: 115.260,
            },
            SpawnPoint {
                team: 1,
                x: -126.804,
                y: 42.789,
                z: 118.284,
            },
            SpawnPoint {
                team: 1,
                x: -166.608,
                y: 42.4292,
                z: 167.148,
            },
        ],
        bases: vec![
            CaptureBase {
                team: 0,
                x: 143.880,
                y: 42.270,
                z: -151.440,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_BASE_PERCENT_PER_SECOND,
            },
            CaptureBase {
                team: 1,
                x: -149.520,
                y: 42.290,
                z: 147.240,
                radius: DEFAULT_BASE_RADIUS,
                percent_per_second: DEFAULT_BASE_PERCENT_PER_SECOND,
            },
        ],
        capture_points: vec![], // TODO
        equalizer: Point { // TODO
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    });
    map
}

/*const fn corner_to_center(corner: (f32, f32, f32), radius: f32) -> (f32, f32, f32) {
    (corner.0 + radius, corner.1, corner.2 + radius)
}*/
