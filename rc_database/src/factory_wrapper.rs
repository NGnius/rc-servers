use oj_rc_factory::{VehicleFactoryAdapter, VehicleInfo, VehicleQueryInfo, VehicleThumbnailInfo, VehicleUploadInfo};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, sqlx::types::chrono, ActiveModelTrait, TransactionTrait};

pub struct FactoryDatabase {
    orm: std::sync::Arc<sea_orm::DatabaseConnection>,
    cdn: std::sync::Arc<String>,
    upload_duration: std::time::Duration,
}

impl FactoryDatabase {
    pub async fn init(uri: &str, cdn: std::sync::Arc<String>) -> Result<Self, sea_orm::DbErr>{
        let db = sea_orm::Database::connect(uri).await?;
        Ok(Self {
            orm: std::sync::Arc::new(db),
            cdn,
            upload_duration: std::time::Duration::from_secs(365 * 24 * 60 * 60), // 1 year
        })
    }

    pub fn init_preconnected(orm: std::sync::Arc<sea_orm::DatabaseConnection>, cdn: std::sync::Arc<String>) -> Self {
        Self {
            orm,
            cdn,
            upload_duration: std::time::Duration::from_secs(365 * 24 * 60 * 60), // 1 year
        }
    }

    async fn full_vehicle_by_id(&self, id: i32) -> Result<Option<(crate::schema::factory::vehicle::Model, crate::schema::user::Model)>, sea_orm::DbErr> {
        Ok(crate::schema::factory::vehicle::Entity::find_by_id(id)
            .find_also_related(crate::schema::user::Entity)
            .one(self.orm.as_ref())
            .await?
            .and_then(|(vehicle, user_opt)| user_opt.map(|user| (vehicle, user))))
    }

