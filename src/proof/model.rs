use crate::api_error::ApiError;
use crate::db;
use crate::schema::proof;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "proof"]
pub struct Proof {
    pub id: String,
    pub fid: Uuid,
    pub result: bool
}

impl Proof {

    pub fn new(id: String, fid:Uuid, result: bool) -> Self {
        Proof{
            id, fid, result
        }
    }

    pub fn find_all() -> Result<Vec<String>, ApiError> {
        let conn = db::connection()?;

        let proofs = proof::table.load::<Proof>(&conn)?;
        let pids = proofs.iter().map(|p| format!("{}", p.id)).collect();

        Ok(pids)
    }

    pub fn find(id: String) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let proof = proof::table.filter(proof::id.eq(id)).first(&conn)?;

        Ok(proof)
    }

    pub fn create(proof: Proof) -> Result<Self,ApiError> {
        let conn = db::connection()?;

        let proof = diesel::insert_into(proof::table)
            .values(proof)
            .get_result(&conn)?;

        Ok(proof)
    }


    pub fn delete(id: String) -> Result<usize, ApiError> {
        let conn = db::connection()?;
        let res = diesel::delete(proof::table.filter(proof::id.eq(id)))
            .execute(&conn)?;

        Ok(res)
    }

}
