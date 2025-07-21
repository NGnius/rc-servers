pub struct CpuInfo {
    pub total: u32,
    pub cosmetic: u32,
}

pub struct CpuListParser {
    cpu_values: std::collections::HashMap<u32, u32>,
    cosmetics: std::collections::HashSet<u32>,
}

impl CpuListParser {
    pub fn with_cubes<'a, I: std::iter::Iterator<Item=&'a crate::persist::Cube>>(iter: I) -> Self {
        let mut cpu_values = std::collections::HashMap::new();
        let mut cosmetics = std::collections::HashSet::new();
        for item in iter {
            cpu_values.insert(item.id, item.info.cpu);
            if item.info.cosmetic {
                cosmetics.insert(item.id);
            }
        }
        Self {
            cpu_values,
            cosmetics
        }
    }

    pub fn calculate_cpu(&self, r: &mut dyn std::io::Read) -> CpuInfo {
        match super::parser::Cube::parse_list(r) {
            Ok(cubes) => {
                let mut totals = CpuInfo {
                    total: 0,
                    cosmetic: 0,
                };
                for cube in cubes {
                    if let Some(cpu_val) = self.cpu_values.get(&cube.id) {
                        totals.total += *cpu_val;
                        if self.cosmetics.contains(&cube.id) {
                            totals.cosmetic += *cpu_val;
                        }
                    } else {
                        totals.total = 10_404;
                        totals.cosmetic = cube.id;
                        return totals;
                    }
                }
                totals
            }
            Err(e) => {
                log::error!("Failed to parse cube data to count cpu: {}", e);
                CpuInfo {
                    total: 10_400,
                    cosmetic: 10_400,
                }
            }
        }
    }
}
