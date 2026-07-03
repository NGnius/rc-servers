const COLOUR_COUNT: usize = 32;

struct ClassicModernTranslation {
    id_mapping: ModernMapping,
    offset: (i16, i16, i16),
    orientation_mapping: OrientationMapping,
}

impl ClassicModernTranslation {
    fn convert(&self, cube: &super::Cube) -> (super::Cube, super::Colour){
        let (new_id, new_colour) = self.id_mapping.convert(cube);
        let new_orientation = self.orientation_mapping.convert(cube);
        let new_cube = super::Cube {
            id: new_id,
            x: (cube.x as u16).saturating_add_signed(self.offset.0) as _,
            y: (cube.y as u16).saturating_add_signed(self.offset.1) as _,
            z: (cube.z as u16).saturating_add_signed(self.offset.2) as _,
            orientation: new_orientation,
        };
        let new_colour = super::Colour {
            colour: new_colour,
            x: new_cube.x,
            y: new_cube.y,
            z: new_cube.z,
        };
        (new_cube, new_colour)
    }
}

#[allow(dead_code)]
enum ModernMapping {
    Always(u32, u8),
    ByOrientation(Vec<(u32, u8)>),
}

impl ModernMapping {
    fn convert(&self, cube: &super::Cube) -> (u32, u8) {
        match self {
            Self::Always(id, colour) => (*id, *colour),
            Self::ByOrientation(v) => v[cube.orientation as usize],
        }
    }

    fn pretty(&self) -> String {
        match self {
            Self::Always(id, colour) => format!("ID:{}|colour#{}", id, colour),
            Self::ByOrientation(mapping) => format!("o[0]ID:{}|colour#{}", mapping[0].0, mapping[0].1),
        }
    }
}

struct ModernClassicTranslation {
    id_mapping: ClassicMapping,
    offset: (i16, i16, i16),
    orientation_mapping: OrientationMapping,
}

impl ModernClassicTranslation {
    fn convert(&self, cube: &super::Cube, colour: &super::Colour) -> super::Cube {
        let new_id = self.id_mapping.convert(cube, colour);
        let new_orientation = self.orientation_mapping.convert(cube);
        super::Cube {
            id: new_id,
            x: (cube.x as u16).saturating_add_signed(self.offset.0) as _,
            y: (cube.y as u16).saturating_add_signed(self.offset.1) as _,
            z: (cube.z as u16).saturating_add_signed(self.offset.2) as _,
            orientation: new_orientation,
        }
    }
}

#[allow(dead_code)]
enum ClassicMapping {
    Always(u32),
    ByColour(Vec<u32>),
    ByOrientation(Vec<u32>),
}

impl ClassicMapping {
    fn convert(&self, cube: &super::Cube, colour: &super::Colour) -> u32 {
        match self {
            Self::Always(x) => *x,
            Self::ByColour(v) => v[colour.colour as usize],
            Self::ByOrientation(v) => v[cube.orientation as usize],
        }
    }
}

#[allow(dead_code)]
enum OrientationMapping {
    Passthrough,
    ByOrientation(Vec<u8>),
}

impl OrientationMapping {
    fn convert(&self, cube: &super::Cube) -> u8 {
        match self {
            Self::Passthrough => cube.orientation,
            Self::ByOrientation(v) => v[cube.orientation as usize],
        }
    }
}

#[derive(Debug)]
pub enum ConversionError {
    UnknownCube {
        id: u32,
        index: usize,
        position: (u8, u8, u8)
    },
    CubeParse(std::io::Error),
    CubeDump(std::io::Error),
    ColourParse(std::io::Error),
    ColourDump(std::io::Error),
}

impl core::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownCube { id, index, position } => write!(f, "Unknown cube ID {} at {} {:?}", id, index, position),
            Self::CubeParse(e) => write!(f, "Failed to parse cube data: {}", e),
            Self::CubeDump(e) => write!(f, "Failed to dump cube data: {}", e),
            Self::ColourParse(e) => write!(f, "Failed to parse colour data: {}", e),
            Self::ColourDump(e) => write!(f, "Failed to dump colour data: {}", e),
        }
    }
}

impl core::error::Error for ConversionError {}

pub struct ModernConversionResult{
    pub cube_data: Vec<u8>,
    pub colour_data: Vec<u8>,
}

pub struct CubeConversionParser {
    classic_to_modern: std::collections::HashMap<u32, ClassicModernTranslation>,
    modern_to_classic: std::collections::HashMap<u32, ModernClassicTranslation>,
    known: std::collections::HashSet<u32>,
}

