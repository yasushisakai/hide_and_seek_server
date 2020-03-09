use crate::api_error::ApiError;
use crate::db;
use crate::schema::fence;
use chrono::prelude::*;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "fence"]
pub struct FenceMessage {
    pub ts: Vec<i32>,
    pub lat: Vec<f32>,
    pub lng: Vec<f32>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "fence"]
pub struct Fence {
    pub id: Uuid,
    pub ts: Vec<NaiveDateTime>,
    pub lat: Vec<f32>,
    pub lng: Vec<f32>,
    pub tcount: i32,
    pub fcount: i32,
}

impl Fence {
    pub fn find_all() -> Result<Vec<Uuid>, ApiError> {
        let conn = db::connection()?;

        let fences = fence::table.load::<Fence>(&conn)?;

        let fids = fences.iter().map(|f| f.id).collect();

        Ok(fids)
    }

    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let fence = fence::table.filter(fence::id.eq(id)).first(&conn)?;

        Ok(fence)
    }

    pub fn create(fence: FenceMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let fence = Fence::from(fence);
        let fence = diesel::insert_into(fence::table)
            .values(fence)
            .get_result(&conn)?;

        Ok(fence)
    }

    pub fn update(id: Uuid, fence: Fence) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        // let fence = Fence::from(fence);

        let fence = diesel::update(fence::table)
            .filter(fence::id.eq(id))
            .set(fence)
            .get_result(&conn)?;

        Ok(fence)
    }

    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(fence::table.filter(fence::id.eq(id))).execute(&conn)?;

        Ok(res)
    }

    pub fn seek(id: Uuid) -> Result<bool, ApiError> {
        let conn = db::connection()?;

        let fence: Fence = fence::table.filter(fence::id.eq(id)).first(&conn)?;

        // let result = fence.lat[0] == 1.0;

        Ok(true)
    }
}

impl From<FenceMessage> for Fence {
    fn from(fence: FenceMessage) -> Self {
        let new_ts: Vec<NaiveDateTime> = fence
            .ts
            .iter()
            .map(|t| Utc.timestamp(*t as i64, 0_u32).naive_utc())
            .collect();

        Fence {
            id: Uuid::new_v4(),
            ts: new_ts,
            lat: fence.lat,
            lng: fence.lng,
            tcount: 0,
            fcount: 0
        }
    }
}

impl From<Fence> for FenceMessage {
    fn from(fence: Fence) -> Self {
        let new_ts: Vec<i32> = fence.ts.iter().map(|t| t.timestamp() as i32).collect();

        FenceMessage {
            ts: new_ts,
            lat: fence.lat,
            lng: fence.lng,
        }
    }
}
