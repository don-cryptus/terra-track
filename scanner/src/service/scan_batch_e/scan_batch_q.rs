use super::scan_batch_m;
use crate::{
    scanner::ip_iterator::Ipv4Iter,
    utils::{convert_i32_to_ipv4_string, convert_ipv4_string_to_i32},
    BATCH_SIZE,
};
use ::entity::scan_batch;
use migration::db::get_db_connection;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};

pub struct Query;

impl Query {
    pub async fn find_scan_batch_by_id(id: i64) -> anyhow::Result<Option<scan_batch::Model>> {
        let db = get_db_connection().await?;
        let model = scan_batch::Entity::find_by_id(id).one(&db).await?;

        Ok(model)
    }

    pub async fn find_last_scan_batch() -> anyhow::Result<Option<scan_batch::Model>> {
        let db = get_db_connection().await?;
        let model = scan_batch::Entity::find()
            .order_by_desc(scan_batch::Column::Id)
            .one(&db)
            .await?;

        Ok(model)
    }

    pub async fn find_open_scan_batch() -> anyhow::Result<Vec<scan_batch::Model>> {
        let db = get_db_connection().await?;
        let models = scan_batch::Entity::find()
            .filter(scan_batch::Column::End.is_null())
            .order_by_asc(scan_batch::Column::UpdatedAt)
            .all(&db)
            .await?;

        Ok(models)
    }

    pub async fn next_scan_batch() -> anyhow::Result<scan_batch::Model> {
        let scans = Self::find_open_scan_batch().await?;
        let date = chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap());

        #[allow(unused_assignments)]
        let mut scan: Option<scan_batch::Model> = None;
        if scans.len() == 0 {
            scan = Self::find_last_scan_batch().await?;
            if scan.is_none() {
                let ip_iter = Ipv4Iter::new("0.0.0.0").next().unwrap();
                scan = Some(
                    scan_batch_m::Mutation::create_scan_batch(scan_batch::ActiveModel {
                        ip: Set(ip_iter.to_string()),
                        cursor: Set(convert_ipv4_string_to_i32(ip_iter.to_string().as_str())),
                        start: Set(date),
                        batch_size: Set(BATCH_SIZE),
                        ..Default::default()
                    })
                    .await?,
                );
            } else {
                let ip_iter = Ipv4Iter::batched(
                    &convert_i32_to_ipv4_string(scan.as_ref().unwrap().cursor),
                    scan.as_ref().unwrap().batch_size,
                )
                .skip_batch(scan.as_ref().unwrap().batch_size);

                scan = Some(
                    scan_batch_m::Mutation::create_scan_batch(scan_batch::ActiveModel {
                        ip: Set(ip_iter.to_string()),
                        cursor: Set(convert_ipv4_string_to_i32(ip_iter.to_string().as_str())),
                        start: Set(date),
                        batch_size: Set(BATCH_SIZE),
                        ..Default::default()
                    })
                    .await?,
                );
            }
        } else {
            scan = Some(scans[0].clone());
            scan_batch_m::Mutation::update_scan_batch(scan_batch::ActiveModel {
                id: Set(scan.as_ref().unwrap().id),
                updated_at: Set(Some(date)),
                ..Default::default()
            })
            .await?;
        }

        Ok(scan.unwrap())
    }
}
