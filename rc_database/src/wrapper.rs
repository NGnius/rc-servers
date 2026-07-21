use sea_orm_migration::MigratorTrait;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, TransactionTrait, sea_query::ExprTrait, sqlx::types::chrono};

pub struct Database {
    orm: std::sync::Arc<sea_orm::DatabaseConnection>,
    metrics: std::sync::Arc<std::sync::Mutex<super::metrics::MetricsState>>,
}

impl Database {
    pub async fn init(uri: &str) -> Result<Self, sea_orm::DbErr>{
        let mut db = sea_orm::Database::connect(uri).await?;
        let metrics_data = std::sync::Arc::new(std::sync::Mutex::new(super::metrics::MetricsState::new()));
        db.set_metric_callback(super::metrics::metrics_cb(metrics_data.clone()));
        //let schema_manager = SchemaManager::new(&db);
        super::Migrator::up(&db, None).await?;
        Ok(Self {
            orm: std::sync::Arc::new(db),
            metrics: metrics_data,
        })
    }

    pub fn vehicle_factory(&self, cdn: std::sync::Arc<String>) -> super::FactoryDatabase {
        super::FactoryDatabase::init_preconnected(self.orm.clone(), cdn)
    }

    pub async fn user_count(&self) -> Result<u64, sea_orm::DbErr> {
        crate::schema::user::Entity::find()
            .count(self.orm.as_ref())
            .await
    }

    pub async fn user_by_display_name(&self, public_id: String) -> Result<Option<crate::schema::user::Model>, sea_orm::DbErr> {
        crate::schema::user::Entity::find()
            .filter(sea_orm::sea_query::Expr::expr(
                sea_orm::sea_query::Func::lower(crate::schema::user::Column::DisplayName.into_expr())
                ).eq(public_id.to_lowercase())
            )
            .one(self.orm.as_ref())
            .await
    }

    pub async fn user_by_display_name_and_federation(&self, public_id: String, federation_id: i32) -> Result<Option<crate::schema::user::Model>, sea_orm::DbErr> {
        crate::schema::user::Entity::find()
            .filter(sea_orm::sea_query::Expr::expr(
                sea_orm::sea_query::Func::lower(crate::schema::user::Column::DisplayName.into_expr())
                ).eq(public_id.to_lowercase())
            )
            .filter(crate::schema::user::Column::FederationId.eq(Some(federation_id)))
            .one(self.orm.as_ref())
            .await
    }

    pub async fn user_by_public_id(&self, public_id: String) -> Result<Option<crate::schema::user::Model>, sea_orm::DbErr> {
        crate::schema::user::Entity::find()
            .filter(crate::schema::user::Column::PublicId.eq(public_id))
            .one(self.orm.as_ref())
            .await
    }

