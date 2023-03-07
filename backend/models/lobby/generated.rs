/* This file is generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};
use crate::models::repository::Repository;

type Connection = create_rust_app::Connection;

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Identifiable, Associations, Selectable)]
#[diesel(table_name=lobby, primary_key(id), belongs_to(Repository, foreign_key=repository))]
pub struct Lobby {
    pub id: String,
    pub repository: String,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=lobby)]
pub struct CreateLobby {
    pub repository: String,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
}

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=lobby)]
pub struct UpdateLobby {
    pub repository: Option<String>,
    pub start_time: Option<Option<chrono::DateTime<chrono::Utc>>>,
    pub end_time: Option<Option<chrono::DateTime<chrono::Utc>>>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[tsync::tsync]
#[derive(Debug, Serialize)]
pub struct PaginationResult<T> {
    pub items: Vec<T>,
    pub total_items: i64,
    /// 0-based index
    pub page: i64,
    pub page_size: i64,
    pub num_pages: i64,
}

impl Lobby {

    pub fn create(db: &mut Connection, item: &CreateLobby) -> QueryResult<Self> {
        use crate::schema::lobby::dsl::*;

        insert_into(lobby).values(item).get_result::<Self>(db)
    }

    pub fn read(db: &mut Connection, param_id: String) -> QueryResult<Self> {
        use crate::schema::lobby::dsl::*;

        lobby.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut Connection, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::lobby::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = lobby.count().get_result(db)?;
        let items = lobby.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    pub fn update(db: &mut Connection, param_id: String, item: &UpdateLobby) -> QueryResult<Self> {
        use crate::schema::lobby::dsl::*;

        diesel::update(lobby.filter(id.eq(param_id))).set(item).get_result(db)
    }

    pub fn delete(db: &mut Connection, param_id: String) -> QueryResult<usize> {
        use crate::schema::lobby::dsl::*;

        diesel::delete(lobby.filter(id.eq(param_id))).execute(db)
    }

}