    async fn list_query(&self, query: libfj::robocraft::ListQuery) -> Result<Vec<(crate::schema::factory::vehicle_query::VehicleInfo, crate::schema::factory::vehicle_query::VehicleOwnerInfo)>, sea_orm::DbErr> {
        let mut query_builder = crate::schema::factory::vehicle::Entity::find()
            .filter(crate::schema::factory::vehicle::Column::RemovedDate.is_null())
            .filter(crate::schema::factory::vehicle::Column::BanDate.is_null())
            .filter(crate::schema::factory::vehicle::Column::Buyable.eq(true));

        match query.order {
            libfj::robocraft::FactoryOrderType::Suggested => { query_builder = query_builder.order_by_desc(crate::schema::factory::vehicle::Column::GetCount); },
            libfj::robocraft::FactoryOrderType::CombatRating => { query_builder = query_builder.order_by_desc(crate::schema::factory::vehicle::Column::CombatRating); },
            libfj::robocraft::FactoryOrderType::CosmeticRating => { query_builder = query_builder.order_by_desc(crate::schema::factory::vehicle::Column::CosmeticRating); },
            libfj::robocraft::FactoryOrderType::Added => { query_builder = query_builder.order_by_desc(crate::schema::factory::vehicle::Column::AddedTime); },
            libfj::robocraft::FactoryOrderType::CPU => { query_builder = query_builder.order_by_desc(crate::schema::factory::vehicle::Column::Cpu); },
            libfj::robocraft::FactoryOrderType::MostBought => { query_builder = query_builder.order_by_desc(crate::schema::factory::vehicle::Column::GetCount); },
        }

        if !query.text_filter.is_empty() {
            let query_text = format!("%{}%", query.text_filter.replace('%', ""));
            if query.player_filter {
                query_builder = query_builder.filter(
                    sea_orm::sea_query::Condition::any()
                        .add(crate::schema::user::Column::PublicId.like(query_text.clone()))
                        .add(crate::schema::user::Column::DisplayName.like(query_text))
                );
            } else {
                match query.text_search_field {
                    libfj::robocraft::FactoryTextSearchField::All => {
                        query_builder = query_builder.filter(
                            sea_orm::sea_query::Condition::any()
                                .add(crate::schema::user::Column::PublicId.like(query_text.clone()))
                                .add(crate::schema::user::Column::DisplayName.like(query_text.clone()))
                                .add(crate::schema::factory::vehicle::Column::Name.like(query_text.clone()))
                                .add(crate::schema::factory::vehicle::Column::Description.like(query_text))
                        );
                    },
                    libfj::robocraft::FactoryTextSearchField::Name => {
                        query_builder = query_builder.filter(
                            sea_orm::sea_query::Condition::any()
                                .add(crate::schema::factory::vehicle::Column::Name.like(query_text.clone()))
                        );
                    },
                    libfj::robocraft::FactoryTextSearchField::Player => {
                        query_builder = query_builder.filter(
                            sea_orm::sea_query::Condition::any()
                                .add(crate::schema::user::Column::PublicId.like(query_text.clone()))
                                .add(crate::schema::user::Column::DisplayName.like(query_text))
                        );
                    },
                }
            }
        }
        // TODO support movement and weapon filters
        if query.minimum_cpu > 0 {
            query_builder = query_builder.filter(crate::schema::factory::vehicle::Column::Cpu.gte(query.minimum_cpu as u32));
        }
        if query.maximum_cpu < usize::MAX {
            query_builder = query_builder.filter(crate::schema::factory::vehicle::Column::Cpu.lte(query.maximum_cpu as u32));
        }

        let query_builder = query_builder
            .find_also_related(crate::schema::user::Entity)
            .select_only()
            // WARNING: jank sea_orm query generation (fix)
            .column_as(crate::schema::factory::vehicle::Column::Id, "A_id")
            .column_as(crate::schema::factory::vehicle::Column::UserId, "A_user_id")
            .column_as(crate::schema::factory::vehicle::Column::CreationTime, "A_creation_time")
            .column_as(crate::schema::factory::vehicle::Column::Name, "A_name")
            .column_as(crate::schema::factory::vehicle::Column::Description, "A_description")
            .column_as(crate::schema::factory::vehicle::Column::AddedTime, "A_added_time")
            .column_as(crate::schema::factory::vehicle::Column::ExpiryTime, "A_expiry_time")
            .column_as(crate::schema::factory::vehicle::Column::Cpu, "A_cpu")
            .column_as(crate::schema::factory::vehicle::Column::TotalRobotRanking, "A_total_robot_ranking")
            .column_as(crate::schema::factory::vehicle::Column::GetCount, "A_get_count")
            .column_as(crate::schema::factory::vehicle::Column::Buyable, "A_buyable")
            .column_as(crate::schema::factory::vehicle::Column::RemovedDate, "A_removed_date")
            .column_as(crate::schema::factory::vehicle::Column::BanDate, "A_ban_date")
            .column_as(crate::schema::factory::vehicle::Column::Featured, "A_featured")
            .column_as(crate::schema::factory::vehicle::Column::BannerMessage, "A_banner_message")
            .column_as(crate::schema::factory::vehicle::Column::CombatRating, "A_combat_rating")
            .column_as(crate::schema::factory::vehicle::Column::CosmeticRating, "A_cosmetic_rating")
            .column_as(crate::schema::factory::vehicle::Column::CubeAmounts, "A_cube_amounts")
            .column_as(crate::schema::user::Column::PublicId, "B_public_id")
            .column_as(crate::schema::user::Column::DisplayName, "B_display_name")
            .into_model::<crate::schema::factory::vehicle_query::VehicleInfo, crate::schema::factory::vehicle_query::VehicleOwnerInfo>();
            //.into_partial_model::<crate::schema::factory::vehicle_query::VehicleInfo, crate::schema::factory::vehicle_query::VehicleOwnerInfo>();
        let pagination = query_builder.paginate(self.orm.as_ref(), query.page_size as u64);
        Ok(pagination.fetch_page((query.page - 1) as u64).await?
            .into_iter()
            .filter_map(|(vehicle, user_opt)| user_opt.map(|user| (vehicle, user)))
            .collect())
    }

    fn thumbnail_url(&self, id: i32) -> String {
        format!("{}/roboshop/Live/{}", self.cdn, id)
    }

    fn calculate_cube_amounts(&self, data: &[u8]) -> String {
        let mut counts: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
        let data = &data[4..];
        for chunk in data.chunks_exact(8) {
            let id_bytes: [u8; 4] = chunk[0..4].try_into().unwrap();
            let part_id = u32::from_le_bytes(id_bytes);

            *counts.entry(part_id).or_insert(0) += 1;
        }
        let mut str_map: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
        for (k, v) in counts {
            str_map.insert(k.to_string(), v);
        }

        serde_json::to_string(&str_map).unwrap_or_else(|_| "{}".to_string())
    }
}

#[async_trait::async_trait]
impl VehicleFactoryAdapter for FactoryDatabase {
    async fn vehicle(&self, id: i32) -> Result<Option<(VehicleInfo, VehicleQueryInfo)>, Box<dyn std::error::Error>> {
        let entities = self.full_vehicle_by_id(id).await?;
        Ok(entities.map(|(vehicle, user)| (
            VehicleInfo {
                id,
                cube_data: vehicle.cube_data,
                colour_data: vehicle.colour_data,
            },
            VehicleQueryInfo {
                id,
                name: vehicle.name,
                description: vehicle.description,
                thumbnail: self.thumbnail_url(id),
                added_by: user.public_id,
                added_by_display_name: user.display_name,
                added_date: vehicle.added_time.and_utc(),
                expiry_date: vehicle.expiry_time.and_utc(),
                cpu: vehicle.cpu as _,
                total_robot_ranking: vehicle.total_robot_ranking as _,
                rent_count: vehicle.get_count as _,
                buy_count: vehicle.get_count as _,
                removed_date: vehicle.removed_date.map(|t| t.and_utc()),
                ban_date: vehicle.ban_date.map(|t| t.and_utc()),
                featured: vehicle.featured,
                buyable: vehicle.buyable,
                banner_message: vehicle.banner_message,
                combat_rating: vehicle.combat_rating,
                cosmetic_rating: vehicle.cosmetic_rating,
                cube_amounts: serde_json::from_str(&vehicle.cube_amounts).unwrap_or_default(),
            },
        )))
    }

