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
    pub team: Option<u8>,
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
                team: Some(0),
                x: -226.255,
                y: -6.475 + 10.0,
                z: -99.293,
            },
            SpawnPoint {
                team: Some(0),
                x: -190.662,
                y: -6.083,
                z: -85.001,
            },
            SpawnPoint {
                team: Some(0),
                x: -176.454,
                y: -6.308,
                z: -95.670,
            },
            SpawnPoint {
                team: Some(0),
                x: -171.310,
                y: -6.534,
                z: -113.252,
            },
            SpawnPoint {
                team: Some(0),
                x: -228.559,
                y: -6.724,
                z: -114.155,
            },
            SpawnPoint {
                team: Some(0),
                x: -174.672,
                y: -6.653,
                z: -80.095,
            },
            SpawnPoint {
                team: Some(0),
                x: -241.176,
                y: -6.653,
                z: -104.069,
            },
            SpawnPoint {
                team: Some(0),
                x: -175.586,
                y: -6.653,
                z: -129.159,
            },
            SpawnPoint {
                team: Some(0),
                x:-222.358,
                y: -6.653,
                z: -130.716,
            },
            SpawnPoint {
                team: Some(0),
                x: -189.260,
                y: -6.653,
                z: -139.542,
            },
            // team 1
            SpawnPoint {
                team: Some(1),
                x: -183.190,
                y: -6.300,
                z: 358.657,
            },
            SpawnPoint {
                team: Some(1),
                x: -229.629,
                y: -7.259,
                z: 352.230,
            },
            SpawnPoint {
                team: Some(1),
                x: -178.319,
                y: -6.415,
                z: 377.784,
            },
            SpawnPoint {
                team: Some(1),
                x: -193.644,
                y: -6.300,
                z: 347.371,
            },
            SpawnPoint {
                team: Some(1),
                x: -186.017,
                y: -5.857,
                z: 392.741,
            },
            SpawnPoint {
                team: Some(1),
                x: -237.030,
                y: -6.202,
                z: 366.760,
            },
            SpawnPoint {
                team: Some(1),
                x: -179.863,
                y: -6.300,
                z: 344.639,
            },
            SpawnPoint {
                team: Some(1),
                x: -168.506,
                y: -6.439,
                z: 389.723,
            },
            SpawnPoint {
                team: Some(1),
                x: -244.407,
                y: -6.154,
                z: 353.739,
            },
            SpawnPoint {
                team: Some(1),
                x: -235.521,
                y: -6.159,
                z: -383.094,
            },
            // pit
            SpawnPoint {
                team: None,
                x: -192.931,
                y: -1.782,
                z: 255.895,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.328176,
                z: 0.000000,
                w: -0.944617,
            })*/,
            SpawnPoint {
                team: None,
                x: -191.862,
                y: -1.782,
                z: 3.920,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.294482,
                z: 0.000000,
                w: 0.955657,
            })*/,
            SpawnPoint {
                team: None,
                x: -313.513,
                y: 23.285,
                z: 127.710,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.008125,
                z: 0.000000,
                w: 0.999967,
            })*/,
            SpawnPoint {
                team: None,
                x: 14.018,
                y: 11.999,
                z: 119.038,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.792281,
                z: 0.000000,
                w: 0.610156,
            })*/,
            SpawnPoint {
                team: None,
                x: -166.320,
                y: 6.890,
                z: 126.641,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: -0.047861,
                y: 0.686249,
                z: 0.095085,
                w: 0.719535,
            })*/,
            SpawnPoint {
                team: None,
                x: -52.034,
                y: -6.772,
                z: -26.136,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.211982,
                z: 0.000000,
                w: -0.977274,
            })*/,
            SpawnPoint {
                team: None,
                x: -52.748,
                y: -6.534,
                z: 281.912,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.980289,
                z: 0.000000,
                w: -0.197571,
            })*/,
            SpawnPoint {
                team: None,
                x: -155.153,
                y: -7.603,
                z: -120.463,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.765695,
                z: 0.000000,
                w: 0.643203,
            })*/,
            SpawnPoint {
                team: None,
                x: -152.183,
                y: -7.603,
                z: 379.328,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.693062,
                z: 0.000000,
                w: 0.720878,
            })*/,
            SpawnPoint {
                team: None,
                x: -124.859,
                y: -4.039,
                z: 265.399,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.946184,
                z: 0.000000,
                w: -0.323628,
            })*/,
            SpawnPoint {
                team: None,
                x: -48.470,
                y: -5.940,
                z: 379.685,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.996737,
                z: 0.000000,
                w: 0.080716,
            })*/,
            SpawnPoint {
                team: None,
                x: -58.331,
                y: -5.227,
                z: -124.384,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.506744,
                z: 0.000000,
                w: 0.862097,
            })*/,
            SpawnPoint {
                team: None,
                x: -140.421,
                y: -2.970,
                z: 318.740,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.990376,
                z: 0.000000,
                w: 0.138402,
            })*/,
            SpawnPoint {
                team: None,
                x: 7.366,
                y: -6.772,
                z: -59.756,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.211982,
                z: 0.000000,
                w: -0.977274,
            })*/,
            SpawnPoint {
                team: None,
                x: -110.009,
                y: 8.316,
                z: 129.017,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: -0.047861,
                y: 0.686249,
                z: 0.095085,
                w: 0.719535,
            })*/,
            SpawnPoint {
                team: None,
                x: 25.435,
                y: 10.336,
                z: 75.806,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.396630,
                z: 0.000000,
                w: 0.917979,
            })*/,
            SpawnPoint {
                team: None,
                x: -260.326,
                y: 23.285,
                z: 126.856,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.964735,
                z: 0.000000,
                w: -0.263221,
            })*/,
            SpawnPoint {
                team: None,
                x: -266.706,
                y: 21.384,
                z: 42.649,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.924445,
                z: 0.000000,
                w: 0.381315,
            })*/,
            SpawnPoint {
                team: None,
                x: -256.941,
                y: 19.863,
                z: 217.618,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.512346,
                z: 0.000000,
                w: -0.858779,
            })*/,
            SpawnPoint {
                team: None,
                x: -121.889,
                y: 9.029,
                z: 208.969,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.688172,
                z: 0.000000,
                w: 0.725547,
            })*/,
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
                team: Some(0),
                x: 225.119,
                y: -70.621,
                z: -243.719,
            },
            SpawnPoint {
                team: Some(0),
                x: 238.440,
                y: -71.040,
                z: -230.640,
            },
            SpawnPoint {
                team: Some(0),
                x: 255.238,
                y: -70.644,
                z: -225.356,
            },
            SpawnPoint {
                team: Some(0),
                x: 220.075,
                y: -71.426,
                z: -262.486,
            },
            SpawnPoint {
                team: Some(0),
                x: 272.108,
                y: -71.222,
                z: -231.268,
            },
            SpawnPoint {
                team: Some(0),
                x: 227.268,
                y: -72.461,
                z: -278.220,
            },
            SpawnPoint {
                team: Some(0),
                x: 222.239,
                y: -71.984,
                z: -226.676,
            },
            SpawnPoint {
                team: Some(0),
                x: 243.700,
                y: -70.474,
                z: -285.800,
            },
            SpawnPoint {
                team: Some(0),
                x: 282.100,
                y: -72.200,
                z: -261.200,
            },
            SpawnPoint {
                team: Some(0),
                x: 284.372,
                y: -71.781,
                z: -243.522,
            },
            SpawnPoint {
                team: Some(0),
                x: 283.800,
                y: -71.222,
                z: -278.700,
            },
            SpawnPoint {
                team: Some(0),
                x: 255.400,
                y: -71.426,
                z: -292.200,
            },
            SpawnPoint {
                team: Some(0),
                x: 291.800,
                y: -70.644,
                z: -253.800,
            },
            SpawnPoint {
                team: Some(0),
                x: 268.700,
                y: -71.040,
                z: -289.900,
            },
            SpawnPoint {
                team: Some(0),
                x: 212.800,
                y: -70.621,
                z: -249.600,
            },
            // team 1
            SpawnPoint {
                team: Some(1),
                x: -232.320,
                y: -70.143,
                z: 247.800,
            },
            SpawnPoint {
                team: Some(1),
                x: -246.719,
                y: -70.488,
                z: 231.837,
            },
            SpawnPoint {
                team: Some(1),
                x: -231.120,
                y: -70.868,
                z: 266.880,
            },
            SpawnPoint {
                team: Some(1),
                x: -265.437,
                y: -70.994,
                z: 227.632,
            },
            SpawnPoint {
                team: Some(1),
                x: -238.680,
                y: -70.822,
                z: 280.680,
            },
            SpawnPoint {
                team: Some(1),
                x: -280.197,
                y: -70.480,
                z: 236.397,
            },
            SpawnPoint {
                team: Some(1),
                x: -279.238,
                y: -70.681,
                z: 219.475,
            },
            SpawnPoint {
                team: Some(1),
                x: -230.638,
                y: -70.933,
                z: 230.638,
            },
            SpawnPoint {
                team: Some(1),
                x: -222.358,
                y: -71.183,
                z: 278.399,
            },
            SpawnPoint {
                team: Some(1),
                x: -289.680,
                y: -71.445,
                z: 250.320,
            },
            SpawnPoint {
                team: Some(1),
                x: -279.600,
                y: -70.480,
                z: 277.700,
            },
            SpawnPoint {
                team: Some(1),
                x: -269.200,
                y: -70.681,
                z: 292.100,
            },
            SpawnPoint {
                team: Some(1),
                x: -255.400,
                y: -70.933,
                z: 286.600,
            },
            SpawnPoint {
                team: Some(1),
                x: -221.700,
                y: -71.183,
                z: 257.500,
            },
            SpawnPoint {
                team: Some(1),
                x: -292.500,
                y: -71.445,
                z: 266.000,
            },
            // pit
            SpawnPoint {
                team: None,
                x: -138.841,
                y: -68.400,
                z: 287.877,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.863723,
                z: 0.000000,
                w: 0.503967,
            })*/,
            SpawnPoint {
                team: None,
                x: 293.397,
                y: -63.000,
                z: -76.800,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.194957,
                z: 0.000000,
                w: 0.980812,
            })*/,
            SpawnPoint {
                team: None,
                x: -235.319,
                y: -66.720,
                z: -231.596,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.376419,
                z: 0.000000,
                w: 0.926450,
            })*/,
            SpawnPoint {
                team: None,
                x: 251.280,
                y: -55.200,
                z: 138.598,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.786001,
                z: 0.000000,
                w: 0.618225,
            })*/,
            SpawnPoint {
                team: None,
                x: -241.198,
                y: -72.600,
                z: 67.438,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: -0.102277,
                y: 0.541891,
                z: -0.029518,
                w: 0.833680,
            })*/,
            SpawnPoint {
                team: None,
                x: 73.917,
                y: -66.720,
                z: -212.039,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.129287,
                z: 0.000000,
                w: -0.991607,
            })*/,
            SpawnPoint {
                team: None,
                x: 95.520,
                y: -50.520,
                z: 51.840,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.918493,
                z: 0.000000,
                w: -0.395436,
            })*/,
            SpawnPoint {
                team: None,
                x: 48.721,
                y: -50.640,
                z: 95.759,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.931840,
                z: 0.000000,
                w: -0.362871,
            })*/,
            SpawnPoint {
                team: None,
                x: 133.440,
                y: -55.440,
                z: 251.878,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.991527,
                z: 0.000000,
                w: 0.129902,
            })*/,
            SpawnPoint {
                team: None,
                x: -132.132,
                y: -67.920,
                z: -128.208,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.361638,
                z: 0.000000,
                w: 0.932319,
            })*/,
            SpawnPoint {
                team: None,
                x: -86.160,
                y: -75.840,
                z: -283.200,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.361638,
                z: 0.000000,
                w: 0.932319,
            })*/,
            SpawnPoint {
                team: None,
                x: 251.760,
                y: -50.400,
                z: 238.920,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.991527,
                z: 0.000000,
                w: 0.129902,
            })*/,
            SpawnPoint {
                team: None,
                x: -84.408,
                y: -55.776,
                z: 80.208,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.931840,
                z: 0.000000,
                w: -0.362871,
            })*/,
            SpawnPoint {
                team: None,
                x: 205.799,
                y: -62.160,
                z: -52.559,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.805062,
                z: 0.000000,
                w: 0.593190,
            })*/,
            SpawnPoint {
                team: None,
                x: 138.600,
                y: -63.240,
                z: -175.079,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.565129,
                z: 0.000000,
                w: -0.825002,
            })*/,
            SpawnPoint {
                team: None,
                x: -309.359,
                y: -77.520,
                z: -48.122,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: -0.048814,
                y: 0.978080,
                z: -0.094599,
                w: 0.178962,
            })*/,
            SpawnPoint {
                team: None,
                x: 194.880,
                y: -59.880,
                z: 29.520,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.786001,
                z: 0.000000,
                w: 0.618225,
            })*/,
            SpawnPoint {
                team: None,
                x: -258.240,
                y: -52.080,
                z: -297.720,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.376419,
                z: 0.000000,
                w: 0.926450,
            })*/,
            SpawnPoint {
                team: None,
                x: 276.600,
                y: -69.480,
                z: -192.120,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.194957,
                z: 0.000000,
                w: 0.980812,
            })*/,
            SpawnPoint {
                team: None,
                x: -64.812,
                y: -62.844,
                z: 290.783,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.855783,
                z: 0.000000,
                w: 0.517334,
            })*/,
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
        capture_points: vec![
            CapturePoint {
                x: 169.418,
                y: 20.664,
                z: -22.416,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            }/*.rotated(num_quaternion::Quaternion {
                w: -0.723291,
                x: 0.0,
                y: -0.690544,
                z: 0.0,
            })*/,
            CapturePoint {
                x: -125.484,
                y: 14.616,
                z: -129.156,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            }/*.rotated(num_quaternion::Quaternion {
                w: 0.707107,
                x: 0.0,
                y: -0.707107,
                z: 0.0,
            })*/,
            CapturePoint {
                x: -19.332,
                y: 20.424,
                z: 166.128,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            }/*.rotated(num_quaternion::Quaternion {
                w: 0.693754,
                x: 0.0,
                y: 0.720212,
                z: 0.0,
            })*/,
        ].into_iter().map(|p| p.rotated(num_quaternion::Quaternion {
            w: 0.707107,
            x: 0.0,
            y: 0.707107,
            z: 0.0,
        }).offset(13.320, 0.0, -9.84)).collect(),
        equalizer: Point {
            x: -6.048,
            y: 25.524,
            z: 0.984,
        },
    });
    map.insert(super::combat::GameMap::Mars1, MapConfig { // level5
        spawn_points: vec![
            // team 0
            SpawnPoint {
                team: Some(0),
                x: 32.07,
                y: 1.73,
                z: 49.75,
            },
            SpawnPoint {
                team: Some(0),
                x: 39.20,
                y: 1.73,
                z: 40.21,
            },
            SpawnPoint {
                team: Some(0),
                x: 21.14,
                y: 1.73,
                z: 44.16,
            },
            SpawnPoint {
                team: Some(0),
                x: 32.50,
                y: 1.73,
                z: 30.66,
            },
            SpawnPoint {
                team: Some(0),
                x: 31.1,
                y: 1.73,
                z: 6.80,
            },
            SpawnPoint {
                team: Some(0),
                x: 43.40,
                y: 1.73,
                z: 8.60,
            },
            SpawnPoint {
                team: Some(0),
                x: 36.00,
                y: 1.73,
                z: 18.70,
            },
            SpawnPoint {
                team: Some(0),
                x: 3.00,
                y: 1.73,
                z: 57.4,
            },
            SpawnPoint {
                team: Some(0),
                x: 9.90,
                y: 1.73,
                z: 47.90,
            },
            SpawnPoint {
                team: Some(0),
                x: -2.40,
                y: 1.73,
                z: 46.40,
            },
            // team 1
            SpawnPoint {
                team: Some(1),
                x: 346.09,
                y: 8.10,
                z: 339.18,
            },
            SpawnPoint {
                team: Some(1),
                x: 337.10,
                y: 8.10,
                z: 346.80,
            },
            SpawnPoint {
                team: Some(1),
                x: 356.10,
                y: 8.10,
                z: 344.90,
            },
            SpawnPoint {
                team: Some(1),
                x: 340.10,
                y: 8.10,
                z: 358.20,
            },
            SpawnPoint {
                team: Some(1),
                x: 327.15,
                y: 8.10,
                z: 381.87,
            },
            SpawnPoint {
                team: Some(1),
                x: 339.10,
                y: 8.10,
                z: 383.60,
            },
            SpawnPoint {
                team: Some(1),
                x: 334.80,
                y: 8.10,
                z: 372.40,
            },
            SpawnPoint {
                team: Some(1),
                x: 382.50,
                y: 8.10,
                z: 335.10,
            },
            SpawnPoint {
                team: Some(1),
                x: 373.10,
                y: 8.10,
                z: 342.40,
            },
            SpawnPoint {
                team: Some(1),
                x: 384.50,
                y: 8.10,
                z: 346.80,
            },
            // pit
            SpawnPoint {
                team: None,
                x: 366.900,
                y: 8.140,
                z: 362.000,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.912531,
                z: 0.000000,
                w: 0.409007,
            })*/,
            SpawnPoint {
                team: None,
                x: 16.100,
                y: 1.640,
                z: 28.300,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.377899,
                z: 0.000000,
                w: 0.925847,
            })*/,
            SpawnPoint {
                team: None,
                x: 39.300,
                y: 24.840,
                z: 311.200,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.897334,
                z: 0.000000,
                w: 0.441352,
            })*/,
            SpawnPoint {
                team: None,
                x: 421.200,
                y: 24.640,
                z: 110.700,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.565739,
                z: 0.000000,
                w: -0.824585,
            })*/,
            SpawnPoint {
                team: None,
                x: 200.700,
                y: 27.340,
                z: -22.500,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.116625,
                z: 0.000000,
                w: 0.993176,
            })*/,
            SpawnPoint {
                team: None,
                x: 175.600,
                y: 24.740,
                z: 375.200,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.996377,
                z: 0.000000,
                w: -0.085048,
            })*/,
            SpawnPoint {
                team: None,
                x: 8.400,
                y: 19.640,
                z: 161.300,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.639620,
                z: 0.000000,
                w: 0.768691,
            })*/,
            SpawnPoint {
                team: None,
                x: 407.400,
                y: 20.340,
                z: 237.700,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.873372,
                z: 0.000000,
                w: 0.487054,
            })*/,
            SpawnPoint {
                team: None,
                x: 254.800,
                y: 7.340,
                z: 76.600,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.467171,
                z: 0.000000,
                w: 0.884167,
            })*/,
            SpawnPoint {
                team: None,
                x: 127.000,
                y: 8.840,
                z: 326.300,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.595071,
                z: 0.000000,
                w: 0.803673,
            })*/,
            SpawnPoint {
                team: None,
                x: 240.090,
                y: 6.550,
                z: 139.940,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.595071,
                z: 0.000000,
                w: 0.803673,
            })*/,
            SpawnPoint {
                team: None,
                x: 137.200,
                y: 7.340,
                z: 87.400,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.467171,
                z: 0.000000,
                w: 0.884167,
            })*/,
            SpawnPoint {
                team: None,
                x: 367.980,
                y: 24.970,
                z: 213.350,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.873372,
                z: 0.000000,
                w: 0.487054,
            })*/,
            SpawnPoint {
                team: None,
                x: 23.070,
                y: 29.390,
                z: 137.580,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.639620,
                z: 0.000000,
                w: 0.768691,
            })*/,
            SpawnPoint {
                team: None,
                x: 248.600,
                y: 22.500,
                z: 418.200,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.996377,
                z: 0.000000,
                w: -0.085048,
            })*/,
            SpawnPoint {
                team: None,
                x: 248.040,
                y: 27.340,
                z: -11.230,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.116625,
                z: 0.000000,
                w: 0.993176,
            })*/,
            SpawnPoint {
                team: None,
                x: 424.900,
                y: 24.640,
                z: 134.300,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.565739,
                z: 0.000000,
                w: -0.824585,
            })*/,
            SpawnPoint {
                team: None,
                x: 2.300,
                y: 28.300,
                z: 263.200,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.897334,
                z: 0.000000,
                w: 0.441352,
            })*/,
            SpawnPoint {
                team: None,
                x: -12.300,
                y: 9.100,
                z: -0.600,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.377899,
                z: 0.000000,
                w: 0.925847,
            })*/,
            SpawnPoint {
                team: None,
                x: 316.560,
                y: 17.800,
                z: 294.090,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.912531,
                z: 0.000000,
                w: 0.409007,
            })*/,
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
        capture_points: vec![
            CapturePoint {
                x: -24.600,
                y: 25.850,
                z: 242.100,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            }/*.rotated(num_quaternion::Quaternion {
                w: -0.951521,
                x: 0.0,
                y: 0.307583,
                z: 0.0,
            })*/,
            CapturePoint {
                x: 339.060,
                y: 24.240,
                z: 57.800,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            }/*.rotated(num_quaternion::Quaternion {
                w: -0.940705,
                x: 0.0,
                y: -0.339225,
                z: 0.0,
            })*/,
            CapturePoint {
                x: 141.550,
                y: 8.240,
                z: 308.790,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            }/*.rotated(num_quaternion::Quaternion {
                w: -0.050476,
                x: 0.0,
                y: -0.998725,
                z: 0.0,
            })*/,
        ],
        equalizer: Point {
            x: 180.732,
            y: 7.580,
            z: 191.970,
        },
    });
    map.insert(super::combat::GameMap::Mars2, MapConfig { // level6
        spawn_points: vec![
            // team 0
            SpawnPoint {
                team: Some(0),
                x: -198.924,
                y: -14.583,
                z: -201.221,
            },
            SpawnPoint {
                team: Some(0),
                x: -181.922,
                y: -14.038,
                z: -213.642,
            },
            SpawnPoint {
                team: Some(0),
                x: -183.744,
                y: -13.899,
                z: -196.812,
            },
            SpawnPoint {
                team: Some(0),
                x: -237.587,
                y: -12.163,
                z: -213.035,
            },
            SpawnPoint {
                team: Some(0),
                x: -222.539,
                y: -12.982,
                z: -207.438,
            },
            SpawnPoint {
                team: Some(0),
                x: -232.544,
                y: -12.942,
                z: -229.363,
            },
            SpawnPoint {
                team: Some(0),
                x: -165.911,
                y: -12.618,
                z: -243.646,
            },
            SpawnPoint {
                team: Some(0),
                x: -177.804,
                y: -13.899,
                z: -231.290,
            },
            SpawnPoint {
                team: Some(0),
                x: -186.688,
                y: -13.650,
                z: -251.367,
            },
            SpawnPoint {
                team: Some(0),
                x: -219.357,
                y: -13.650,
                z: -252.358,
            },
            // team 1
            SpawnPoint {
                team: Some(1),
                x: 202.422,
                y: -14.396,
                z: 199.954,
            },
            SpawnPoint {
                team: Some(1),
                x: 214.672,
                y: -13.189,
                z: 181.421,
            },
            SpawnPoint {
                team: Some(1),
                x: 252.463,
                y: -14.752,
                z: 221.509,
            },
            SpawnPoint {
                team: Some(1),
                x: 237.125,
                y: -13.703,
                z: 179.705,
            },
            SpawnPoint {
                team: Some(1),
                x: 255.156,
                y: -14.128,
                z: 176.603,
            },
            SpawnPoint {
                team: Some(1),
                x: 255.433,
                y: -10.939,
                z: 197.208,
            },
            SpawnPoint {
                team: Some(1),
                x: 217.628,
                y: -11.689,
                z: 241.903,
            },
            SpawnPoint {
                team: Some(1),
                x: 234.023,
                y: -8.302,
                z: 233.468,
            },
            SpawnPoint {
                team: Some(1),
                x: 207.966,
                y: -13.330,
                z: 226.684,
            },
            SpawnPoint {
                team: Some(1),
                x: 197.538,
                y: -13.869,
                z: 184.945,
            },
            // pit
            SpawnPoint {
                team: None,
                x: 231.264,
                y: -15.180,
                z: 209.484,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.809714,
                z: 0.000000,
                w: -0.586824,
            })*/,
            SpawnPoint {
                team: None,
                x: -204.600,
                y: -15.180,
                z: -230.472,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.210564,
                z: 0.000000,
                w: -0.977580,
            })*/,
            SpawnPoint {
                team: None,
                x: 19.404,
                y: -15.180,
                z: 223.212,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: -0.013317,
                y: 0.982498,
                z: 0.044809,
                w: 0.180310,
            })*/,
            SpawnPoint {
                team: None,
                x: 182.424,
                y: -13.332,
                z: -39.864,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.280839,
                z: 0.000000,
                w: -0.959755,
            })*/,
            SpawnPoint {
                team: None,
                x: 40.392,
                y: -13.728,
                z: -185.460,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.434608,
                z: 0.000000,
                w: -0.900620,
            })*/,
            SpawnPoint {
                team: None,
                x: -59.796,
                y: -33.528,
                z: 76.956,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.026134,
                y: 0.945733,
                z: 0.043401,
                w: 0.320971,
            })*/,
            SpawnPoint {
                team: None,
                x: 90.156,
                y: -28.644,
                z: -88.836,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.370990,
                z: 0.000000,
                w: -0.928637,
            })*/,
            SpawnPoint {
                team: None,
                x: -226.248,
                y: -12.804,
                z: -36.036,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.197524,
                z: 0.000000,
                w: -0.980298,
            })*/,
            SpawnPoint {
                team: None,
                x: -71.940,
                y: -12.936,
                z: -199.716,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.101234,
                z: 0.000000,
                w: -0.994863,
            })*/,
            SpawnPoint {
                team: None,
                x: 71.148,
                y: -2.455,
                z: 68.772,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.850269,
                z: 0.000000,
                w: -0.526349,
            })*/,
            SpawnPoint {
                team: None,
                x: 102.841,
                y: -15.734,
                z: 5.148,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.850269,
                z: 0.000000,
                w: -0.526349,
            })*/,
            SpawnPoint {
                team: None,
                x: -66.158,
                y: -7.814,
                z: -119.064,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.101234,
                z: 0.000000,
                w: -0.994863,
            })*/,
            SpawnPoint {
                team: None,
                x: -204.745,
                y: -11.880,
                z: -22.044,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.879250,
                z: 0.000000,
                w: -0.476361,
            })*/,
            SpawnPoint {
                team: None,
                x: 67.082,
                y: -28.644,
                z: -64.574,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.370990,
                z: 0.000000,
                w: -0.928637,
            })*/,
            SpawnPoint {
                team: None,
                x: -85.391,
                y: -31.033,
                z: 57.420,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: -0.001132,
                y: 0.970763,
                z: 0.050649,
                w: -0.234632,
            })*/,
            SpawnPoint {
                team: None,
                x: -24.460,
                y: -10.877,
                z: -133.914,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.146073,
                z: 0.000000,
                w: -0.989274,
            })*/,
            SpawnPoint {
                team: None,
                x: 167.864,
                y: -13.966,
                z: -126.944,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.379708,
                z: 0.000000,
                w: -0.925106,
            })*/,
            SpawnPoint {
                team: None,
                x: 28.670,
                y: -11.326,
                z: 143.748,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.021835,
                y: 0.575837,
                z: 0.041333,
                w: 0.816227,
            })*/,
            SpawnPoint {
                team: None,
                x: -246.708,
                y: -5.280,
                z: -135.168,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.210564,
                z: 0.000000,
                w: -0.977580,
            })*/,
            SpawnPoint {
                team: None,
                x: 144.672,
                y: -6.283,
                z: 245.916,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.809714,
                z: 0.000000,
                w: -0.586824,
            })*/,
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
        capture_points: vec![
            CapturePoint {
                x: -210.000,
                y: 18.360,
                z: -24.000,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            }/*.rotated(num_quaternion::Quaternion {
                w: 0.997547,
                x: 0.0,
                y: 0.070007,
                z: 0.0,
            })*/,
            CapturePoint {
                x: 90.000,
                y: 5.004,
                z: -90.000,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            },
            CapturePoint {
                x: 24.000,
                y: 18.360,
                z: 210.000,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            }/*.rotated(num_quaternion::Quaternion {
                w: -0.997883,
                x: 0.0,
                y: -0.065034,
                z: 0.0,
            })*/,
        ],
        equalizer: Point {
            x: -64.044,
            y: 0.0,
            z: 64.284,
        },
    });
    map.insert(super::combat::GameMap::Mars3, MapConfig { // level7
        spawn_points: vec![
            // team 0
            SpawnPoint {
                team: Some(0),
                x: 86.581,
                y: 36.306,
                z: -234.321,
            },
            SpawnPoint {
                team: Some(0),
                x: 73.656,
                y: 38.254,
                z: -248.292,
            },
            SpawnPoint {
                team: Some(0),
                x: 103.320,
                y: 38.896,
                z: -232.269,
            },
            SpawnPoint {
                team: Some(0),
                x: 121.271,
                y: 38.397,
                z: -244.241,
            },
            SpawnPoint {
                team: Some(0),
                x: 129.254,
                y: 37.3032,
                z: -261.716,
            },
            SpawnPoint {
                team: Some(0),
                x: 60.802,
                y: 36.021,
                z: -256.596,
            },
            SpawnPoint {
                team: Some(0),
                x: 73.300,
                y: 37.185,
                z: -267.537,
            },
            SpawnPoint {
                team: Some(0),
                x: 138.259,
                y: 35.640,
                z: -248.434,
            },
            SpawnPoint {
                team: Some(0),
                x: 90.537,
                y: 36.971,
                z: -218.770,
            },
            SpawnPoint {
                team: Some(0),
                x: 82.328,
                y: 38.016,
                z: -279.655,
            },
            // team 1
            SpawnPoint {
                team: Some(1),
                x: 72.801,
                y: 38.373,
                z: 251.072,
            },
            SpawnPoint {
                team: Some(1),
                x: 85.298,
                y: 38.302,
                z: 237.315,
            },
            SpawnPoint {
                team: Some(1),
                x: 105.043,
                y: 38.397,
                z: 234.796,
            },
            SpawnPoint {
                team: Some(1),
                x: 121.877,
                y: 38.005,
                z: 246.166,
            },
            SpawnPoint {
                team: Some(1),
                x: 128.399,
                y: 38.254,
                z: 264.461,
            },
            SpawnPoint {
                team: Some(1),
                x: 92.902,
                y: 37.874,
                z: 222.394,
            },
            SpawnPoint {
                team: Some(1),
                x: 69.902,
                y: 37.304,
                z: 269.355,
            },
            SpawnPoint {
                team: Some(1),
                x: 57.143,
                y: 37.767,
                z: 256.845,
            },
            SpawnPoint {
                team: Some(1),
                x: 139.115,
                y: 36.745,
                z: 249.598,
            },
            SpawnPoint {
                team: Some(1),
                x: 84.027,
                y: 39.537,
                z: 283.955,
            },
            // pit
            SpawnPoint {
                team: None,
                x: 81.473,
                y: 37.458,
                z: 207.235,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.971619,
                z: 0.000000,
                w: 0.236552,
            })*/,
            SpawnPoint {
                team: None,
                x: 81.164,
                y: 35.545,
                z: -205.144,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.153836,
                z: 0.000000,
                w: 0.988096,
            })*/,
            SpawnPoint {
                team: None,
                x: 158.836,
                y: 30.912,
                z: 112.860,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.988369,
                z: 0.000000,
                w: -0.152074,
            })*/,
            SpawnPoint {
                team: None,
                x: -57.974,
                y: 2.709,
                z: 0.404,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.710661,
                z: 0.000000,
                w: -0.703535,
            })*/,
            SpawnPoint {
                team: None,
                x: 240.688,
                y: 30.686,
                z: 26.848,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.921473,
                z: 0.000000,
                w: -0.388442,
            })*/,
            SpawnPoint {
                team: None,
                x: 173.115,
                y: 37.268,
                z: -220.160,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.046333,
                z: 0.000000,
                w: 0.998926,
            })*/,
            SpawnPoint {
                team: None,
                x: -17.464,
                y: 7.484,
                z: 90.169,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.934852,
                z: 0.000000,
                w: -0.355037,
            })*/,
            SpawnPoint {
                team: None,
                x: -96.347,
                y: 3.326,
                z: -152.777,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.557413,
                z: 0.000000,
                w: 0.830235,
            })*/,
            SpawnPoint {
                team: None,
                x: -62.857,
                y: 12.712,
                z: -245.120,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.046910,
                z: 0.000000,
                w: 0.998899,
            })*/,
            SpawnPoint {
                team: None,
                x: -63.914,
                y: 11.999,
                z: 232.491,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.997622,
                z: 0.000000,
                w: -0.068919,
            })*/,
            SpawnPoint {
                team: None,
                x: 18.806,
                y: 38.016,
                z: 259.079,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.661650,
                z: 0.000000,
                w: -0.749813,
            })*/,
            SpawnPoint {
                team: None,
                x: 6.772,
                y: 35.070,
                z: -260.766,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.690160,
                z: 0.000000,
                w: 0.723657,
            })*/,
            SpawnPoint {
                team: None,
                x: -39.441,
                y: 7.520,
                z: -139.353,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.557413,
                z: 0.000000,
                w: 0.830235,
            })*/,
            SpawnPoint {
                team: None,
                x: -41.342,
                y: 7.330,
                z: 148.262,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.297753,
                z: 0.000000,
                w: 0.954643,
            })*/,
            SpawnPoint {
                team: None,
                x: 147.431,
                y: 29.985,
                z: -144.461,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.046333,
                z: 0.000000,
                w: 0.998926,
            })*/,
            SpawnPoint {
                team: None,
                x: 168.577,
                y: 34.903,
                z: -43.481,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.704672,
                z: 0.000000,
                w: -0.709533,
            })*/,
            SpawnPoint {
                team: None,
                x: -74.713,
                y: 2.103,
                z: -40.024,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.471442,
                z: 0.000000,
                w: -0.881897,
            })*/,
            SpawnPoint {
                team: None,
                x: 31.126,
                y: 30.769,
                z: 48.352,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.989880,
                z: 0.000000,
                w: 0.141904,
            })*/,
            SpawnPoint {
                team: None,
                x: 224.413,
                y: 34.808,
                z: -96.822,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.153836,
                z: 0.000000,
                w: 0.988096,
            })*/,
            SpawnPoint {
                team: None,
                x: 32.432,
                y: 22.334,
                z: 126.047,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: -0.001323,
                y: -0.411074,
                z: -0.024545,
                w: -0.911271,
            })*/,
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
        capture_points: vec![
            CapturePoint {
                x: -44.400,
                y: 4.800,
                z: -141.600,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            }/*.rotated(num_quaternion::Quaternion {
                w: -0.868262,
                x: 0.0,
                y: -0.496106,
                z: 0.0,
            })*/,
            CapturePoint {
                x: 217.608,
                y: 27.024,
                z: -1.680,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            }/*.rotated(num_quaternion::Quaternion {
                w: -0.322062,
                x: 0.0,
                y: -0.946719,
                z: 0.0,
            })*/,
            CapturePoint {
                x: -44.400,
                y: 4.800,
                z: 141.600,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            }/*.rotated(num_quaternion::Quaternion {
                w: -0.195041,
                x: 0.0,
                y: -0.980795,
                z: 0.0,
            })*/,
        ],
        equalizer: Point {
            x: -52.920,
            y: 0.384,
            z: 0.0,
        },
    });
    map.insert(super::combat::GameMap::Neptune1, MapConfig { // level8
        spawn_points: vec![
            // team 0
            SpawnPoint { // FIXME: at least this spawn is wrong
                team: Some(0),
                x: -2.100,
                y: -0.669,
                z: -42.500,
            },
            SpawnPoint {
                team: Some(0),
                x: -6.000,
                y: -2.493,
                z: -27.600,
            },
            SpawnPoint {
                team: Some(0),
                x: -7.600,
                y: -1.200,
                z: -61.900,
            },
            SpawnPoint {
                team: Some(0),
                x: 9.300,
                y: -1.598,
                z: -29.800,
            },
            SpawnPoint {
                team: Some(0),
                x: 24.500,
                y: -2.361,
                z: -22.900,
            },
            SpawnPoint {
                team: Some(0),
                x: -15.500,
                y: -2.818,
                z: -75.900,
            },
            SpawnPoint {
                team: Some(0),
                x: -2.400,
                y: -3.722,
                z: -83.100,
            },
            SpawnPoint {
                team: Some(0),
                x: 39.200,
                y: -3.310,
                z: -13.900,
            },
            SpawnPoint {
                team: Some(0),
                x: 46.000,
                y: -4.140,
                z: -27.300,
            },
            SpawnPoint {
                team: Some(0),
                x: 57.800,
                y: -3.741,
                z: -35.700,
            },
            // team 1
            SpawnPoint {
                team: Some(1),
                x: -378.600,
                y: -5.531,
                z: 379.400,
            },
            SpawnPoint {
                team: Some(1),
                x: -393.800,
                y: -5.262,
                z: 382.200,
            },
            SpawnPoint {
                team: Some(1),
                x: -381.300,
                y: -4.766,
                z: 394.500,
            },
            SpawnPoint {
                team: Some(1),
                x: -377.400,
                y: -5.798,
                z: 418.700,
            },
            SpawnPoint {
                team: Some(1),
                x: -367.400,
                y: -5.523,
                z: 407.300,
            },
            SpawnPoint {
                team: Some(1),
                x: -425.100,
                y: -5.567,
                z: 363.000,
            },
            SpawnPoint {
                team: Some(1),
                x: -412.400,
                y: -4.945,
                z: 373.200,
            },
            SpawnPoint {
                team: Some(1),
                x: -430.900,
                y: -5.413,
                z: 377.600,
            },
            SpawnPoint {
                team: Some(1),
                x: -372.500,
                y: -5.051,
                z: 434.100,
            },
            SpawnPoint {
                team: Some(1),
                x: -388.300,
                y: -4.076,
                z: 435.700,
            },
            // pit
            SpawnPoint {
                team: None,
                x: -178.300,
                y: -5.500,
                z: 9.300,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.054314,
                z: 0.000000,
                w: 0.998524,
            })*/,
            SpawnPoint {
                team: None,
                x: -187.350,
                y: 4.240,
                z: 182.080,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.017162,
                y: -0.478492,
                z: 0.016116,
                w: 0.877777,
            })*/,
            SpawnPoint {
                team: None,
                x: 4.800,
                y: -7.210,
                z: 104.300,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.199332,
                z: 0.000000,
                w: 0.979932,
            })*/,
            SpawnPoint {
                team: None,
                x: -334.600,
                y: -8.480,
                z: 138.500,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.973971,
                z: 0.000000,
                w: -0.226671,
            })*/,
            SpawnPoint {
                team: None,
                x: -119.100,
                y: -5.460,
                z: -67.100,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.676459,
                z: 0.000000,
                w: 0.736480,
            })*/,
            SpawnPoint {
                team: None,
                x: -75.870,
                y: -4.200,
                z: 27.110,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.467476,
                z: 0.000000,
                w: 0.884006,
            })*/,
            SpawnPoint {
                team: None,
                x: -143.500,
                y: 2.900,
                z: 105.700,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.379128,
                z: 0.000000,
                w: 0.925344,
            })*/,
            SpawnPoint {
                team: None,
                x: -8.500,
                y: -7.860,
                z: 248.500,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.879910,
                z: 0.000000,
                w: 0.475141,
            })*/,
            SpawnPoint {
                team: None,
                x: -305.700,
                y: -7.360,
                z: 217.500,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.588514,
                z: 0.000000,
                w: 0.808487,
            })*/,
            SpawnPoint {
                team: None,
                x: -76.900,
                y: 0.900,
                z: 408.800,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.788453,
                z: 0.000000,
                w: 0.615095,
            })*/,
            SpawnPoint {
                team: None,
                x: -81.500,
                y: -7.100,
                z: 295.000,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.788453,
                z: 0.000000,
                w: 0.615095,
            })*/,
            SpawnPoint {
                team: None,
                x: -237.100,
                y: 6.600,
                z: 185.500,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.778665,
                z: 0.000000,
                w: 0.627440,
            })*/,
            SpawnPoint {
                team: None,
                x: 22.400,
                y: -6.600,
                z: 311.100,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.766853,
                z: 0.000000,
                w: 0.641822,
            })*/,
            SpawnPoint {
                team: None,
                x: -265.100,
                y: -9.300,
                z: -14.000,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.379128,
                z: 0.000000,
                w: 0.925344,
            })*/,
            SpawnPoint {
                team: None,
                x: -226.900,
                y: -9.560,
                z: 53.800,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.858236,
                z: 0.000000,
                w: 0.513255,
            })*/,
            SpawnPoint {
                team: None,
                x: -177.900,
                y: -7.400,
                z: -68.900,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.676459,
                z: 0.000000,
                w: 0.736480,
            })*/,
            SpawnPoint {
                team: None,
                x: -257.400,
                y: 5.700,
                z: 133.900,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.876931,
                z: 0.000000,
                w: 0.480616,
            })*/,
            SpawnPoint {
                team: None,
                x: 58.300,
                y: -1.300,
                z: 166.500,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.977890,
                z: 0.000000,
                w: -0.209120,
            })*/,
            SpawnPoint {
                team: None,
                x: -143.300,
                y: 5.100,
                z: 153.700,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.017162,
                y: -0.478492,
                z: 0.016116,
                w: 0.877777,
            })*/,
            SpawnPoint {
                team: None,
                x: -113.900,
                y: -2.800,
                z: 62.400,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.418763,
                z: 0.000000,
                w: 0.908095,
            })*/,
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
        capture_points: vec![
            CapturePoint {
                x: 91.580,
                y: -7.770,
                z: 10.700,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            }/*.rotated(num_quaternion::Quaternion {
                w: 0.964756,
                x: 0.0,
                y: 0.263146,
                z: 0.0,
            })*/,
            CapturePoint {
                x: 270.200,
                y: -6.390,
                z: 172.480,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            }/*.rotated(num_quaternion::Quaternion {
                w: -0.291923,
                x: 0.0,
                y: -0.956442,
                z: 0.0,
            })*/,
            CapturePoint {
                x: -73.500,
                y: -7.870,
                z: 64.400,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            }/*.rotated(num_quaternion::Quaternion {
                w: 0.964756,
                x: 0.0,
                y: 0.263146,
                z: 0.0,
            })*/,
        ],
        equalizer: Point {
            x: 140.600,
            y: 3.270,
            z: 115.000,
        },
    });
    map.insert(super::combat::GameMap::Neptune2, MapConfig { // level9
        spawn_points: vec![
            // team 0
            SpawnPoint {
                team: Some(0),
                x: -211.622,
                y: -84.836,
                z: -146.150,
            },
            SpawnPoint {
                team: Some(0),
                x: -218.613,
                y: -84.447,
                z: -160.734,
            },
            SpawnPoint {
                team: Some(0),
                x: -188.728,
                y: -84.449,
                z: -137.079,
            },
            SpawnPoint {
                team: Some(0),
                x: -214.104,
                y: -85.091,
                z: -180.661,
            },
            SpawnPoint {
                team: Some(0),
                x: -166.816,
                y: -84.447,
                z: -156.735,
            },
            SpawnPoint {
                team: Some(0),
                x: -203.100,
                y: -84.447,
                z: -191.041,
            },
            SpawnPoint {
                team: Some(0),
                x: -171.769,
                y: -83.701,
                z: -139.698,
            },
            SpawnPoint {
                team: Some(0),
                x: -226.649,
                y: -84.441,
                z: -184.727,
            },
            SpawnPoint {
                team: Some(0),
                x: -178.042,
                y: -84.407,
                z: -126.921,
            },
            SpawnPoint {
                team: Some(0),
                x: -563.460,
                y: -84.447,
                z: 185.603,
            },
            // team 1
            SpawnPoint {
                team: Some(1),
                x: -563.460,
                y: -84.447,
                z: 185.603,
            },
            SpawnPoint {
                team: Some(1),
                x: -545.107,
                y: -84.501,
                z: 225.604,
            },
            SpawnPoint {
                team: Some(1),
                x: -589.174,
                y: -85.324,
                z: 192.699,
            },
            SpawnPoint {
                team: Some(1),
                x: -596.323,
                y: -84.447,
                z: 222.468,
            },
            SpawnPoint {
                team: Some(1),
                x: -549.553,
                y: -84.667,
                z: 193.565,
            },
            SpawnPoint {
                team: Some(1),
                x: -596.682,
                y: -85.404,
                z: 205.550,
            },
            SpawnPoint {
                team: Some(1),
                x: -551.084,
                y: -85.303,
                z: 178.306,
            },
            SpawnPoint {
                team: Some(1),
                x: -530.545,
                y: -85.757,
                z: 221.274,
            },
            SpawnPoint {
                team: Some(1),
                x: -603.821,
                y: -83.181,
                z: 190.830,
            },
            SpawnPoint {
                team: Some(1),
                x: -539.510,
                y: -84.908,
                z: 208.982,
            },
            // pit
            SpawnPoint {
                team: None,
                x: -283.536,
                y: -93.139,
                z: -113.203,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.407554,
                z: 0.000000,
                w: 0.913181,
            })*/,
            SpawnPoint {
                team: None,
                x: -332.946,
                y: -99.697,
                z: -141.715,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.999785,
                z: 0.000000,
                w: -0.020740,
            })*/,
            SpawnPoint {
                team: None,
                x: -400.150,
                y: -87.711,
                z: -47.795,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.613367,
                z: 0.000000,
                w: 0.789798,
            })*/,
            SpawnPoint {
                team: None,
                x: -234.115,
                y: -86.592,
                z: -24.922,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.550372,
                z: 0.000000,
                w: 0.834920,
            })*/,
            SpawnPoint {
                team: None,
                x: -400.963,
                y: -91.450,
                z: -132.739,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.058189,
                z: 0.000000,
                w: 0.998306,
            })*/,
            SpawnPoint {
                team: None,
                x: -528.000,
                y: -93.034,
                z: 3.590,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.735801,
                z: 0.000000,
                w: 0.677198,
            })*/,
            SpawnPoint {
                team: None,
                x: -595.162,
                y: -86.909,
                z: 68.746,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.998860,
                z: 0.000000,
                w: 0.047732,
            })*/,
            SpawnPoint {
                team: None,
                x: -433.837,
                y: -88.144,
                z: 178.390,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.953253,
                z: 0.000000,
                w: 0.302173,
            })*/,
            SpawnPoint {
                team: None,
                x: -472.190,
                y: -86.592,
                z: 78.155,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.920108,
                z: 0.000000,
                w: 0.391665,
            })*/,
            SpawnPoint {
                team: None,
                x: -344.647,
                y: -86.655,
                z: 67.668,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.986668,
                z: 0.000000,
                w: -0.162743,
            })*/,
            SpawnPoint {
                team: None,
                x: -333.907,
                y: -84.586,
                z: -1.584,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.949104,
                z: 0.000000,
                w: -0.314962,
            })*/,
            SpawnPoint {
                team: None,
                x: -430.848,
                y: -86.592,
                z: 38.438,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.271948,
                z: 0.000000,
                w: 0.962312,
            })*/,
            SpawnPoint {
                team: None,
                x: -388.080,
                y: -83.741,
                z: 97.891,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.981779,
                z: 0.000000,
                w: -0.190025,
            })*/,
            SpawnPoint {
                team: None,
                x: -639.408,
                y: -78.957,
                z: -9.293,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.804873,
                z: 0.000000,
                w: 0.593447,
            })*/,
            SpawnPoint {
                team: None,
                x: -483.437,
                y: -90.922,
                z: 32.947,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.735801,
                z: 0.000000,
                w: 0.677198,
            })*/,
            SpawnPoint {
                team: None,
                x: -415.430,
                y: -77.088,
                z: -256.397,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.058189,
                z: 0.000000,
                w: 0.998306,
            })*/,
            SpawnPoint {
                team: None,
                x: -188.707,
                y: -86.592,
                z: -53.222,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.368935,
                z: 0.000000,
                w: 0.929455,
            })*/,
            SpawnPoint {
                team: None,
                x: -495.496,
                y: -91.344,
                z: -83.920,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.922988,
                z: 0.000000,
                w: 0.384829,
            })*/,
            SpawnPoint {
                team: None,
                x: -315.955,
                y: -82.579,
                z: -199.690,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.452432,
                z: 0.000000,
                w: -0.891799,
            })*/,
            SpawnPoint {
                team: None,
                x: -331.056,
                y: -83.139,
                z: -70.541,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.407554,
                z: 0.000000,
                w: 0.913181,
            })*/,
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
        capture_points: vec![
            CapturePoint {
                x: -26.760,
                y: 23.316,
                z: -247.920,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            }/*.rotated(num_quaternion::Quaternion {
                w: -0.359809,
                x: 0.0,
                y: -0.933026,
                z: 0.0,
            })*/,
            CapturePoint {
                x: 36.000,
                y: 16.944,
                z: 36.000,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            }/*.rotated(num_quaternion::Quaternion {
                w: -0.876506,
                x: 0.0,
                y: -0.481392,
                z: 0.0,
            })*/,
            CapturePoint {
                x: -247.920,
                y: 23.316,
                z: -24.720,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            }/*.rotated(num_quaternion::Quaternion {
                w: 0.937230,
                x: 0.0,
                y: -0.348713,
                z: 0.0,
            })*/,
        ],
        equalizer: Point {
            x: -108.000,
            y: 9.276,
            z: -108.000,
        },
    });
    map.insert(super::combat::GameMap::Neptune3, MapConfig { // level10
        spawn_points: vec![
            // team 0
            SpawnPoint {
                team: Some(0),
                x: 112.560,
                y: 41.0682,
                z: -177.072,
            },
            SpawnPoint {
                team: Some(0),
                x: 172.716,
                y: 42.917,
                z: -143.796,
            },
            SpawnPoint {
                team: Some(0),
                x: 165.696,
                y: 43.337,
                z: -166.644,
            },
            SpawnPoint {
                team: Some(0),
                x: 174.360,
                y: 41.7932,
                z: -124.560,
            },
            SpawnPoint {
                team: Some(0),
                x: 115.944,
                y: 41.0492,
                z: -125.136,
            },
            SpawnPoint {
                team: Some(0),
                x: 121.488,
                y: 42.8753,
                z: -139.692,
            },
            SpawnPoint {
                team: Some(0),
                x: 161.700,
                y: 42.803,
                z: -130.848,
            },
            SpawnPoint {
                team: Some(0),
                x: 128.760,
                y: 41.9734,
                z: -175.392,
            },
            SpawnPoint {
                team: Some(0),
                x: 118.152,
                y: 43.127,
                z: -161.844,
            },
            SpawnPoint {
                team: Some(0),
                x: 131.712,
                y: 43.1972,
                z: -127.668,
            },
            // team 1
            SpawnPoint {
                team: Some(1),
                x: -141.600,
                y: 43.090,
                z: 122.496,
            },
            SpawnPoint {
                team: Some(1),
                x: -158.124,
                y: 43.122,
                z: 121.536,
            },
            SpawnPoint {
                team: Some(1),
                x: -126.492,
                y: 42.652,
                z: 166.764,
            },
            SpawnPoint {
                team: Some(1),
                x: -128.352,
                y: 42.895,
                z: 132.768,
            },
            SpawnPoint {
                team: Some(1),
                x: -171.864,
                y: 43.1253,
                z: 130.596,
            },
            SpawnPoint {
                team: Some(1),
                x: -140.580,
                y: 42.8625,
                z: 176.520,
            },
            SpawnPoint {
                team: Some(1),
                x: -120.744,
                y: 42.014,
                z: 179.496,
            },
            SpawnPoint {
                team: Some(1),
                x: -171.708,
                y: 43.7184,
                z: 115.260,
            },
            SpawnPoint {
                team: Some(1),
                x: -126.804,
                y: 42.789,
                z: 118.284,
            },
            SpawnPoint {
                team: Some(1),
                x: -166.608,
                y: 42.4292,
                z: 167.148,
            },
            // pit
            SpawnPoint {
                team: None,
                x: -14.652,
                y: 17.145,
                z: -8.028,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.309773,
                z: 0.000000,
                w: -0.950811,
            })*/,
            SpawnPoint {
                team: None,
                x: -99.480,
                y: 35.589,
                z: -105.240,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.801914,
                z: 0.000000,
                w: -0.597439,
            })*/,
            SpawnPoint {
                team: None,
                x: -42.204,
                y: 42.153,
                z: -41.256,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.866982,
                z: 0.000000,
                w: 0.498340,
            })*/,
            SpawnPoint {
                team: None,
                x: 72.072,
                y: 13.689,
                z: 175.692,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.999867,
                z: 0.000000,
                w: 0.016332,
            })*/,
            SpawnPoint {
                team: None,
                x: 90.096,
                y: 11.121,
                z: 89.964,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.154486,
                z: 0.000000,
                w: -0.987995,
            })*/,
            SpawnPoint {
                team: None,
                x: -172.284,
                y: 37.701,
                z: 29.940,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.506564,
                z: 0.000000,
                w: 0.862203,
            })*/,
            SpawnPoint {
                team: None,
                x: -44.736,
                y: 35.193,
                z: -125.076,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.072266,
                z: 0.000000,
                w: 0.997385,
            })*/,
            SpawnPoint {
                team: None,
                x: 115.668,
                y: 24.849,
                z: -27.408,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.750026,
                z: 0.000000,
                w: 0.661408,
            })*/,
            SpawnPoint {
                team: None,
                x: 181.152,
                y: 17.241,
                z: -8.268,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.299827,
                z: 0.000000,
                w: 0.953994,
            })*/,
            SpawnPoint {
                team: None,
                x: 73.932,
                y: 43.713,
                z: -74.148,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.368328,
                z: 0.000000,
                w: 0.929696,
            })*/,
            SpawnPoint {
                team: None,
                x: 49.488,
                y: 16.713,
                z: -20.004,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.368328,
                z: 0.000000,
                w: 0.929696,
            })*/,
            SpawnPoint {
                team: None,
                x: 155.532,
                y: 16.413,
                z: 77.208,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.684988,
                z: 0.000000,
                w: 0.728555,
            })*/,
            SpawnPoint {
                team: None,
                x: -124.620,
                y: 37.437,
                z: -112.416,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.036114,
                z: 0.000000,
                w: 0.999348,
            })*/,
            SpawnPoint {
                team: None,
                x: 0.240,
                y: 45.117,
                z: -62.880,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.072266,
                z: 0.000000,
                w: 0.997385,
            })*/,
            SpawnPoint {
                team: None,
                x: -166.080,
                y: 44.565,
                z: 90.720,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.995887,
                z: 0.000000,
                w: 0.090600,
            })*/,
            SpawnPoint {
                team: None,
                x: 16.920,
                y: 17.625,
                z: 165.360,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: -0.847469,
                z: 0.000000,
                w: -0.530845,
            })*/,
            SpawnPoint {
                team: None,
                x: 42.120,
                y: 13.689,
                z: 153.720,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.659050,
                z: 0.000000,
                w: 0.752099,
            })*/,
            SpawnPoint {
                team: None,
                x: -75.600,
                y: 46.185,
                z: 27.120,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.866982,
                z: 0.000000,
                w: 0.498340,
            })*/,
            SpawnPoint {
                team: None,
                x: -28.164,
                y: 41.205,
                z: -87.228,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.801914,
                z: 0.000000,
                w: -0.597439,
            })*/,
            SpawnPoint {
                team: None,
                x: 33.120,
                y: 13.269,
                z: 25.200,
            }/*.with_rotation(num_quaternion::Quaternion {
                x: 0.000000,
                y: 0.309773,
                z: 0.000000,
                w: -0.950811,
            })*/,
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
        capture_points: vec![
            CapturePoint {
                x: 155.64,
                y: 13.560,
                z: 22.200,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            },
            CapturePoint {
                x: -90.000,
                y: 33.290,
                z: -90.000,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            },
            CapturePoint {
                x: 22.212,
                y: 13.560,
                z: 157.656,
                radius: DEFAULT_CAPTURE_RADIUS,
                percent_per_second: DEFAULT_CAPTURE_PERCENT_PER_SECOND,
            },
        ],
        equalizer: Point {
            x: 26.040,
            y: 11.436,
            z: 26.076,
        },
    });
    map
}

/*const fn corner_to_center(corner: (f32, f32, f32), radius: f32) -> (f32, f32, f32) {
    (corner.0 + radius, corner.1, corner.2 + radius)
}*/
