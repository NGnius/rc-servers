use sea_orm_migration::MigratorTrait;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect, TransactionTrait};

pub struct Database {
    orm: sea_orm::DatabaseConnection,
}

impl Database {
    pub async fn init(uri: &str) -> Result<Self, sea_orm::DbErr>{
        let db = sea_orm::Database::connect(uri).await?;
        //let schema_manager = SchemaManager::new(&db);
        super::Migrator::up(&db, None).await?;
        Ok(Self {
            orm: db,
        })
    }

    pub async fn user_by_public_id(&self, public_id: String) -> Result<Option<crate::schema::user::Model>, sea_orm::DbErr> {
        crate::schema::user::Entity::find()
            .filter(crate::schema::user::Column::PublicId.eq(public_id))
            .one(&self.orm)
            .await
    }

    pub async fn insert_user(&self, entity: crate::schema::user::ActiveModel) -> Result<crate::schema::user::Model, sea_orm::DbErr> {
        entity.insert(&self.orm).await
    }

    pub async fn user_aux_by_user_id(&self, user_id: u32) -> Result<Vec<crate::schema::user_aux::Model>, sea_orm::DbErr> {
        crate::schema::user_aux::Entity::find()
            .filter(crate::schema::user_aux::Column::UserId.eq(user_id))
            .all(&self.orm)
            .await
    }

    pub async fn user_aux_by_user_id_and_descriptor(&self, user_id: u32, descriptor: crate::schema::user_aux::Descriptor) -> Result<Option<crate::schema::user_aux::Model>, sea_orm::DbErr> {
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

    pub async fn update_user_aux_by_user_id_and_descriptor(&self, mut entity: crate::schema::user_aux::ActiveModel, user_id: u32, descriptor: crate::schema::user_aux::Descriptor) -> Result<Option<crate::schema::user_aux::Model>, sea_orm::DbErr> {
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

    pub async fn perms_by_user_id(&self, user_id: u32) -> Result<Option<crate::schema::permissions::Model>, sea_orm::DbErr> {
        crate::schema::permissions::Entity::find()
            .filter(crate::schema::permissions::Column::UserId.eq(user_id))
            .one(&self.orm)
            .await
    }

    pub async fn insert_perms(&self, entity: crate::schema::permissions::ActiveModel) -> Result<crate::schema::permissions::Model, sea_orm::DbErr> {
        entity.insert(&self.orm).await
    }

    pub async fn garage_max_slot_by_user_id(&self, user_id: u32) -> Result<u32, sea_orm::DbErr> {
        let result = crate::schema::garage::Entity::find()
            .select_only()
            .column_as(crate::schema::garage::Column::Slot.max(), "column")
            .filter(crate::schema::garage::Column::UserId.eq(user_id))
            .into_model::<crate::schema::common_query::SingleColumn<u32>>()
            .one(&self.orm)
            .await?;
        Ok(result.map(|x| *x).unwrap_or(0))
    }

    pub async fn garage_selected(&self, user_id: u32) -> Result<Option<crate::schema::garage::Model>, sea_orm::DbErr> {
        crate::schema::garage::Entity::find()
            .filter(crate::schema::garage::Column::UserId.eq(user_id))
            .filter(crate::schema::garage::Column::Selected.eq(true))
            .one(&self.orm)
            .await
    }

    pub async fn garage_by_user_id_and_slot(&self, user_id: u32, garage_slot: u32) -> Result<Option<crate::schema::garage::Model>, sea_orm::DbErr> {
        crate::schema::garage::Entity::find()
            .filter(crate::schema::garage::Column::UserId.eq(user_id))
            .filter(crate::schema::garage::Column::Slot.eq(garage_slot))
            .one(&self.orm)
            .await
    }

    pub async fn garages_by_user_id(&self, user_id: u32) -> Result<Vec<crate::schema::garage::Model>, sea_orm::DbErr> {
        crate::schema::garage::Entity::find()
            .filter(crate::schema::garage::Column::UserId.eq(user_id))
            .order_by_asc(crate::schema::garage::Column::Slot)
            .all(&self.orm)
            .await
    }

    pub async fn insert_garages(&self, entities: Vec<crate::schema::garage::ActiveModel>) -> Result<(), sea_orm::DbErr> {
        crate::schema::garage::Entity::insert_many(entities.into_iter()).exec(&self.orm).await?;
        Ok(())
    }

    pub async fn insert_garage(&self, entity: crate::schema::garage::ActiveModel) -> Result<crate::schema::garage::Model, sea_orm::DbErr> {
        entity.insert(&self.orm).await
    }

    /*pub async fn update_garage(&self, entity: crate::schema::garage::ActiveModel, id: u32) -> Result<crate::schema::garage::Model, sea_orm::DbErr> {
        crate::schema::garage::Entity::update(entity)
            .filter(crate::schema::garage::Column::Id.eq(id))
            .exec(&self.orm)
            .await
    }*/

    pub async fn update_garage_by_user_id_and_slot(&self, mut entity: crate::schema::garage::ActiveModel, user_id: u32, slot: u32) -> Result<Option<crate::schema::garage::Model>, sea_orm::DbErr> {
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

    pub async fn update_garage_selected_by_user_id_and_slot(&self, user_id: u32, slot: u32) -> Result<(), sea_orm::DbErr> {
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
}