impl CubeConversionParser {
    pub fn with_cubes<'a, I: std::iter::Iterator<Item=&'a crate::persist::Cube>>(cubes: I) -> Self {
        let mut classic_to_modern = std::collections::HashMap::<u32, ClassicModernTranslation>::new();
        let mut modern_to_classic = std::collections::HashMap::<u32, ModernClassicTranslation>::new();
        let mut known = std::collections::HashSet::new();
        for cube in cubes {
            // populate classic_to_modern
            let conversion_data = cube.conversion.clone().unwrap_or_else(crate::persist::conversion::actual_default_conversion);
            for from in conversion_data.from.iter() {
                let mapping = match from {
                    crate::persist::FromConversionData::Simple(_id) => ClassicModernTranslation {
                        id_mapping: ModernMapping::Always(cube.id, 0),
                        offset: (0, 0, 0),
                        orientation_mapping: OrientationMapping::Passthrough,
                    },
                    crate::persist::FromConversionData::Complex { id: _, offset, colour } => ClassicModernTranslation {
                        id_mapping: ModernMapping::Always(cube.id, *colour),
                        offset: offset.unwrap_or_default(),
                        orientation_mapping: OrientationMapping::Passthrough,
                    },
                };
                #[cfg(debug_assertions)]
                if let Some(old_mapping) = classic_to_modern.get(&from.id()) {
                    log::debug!("Not overriding mapping classic ID {} from modern {} to modern {}", from.id(), old_mapping.id_mapping.pretty(), mapping.id_mapping.pretty());
                } else {
                    classic_to_modern.insert(from.id(), mapping);
                }
                #[cfg(not(debug_assertions))]
                if !classic_to_modern.contains_key(&from.id()) {
                    classic_to_modern.insert(from.id(), mapping);
                }
            }
            // populate modern_to_classic
            let mapping = if let Some(to) = conversion_data.to {
                match to {
                    crate::persist::ToConversionData::Simple(id) => ModernClassicTranslation {
                        id_mapping: ClassicMapping::Always(id),
                        offset: (0, 0, 0),
                        orientation_mapping: OrientationMapping::Passthrough,
                    },
                    crate::persist::ToConversionData::Complex { id, offset } => ModernClassicTranslation {
                        id_mapping: ClassicMapping::Always(id),
                        offset: offset.unwrap_or_default(),
                        orientation_mapping: OrientationMapping::Passthrough,
                    }
                }
            } else if !conversion_data.from.is_empty() {
                if conversion_data.from.len() == 1 {
                    let from_first = conversion_data.from.first().unwrap();
                    match from_first {
                        crate::persist::FromConversionData::Simple(id) => ModernClassicTranslation {
                            id_mapping: ClassicMapping::Always(*id),
                            offset: (0, 0, 0),
                            orientation_mapping: OrientationMapping::Passthrough,
                        },
                        crate::persist::FromConversionData::Complex { id, offset, colour: _ } => ModernClassicTranslation {
                            id_mapping: ClassicMapping::Always(*id),
                            offset: offset.map(|(x, y, z)| (-x, -y, -z)).unwrap_or_default(),
                            orientation_mapping: OrientationMapping::Passthrough,
                        },
                    }
                } else {
                    // more than one conversion, build map
                    let default_id = conversion_data.from.first().unwrap().id();
                    let mut colour_map: Vec<u32> = (0..COLOUR_COUNT).map(|_| default_id).collect();
                    let mut is_default_set = false;
                    for from in conversion_data.from.iter() {
                        let colour = match from {
                            crate::persist::FromConversionData::Simple(_) => 0,
                            crate::persist::FromConversionData::Complex { colour, .. } => *colour,
                        };
                        if colour_map[colour as usize] != default_id { continue; }
                        if colour == 0 {
                            if !is_default_set {
                                is_default_set = true;
                            } else {
                                continue;
                            }
                        }
                        colour_map[colour as usize] = from.id();
                    }
                    ModernClassicTranslation {
                        id_mapping: ClassicMapping::ByColour(colour_map),
                        offset: (0, 0, 0),
                        orientation_mapping: OrientationMapping::Passthrough,
                    }
                }
            } else {
                ModernClassicTranslation {
                    id_mapping: ClassicMapping::Always(cube.id),
                    offset: (0, 0, 0),
                    orientation_mapping: OrientationMapping::Passthrough,
                }
            };
            modern_to_classic.insert(cube.id, mapping);
            // populate known cubes
            known.insert(cube.id);
        }
        Self {
            classic_to_modern,
            modern_to_classic,
            known,
        }
    }