    pub async fn users_by_public_id<'a>(&self, public_ids: impl std::iter::Iterator<Item=&'a String>) -> Result<Vec<crate::schema::user::Model>, sea_orm::DbErr> {
        crate::schema::user::Entity::find()
            .filter(crate::schema::user::Column::PublicId.is_in(public_ids))
            .all(self.orm.as_ref())
            .await
    }

    pub async fn user_by_some_social_id(&self, public_id: String) -> Result<Option<crate::schema::user::Model>, sea_orm::DbErr> {
        let lower_public_id = public_id.to_lowercase();
        crate::schema::user::Entity::find()
            .filter(sea_orm::sea_query::Expr::expr(
                    sea_orm::sea_query::Func::lower(crate::schema::user::Column::DisplayName.into_expr())
                ).eq(&lower_public_id).or(
                    sea_orm::sea_query::Func::lower(crate::schema::user::Column::PublicId.into_expr())
                        .eq(&lower_public_id)
                ).or(
                    sea_orm::sea_query::Func::lower(crate::schema::user::Column::Email.into_expr())
                        .eq(&lower_public_id)
                )
            )
            .one(self.orm.as_ref())
            .await
    }

    pub async fn user_by_steam_id(&self, steam_id: u64) -> Result<Option<crate::schema::user::Model>, sea_orm::DbErr> {
        crate::schema::user::Entity::find()
            .filter(crate::schema::user::Column::SteamId.eq(Some(steam_id.to_string())))
            .one(self.orm.as_ref())
            .await
    }

    pub async fn user_by_email(&self, email: String) -> Result<Option<crate::schema::user::Model>, sea_orm::DbErr> {
        crate::schema::user::Entity::find()
            .filter(sea_orm::sea_query::Expr::expr(
                sea_orm::sea_query::Func::lower(crate::schema::user::Column::Email.into_expr())
                ).eq(email.to_lowercase())
            )
            .one(self.orm.as_ref())
            .await
    }

    pub async fn user_by_any_unique_id(&self, id: String) -> Result<Option<crate::schema::user::Model>, sea_orm::DbErr> {
        if let Ok(steam_id) = id.parse::<u64>() {
            if let Some(res) = self.user_by_steam_id(steam_id).await? {
                return Ok(Some(res));
            }
        }
        if id.contains('@') {
            if let Some(res) = self.user_by_email(id.clone()).await? {
                return Ok(Some(res));
            }
        }
        self.user_by_display_name(id.clone()).await
    }

    pub async fn insert_user(&self, entity: crate::schema::user::ActiveModel) -> Result<crate::schema::user::Model, sea_orm::DbErr> {
        entity.insert(self.orm.as_ref()).await
    }

    pub async fn user_aux_by_user_id(&self, user_id: i32) -> Result<Vec<crate::schema::user_aux::Model>, sea_orm::DbErr> {
        crate::schema::user_aux::Entity::find()
            .filter(crate::schema::user_aux::Column::UserId.eq(user_id))
            .all(self.orm.as_ref())
            .await
    }

    pub async fn user_aux_by_user_id_and_descriptor(&self, user_id: i32, descriptor: crate::schema::user_aux::Descriptor) -> Result<Option<crate::schema::user_aux::Model>, sea_orm::DbErr> {
        crate::schema::user_aux::Entity::find()
            .filter(crate::schema::user_aux::Column::UserId.eq(user_id))
            .filter(crate::schema::user_aux::Column::Descriptor.eq(descriptor))
            .one(self.orm.as_ref())
            .await
    }

    pub async fn user_auxs_by_user_ids_and_descriptor(&self, user_ids: impl std::iter::Iterator<Item=i32>, descriptor: crate::schema::user_aux::Descriptor) -> Result<Vec<crate::schema::user_aux::Model>, sea_orm::DbErr> {
        crate::schema::user_aux::Entity::find()
            .filter(crate::schema::user_aux::Column::UserId.is_in(user_ids))
            .filter(crate::schema::user_aux::Column::Descriptor.eq(descriptor))
            .all(self.orm.as_ref())
            .await
    }

    pub async fn insert_user_aux(&self, entities: Vec<crate::schema::user_aux::ActiveModel>) -> Result<(), sea_orm::DbErr> {
        crate::schema::user_aux::Entity::insert_many(entities).exec(self.orm.as_ref()).await?;
        Ok(())
    }

    pub async fn update_user_aux_by_user_id_and_descriptor(&self, mut entity: crate::schema::user_aux::ActiveModel, user_id: i32, descriptor: crate::schema::user_aux::Descriptor) -> Result<Option<crate::schema::user_aux::Model>, sea_orm::DbErr> {
        let id_opt = crate::schema::user_aux::Entity::find()
            .select_only()
            .column(crate::schema::user_aux::Column::Id)
            .filter(crate::schema::user_aux::Column::UserId.eq(user_id))
            .filter(crate::schema::user_aux::Column::Descriptor.eq(descriptor.clone()))
            .into_model::<crate::schema::common_query::Id>()
            .one(self.orm.as_ref())
            .await?;
        if let Some(id) = id_opt {
            // update
            entity.id = sea_orm::ActiveValue::Set(id.id);
            Ok(Some(crate::schema::user_aux::Entity::update(entity)
                .exec(self.orm.as_ref())
                .await?))
        } else {
            Ok(None)
        }
    }

    pub async fn update_user_aux_by_user_id_and_descriptor_custom(&self, user_id: i32, descriptor: crate::schema::user_aux::Descriptor, custom: impl (FnOnce(&crate::schema::user_aux::Model) -> Option<crate::schema::user_aux::ActiveModel>) + Send + 'static) -> Result<Option<crate::schema::user_aux::Model>, sea_orm::DbErr> {
        self.orm.transaction(|txn| {
            Box::pin(async move {
                let opt = crate::schema::user_aux::Entity::find()
                    .filter(crate::schema::user_aux::Column::UserId.eq(user_id))
                    .filter(crate::schema::user_aux::Column::Descriptor.eq(descriptor))
                    .one(txn)
                    .await?;

                if let Some(model) = opt {
                    if let Some(updated_model) = custom(&model) {
                        Ok(Some(crate::schema::user_aux::Entity::update(updated_model)
                            .exec(txn)
                            .await?))
                    } else {
                        Ok(Some(model))
                    }
                } else {
                    Ok(None)
                }
            })
        }).await.map_err(|e| {
            match e {
                sea_orm::TransactionError::Connection(db) => db,
                sea_orm::TransactionError::Transaction(txn) => txn,
            }
        })
    }

    pub async fn perms_by_user_id(&self, user_id: i32) -> Result<Option<crate::schema::permissions::Model>, sea_orm::DbErr> {
        crate::schema::permissions::Entity::find()
            .filter(crate::schema::permissions::Column::UserId.eq(user_id))
            .one(self.orm.as_ref())
            .await
    }

    pub async fn insert_perms(&self, entity: crate::schema::permissions::ActiveModel) -> Result<crate::schema::permissions::Model, sea_orm::DbErr> {
        entity.insert(self.orm.as_ref()).await
    }

    pub async fn update_perms_by_user_id(&self, mut entity: crate::schema::permissions::ActiveModel, user_id: i32) -> Result<Option<crate::schema::permissions::Model>, sea_orm::DbErr> {
        let id_opt = crate::schema::permissions::Entity::find()
            .select_only()
            .column(crate::schema::permissions::Column::Id)
            .filter(crate::schema::permissions::Column::UserId.eq(user_id))
            .into_model::<crate::schema::common_query::Id>()
            .one(self.orm.as_ref())
            .await?;
        if let Some(id) = id_opt {
            entity.id = sea_orm::ActiveValue::Set(id.id);
            Ok(Some(crate::schema::permissions::Entity::update(entity)
                .exec(self.orm.as_ref())
                .await?))
        } else {
            Ok(None)
        }
    }

    pub async fn garage_count(&self) -> Result<u64, sea_orm::DbErr> {
        crate::schema::garage::Entity::find()
            .count(self.orm.as_ref())
            .await
    }

    pub async fn garage_count_by_user_id(&self, user_id: i32) -> Result<u64, sea_orm::DbErr> {
        crate::schema::garage::Entity::find()
            .filter(crate::schema::garage::Column::UserId.eq(user_id))
            .count(self.orm.as_ref())
            .await
    }

    pub async fn garage_count_by_user_id_between(&self, user_id: i32, max_cpu: u64, min_cpu: u64) -> Result<u64, sea_orm::DbErr> {
        crate::schema::garage::Entity::find()
            .filter(crate::schema::garage::Column::UserId.eq(user_id))
            .filter(crate::schema::garage::Column::TotalRobotCpu.lte(max_cpu))
            .filter(crate::schema::garage::Column::TotalRobotCpu.gte(min_cpu))
            .count(self.orm.as_ref())
            .await
    }

    pub async fn garage_count_by_user_id_factory(&self, user_id: i32) -> Result<u64, sea_orm::DbErr> {
        crate::schema::garage::Entity::find()
            .filter(crate::schema::garage::Column::UserId.eq(user_id))
            .filter(crate::schema::garage::Column::CrfId.is_not_null())
            .count(self.orm.as_ref())
            .await
    }

    pub async fn garage_storage_by_user_id(&self, user_id: i32) -> Result<Option<u64>, sea_orm::DbErr> {
        let size = match self.orm.as_ref() {
            // TODO implement support in other databases
            sea_orm::DatabaseConnection::SqlxPostgresPoolConnection(_) => {

                #[cfg(debug_assertions)]
                {
                    /*let query = sea_orm::sea_query::raw_query!(
                        PostgresQueryBuilder,
                        r#"select sum(pg_column_size(g.*)) from garages g where g.user_id = $user_id;"#
                    );*/
                    struct PgColumnSize;
                    impl sea_orm::sea_query::Iden for PgColumnSize {
                        fn unquoted(&self, s: &mut dyn std::fmt::Write) {
                            write!(s, "pg_column_size").unwrap();
                        }
                    }
                    let result = crate::schema::garage::Entity::find()
                        .select_only()
                        .expr_as(
                            sea_orm::sea_query::Func::cust(PgColumnSize)
                                .arg(sea_orm::sea_query::Expr::col((crate::schema::garage::Entity, sea_orm::sea_query::Asterisk)))
                        , "column")
                        .filter(crate::schema::garage::Column::UserId.eq(user_id))
                        .into_model::<crate::schema::common_query::SingleColumn<u64>>()
                        .one(self.orm.as_ref())
                        .await?;
                    result.map(|x| x.column)
                }
                #[cfg(not(debug_assertions))]
                {
                    let _ = user_id;
                    None
                }
            },
            _ => None,
        };
        Ok(size)
    }

    pub async fn garage_max_slot_by_user_id(&self, user_id: i32) -> Result<i32, sea_orm::DbErr> {
        let result = crate::schema::garage::Entity::find()
            .select_only()
            .column_as(crate::schema::garage::Column::Slot.max(), "column")
            .filter(crate::schema::garage::Column::UserId.eq(user_id))
            .into_model::<crate::schema::common_query::SingleColumn<i32>>()
            .one(self.orm.as_ref())
            .await?;
        Ok(result.map(|x| *x).unwrap_or(0))
    }

    pub async fn garage_selected(&self, user_id: i32) -> Result<Option<crate::schema::garage::Model>, sea_orm::DbErr> {
        crate::schema::garage::Entity::find()
            .filter(crate::schema::garage::Column::UserId.eq(user_id))
            .filter(crate::schema::garage::Column::Selected.eq(true))
            .one(self.orm.as_ref())
            .await
    }

    pub async fn garage_by_user_id_and_slot(&self, user_id: i32, garage_slot: i32) -> Result<Option<crate::schema::garage::Model>, sea_orm::DbErr> {
        crate::schema::garage::Entity::find()
            .filter(crate::schema::garage::Column::UserId.eq(user_id))
            .filter(crate::schema::garage::Column::Slot.eq(garage_slot))
            .one(self.orm.as_ref())
            .await
    }

    pub async fn garages_by_user_id(&self, user_id: i32) -> Result<Vec<crate::schema::garage::Model>, sea_orm::DbErr> {
        crate::schema::garage::Entity::find()
            .filter(crate::schema::garage::Column::UserId.eq(user_id))
            .order_by_asc(crate::schema::garage::Column::Slot)
            .all(self.orm.as_ref())
            .await
    }

    pub async fn garage_by_uuid(&self, uuid: i64) -> Result<Option<crate::schema::garage::Model>, sea_orm::DbErr> {
        crate::schema::garage::Entity::find()
            .filter(crate::schema::garage::Column::Uuid.eq(uuid))
            .one(self.orm.as_ref())
            .await
    }

    pub async fn garage_by_id(&self, id: i32) -> Result<Option<crate::schema::garage::Model>, sea_orm::DbErr> {
        crate::schema::garage::Entity::find_by_id(id)
            .one(self.orm.as_ref())
            .await
    }

    pub async fn slot_of_garage_by_id_and_user_id(&self, garage_id: i32, user_id: i32) -> Result<Option<i32>, sea_orm::DbErr> {
        let opt = crate::schema::garage::Entity::find()
            .select_only()
            .column_as(crate::schema::garage::Column::Slot, "column")
            .filter(crate::schema::garage::Column::Id.eq(garage_id))
            .filter(crate::schema::garage::Column::UserId.eq(user_id))
            .into_model::<crate::schema::common_query::SingleColumn<i32>>()
            .one(self.orm.as_ref())
            .await?;
        Ok(opt.map(|x| *x))
    }

    pub async fn insert_garages(&self, entities: Vec<crate::schema::garage::ActiveModel>) -> Result<(), sea_orm::DbErr> {
        crate::schema::garage::Entity::insert_many(entities).exec(self.orm.as_ref()).await?;
        Ok(())
    }

    pub async fn insert_garage(&self, entity: crate::schema::garage::ActiveModel) -> Result<crate::schema::garage::Model, sea_orm::DbErr> {
        entity.insert(self.orm.as_ref()).await
    }

    pub async fn update_garage(&self, entity: crate::schema::garage::ActiveModel) -> Result<crate::schema::garage::Model, sea_orm::DbErr> {
        crate::schema::garage::Entity::update(entity)
            .exec(self.orm.as_ref())
            .await
    }

    pub async fn update_garage_by_user_id_and_slot(&self, mut entity: crate::schema::garage::ActiveModel, user_id: i32, slot: i32) -> Result<Option<crate::schema::garage::Model>, sea_orm::DbErr> {
        let id_opt = crate::schema::garage::Entity::find()
            .select_only()
            .column(crate::schema::garage::Column::Id)
            .column(crate::schema::garage::Column::ThumbnailVersion)
            .filter(crate::schema::garage::Column::UserId.eq(user_id))
            .filter(crate::schema::garage::Column::Slot.eq(slot))
            .into_model::<crate::schema::common_query::IdAndThumbnailVersion>()
            .one(self.orm.as_ref())
            .await?;
        if let Some(id) = id_opt {
            entity.id = sea_orm::ActiveValue::Set(id.id);
            if entity.robot_data.is_set() || entity.colour_data.is_set() {
                entity.thumbnail_version = sea_orm::ActiveValue::Set(id.thumbnail_version.wrapping_add(1));
            }
            Ok(Some(crate::schema::garage::Entity::update(entity)
                .exec(self.orm.as_ref())
                .await?))
        } else {
            Ok(None)
        }
    }

    pub async fn update_garage_by_uuid(&self, mut entity: crate::schema::garage::ActiveModel, uuid: i64) -> Result<Option<crate::schema::garage::Model>, sea_orm::DbErr> {
        let id_opt = crate::schema::garage::Entity::find()
            .select_only()
            .column(crate::schema::garage::Column::Id)
            .filter(crate::schema::garage::Column::Uuid.eq(uuid))
            .into_model::<crate::schema::common_query::Id>()
            .one(self.orm.as_ref())
            .await?;
        if let Some(id) = id_opt {
            entity.id = sea_orm::ActiveValue::Set(id.id);
            Ok(Some(crate::schema::garage::Entity::update(entity)
                .exec(self.orm.as_ref())
                .await?))
        } else {
            Ok(None)
        }
    }

    pub async fn update_garage_selected_by_user_id_and_slot(&self, user_id: i32, slot: i32) -> Result<(), sea_orm::DbErr> {
        self.orm.transaction(|txn| {
            Box::pin(async move {
                crate::schema::garage::Entity::update_many()
                    .col_expr(crate::schema::garage::Column::Selected, sea_orm::sea_query::Expr::value(false))
                    .filter(crate::schema::garage::Column::UserId.eq(user_id))
                    .filter(crate::schema::garage::Column::Slot.ne(slot))
                    .exec(txn)
                    .await?;

                crate::schema::garage::Entity::update_many()
                    .col_expr(crate::schema::garage::Column::Selected, sea_orm::sea_query::Expr::value(true))
                    .filter(crate::schema::garage::Column::UserId.eq(user_id))
                    .filter(crate::schema::garage::Column::Slot.eq(slot))
                    .exec(txn)
                    .await?;

                Ok(())
            })
        }).await.map_err(|e| {
            match e {
                sea_orm::TransactionError::Connection(db) => db,
                sea_orm::TransactionError::Transaction(txn) => txn,
            }
        })?;
        Ok(())
    }

    pub async fn count_sanctions_to_ack_by_user_id_and_descriptor(&self, user_id: i32, desc: crate::schema::sanction::Descriptor) -> Result<u64, sea_orm::DbErr> {
        crate::schema::sanction::Entity::find()
            .filter(crate::schema::sanction::Column::UserId.eq(user_id))
            .filter(crate::schema::sanction::Column::Descriptor.eq(desc))
            .filter(crate::schema::sanction::Column::Acknowledged.is_null())
            .count(self.orm.as_ref())
            .await
    }

    pub async fn count_sanctions_by_user_id(&self, user_id: i32) -> Result<u64, sea_orm::DbErr> {
        crate::schema::sanction::Entity::find()
            .filter(crate::schema::sanction::Column::UserId.eq(user_id))
            //.order_by_asc(crate::schema::sanction::Column::CreationTime)
            .count(self.orm.as_ref())
            .await
    }

    pub async fn count_sanctions_by_user_id_and_ack(&self, user_id: i32, is_acked: bool) -> Result<u64, sea_orm::DbErr> {
        crate::schema::sanction::Entity::find()
            .filter(crate::schema::sanction::Column::UserId.eq(user_id))
            .filter(crate::schema::sanction::Column::Acknowledged.eq(is_acked))
            //.order_by_asc(crate::schema::sanction::Column::CreationTime)
            .count(self.orm.as_ref())
            .await
    }

    pub async fn count_sanctions_by_user_id_and_descriptor(&self, user_id: i32, desc: crate::schema::sanction::Descriptor) -> Result<u64, sea_orm::DbErr> {
        crate::schema::sanction::Entity::find()
            .filter(crate::schema::sanction::Column::UserId.eq(user_id))
            .filter(crate::schema::sanction::Column::Descriptor.eq(desc))
            //.order_by_asc(crate::schema::sanction::Column::CreationTime)
            .count(self.orm.as_ref())
            .await
    }

    pub async fn sanctions_by_user_id(&self, user_id: i32) -> Result<Vec<crate::schema::sanction::Model>, sea_orm::DbErr> {
        crate::schema::sanction::Entity::find()
            .filter(crate::schema::sanction::Column::UserId.eq(user_id))
            .filter(crate::schema::sanction::Column::Acknowledged.is_null())
            .order_by_asc(crate::schema::sanction::Column::CreationTime)
            .all(self.orm.as_ref())
            .await
    }

    pub async fn sanction_by_user_id_and_descriptor_and_active(&self, user_id: i32, desc: crate::schema::sanction::Descriptor) -> Result<Option<crate::schema::sanction::Model>, sea_orm::DbErr> {
        let now = chrono::Utc::now().timestamp();
        let expiry_expr = sea_orm::sea_query::Expr::col(crate::schema::sanction::Column::CreationTime).add(sea_orm::sea_query::Expr::col(crate::schema::sanction::Column::Duration));
        crate::schema::sanction::Entity::find()
            .filter(crate::schema::sanction::Column::UserId.eq(user_id))
            .filter(crate::schema::sanction::Column::Descriptor.eq(desc))
            .filter(crate::schema::sanction::Column::Duration.is_not_null())
            .filter(expiry_expr.clone().gte(now))
            .order_by_asc(crate::schema::sanction::Column::CreationTime)
            .one(self.orm.as_ref())
            .await
    }

    pub async fn insert_sanction(&self, entity: crate::schema::sanction::ActiveModel) -> Result<crate::schema::sanction::Model, sea_orm::DbErr> {
        entity.insert(self.orm.as_ref()).await
    }

    pub async fn game_count(&self) -> Result<u64, sea_orm::DbErr> {
        crate::schema::multiplayer_game::Entity::find()
            .count(self.orm.as_ref())
            .await
    }

    pub async fn game_by_guid(&self, game_guid: i64) -> Result<Option<crate::schema::multiplayer_game::Model>, sea_orm::DbErr> {
        crate::schema::multiplayer_game::Entity::find()
            .filter(crate::schema::multiplayer_game::Column::Guid.eq(game_guid))
            .order_by_desc(crate::schema::multiplayer_game::Column::CreationTime)
            .one(self.orm.as_ref())
            .await
    }

    pub async fn count_games_by_user_id(&self, user_id: i32) -> Result<u64, sea_orm::DbErr> {
        crate::schema::multiplayer_game_player::Entity::find()
            .filter(crate::schema::multiplayer_game_player::Column::UserId.eq(user_id))
            .count(self.orm.as_ref())
            .await
    }

    pub async fn game_by_user_id_and_completion(&self, user_id: i32, is_complete: bool) -> Result<Option<crate::schema::multiplayer_game::Model>, sea_orm::DbErr> {
        Ok(crate::schema::multiplayer_game::Entity::find()
            .find_also_related(crate::schema::multiplayer_game_player::Entity)
            //.join(sea_orm::JoinType::InnerJoin, crate::schema::multiplayer_game_player::Relation::Game.def())
            .filter(crate::schema::multiplayer_game::Column::IsComplete.eq(is_complete))
            .filter(crate::schema::multiplayer_game_player::Column::UserId.eq(user_id))
            .order_by_desc(crate::schema::multiplayer_game::Column::CreationTime)
            //.into_model()
            .one(self.orm.as_ref())
            .await?
            .map(|(x, _)| x))
    }

    pub async fn game_and_player_by_user_id_and_completion(&self, user_id: i32, is_complete: bool) -> Result<Option<(crate::schema::multiplayer_game::Model, crate::schema::multiplayer_game_player::Model)>, sea_orm::DbErr> {
        Ok(crate::schema::multiplayer_game::Entity::find()
            .find_also_related(crate::schema::multiplayer_game_player::Entity)
            //.join(sea_orm::JoinType::InnerJoin, crate::schema::multiplayer_game_player::Relation::Game.def())
            .filter(crate::schema::multiplayer_game::Column::IsComplete.eq(is_complete))
            .filter(crate::schema::multiplayer_game_player::Column::UserId.eq(user_id))
            .order_by_asc(crate::schema::multiplayer_game::Column::CreationTime)
            //.into_model()
            .one(self.orm.as_ref())
            .await?
            .and_then(|(game, player)| player.map(|player| (game, player))))
    }

    pub async fn update_complete_game_by_game_guid(&self, game_guid: i64) -> Result<(), sea_orm::DbErr> {
        crate::schema::multiplayer_game::Entity::update_many()
            .col_expr(crate::schema::multiplayer_game::Column::IsComplete, sea_orm::sea_query::Expr::value(true))
            .filter(crate::schema::multiplayer_game::Column::Guid.eq(game_guid))
            .exec(self.orm.as_ref())
            .await?;
        Ok(())
    }

    pub async fn complete_all_games(&self) -> Result<(), sea_orm::DbErr> {
        crate::schema::multiplayer_game::Entity::update_many()
            .col_expr(crate::schema::multiplayer_game::Column::IsComplete, sea_orm::sea_query::Expr::value(true))
            .filter(crate::schema::multiplayer_game::Column::IsComplete.eq(false))
            .exec(self.orm.as_ref())
            .await?;
        Ok(())
    }

    pub async fn insert_game(&self, entity: crate::schema::multiplayer_game::ActiveModel) -> Result<crate::schema::multiplayer_game::Model, sea_orm::DbErr> {
        entity.insert(self.orm.as_ref()).await
    }

    pub async fn players_by_game_guid_and_completion(&self, game_guid: i64, is_complete: bool) -> Result<Vec<crate::schema::multiplayer_game_player::Model>, sea_orm::DbErr> {
        Ok(crate::schema::multiplayer_game_player::Entity::find()
            .find_also_related(crate::schema::multiplayer_game::Entity)
            .filter(crate::schema::multiplayer_game::Column::Guid.eq(game_guid))
            .filter(crate::schema::multiplayer_game::Column::IsComplete.eq(is_complete))
            //.into_model::<crate::schema::multiplayer_game_player::Model>()
            .all(self.orm.as_ref())
            .await?
            .into_iter()
            .map(|(player, _)| player)
            .collect())
    }

    /*pub async fn players_by_game_guid_and_completion_heavy(&self, game_guid: i64, is_complete: bool) -> Result<Vec<(crate::schema::multiplayer_game_player::Model, crate::schema::user::Model)>, sea_orm::DbErr> {
        Ok(crate::schema::multiplayer_game_player::Entity::find()
            .find_also_related(crate::schema::user::Entity)
            .find_also_related(crate::schema::multiplayer_game::Entity)
            //.join(sea_orm::JoinType::InnerJoin, crate::schema::multiplayer_game::Relation::Player.def())
            //.join(sea_orm::JoinType::InnerJoin, crate::schema::user::Relation::Player.def())
            .filter(crate::schema::multiplayer_game::Column::Guid.eq(game_guid))
            .filter(crate::schema::multiplayer_game::Column::IsComplete.eq(is_complete))
            .all(self.orm.as_ref())
            .await?
            .into_iter()
            .filter_map(|(player, user, _)| user.map(|user| (player, user)))
            .collect())
    }*/

    /*pub async fn players_by_game_guid_and_completion_and_team_heavy(&self, game_guid: i64, team: i32, is_complete: bool) -> Result<Vec<(crate::schema::multiplayer_game_player::Model, crate::schema::user::Model)>, sea_orm::DbErr> {
        Ok(crate::schema::multiplayer_game_player::Entity::find()
            .find_also_related(crate::schema::user::Entity)
            .find_also_related(crate::schema::multiplayer_game::Entity)
            //.join(sea_orm::JoinType::InnerJoin, crate::schema::multiplayer_game::Relation::Player.def())
            //.join(sea_orm::JoinType::InnerJoin, crate::schema::user::Relation::Player.def())
            .filter(crate::schema::multiplayer_game::Column::Guid.eq(game_guid))
            .filter(crate::schema::multiplayer_game::Column::IsComplete.eq(is_complete))
            .filter(crate::schema::multiplayer_game_player::Column::Team.eq(team))
            .all(self.orm.as_ref())
            .await?
            .into_iter()
            .filter_map(|(player, user, _)| user.map(|user| (player, user)))
            .collect())
    }*/

    pub async fn players_by_game_guid_and_completion_and_team(&self, game_guid: i64, team: i32, is_complete: bool) -> Result<Vec<crate::schema::multiplayer_game_player::Model>, sea_orm::DbErr> {
        Ok(crate::schema::multiplayer_game_player::Entity::find()
            .find_also_related(crate::schema::multiplayer_game::Entity)
            //.join(sea_orm::JoinType::InnerJoin, crate::schema::multiplayer_game::Relation::Player.def())
            //.join(sea_orm::JoinType::InnerJoin, crate::schema::user::Relation::Player.def())
            .filter(crate::schema::multiplayer_game::Column::Guid.eq(game_guid))
            .filter(crate::schema::multiplayer_game::Column::IsComplete.eq(is_complete))
            .filter(crate::schema::multiplayer_game_player::Column::Team.eq(team))
            .all(self.orm.as_ref())
            .await?
            .into_iter()
            .map(|(player, _)| player)
            .collect())
    }

    pub async fn players_by_game_id(&self, game_id: i32) -> Result<Vec<crate::schema::multiplayer_game_player::Model>, sea_orm::DbErr> {
        Ok(crate::schema::multiplayer_game_player::Entity::find()
            .find_also_related(crate::schema::multiplayer_game::Entity)
            .filter(crate::schema::multiplayer_game::Column::Id.eq(game_id))
            .all(self.orm.as_ref())
            .await?
            .into_iter()
            .map(|(player, _)| player)
            .collect())
    }

    pub async fn player_by_user_id_and_game_guid(&self, user_id: i32, game_guid: i64) -> Result<Option<crate::schema::multiplayer_game_player::Model>, sea_orm::DbErr> {
        Ok(crate::schema::multiplayer_game::Entity::find()
            .find_also_related(crate::schema::multiplayer_game_player::Entity)
            .filter(crate::schema::multiplayer_game::Column::Guid.eq(game_guid))
            .filter(crate::schema::multiplayer_game_player::Column::UserId.eq(user_id))
            //.order_by_asc(crate::schema::multiplayer_game::Column::CreationTime)
            .one(self.orm.as_ref())
            .await?
            .and_then(|(_, player)| player))
    }

    pub async fn insert_player(&self, entity: crate::schema::multiplayer_game_player::ActiveModel) -> Result<crate::schema::multiplayer_game_player::Model, sea_orm::DbErr> {
        entity.insert(self.orm.as_ref()).await
    }

    pub async fn insert_players(&self, entities: Vec<crate::schema::multiplayer_game_player::ActiveModel>) -> Result<(), sea_orm::DbErr> {
        crate::schema::multiplayer_game_player::Entity::insert_many(entities).exec(self.orm.as_ref()).await?;
        Ok(())
    }

    pub async fn player_claim(&self, player_id: i32, is_claimed: bool) -> Result<(), sea_orm::DbErr> {
        crate::schema::multiplayer_game_player::Entity::update(crate::schema::multiplayer_game_player::ActiveModel {
            id: sea_orm::ActiveValue::Set(player_id),
            is_claimed: sea_orm::Set(is_claimed),
            ..Default::default()
        })
        .exec(self.orm.as_ref())
        .await?;
        Ok(())
    }

    pub async fn game_event_at_time(&self, time: i64, variant: crate::schema::game_event::EventVariant) -> Result<Option<crate::schema::game_event::Model>, sea_orm::DbErr> {
        crate::schema::game_event::Entity::find()
            .filter(crate::schema::game_event::Column::Start.lte(time))
            .filter(crate::schema::game_event::Column::End.gte(time))
            .filter(crate::schema::game_event::Column::Variant.eq(variant))
            .order_by_desc(crate::schema::game_event::Column::Start)
            .one(self.orm.as_ref())
            .await
    }

    pub async fn insert_game_event(&self, entity: crate::schema::game_event::ActiveModel) -> Result<crate::schema::game_event::Model, sea_orm::DbErr> {
        entity.insert(self.orm.as_ref()).await
    }

    pub async fn score_by_player_id(&self, player_id: i32) -> Result<Option<crate::schema::multiplayer_game_score::Model>, sea_orm::DbErr> {
        crate::schema::multiplayer_game_score::Entity::find()
            .filter(crate::schema::multiplayer_game_score::Column::PlayerId.eq(player_id))
            .one(self.orm.as_ref())
            .await
    }

    pub async fn score_by_user_id_and_claimed_oldest(&self, user_id: i32, is_claimed: bool) -> Result<Option<crate::schema::multiplayer_game_score::Model>, sea_orm::DbErr> {
        crate::schema::multiplayer_game_player::Entity::find()
            .find_also_related(crate::schema::multiplayer_game_score::Entity)
            .filter(crate::schema::multiplayer_game_player::Column::UserId.eq(user_id))
            .filter(crate::schema::multiplayer_game_score::Column::IsClaimed.eq(is_claimed))
            .order_by_asc(crate::schema::multiplayer_game_player::Column::CreationTime)
            .one(self.orm.as_ref())
            .await
            .map(|opt| opt.and_then(|(_player, score)| score))
    }

    pub async fn insert_score(&self, entity: crate::schema::multiplayer_game_score::ActiveModel) -> Result<crate::schema::multiplayer_game_score::Model, sea_orm::DbErr> {
        entity.insert(self.orm.as_ref()).await
    }

    pub async fn update_score(&self, entity: crate::schema::multiplayer_game_score::ActiveModel) -> Result<crate::schema::multiplayer_game_score::Model, sea_orm::DbErr> {
        crate::schema::multiplayer_game_score::Entity::update(entity).exec(self.orm.as_ref()).await
    }

    pub async fn score_claim(&self, score_id: i32) -> Result<(), sea_orm::DbErr> {
        crate::schema::multiplayer_game_score::Entity::update(crate::schema::multiplayer_game_score::ActiveModel {
            id: sea_orm::ActiveValue::Set(score_id),
            is_claimed: sea_orm::Set(true),
            ..Default::default()
        })
        .exec(self.orm.as_ref())
        .await?;
        Ok(())
    }

    pub async fn count_score_by_user_id_and_claimed(&self, user_id: i32, is_claimed: bool) -> Result<u64, sea_orm::DbErr> {
        crate::schema::multiplayer_game_player::Entity::find()
            .find_also_related(crate::schema::multiplayer_game_score::Entity)
            .filter(crate::schema::multiplayer_game_player::Column::UserId.eq(user_id))
            .filter(crate::schema::multiplayer_game_score::Column::IsClaimed.eq(is_claimed))
            .order_by_asc(crate::schema::multiplayer_game_player::Column::CreationTime)
            .count(self.orm.as_ref())
            .await
    }

    pub async fn count_friends_by_user_id(&self, user_id: i32, status_in: crate::schema::friend::FriendStatus) -> Result<u64, sea_orm::DbErr> {
        crate::schema::friend::Entity::find()
            .filter(crate::schema::friend::Column::FriendSource.eq(user_id))
            .filter(crate::schema::friend::Column::State.eq(status_in))
            //.order_by_asc(crate::schema::friend::Column::CreationTime)
            .count(self.orm.as_ref())
            .await
    }

    pub async fn count_friends_target_by_user_id(&self, user_id: i32, status_in: crate::schema::friend::FriendStatus) -> Result<u64, sea_orm::DbErr> {
        crate::schema::friend::Entity::find()
            .filter(crate::schema::friend::Column::FriendTarget.eq(user_id))
            .filter(crate::schema::friend::Column::State.eq(status_in))
            //.order_by_asc(crate::schema::friend::Column::CreationTime)
            .count(self.orm.as_ref())
            .await
    }

    pub async fn friends_by_user_id(&self, user_id: i32, status_not_in: impl IntoIterator<Item=crate::schema::friend::FriendStatus>) -> Result<Vec<(crate::schema::friend::Model, crate::schema::user::Model)>, sea_orm::DbErr> {
        Ok(crate::schema::friend::Entity::find()
            .find_also_related(crate::schema::user::Entity)
            .filter(crate::schema::friend::Column::FriendSource.eq(user_id))
            .filter(crate::schema::friend::Column::State.is_not_in(status_not_in))
            .order_by_asc(crate::schema::friend::Column::CreationTime)
            .all(self.orm.as_ref())
            .await?
            .into_iter()
            .filter_map(|(friend, user_opt)| user_opt.map(|user| (friend, user)))
            .collect())
    }

    pub async fn insert_friends(&self, entities: impl std::iter::IntoIterator<Item=crate::schema::friend::ActiveModel>) -> Result<(), sea_orm::DbErr> {
        crate::schema::friend::Entity::insert_many(entities).exec(self.orm.as_ref()).await?;
        Ok(())
    }

    pub async fn update_friends_state(&self, user_id_1: i32, user_id_2: i32, state: crate::schema::friend::FriendStatus) -> Result<(), sea_orm::DbErr> {
        crate::schema::friend::Entity::update_many()
            .filter(
                sea_orm::sea_query::Condition::any()
                    .add(
                        sea_orm::sea_query::Expr::expr(crate::schema::friend::Column::FriendSource.eq(user_id_1))
                            .and(crate::schema::friend::Column::FriendTarget.eq(user_id_2))
                    ).add(
                        sea_orm::sea_query::Expr::expr(crate::schema::friend::Column::FriendSource.eq(user_id_2))
                            .and(crate::schema::friend::Column::FriendTarget.eq(user_id_1))
                    )
            )
            .filter(crate::schema::friend::Column::State.is_not_in(crate::schema::friend::FINAL_STATUSES))
            .col_expr(crate::schema::friend::Column::State, sea_orm::sea_query::Expr::value(state))
            .exec(self.orm.as_ref())
            .await?;
        Ok(())
    }

    pub async fn clan_by_name(&self, name: String) -> Result<Option<crate::schema::clan::Model>, sea_orm::DbErr> {
        crate::schema::clan::Entity::find()
            .filter(
                sea_orm::sea_query::Func::lower(crate::schema::clan::Column::Name.into_expr())
                .eq(name.to_lowercase())
            )
            .one(self.orm.as_ref()).await
    }

    pub async fn clan_by_user_id(&self, user_id: i32) -> Result<Option<(crate::schema::clan::Model, crate::schema::clan_member::Model)>, sea_orm::DbErr> {
        Ok(crate::schema::clan::Entity::find()
            .find_also_related(crate::schema::clan_member::Entity)
            .filter(crate::schema::clan_member::Column::UserId.eq(user_id))
            .filter(crate::schema::clan_member::Column::Status.eq(crate::schema::clan_member::ClanMemberStatus::Confirmed))
            .one(self.orm.as_ref())
            .await?
            .and_then(|(clan, member)| member.map(|member| (clan, member))))
    }

    pub async fn clans_invited_to_for_user_id(&self, user_id: i32) -> Result<Vec<(crate::schema::clan::Model, crate::schema::clan_member::Model)>, sea_orm::DbErr> {
        Ok(crate::schema::clan::Entity::find()
            .find_also_related(crate::schema::clan_member::Entity)
            .filter(crate::schema::clan_member::Column::UserId.eq(user_id))
            .filter(crate::schema::clan_member::Column::Status.eq(crate::schema::clan_member::ClanMemberStatus::Invited))
            .all(self.orm.as_ref())
            .await?
            .into_iter()
            .filter_map(|(clan, member)| member.map(|member| (clan, member)))
            .collect())
    }

    pub async fn clan_invited_to_for_user_id_and_clan_id(&self, user_id: i32, clan_id: i32) -> Result<Option<(crate::schema::clan::Model, crate::schema::clan_member::Model)>, sea_orm::DbErr> {
        Ok(crate::schema::clan::Entity::find()
            .find_also_related(crate::schema::clan_member::Entity)
            .filter(crate::schema::clan_member::Column::UserId.eq(user_id))
            .filter(crate::schema::clan_member::Column::ClanId.eq(clan_id))
            .filter(crate::schema::clan_member::Column::Status.eq(crate::schema::clan_member::ClanMemberStatus::Invited))
            .one(self.orm.as_ref())
            .await?
            .and_then(|(clan, member)| member.map(|member| (clan, member))))
    }

    pub async fn clan_invited_to_for_user_id_and_clan_name(&self, user_id: i32, clan_name: String) -> Result<Option<(crate::schema::clan::Model, crate::schema::clan_member::Model)>, sea_orm::DbErr> {
        Ok(crate::schema::clan::Entity::find()
            .find_also_related(crate::schema::clan_member::Entity)
            .filter(crate::schema::clan_member::Column::UserId.eq(user_id))
            .filter(
                sea_orm::sea_query::Func::lower(crate::schema::clan::Column::Name.into_expr())
                .eq(clan_name.to_lowercase())
            )
            .filter(crate::schema::clan_member::Column::Status.eq(crate::schema::clan_member::ClanMemberStatus::Invited))
            .one(self.orm.as_ref())
            .await?
            .and_then(|(clan, member)| member.map(|member| (clan, member))))
    }

    pub async fn clans_by_search(&self, s: String, start: u64, _end: u64, types: impl std::iter::Iterator<Item=crate::schema::clan::ClanType>) -> Result<Vec<crate::schema::clan::Model>, sea_orm::DbErr> {
        let lower_s_like = format!("%{}%", s.trim_matches('%').to_lowercase());
        let types: Vec<_> = types.collect();
        crate::schema::clan::Entity::find()
            .filter(
                sea_orm::sea_query::Expr::expr(
                    sea_orm::sea_query::Func::lower(crate::schema::clan::Column::Name.into_expr())
                ).like(&lower_s_like)
            )
            .filter(if types.is_empty() {
                crate::schema::clan::Column::Variant.is_in([
                    crate::schema::clan::ClanType::Public,
                    crate::schema::clan::ClanType::Private,
                ])
            } else {
                crate::schema::clan::Column::Variant.is_in(types)
            })
            // FIXME don't return clans that are not strictly in the start..end range
            .paginate(self.orm.as_ref(), 50)
            .fetch_page(start / 50)
            .await
    }

    pub async fn insert_clan(&self, entity: crate::schema::clan::ActiveModel) -> Result<crate::schema::clan::Model, sea_orm::DbErr> {
        #[cfg(debug_assertions)]
        assert!(matches!(entity.id, sea_orm::ActiveValue::NotSet));
        entity.insert(self.orm.as_ref()).await
    }

    pub async fn update_clan(&self, entity: crate::schema::clan::ActiveModel) -> Result<crate::schema::clan::Model, sea_orm::DbErr> {
        entity.update(self.orm.as_ref()).await
    }

    pub async fn clan_members_by_clan_id(&self, clan_id: i32) -> Result<Vec<(crate::schema::clan_member::Model, crate::schema::user::Model)>, sea_orm::DbErr> {
        Ok(crate::schema::clan_member::Entity::find()
            .find_also_related(crate::schema::user::Entity)
            .filter(crate::schema::clan_member::Column::ClanId.eq(clan_id))
            .filter(crate::schema::clan_member::Column::Status.is_in([
                crate::schema::clan_member::ClanMemberStatus::Invited,
                crate::schema::clan_member::ClanMemberStatus::Confirmed,
            ]))
            .all(self.orm.as_ref())
            .await?
            .into_iter()
            .filter_map(|(member, user)| user.map(|user| (member, user)))
            .collect()
        )
    }

    pub async fn clan_leaders_by_clan_ids(&self, clan_ids: impl std::iter::Iterator<Item=i32>) -> Result<Vec<(crate::schema::clan_member::Model, crate::schema::user::Model)>, sea_orm::DbErr> {
        Ok(crate::schema::clan_member::Entity::find()
            .find_also_related(crate::schema::user::Entity)
            .filter(crate::schema::clan_member::Column::ClanId.is_in(clan_ids))
            .filter(crate::schema::clan_member::Column::Status.eq(crate::schema::clan_member::ClanMemberStatus::Confirmed))
            .filter(crate::schema::clan_member::Column::Rank.eq(crate::schema::clan_member::ClanMemberRank::Leader))
            .all(self.orm.as_ref())
            .await?
            .into_iter()
            .filter_map(|(member, user)| user.map(|user| (member, user)))
            .collect()
        )
    }

    pub async fn insert_clan_member(&self, entity: crate::schema::clan_member::ActiveModel) -> Result<crate::schema::clan_member::Model, sea_orm::DbErr> {
        #[cfg(debug_assertions)]
        assert!(matches!(entity.id, sea_orm::ActiveValue::NotSet));
        entity.insert(self.orm.as_ref()).await
    }

    pub async fn update_clan_member(&self, entity: crate::schema::clan_member::ActiveModel) -> Result<crate::schema::clan_member::Model, sea_orm::DbErr> {
        entity.update(self.orm.as_ref()).await
    }

    pub async fn update_clan_member_decline_all_invites(&self, user_id: i32) -> Result<(), sea_orm::DbErr> {
        crate::schema::clan_member::Entity::update_many()
            .filter(crate::schema::clan_member::Column::UserId.eq(user_id))
            .filter(crate::schema::clan_member::Column::Status.eq(crate::schema::clan_member::ClanMemberStatus::Invited))
            .col_expr(crate::schema::clan_member::Column::Status, sea_orm::sea_query::Expr::value(crate::schema::clan_member::ClanMemberStatus::Deactivated))
            .exec(self.orm.as_ref())
            .await?;
        Ok(())
    }

    pub async fn count_clan_members_by_clan_id(&self, clan_id: i32) -> Result<u64, sea_orm::DbErr> {
        crate::schema::clan_member::Entity::find()
            .find_also_related(crate::schema::user::Entity)
            .filter(crate::schema::clan_member::Column::ClanId.eq(clan_id))
            .filter(crate::schema::clan_member::Column::Status.eq(crate::schema::clan_member::ClanMemberStatus::Confirmed))
            .count(self.orm.as_ref())
            .await
    }

    pub async fn federation_by_id(&self, id: i32) -> Result<Option<crate::schema::federation::Model>, sea_orm::DbErr> {
        crate::schema::federation::Entity::find_by_id(id)
            .one(self.orm.as_ref())
            .await
    }

    pub async fn federation_by_domain(&self, domain: &str) -> Result<Option<crate::schema::federation::Model>, sea_orm::DbErr> {
        crate::schema::federation::Entity::find()
            .filter(crate::schema::federation::Column::Domain.eq(domain))
            .one(self.orm.as_ref())
            .await
    }

    pub async fn federations_all(&self) -> Result<Vec<crate::schema::federation::Model>, sea_orm::DbErr> {
        crate::schema::federation::Entity::find()
            .order_by_asc(crate::schema::federation::Column::Id)
            .all(self.orm.as_ref())
            .await
    }

    pub async fn insert_federation(&self, entity: crate::schema::federation::ActiveModel) -> Result<crate::schema::federation::Model, sea_orm::DbErr> {
        #[cfg(debug_assertions)]
        assert!(matches!(entity.id, sea_orm::ActiveValue::NotSet));
        entity.insert(self.orm.as_ref()).await
    }

    pub async fn update_federation(&self, entity: crate::schema::federation::ActiveModel) -> Result<crate::schema::federation::Model, sea_orm::DbErr> {
        crate::schema::federation::Entity::update(entity)
            .exec(self.orm.as_ref())
            .await
    }

    pub async fn metrics(&self) -> super::DatabaseMetrics {
        self.metrics.lock().unwrap().snapshot()
    }
}
