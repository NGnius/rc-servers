use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

pub struct ArcAdapter {
    orm: sea_orm::DatabaseConnection,
    ignore_expiry: bool,
    cdn: Option<String>,
}

impl ArcAdapter {
    pub async fn init(uri: &str, show_expired: bool, override_cdn: Option<String>) -> Result<Self, sea_orm::DbErr>{
        log::debug!("Connecting to Archive of RoboCraft (ARC) vehicle factory database URI: {}", uri);
        let db = sea_orm::Database::connect(uri).await?;
        let good_cdn = override_cdn.map(|s| if s.ends_with("/") { s } else { format!("{}/", s) });
        Ok(Self {
            orm: db,
            ignore_expiry: show_expired,
            cdn: good_cdn,
        })
    }

    fn default_query(&self) -> sea_orm::Select<super::entities::robot_metadata::Entity> {
        super::entities::robot_metadata::Entity::find()
            .order_by_desc(super::entities::robot_metadata::Column::RentCount)
    }

    fn thumbnail_url(&self, meta: String, id: u32) -> String {
        if let Some(cdn) = &self.cdn {
            format!("{}{}", cdn, id)
        } else {
            meta
        }
    }
}

#[async_trait::async_trait]
impl crate::VehicleFactoryAdapter for ArcAdapter {
    async fn vehicle(&self, id: u32) -> Result<Option<(crate::VehicleInfo, crate::VehicleQueryInfo)>, Box<dyn std::error::Error>> {
        log::debug!("Get vehicle by id {}", id);
        let cubes = super::entities::robot_cubes::Entity::find_by_id(id).one(&self.orm);
        let meta = super::entities::robot_metadata::Entity::find_by_id(id).one(&self.orm);
        let cubes = cubes.await?;
        let meta = meta.await?;
        if let (Some(cubes), Some(meta)) = (cubes, meta) {
            use base64::Engine;
            Ok(Some((
                crate::VehicleInfo {
                    id: id as _,
                    cube_data: base64::prelude::BASE64_STANDARD.decode(cubes.cube_data.as_bytes()).unwrap_or_default(),
                    colour_data: base64::prelude::BASE64_STANDARD.decode(cubes.colour_data.as_bytes()).unwrap_or_default(),
                },
                crate::VehicleQueryInfo {
                    id: meta.id as _,
                    name: meta.name,
                    description: meta.description,
                    thumbnail: self.thumbnail_url(meta.thumbnail, meta.id),
                    added_by: meta.added_by,
                    added_by_display_name: meta.added_by_display_name,
                    added_date: crate::traits::parse_rc_date(&meta.added_date).unwrap_or_default(),
                    expiry_date: if self.ignore_expiry { chrono::Utc::now() + chrono::Duration::weeks(2) } else {  crate::traits::parse_rc_date(&meta.expiry_date).unwrap_or_default() },
                    cpu: meta.cpu as _,
                    total_robot_ranking: meta.total_robot_ranking as _,
                    rent_count: meta.rent_count as _,
                    buy_count: meta.buy_count as _,
                    buyable: meta.buyable != 0,
                    removed_date: Default::default(),
                    ban_date: Default::default(),
                    featured: meta.featured != 0,
                    banner_message: Default::default(),
                    combat_rating: meta.combat_rating,
                    cosmetic_rating: meta.cosmetic_rating,
                    cube_amounts: Default::default(),
                }
            )))
        } else {
            Ok(None)
        }

    }

