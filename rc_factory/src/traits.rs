#[async_trait::async_trait]
pub trait VehicleFactoryAdapter: Send + Sync + 'static {
    async fn vehicle(&self, id: u32) -> Result<Option<(VehicleInfo, VehicleQueryInfo)>, Box<dyn std::error::Error>>;
    async fn list(&self, query: libfj::robocraft::ListQuery) -> Result<Vec<VehicleQueryInfo>, Box<dyn std::error::Error>>;
    async fn upload(&self, vehicle: VehicleUploadInfo) -> Result<bool, Box<dyn std::error::Error>>;
}

#[derive(Debug, Clone)]
pub struct VehicleInfo {
    pub id: i32,
    pub cube_data: Vec<u8>,
    pub colour_data: Vec<u8>,
}

pub fn parse_rc_date(s: &str) -> chrono::ParseResult<chrono::DateTime<chrono::Utc>> {
    let naive = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")?;
    Ok(chrono::DateTime::from_naive_utc_and_offset(naive, chrono::Utc))
}

impl std::convert::From<libfj::robocraft::FactoryRobotGetInfo> for VehicleInfo {
    fn from(value: libfj::robocraft::FactoryRobotGetInfo) -> Self {
        use base64::Engine;
        Self {
            id: value.item_id as _,
            cube_data: base64::prelude::BASE64_STANDARD.decode(value.cube_data.as_bytes()).unwrap_or_default(),
            colour_data: base64::prelude::BASE64_STANDARD.decode(value.colour_data.as_bytes()).unwrap_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct VehicleQueryInfo {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub thumbnail: String, // url
    pub added_by: String,
    pub added_by_display_name: String,
    pub added_date: chrono::DateTime<chrono::Utc>,
    pub expiry_date: chrono::DateTime<chrono::Utc>,
    pub cpu: u32,
    pub total_robot_ranking: u32,
    pub rent_count: u32,
    pub buy_count: u32,
    pub buyable: bool,
    pub removed_date: Option<chrono::DateTime<chrono::Utc>>,
    pub ban_date: Option<chrono::DateTime<chrono::Utc>>,
    pub featured: bool,
    pub banner_message: Option<String>,
    pub combat_rating: f64,
    pub cosmetic_rating: f64,
    pub cube_amounts: std::collections::HashMap<u32, u32>,
}

#[derive(Debug)]
pub struct VehicleUploadInfo {
    pub name: String,
    pub description: String,
    pub thumbnail: Vec<u8>,
    pub added_by: String,
    pub added_by_display_name: String,
    pub cpu: u32,
    pub total_robot_ranking: u32,
    pub cube_data: Vec<u8>,
    pub colour_data: Vec<u8>,
    pub build_version: String,
}
