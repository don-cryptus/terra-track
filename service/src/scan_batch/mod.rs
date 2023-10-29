use crate::db::get_db_connection;
use ::entity::scan_batch;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TryIntoModel};

pub struct Mutation;
pub struct Query;

impl Mutation {
    pub async fn create_scan_batch(
        active_model: scan_batch::ActiveModel,
    ) -> anyhow::Result<scan_batch::Model> {
        let db = get_db_connection().await?;
        let model = active_model.save(&db).await?.try_into_model()?;

        Ok(model)
    }

    pub async fn update_scan_batch(
        id: i32,
        model: scan_batch::Model,
    ) -> anyhow::Result<scan_batch::Model> {
        let db = get_db_connection().await?;
        let model = scan_batch::ActiveModel {
            id: Set(id),
            ..model.into()
        }
        .save(&db)
        .await?
        .try_into_model()?;

        Ok(model)
    }

    pub async fn delete_scan_batch(id: i32) -> anyhow::Result<bool> {
        let db = get_db_connection().await?;
        scan_batch::ActiveModel {
            id: Set(id),
            ..Default::default()
        }
        .delete(&db)
        .await?;

        Ok(true)
    }
}

impl Query {
    pub async fn find_scan_batch_by_id(id: i32) -> anyhow::Result<Option<scan_batch::Model>> {
        let db = get_db_connection().await?;
        let model = scan_batch::Entity::find_by_id(id).one(&db).await?;

        Ok(model)
    }
    pub async fn find_open_scans() -> anyhow::Result<Vec<scan_batch::Model>> {
        let db = get_db_connection().await?;
        let models = scan_batch::Entity::find()
            .filter(scan_batch::Column::End.is_null())
            .all(&db)
            .await?;

        Ok(models)
    }
}