    async fn list(&self, query: libfj::robocraft::ListQuery) -> Result<Vec<crate::VehicleQueryInfo>, Box<dyn std::error::Error>> {
        log::debug!("Search vehicles with query {:?}", query);
        let query_params = if query.default_page {
            log::debug!("Default vehicle list query");
            self.default_query()
        } else {
            let mut query_builder = super::entities::robot_metadata::Entity::find();
            match query.order {
                libfj::robocraft::FactoryOrderType::Suggested => { query_builder = query_builder.order_by_desc(super::entities::robot_metadata::Column::RentCount); },
                libfj::robocraft::FactoryOrderType::CombatRating => { query_builder = query_builder.order_by_desc(super::entities::robot_metadata::Column::CombatRating); },
                libfj::robocraft::FactoryOrderType::CosmeticRating => { query_builder = query_builder.order_by_desc(super::entities::robot_metadata::Column::CosmeticRating); },
                libfj::robocraft::FactoryOrderType::Added => { query_builder = query_builder.order_by_asc(super::entities::robot_metadata::Column::AddedDate); },
                libfj::robocraft::FactoryOrderType::CPU => { query_builder = query_builder.order_by_desc(super::entities::robot_metadata::Column::Cpu); },
                libfj::robocraft::FactoryOrderType::MostBought => { query_builder = query_builder.order_by_desc(super::entities::robot_metadata::Column::BuyCount); },
            }
            if !query.text_filter.is_empty() {
                if query.player_filter {
                    query_builder = query_builder.filter(
                        sea_orm::sea_query::Condition::any()
                            .add(super::entities::robot_metadata::Column::AddedBy.like(query.text_filter.clone()))
                            .add(super::entities::robot_metadata::Column::AddedByDisplayName.like(query.text_filter))
                    );
                } else {
                    query_builder = query_builder.filter(
                        sea_orm::sea_query::Condition::any()
                            .add(super::entities::robot_metadata::Column::AddedBy.like(query.text_filter.clone()))
                            .add(super::entities::robot_metadata::Column::AddedByDisplayName.like(query.text_filter.clone()))
                            .add(super::entities::robot_metadata::Column::Name.like(query.text_filter.clone()))
                            .add(super::entities::robot_metadata::Column::Description.like(query.text_filter.clone()))
                    );
                }
            }
            // movement filters not supported
            // weapon filters not supported
            if query.minimum_cpu > 0 {
                query_builder = query_builder.filter(super::entities::robot_metadata::Column::Cpu.gte(query.minimum_cpu as u32));
            }
            if query.maximum_cpu < usize::MAX {
                query_builder = query_builder.filter(super::entities::robot_metadata::Column::Cpu.lte(query.maximum_cpu as u32));
            }
            if query.buyable {
                query_builder = query_builder.filter(super::entities::robot_metadata::Column::Buyable.ne(0));
            }
            query_builder
        };

        // FIXME add support for query.prepend_featured_bot
        let metadata_pages = query_params.paginate(&self.orm, query.page_size as u64);
        let metadatas = metadata_pages.fetch_page(query.page as u64).await?;
        let mut infos = Vec::with_capacity(metadatas.len());
        for meta in metadatas {
            //let cube_amounts = super::entities::robot_cubes::Entity::find_by_id(meta.id).one(&self.orm).await?.map(|x| x.cube_amounts).unwrap_or_else(|| "".to_owned());
            infos.push(
                crate::VehicleQueryInfo {
                    id: meta.id as _,
                    name: meta.name,
                    description: meta.description,
                    thumbnail: meta.thumbnail,
                    added_by: meta.added_by,
                    added_by_display_name: meta.added_by_display_name,
                    added_date: crate::traits::parse_rc_date(&meta.added_date).unwrap_or_default(),
                    expiry_date: if self.ignore_expiry { chrono::Utc::now() + chrono::Duration::weeks(2) } else {  crate::traits::parse_rc_date(&meta.expiry_date).unwrap_or_default() },
                    cpu: meta.cpu as _,
                    total_robot_ranking: meta.total_robot_ranking as _,
                    rent_count: meta.rent_count as _,
                    buy_count: meta.buy_count as _,
                    buyable: meta.buyable != 0,
                    removed_date: Default::default(),
                    ban_date: Default::default(),
                    featured: meta.featured != 0,
                    banner_message: Default::default(),
                    combat_rating: meta.combat_rating,
                    cosmetic_rating: meta.cosmetic_rating,
                    cube_amounts: Default::default(),
                }
            );
        }
        log::debug!("Search vehicles returned {} results", infos.len());
        Ok(infos)
    }

    async fn upload(&self, _vehicle: crate::VehicleUploadInfo) -> Result<bool, Box<dyn std::error::Error>> {
        log::info!("Arc adapter does not support uploading factory vehicles");
        Ok(false)
    }
}