    async fn list(&self, query: libfj::robocraft::ListQuery) -> Result<Vec<VehicleQueryInfo>, Box<dyn std::error::Error>> {
        let entities = self.list_query(query).await?;
        Ok(entities.into_iter().map(|(vehicle, user)| VehicleQueryInfo {
            id: vehicle.id,
            name: vehicle.name,
            description: vehicle.description,
            thumbnail: self.thumbnail_url(vehicle.id),
            added_by: user.public_id,
            added_by_display_name: user.display_name,
            added_date: vehicle.added_time.and_utc(),
            expiry_date: vehicle.expiry_time.and_utc(),
            cpu: vehicle.cpu as _,
            total_robot_ranking: vehicle.total_robot_ranking as _,
            rent_count: vehicle.get_count as _,
            buy_count: vehicle.get_count as _,
            removed_date: vehicle.removed_date.map(|t| t.and_utc()),
            ban_date: vehicle.ban_date.map(|t| t.and_utc()),
            featured: vehicle.featured,
            buyable: vehicle.buyable,
            banner_message: vehicle.banner_message,
            combat_rating: vehicle.combat_rating,
            cosmetic_rating: vehicle.cosmetic_rating,
            cube_amounts: serde_json::from_str(&vehicle.cube_amounts).unwrap_or_default(),
        }).collect())
    }

