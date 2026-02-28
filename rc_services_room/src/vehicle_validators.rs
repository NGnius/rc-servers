pub type InitedVehicleValidator = Box<dyn oj_rc_plugins::vehicle_validation::VehicleValidatorPlugin>;

pub struct InitedVehicleValidators {
    pub multiplayer: std::sync::Arc<InitedVehicleValidator>,
    pub custom_game: std::sync::Arc<InitedVehicleValidator>,
    pub singleplayer: std::sync::Arc<InitedVehicleValidator>,
    pub campaigns: std::sync::Arc<std::collections::HashMap<String, InitedVehicleValidator>>,
}

pub fn validators_from_conf(conf: &oj_rc_core::persist::config::VehicleValidators, parsers: &oj_rc_core::cubes::CubeParsers, plugins_path: impl AsRef<std::path::Path>) -> InitedVehicleValidators {
    let plugins_path = plugins_path.as_ref();
    InitedVehicleValidators {
        multiplayer: std::sync::Arc::new(validator_from_conf(&conf.multiplayer, parsers, plugins_path)),
        custom_game: std::sync::Arc::new(validator_from_conf(&conf.multiplayer, parsers, plugins_path)),
        singleplayer: std::sync::Arc::new(validator_from_conf(&conf.singleplayer, parsers, plugins_path)),
        campaigns: std::sync::Arc::new(
            conf.campaigns.iter()
                .map(|(campaign_id, validator)| (campaign_id.to_owned(), validator_from_conf(validator, parsers, plugins_path)))
                .collect()
        ),
    }
}

fn validator_from_conf(conf: &oj_rc_core::persist::VehicleValidator, parsers: &oj_rc_core::cubes::CubeParsers, plugins_path: impl AsRef<std::path::Path>) -> InitedVehicleValidator {
    match conf {
        oj_rc_core::persist::VehicleValidator::None => Box::new(AlwaysValid) as _,
        oj_rc_core::persist::VehicleValidator::Cpu { min, max } => Box::new(CpuRange {
            range: *min..=*max,
            parser: parsers.cpu_counter(),
        }) as _,
        oj_rc_core::persist::VehicleValidator::All { all } => Box::new(All::init(all, parsers, plugins_path)) as _,
        oj_rc_core::persist::VehicleValidator::Any { any } => Box::new(Any::init(any, parsers, plugins_path)) as _,
        oj_rc_core::persist::VehicleValidator::Custom { path } => {
            let full_path = plugins_path.as_ref().join(path);
            log::warn!("Custom vehicle validator plugin {} is experimental and insecure", full_path.display());
            let result = oj_rc_plugins::vehicle_validation::VehicleValidatorCPlugin::new(&full_path);
            match result {
                Ok(c_plugin) => Box::new(c_plugin) as _,
                Err(e) => {
                    log::error!("Failed to load custom vehicle validator plugin {}: {} (crashing!)", full_path.display(), e);
                    panic!("Failed to load custom vehicle validator plugin {}: {}", full_path.display(), e)
                }
            }
        },
    }
}

struct AlwaysValid;

impl oj_rc_plugins::vehicle_validation::VehicleValidatorPlugin for AlwaysValid {
    fn validate(&self, _cube_data: &[u8], _colour_data: &[u8]) -> oj_rc_plugins::vehicle_validation::ValidationResultCode {
        oj_rc_plugins::vehicle_validation::ValidationResultCode::Ok
    }
}

impl oj_rc_plugins::Plugin for AlwaysValid {}

struct CpuRange {
    range: std::ops::RangeInclusive<u32>,
    parser: std::sync::Arc<oj_rc_core::cubes::CpuListParser>,
}

impl oj_rc_plugins::vehicle_validation::VehicleValidatorPlugin for CpuRange {
    fn validate(&self, cube_data: &[u8], _colour_data: &[u8]) -> oj_rc_plugins::vehicle_validation::ValidationResultCode {
        let cpu_info = self.parser.calculate_cpu(&mut std::io::Cursor::new(cube_data));
        if self.range.contains(&(cpu_info.total - cpu_info.cosmetic)) {
            oj_rc_plugins::vehicle_validation::ValidationResultCode::Ok
        } else {
            oj_rc_plugins::vehicle_validation::ValidationResultCode::Invalid
        }
    }
}

impl oj_rc_plugins::Plugin for CpuRange {}

struct All(Vec<InitedVehicleValidator>);

impl All {
    fn init(items: &[oj_rc_core::persist::VehicleValidator], parsers: &oj_rc_core::cubes::CubeParsers, plugins_path: impl AsRef<std::path::Path>) -> Self {
        let plugins_path = plugins_path.as_ref();
        All(
            items.iter()
                .map(|item| validator_from_conf(item, parsers, plugins_path))
                .collect()
        )
    }
}

impl oj_rc_plugins::vehicle_validation::VehicleValidatorPlugin for All {
    fn validate(&self, cube_data: &[u8], colour_data: &[u8]) -> oj_rc_plugins::vehicle_validation::ValidationResultCode {
        self.0.iter()
            .filter_map(|item| {
                let code = item.validate(cube_data, colour_data);
                if matches!(code, oj_rc_plugins::vehicle_validation::ValidationResultCode::Ok) {
                    None
                } else {
                    Some(code)
                }
            })
            .next()
            .unwrap_or(oj_rc_plugins::vehicle_validation::ValidationResultCode::Ok)
    }
}

impl oj_rc_plugins::Plugin for All {}

struct Any(Vec<InitedVehicleValidator>);

impl Any {
    fn init(items: &[oj_rc_core::persist::VehicleValidator], parsers: &oj_rc_core::cubes::CubeParsers, plugins_path: impl AsRef<std::path::Path>) -> Self {
        let plugins_path = plugins_path.as_ref();
        Any(
            items.iter()
                .map(|item| validator_from_conf(item, parsers, plugins_path))
                .collect()
        )
    }
}

impl oj_rc_plugins::vehicle_validation::VehicleValidatorPlugin for Any {
    fn validate(&self, cube_data: &[u8], colour_data: &[u8]) -> oj_rc_plugins::vehicle_validation::ValidationResultCode {
        self.0.iter()
            .map_while(|item| {
                let code = item.validate(cube_data, colour_data);
                if matches!(code, oj_rc_plugins::vehicle_validation::ValidationResultCode::Ok) {
                    None
                } else {
                    Some(code)
                }
            })
            .next()
            .unwrap_or(oj_rc_plugins::vehicle_validation::ValidationResultCode::Ok)
    }
}

impl oj_rc_plugins::Plugin for Any {}
