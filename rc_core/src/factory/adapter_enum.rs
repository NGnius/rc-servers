pub enum Factory {
    Arc(oj_rc_factory::arc::ArcAdapter),
    Custom(Box<dyn oj_rc_factory::VehicleFactoryAdapter + Send + Sync + 'static>),
    None,
}

#[async_trait::async_trait]
impl oj_rc_factory::VehicleFactoryAdapter for Factory {
    async fn vehicle(&self, id: u32) -> Result<Option<(oj_rc_factory::VehicleInfo, oj_rc_factory::VehicleQueryInfo)>, Box<dyn std::error::Error>> {
        match self {
            Self::Arc(x) => x.vehicle(id).await,
            Self::Custom(x) => x.vehicle(id).await,
            Self::None => Ok(None),
        }
    }

    async fn list(&self, query: libfj::robocraft::ListQuery) -> Result<Vec<oj_rc_factory::VehicleQueryInfo>, Box<dyn std::error::Error>> {
        match self {
            Self::Arc(x) => x.list(query).await,
            Self::Custom(x) => x.list(query).await,
            Self::None => Ok(Vec::default()),
        }
    }

    async fn upload(&self, vehicle: oj_rc_factory::VehicleUploadInfo) -> Result<bool, Box<dyn std::error::Error>> {
        match self {
            Self::Arc(x) => x.upload(vehicle).await,
            Self::Custom(x) => x.upload(vehicle).await,
            Self::None => Ok(false),
        }
    }
}

impl Factory {
    pub async fn from_config(conf: &crate::persist::FactoryConfig) -> Result<Self, Box<dyn std::error::Error + 'static>> {
        Ok(match &conf.adapter {
            crate::persist::AdapterSettings::Arc(x) => Self::Arc(oj_rc_factory::arc::ArcAdapter::init(&x.uri, x.show_expired, x.cdn.clone(), x.override_cdn, x.spoof_username).await?),
            crate::persist::AdapterSettings::None => Self::None,
        })
    }
}
