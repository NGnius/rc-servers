use sea_orm::{DbBackend, Statement};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250816_000001_add_fake_players"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Make user_id column nullable/optional
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        match manager.get_database_backend() {
            DbBackend::Sqlite => {
                // cannot modify existing column, let's just drop it since this is supposed to be only for dev work
                manager.drop_table(
                    Table::drop()
                        .table(crate::schema::multiplayer_game_player::Entity)
                        .to_owned()
                ).await?;
                manager
                    .create_table(
                        Table::create()
                            .table(crate::schema::multiplayer_game_player::Entity)
                            .col(
                                ColumnDef::new(crate::schema::multiplayer_game_player::Column::Id)
                                    .integer()
                                    .not_null()
                                    .auto_increment()
                                    .primary_key(),
                            )
                            .col(ColumnDef::new(crate::schema::multiplayer_game_player::Column::UserId).integer().null())
                            .foreign_key(
                                ForeignKey::create()
                                    .name("fk-players-user_id")
                                    .from(crate::schema::multiplayer_game_player::Entity, crate::schema::multiplayer_game_player::Column::UserId)
                                    .to(crate::schema::user::Entity, crate::schema::user::Column::Id),
                            )
                            .col(ColumnDef::new(crate::schema::multiplayer_game_player::Column::GameId).integer().not_null())
                            .foreign_key(
                                ForeignKey::create()
                                    .name("fk-players-game_id")
                                    .from(crate::schema::multiplayer_game_player::Entity, crate::schema::multiplayer_game_player::Column::GameId)
                                    .to(crate::schema::multiplayer_game::Entity, crate::schema::multiplayer_game::Column::Id),
                            )
                            .col(ColumnDef::new(crate::schema::multiplayer_game_player::Column::CreationTime).big_integer().not_null())
                            .col(ColumnDef::new(crate::schema::multiplayer_game_player::Column::PlayerId).small_integer().not_null())
                            .col(ColumnDef::new(crate::schema::multiplayer_game_player::Column::Team).integer().not_null())
                            .col(ColumnDef::new(crate::schema::multiplayer_game_player::Column::Group).integer().null())
                            .col(ColumnDef::new(crate::schema::multiplayer_game_player::Column::IsClaimed).boolean().not_null())
                            .col(ColumnDef::new(crate::schema::multiplayer_game_player::Column::PublicId).string().not_null())
                            .col(ColumnDef::new(crate::schema::multiplayer_game_player::Column::DisplayName).string().not_null())
                            .to_owned(),
                    )
                    .await
            },
            _ => {
                manager
                    .alter_table(
                        Table::alter()
                            .table(crate::schema::multiplayer_game_player::Entity)
                            .modify_column(ColumnDef::new(crate::schema::multiplayer_game_player::Column::UserId).integer().null())
                            .add_column(ColumnDef::new(crate::schema::multiplayer_game_player::Column::PublicId).string().not_null().default("???"))
                            .add_column(ColumnDef::new(crate::schema::multiplayer_game_player::Column::DisplayName).string().not_null().default("???"))
                            .to_owned()
                    )
                    .await
            }
        }

    }

    // Define how to rollback this migration: Makes the colum not nullable.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // this will always fail in sqlite, but why are you rolling back migrations anyway?
        match manager.get_database_backend() {
            DbBackend::Sqlite => {
                // cannot modify existing column, just do it in raw sqlite for simplicity
                let statements = [
                    r#"CREATE TEMPORARY TABLE temp AS
                            SELECT
                            id,
                            user_id,
                            game_id,
                            creation_time,
                            player_id,
                            team,
                            "group",
                            is_claimed
                        FROM players;"#,

                        r#"DROP TABLE players;"#,

                        r#"CREATE TABLE players (
                            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                            user_id INTEGER NOT NULL,
                            game_id INTEGER NOT NULL,
                            creation_time BIGINT NOT NULL,
                            player_id SMALLINT NOT NULL,
                            team INTEGER NOT NULL,
                            "group" INTEGER,
                            is_claimed BOOLEAN NOT NULL,
                            CONSTRAINT FK_players_games FOREIGN KEY (game_id) REFERENCES games(id),
                            CONSTRAINT FK_players_users FOREIGN KEY (user_id) REFERENCES users(id)
                        );"#,

                        r#"INSERT INTO players
                            (id,
                            user_id,
                            game_id,
                            creation_time,
                            player_id,
                            team,
                            "group",
                            is_claimed)
                            SELECT
                            id,
                            user_id,
                            game_id,
                            creation_time,
                            player_id,
                            team,
                            "group",
                            is_claimed
                        FROM temp
                        WHERE user_id IS NOT NULL;"#,

                        r#"DROP TABLE temp;"#,
                ];
                for sql in statements {
                    let statement = Statement::from_sql_and_values(DbBackend::Sqlite, sql, []);
                    manager.get_connection().execute(statement).await?;
                }
                Ok(())
            }
            _ => {
                manager
                    .alter_table(
                        Table::alter()
                            .table(crate::schema::multiplayer_game_player::Entity)
                            .modify_column(ColumnDef::new(crate::schema::multiplayer_game_player::Column::UserId).integer().not_null())
                            .drop_column(crate::schema::multiplayer_game_player::Column::PublicId)
                            .drop_column(crate::schema::multiplayer_game_player::Column::DisplayName)
                            .to_owned()
                    )
                    .await
            }
        }

    }
}