    async fn upload(&self, vehicle: VehicleUploadInfo) -> Result<VehicleThumbnailInfo, Box<dyn std::error::Error>> {
        let now = chrono::Utc::now();
        let entity = crate::schema::factory::vehicle::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            user_id: sea_orm::ActiveValue::Set(vehicle.added_by_id),
            garage_id: sea_orm::ActiveValue::Set(vehicle.garage_id),
            creation_time: sea_orm::ActiveValue::Set(now.timestamp()),
            name: sea_orm::ActiveValue::Set(vehicle.name),
            description: sea_orm::ActiveValue::Set(vehicle.description),
            added_time: sea_orm::ActiveValue::Set(now.naive_utc()),
            expiry_time: sea_orm::ActiveValue::Set((now + self.upload_duration).naive_utc()),
            cpu: sea_orm::ActiveValue::Set(vehicle.cpu as i32),
            total_robot_ranking: sea_orm::ActiveValue::Set(vehicle.total_robot_ranking as i32),
            get_count: sea_orm::ActiveValue::Set(0),
            buyable: sea_orm::ActiveValue::Set(true),
            removed_date: sea_orm::ActiveValue::Set(None),
            ban_date: sea_orm::ActiveValue::Set(None),
            featured: sea_orm::ActiveValue::Set(false),
            banner_message: sea_orm::ActiveValue::Set(None),
            combat_rating: sea_orm::ActiveValue::Set(2.5),
            cosmetic_rating: sea_orm::ActiveValue::Set(2.5),
            cube_amounts: sea_orm::ActiveValue::Set(self.calculate_cube_amounts(&vehicle.cube_data)),
            cube_data: sea_orm::ActiveValue::Set(vehicle.cube_data),
            colour_data: sea_orm::ActiveValue::Set(vehicle.colour_data),
        };
        let saved_entity = entity.insert(self.orm.as_ref()).await?;
        Ok(VehicleThumbnailInfo {
            id: saved_entity.id,
            thumbnail: vehicle.thumbnail,
            needs_upload: true,
        })
    }

    async fn rate_vehicle(&self, id: i32, combat: i32, cosmetic: i32) -> Result<(), Box<dyn std::error::Error>> {
        let result = self.orm.as_ref().transaction::<'_, '_, _, (), sea_orm::DbErr>(|txn| {
            Box::pin(async move {
                let select_q = crate::schema::factory::vehicle::Entity::find_by_id(id)
                    .select_only()
                    .columns([
                        crate::schema::factory::vehicle::Column::CombatRating,
                        crate::schema::factory::vehicle::Column::CosmeticRating,
                        crate::schema::factory::vehicle::Column::GetCount,
                    ])
                    .into_tuple::<(f64, f64, i32)>();
                if let Some((curr_combat_rating, curr_cosmetic_rating, get_count)) = select_q.one(txn).await? {
                    let get_count = get_count.clamp(1, 100) as f64;
                    let next_combat_rating = curr_combat_rating + (((combat as f64) - curr_combat_rating) / get_count);
                    let next_cosmetic_rating = curr_cosmetic_rating + (((cosmetic as f64) - curr_cosmetic_rating) / get_count);
                    crate::schema::factory::vehicle::ActiveModel {
                        id: sea_orm::ActiveValue::Set(id),
                        cosmetic_rating: sea_orm::ActiveValue::Set(next_cosmetic_rating),
                        combat_rating: sea_orm::ActiveValue::Set(next_combat_rating),
                        ..Default::default()
                    }.update(txn).await?;
                }
                Ok(())
            })
        }).await;
        if let Err(e) = result {
            return match e {
                sea_orm::TransactionError::Connection(x) => Err(x.into()),
                sea_orm::TransactionError::Transaction(x) => Err(x.into()),
            };
        }
        Ok(())
    }

    /// Just update any purchase trackers
    async fn purchase(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        crate::schema::factory::vehicle::Entity::update_many()
            .col_expr(crate::schema::factory::vehicle::Column::GetCount, crate::schema::factory::vehicle::Column::GetCount.into_expr().add(1))
            .filter(crate::schema::factory::vehicle::Column::Id.eq(id))
            .exec(self.orm.as_ref()).await?;
        Ok(())
    }

    async fn update_vehicle(&self, id: i32, cube_data: Option<Vec<u8>>, colour_data: Option<Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
        let cube_amounts = if let Some(cube_data) = &cube_data {
            sea_orm::ActiveValue::Set(self.calculate_cube_amounts(cube_data))
        } else {
            sea_orm::ActiveValue::NotSet
        };
        let result = self.orm.as_ref().transaction::<'_, '_, _, (), sea_orm::DbErr>(|txn| {
            Box::pin(async move {
                let select_q = crate::schema::factory::vehicle::Entity::find_by_id(id)
                    .select_only()
                    .columns([
                        crate::schema::factory::vehicle::Column::Id,
                    ]);
                if select_q.count(txn).await? == 1 {
                    let mut to_update = crate::schema::factory::vehicle::ActiveModel {
                        id: sea_orm::ActiveValue::Set(id),
                        cube_amounts,
                        ..Default::default()
                    };
                    if let Some(cube_data) = cube_data {
                        to_update.cube_data = sea_orm::ActiveValue::Set(cube_data);
                    }
                    if let Some(colour_data) = colour_data {
                        to_update.colour_data = sea_orm::ActiveValue::Set(colour_data);
                    }
                    to_update.update(txn).await?;
                }
                Ok(())
            })
        }).await;
        if let Err(e) = result {
            return match e {
                sea_orm::TransactionError::Connection(x) => Err(x.into()),
                sea_orm::TransactionError::Transaction(x) => Err(x.into()),
            };
        }
        Ok(())
    }

    async fn remove_vehicle(&self, id: i32, user_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let now = chrono::Utc::now();
        let result = self.orm.as_ref().transaction::<'_, '_, _, (), sea_orm::DbErr>(|txn| {
            Box::pin(async move {
                let select_q = crate::schema::factory::vehicle::Entity::find_by_id(id)
                    .select_only()
                    .columns([
                        crate::schema::factory::vehicle::Column::Id,
                        crate::schema::factory::vehicle::Column::UserId,
                    ])
                    .into_tuple::<(i32, i32)>();
                if let Some((_selected_id, selected_user_id)) = select_q.one(txn).await? {
                    if selected_user_id != user_id {
                        //log::warn!("User {} tried to remove factory vehicle owned by {}", user_id, selected_user_id);
                        return Ok(());
                    }
                    crate::schema::factory::vehicle::ActiveModel {
                        id: sea_orm::ActiveValue::Set(id),
                        buyable: sea_orm::ActiveValue::Set(false),
                        removed_date: sea_orm::ActiveValue::Set(Some(now.naive_utc())),
                        ..Default::default()
                    }.update(txn).await?;
                }
                Ok(())
            })
        }).await;
        if let Err(e) = result {
            return match e {
                sea_orm::TransactionError::Connection(x) => Err(x.into()),
                sea_orm::TransactionError::Transaction(x) => Err(x.into()),
            };
        }
        Ok(())
    }

    async fn set_featured(&self, id: i32, is_featured: bool) -> Result<(), Box<dyn std::error::Error>> {
        crate::schema::factory::vehicle::ActiveModel {
            id: sea_orm::ActiveValue::Set(id),
            featured: sea_orm::ActiveValue::Set(is_featured),
            ..Default::default()
        }.update(self.orm.as_ref()).await?;
        Ok(())
    }
}
