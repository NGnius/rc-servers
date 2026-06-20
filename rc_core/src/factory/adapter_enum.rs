pub enum Factory {
    Arc(oj_rc_factory::arc::ArcAdapter),
    Primary(oj_rc_database::FactoryDatabase),
    Web(oj_rc_factory::web::WebAdapter),
    Custom(Box<dyn oj_rc_factory::VehicleFactoryAdapter + Send + Sync + 'static>),
    None,
}

#[async_trait::async_trait]
impl oj_rc_factory::VehicleFactoryAdapter for Factory {
    async fn vehicle(&self, id: i32) -> Result<Option<(oj_rc_factory::VehicleInfo, oj_rc_factory::VehicleQueryInfo)>, Box<dyn std::error::Error>> {
        match self {
            Self::Arc(x) => x.vehicle(id).await,
            Self::Primary(x) => x.vehicle(id).await,
            Self::Web(x) => x.vehicle(id).await,
            Self::Custom(x) => x.vehicle(id).await,
            Self::None => Ok(None),
        }
    }

    async fn list(&self, query: libfj::robocraft::ListQuery) -> Result<Vec<oj_rc_factory::VehicleQueryInfo>, Box<dyn std::error::Error>> {
        match self {
            Self::Arc(x) => x.list(query).await,
            Self::Primary(x) => x.list(query).await,
            Self::Web(x) => x.list(query).await,
            Self::Custom(x) => x.list(query).await,
            Self::None => Ok(Vec::default()),
        }
    }

    async fn count(&self, query: libfj::robocraft::ListQuery) -> Result<usize, Box<dyn std::error::Error>> {
        match self {
            Self::Arc(x) => x.count(query).await,
            Self::Primary(x) => x.count(query).await,
            Self::Web(x) => x.count(query).await,
            Self::Custom(x) => x.count(query).await,
            Self::None => Ok(0),
        }
    }

    async fn upload(&self, vehicle: oj_rc_factory::VehicleUploadInfo) -> Result<oj_rc_factory::VehicleThumbnailInfo, Box<dyn std::error::Error>> {
        match self {
            Self::Arc(x) => x.upload(vehicle).await,
            Self::Primary(x) => x.upload(vehicle).await,
            Self::Web(x) => x.upload(vehicle).await,
            Self::Custom(x) => x.upload(vehicle).await,
            Self::None => Ok(oj_rc_factory::VehicleThumbnailInfo {
                id: i32::MIN,
                thumbnail: vehicle.thumbnail,
                needs_upload: false,
            }),
        }
    }

    async fn rate_vehicle(&self, id: i32, combat: i32, cosmetic: i32) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Arc(x) => x.rate_vehicle(id, combat, cosmetic).await,
            Self::Primary(x) => x.rate_vehicle(id, combat, cosmetic).await,
            Self::Web(x) => x.rate_vehicle(id, combat, cosmetic).await,
            Self::Custom(x) => x.rate_vehicle(id, combat, cosmetic).await,
            Self::None => Ok(()),
        }
    }

    async fn purchase(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Arc(x) => x.purchase(id).await,
            Self::Primary(x) => x.purchase(id).await,
            Self::Web(x) => x.purchase(id).await,
            Self::Custom(x) => x.purchase(id).await,
            Self::None => Ok(()),
        }
    }

    async fn update_vehicle(&self, id: i32, cube_data: Option<Vec<u8>>, colour_data: Option<Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Arc(x) => x.update_vehicle(id, cube_data, colour_data).await,
            Self::Primary(x) => x.update_vehicle(id, cube_data, colour_data).await,
            Self::Web(x) => x.update_vehicle(id, cube_data, colour_data).await,
            Self::Custom(x) => x.update_vehicle(id, cube_data, colour_data).await,
            Self::None => Ok(()),
        }
    }

    async fn remove_vehicle(&self, id: i32, user_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Arc(x) => x.remove_vehicle(id, user_id).await,
            Self::Primary(x) => x.remove_vehicle(id, user_id).await,
            Self::Web(x) => x.remove_vehicle(id, user_id).await,
            Self::Custom(x) => x.remove_vehicle(id, user_id).await,
            Self::None => Ok(()),
        }
    }

    async fn set_featured(&self, id: i32, is_featured: bool) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Arc(x) => x.set_featured(id, is_featured).await,
            Self::Primary(x) => x.set_featured(id, is_featured).await,
            Self::Web(x) => x.set_featured(id, is_featured).await,
            Self::Custom(x) => x.set_featured(id, is_featured).await,
            Self::None => Ok(()),
        }
    }
}

impl Factory {
    pub async fn from_config(conf: &crate::persist::FactoryConfig, settings: &crate::persist::config::ServerConfig, builtin_factory_provider: &(dyn (Fn() -> oj_rc_database::FactoryDatabase) + Sync)) -> Result<Self, Box<dyn std::error::Error + 'static>> {
        Ok(match &conf.adapter {
            crate::persist::AdapterSettings::Arc(x) => Self::Arc(oj_rc_factory::arc::ArcAdapter::init(&x.uri, x.show_expired, settings.cdn_url.to_owned(), x.override_cdn, x.spoof_username).await?),
            crate::persist::AdapterSettings::BuiltIn => Self::Primary(builtin_factory_provider()),
            crate::persist::AdapterSettings::Web(x) => Self::Web(oj_rc_factory::web::WebAdapter::init(&x.url, &settings.auth_url).await?),
            crate::persist::AdapterSettings::None => Self::None,
        })
    }
}