    /// Replace modern cubes with classic equivalents (based on cube ID)
    pub fn convert_to_classic(&self, cubes: &mut dyn std::io::Read, colours: &mut dyn std::io::Read) -> Result<Vec<u8>, ConversionError> {
        let mut cube_data = super::parser::Cube::parse_list(cubes).map_err(ConversionError::CubeParse)?;
        let colour_data = super::parser::Colour::parse_list(colours).map_err(ConversionError::ColourParse)?;
        for (i, (cube, colour)) in cube_data.iter_mut().zip(colour_data.iter()).enumerate() {
            if let Some(translation) = self.modern_to_classic.get(&cube.id) {
                let new_cube = translation.convert(cube, colour);
                *cube = new_cube;
            } else {
                return Err(ConversionError::UnknownCube {
                    id: cube.id,
                    index: i,
                    position: (cube.x, cube.y, cube.z),
                });
            }
        }
        super::parser::Cube::dump_list(cube_data).map_err(ConversionError::CubeDump)
    }

    /// Replace classic cubes with modern equivalents (based on cube ID)
    pub fn convert_to_modern(&self, cubes: &mut dyn std::io::Read) -> Result<ModernConversionResult, ConversionError> {
        let mut cube_data = super::parser::Cube::parse_list(cubes).map_err(ConversionError::CubeParse)?;
        let mut colour_data = Vec::with_capacity(cube_data.len());
        for (i, cube) in cube_data.iter_mut().enumerate() {
            if let Some(translation) = self.classic_to_modern.get(&cube.id) {
                let (new_cube, new_colour) = translation.convert(cube);
                log::trace!("cube {} RC15 ID {} -> Modern ID {} colour {} ({}, {}, {})", i, cube.id, new_cube.id, new_colour.colour, cube.x, cube.y, cube.z);
                *cube = new_cube;
                colour_data.push(new_colour);
            } else if !self.known.contains(&cube.id){
                return Err(ConversionError::UnknownCube {
                    id: cube.id,
                    index: i,
                    position: (cube.x, cube.y, cube.z),
                });
            } else {
                log::debug!("No translation found for cube {} ({}, {}, {}) id {} (a valid modern ID)", i, cube.x, cube.y, cube.z, cube.id);
                colour_data.push(super::Colour {
                    colour: 0,
                    x: cube.x,
                    y: cube.y,
                    z: cube.z,
                });
            }
        }
        #[cfg(debug_assertions)]
        if cube_data.len() != colour_data.len() {
            log::error!("Emitted different lengths of cube and colour data; {} cubes != {} colours", cube_data.len(), colour_data.len());
            return Err(ConversionError::UnknownCube {
                id: 0,
                index: cube_data.len(),
                position: (0, 0, 0),
            });
        }
        let cube_bytes = super::parser::Cube::dump_list(cube_data).map_err(ConversionError::CubeDump)?;
        let colour_bytes = super::parser::Colour::dump_list(colour_data).map_err(ConversionError::ColourDump)?;
        Ok(ModernConversionResult {
            cube_data: cube_bytes,
            colour_data: colour_bytes,
        })
    }

    /// Replace unknown cubes with modern equivalents (based on cube ID), skipping known cube IDs
    pub fn upgrade_to_modern(&self, cubes: &mut dyn std::io::Read, colours: &mut dyn std::io::Read) -> Result<ModernConversionResult, ConversionError> {
        let mut cube_data = super::parser::Cube::parse_list(cubes).map_err(ConversionError::CubeParse)?;
        let mut colour_data = super::parser::Colour::parse_list(colours).map_err(ConversionError::ColourParse)?;
        for (i, (cube, colour)) in cube_data.iter_mut().zip(colour_data.iter_mut()).enumerate() {
            if self.known.contains(&cube.id) { continue; }
            if let Some(translation) = self.classic_to_modern.get(&cube.id) {
                let (new_cube, new_colour) = translation.convert(cube);
                *cube = new_cube;
                *colour = new_colour;
            } else {
                return Err(ConversionError::UnknownCube {
                    id: cube.id,
                    index: i,
                    position: (cube.x, cube.y, cube.z),
                });
            }
        }
        let cube_bytes = super::parser::Cube::dump_list(cube_data).map_err(ConversionError::CubeDump)?;
        let colour_bytes = super::parser::Colour::dump_list(colour_data).map_err(ConversionError::ColourDump)?;
        Ok(ModernConversionResult {
            cube_data: cube_bytes,
            colour_data: colour_bytes,
        })
    }
}
