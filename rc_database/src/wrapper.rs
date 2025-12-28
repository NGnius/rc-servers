use sea_orm_migration::MigratorTrait;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, TransactionTrait};

pub struct Database {
    orm: sea_orm::DatabaseConnection,
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
            orm: db,
            metrics: metrics_data,
        })
    }

    pub async fn user_count(&self) -> Result<u64, sea_orm::DbErr> {
        crate::schema::user::Entity::find()
            .count(&self.orm)
            .await
    }

    pub async fn user_by_display_name(&self, public_id: String) -> Result<Option<crate::schema::user::Model>, sea_orm::DbErr> {
        crate::schema::user::Entity::find()
            .filter(sea_orm::sea_query::Expr::expr(
                sea_orm::sea_query::Func::lower(crate::schema::user::Column::DisplayName.into_expr())
                ).eq(public_id.to_lowercase())
            )
            .one(&self.orm)
            .await
    }

    pub async fn user_by_steam_id(&self, steam_id: u64) -> Result<Option<crate::schema::user::Model>, sea_orm::DbErr> {
        crate::schema::user::Entity::find()
            .filter(crate::schema::user::Column::SteamId.eq(Some(steam_id.to_string())))
            .one(&self.orm)
            .await
    }

    pub async fn user_by_email(&self, email: String) -> Result<Option<crate::schema::user::Model>, sea_orm::DbErr> {
        crate::schema::user::Entity::find()
            .filter(sea_orm::sea_query::Expr::expr(
                sea_orm::sea_query::Func::lower(crate::schema::user::Column::Email.into_expr())
                ).eq(email.to_lowercase())
            )
            .one(&self.orm)
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
        entity.insert(&self.orm).await
    }

    pub async fn user_aux_by_user_id(&self, user_id: i32) -> Result<Vec<crate::schema::user_aux::Model>, sea_orm::DbErr> {
        crate::schema::user_aux::Entity::find()
            .filter(crate::schema::user_aux::Column::UserId.eq(user_id))
            .all(&self.orm)
            .await
    }

    pub async fn user_aux_by_user_id_and_descriptor(&self, user_id: i32, descriptor: crate::schema::user_aux::Descriptor) -> Result<Option<crate::schema::user_aux::Model>, sea_orm::DbErr> {
        crate::schema::user_aux::Entity::find()
            .filter(crate::schema::user_aux::Column::UserId.eq(user_id))
            .filter(crate::schema::user_aux::Column::Descriptor.eq(descriptor))
            .one(&self.orm)
            .await
    }

    pub async fn insert_user_aux(&self, entities: Vec<crate::schema::user_aux::ActiveModel>) -> Result<(), sea_orm::DbErr> {
        crate::schema::user_aux::Entity::insert_many(entities.into_iter()).exec(&self.orm).await?;
        Ok(())
    }

    pub async fn update_user_aux_by_user_id_and_descriptor(&self, mut entity: crate::schema::user_aux::ActiveModel, user_id: i32, descriptor: crate::schema::user_aux::Descriptor) -> Result<Option<crate::schema::user_aux::Model>, sea_orm::DbErr> {
        let id_opt = crate::schema::user_aux::Entity::find()
            .select_only()
            .column(crate::schema::user_aux::Column::Id)
            .filter(crate::schema::user_aux::Column::UserId.eq(user_id))
            .filter(crate::schema::user_aux::Column::Descriptor.eq(descriptor.clone()))
            .into_model::<crate::schema::common_query::Id>()
            .one(&self.orm)
            .await?;
        if let Some(id) = id_opt {
            // update
            entity.id = sea_orm::ActiveValue::Set(id.id);
            Ok(Some(crate::schema::user_aux::Entity::update(entity)
                .exec(&self.orm)
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
            .one(&self.orm)
            .await
    }

    pub async fn insert_perms(&self, entity: crate::schema::permissions::ActiveModel) -> Result<crate::schema::permissions::Model, sea_orm::DbErr> {
        entity.insert(&self.orm).await
    }

    pub async fn update_perms_by_user_id(&self, mut entity: crate::schema::permissions::ActiveModel, user_id: i32) -> Result<Option<crate::schema::permissions::Model>, sea_orm::DbErr> {
        let id_opt = crate::schema::permissions::Entity::find()
            .select_only()
            .column(crate::schema::permissions::Column::Id)
            .filter(crate::schema::permissions::Column::UserId.eq(user_id))
            .into_model::<crate::schema::common_query::Id>()
            .one(&self.orm)
            .await?;
        if let Some(id) = id_opt {
            entity.id = sea_orm::ActiveValue::Set(id.id);
            Ok(Some(crate::schema::permissions::Entity::update(entity)
                .exec(&self.orm)
                .await?))
        } else {
            Ok(None)
        }
    }

    pub async fn garage_max_slot_by_user_id(&self, user_id: i32) -> Result<i32, sea_orm::DbErr> {
        let result = crate::schema::garage::Entity::find()
            .select_only()
            .column_as(crate::schema::garage::Column::Slot.max(), "column")
            .filter(crate::schema::garage::Column::UserId.eq(user_id))
            .into_model::<crate::schema::common_query::SingleColumn<i32>>()
            .one(&self.orm)
            .await?;
        Ok(result.map(|x| *x).unwrap_or(0))
    }

    pub async fn garage_selected(&self, user_id: i32) -> Result<Option<crate::schema::garage::Model>, sea_orm::DbErr> {
        crate::schema::garage::Entity::find()
            .filter(crate::schema::garage::Column::UserId.eq(user_id))
            .filter(crate::schema::garage::Column::Selected.eq(true))
            .one(&self.orm)
            .await
    }

    pub async fn garage_by_user_id_and_slot(&self, user_id: i32, garage_slot: i32) -> Result<Option<crate::schema::garage::Model>, sea_orm::DbErr> {
        crate::schema::garage::Entity::find()
            .filter(crate::schema::garage::Column::UserId.eq(user_id))
            .filter(crate::schema::garage::Column::Slot.eq(garage_slot))
            .one(&self.orm)
            .await
    }

    pub async fn garages_by_user_id(&self, user_id: i32) -> Result<Vec<crate::schema::garage::Model>, sea_orm::DbErr> {
        crate::schema::garage::Entity::find()
            .filter(crate::schema::garage::Column::UserId.eq(user_id))
            .order_by_asc(crate::schema::garage::Column::Slot)
            .all(&self.orm)
            .await
    }

    pub async fn garage_by_uuid(&self, uuid: i64) -> Result<Option<crate::schema::garage::Model>, sea_orm::DbErr> {
        crate::schema::garage::Entity::find()
            .filter(crate::schema::garage::Column::Uuid.eq(uuid))
            .one(&self.orm)
            .await
    }

    pub async fn garage_by_id(&self, id: i32) -> Result<Option<crate::schema::garage::Model>, sea_orm::DbErr> {
        crate::schema::garage::Entity::find_by_id(id)
            .one(&self.orm)
            .await
    }

    pub async fn insert_garages(&self, entities: Vec<crate::schema::garage::ActiveModel>) -> Result<(), sea_orm::DbErr> {
        crate::schema::garage::Entity::insert_many(entities.into_iter()).exec(&self.orm).await?;
        Ok(())
    }

    pub async fn insert_garage(&self, entity: crate::schema::garage::ActiveModel) -> Result<crate::schema::garage::Model, sea_orm::DbErr> {
        entity.insert(&self.orm).await
    }

    pub async fn update_garage(&self, entity: crate::schema::garage::ActiveModel) -> Result<crate::schema::garage::Model, sea_orm::DbErr> {
        crate::schema::garage::Entity::update(entity)
            .exec(&self.orm)
            .await
    }

    pub async fn update_garage_by_user_id_and_slot(&self, mut entity: crate::schema::garage::ActiveModel, user_id: i32, slot: i32) -> Result<Option<crate::schema::garage::Model>, sea_orm::DbErr> {
        let id_opt = crate::schema::garage::Entity::find()
            .select_only()
            .column(crate::schema::garage::Column::Id)
            .filter(crate::schema::garage::Column::UserId.eq(user_id))
            .filter(crate::schema::garage::Column::Slot.eq(slot))
            .into_model::<crate::schema::common_query::Id>()
            .one(&self.orm)
            .await?;
        if let Some(id) = id_opt {
            entity.id = sea_orm::ActiveValue::Set(id.id);
            Ok(Some(crate::schema::garage::Entity::update(entity)
                .exec(&self.orm)
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
            .one(&self.orm)
            .await?;
        if let Some(id) = id_opt {
            entity.id = sea_orm::ActiveValue::Set(id.id);
            Ok(Some(crate::schema::garage::Entity::update(entity)
                .exec(&self.orm)
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
            .count(&self.orm)
            .await
    }

    pub async fn sanctions_by_user_id(&self, user_id: i32) -> Result<Vec<crate::schema::sanction::Model>, sea_orm::DbErr> {
        crate::schema::sanction::Entity::find()
            .filter(crate::schema::sanction::Column::UserId.eq(user_id))
            .filter(crate::schema::sanction::Column::Acknowledged.is_null())
            .order_by_asc(crate::schema::sanction::Column::CreationTime)
            .all(&self.orm)
            .await
    }

    pub async fn insert_sanction(&self, entity: crate::schema::sanction::ActiveModel) -> Result<crate::schema::sanction::Model, sea_orm::DbErr> {
        entity.insert(&self.orm).await
    }

    pub async fn game_by_guid(&self, game_guid: i64) -> Result<Option<crate::schema::multiplayer_game::Model>, sea_orm::DbErr> {
        crate::schema::multiplayer_game::Entity::find()
            .filter(crate::schema::multiplayer_game::Column::Guid.eq(game_guid))
            .order_by_desc(crate::schema::multiplayer_game::Column::CreationTime)
            .one(&self.orm)
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
            .one(&self.orm)
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
            .one(&self.orm)
            .await?
            .and_then(|(game, player)| player.map(|player| (game, player))))
    }

    pub async fn update_complete_game_by_game_guid(&self, game_guid: i64) -> Result<(), sea_orm::DbErr> {
        crate::schema::multiplayer_game::Entity::update_many()
            .col_expr(crate::schema::multiplayer_game::Column::IsComplete, sea_orm::sea_query::Expr::value(true))
            .filter(crate::schema::multiplayer_game::Column::Guid.eq(game_guid))
            .exec(&self.orm)
            .await?;
        Ok(())
    }

    pub async fn complete_all_games(&self) -> Result<(), sea_orm::DbErr> {
        crate::schema::multiplayer_game::Entity::update_many()
            .col_expr(crate::schema::multiplayer_game::Column::IsComplete, sea_orm::sea_query::Expr::value(true))
            .filter(crate::schema::multiplayer_game::Column::IsComplete.eq(false))
            .exec(&self.orm)
            .await?;
        Ok(())
    }

    pub async fn insert_game(&self, entity: crate::schema::multiplayer_game::ActiveModel) -> Result<crate::schema::multiplayer_game::Model, sea_orm::DbErr> {
        entity.insert(&self.orm).await
    }

    pub async fn players_by_game_guid_and_completion(&self, game_guid: i64, is_complete: bool) -> Result<Vec<crate::schema::multiplayer_game_player::Model>, sea_orm::DbErr> {
        Ok(crate::schema::multiplayer_game_player::Entity::find()
            .find_also_related(crate::schema::multiplayer_game::Entity)
            .filter(crate::schema::multiplayer_game::Column::Guid.eq(game_guid))
            .filter(crate::schema::multiplayer_game::Column::IsComplete.eq(is_complete))
            //.into_model::<crate::schema::multiplayer_game_player::Model>()
            .all(&self.orm)
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
            .all(&self.orm)
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
            .all(&self.orm)
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
            .all(&self.orm)
            .await?
            .into_iter()
            .map(|(player, _)| player)
            .collect())
    }

    pub async fn players_by_game_id(&self, game_id: i32) -> Result<Vec<crate::schema::multiplayer_game_player::Model>, sea_orm::DbErr> {
        Ok(crate::schema::multiplayer_game_player::Entity::find()
            .find_also_related(crate::schema::multiplayer_game::Entity)
            .filter(crate::schema::multiplayer_game::Column::Id.eq(game_id))
            .all(&self.orm)
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
            .one(&self.orm)
            .await?
            .and_then(|(_, player)| player))
    }

    pub async fn insert_players(&self, entities: Vec<crate::schema::multiplayer_game_player::ActiveModel>) -> Result<(), sea_orm::DbErr> {
        crate::schema::multiplayer_game_player::Entity::insert_many(entities.into_iter()).exec(&self.orm).await?;
        Ok(())
    }

    pub async fn player_claim(&self, player_id: i32, is_claimed: bool) -> Result<(), sea_orm::DbErr> {
        crate::schema::multiplayer_game_player::Entity::update(crate::schema::multiplayer_game_player::ActiveModel {
            id: sea_orm::ActiveValue::Set(player_id),
            is_claimed: sea_orm::Set(is_claimed),
            ..Default::default()
        })
        .exec(&self.orm)
        .await?;
        Ok(())
    }

    pub async fn game_event_at_time(&self, time: i64, variant: crate::schema::game_event::EventVariant) -> Result<Option<crate::schema::game_event::Model>, sea_orm::DbErr> {
        crate::schema::game_event::Entity::find()
            .filter(crate::schema::game_event::Column::Start.lte(time))
            .filter(crate::schema::game_event::Column::End.gte(time))
            .filter(crate::schema::game_event::Column::Variant.eq(variant))
            .order_by_desc(crate::schema::game_event::Column::Start)
            .one(&self.orm)
            .await
    }

    pub async fn insert_game_event(&self, entity: crate::schema::game_event::ActiveModel) -> Result<crate::schema::game_event::Model, sea_orm::DbErr> {
        entity.insert(&self.orm).await
    }

    pub async fn score_by_player_id(&self, player_id: i32) -> Result<Option<crate::schema::multiplayer_game_score::Model>, sea_orm::DbErr> {
        crate::schema::multiplayer_game_score::Entity::find()
            .filter(crate::schema::multiplayer_game_score::Column::PlayerId.eq(player_id))
            .one(&self.orm)
            .await
    }

    pub async fn score_by_user_id_and_claimed_oldest(&self, user_id: i32, is_claimed: bool) -> Result<Option<crate::schema::multiplayer_game_score::Model>, sea_orm::DbErr> {
        crate::schema::multiplayer_game_player::Entity::find()
            .find_also_related(crate::schema::multiplayer_game_score::Entity)
            .filter(crate::schema::multiplayer_game_player::Column::UserId.eq(user_id))
            .filter(crate::schema::multiplayer_game_score::Column::IsClaimed.eq(is_claimed))
            .order_by_asc(crate::schema::multiplayer_game_player::Column::CreationTime)
            .one(&self.orm)
            .await
            .map(|opt| opt.and_then(|(_player, score)| score))
    }

    pub async fn insert_score(&self, entity: crate::schema::multiplayer_game_score::ActiveModel) -> Result<crate::schema::multiplayer_game_score::Model, sea_orm::DbErr> {
        entity.insert(&self.orm).await
    }

    pub async fn update_score(&self, entity: crate::schema::multiplayer_game_score::ActiveModel) -> Result<crate::schema::multiplayer_game_score::Model, sea_orm::DbErr> {
        crate::schema::multiplayer_game_score::Entity::update(entity).exec(&self.orm).await
    }

    pub async fn score_claim(&self, score_id: i32) -> Result<(), sea_orm::DbErr> {
        crate::schema::multiplayer_game_score::Entity::update(crate::schema::multiplayer_game_score::ActiveModel {
            id: sea_orm::ActiveValue::Set(score_id),
            is_claimed: sea_orm::Set(true),
            ..Default::default()
        })
        .exec(&self.orm)
        .await?;
        Ok(())
    }

    pub async fn count_score_by_user_id_and_claimed(&self, user_id: i32, is_claimed: bool) -> Result<u64, sea_orm::DbErr> {
        crate::schema::multiplayer_game_player::Entity::find()
            .find_also_related(crate::schema::multiplayer_game_score::Entity)
            .filter(crate::schema::multiplayer_game_player::Column::UserId.eq(user_id))
            .filter(crate::schema::multiplayer_game_score::Column::IsClaimed.eq(is_claimed))
            .order_by_asc(crate::schema::multiplayer_game_player::Column::CreationTime)
            .count(&self.orm)
            .await
    }

    pub async fn metrics(&self) -> super::DatabaseMetrics {
        self.metrics.lock().unwrap().snapshot()
    }
}
