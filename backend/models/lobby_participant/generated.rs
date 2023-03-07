/* This file is generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};
use crate::models::lobby::Lobby;
use crate::models::users::User;

type Connection = create_rust_app::Connection;

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Identifiable, Associations, Selectable)]
#[diesel(table_name=lobby_participant, primary_key(id), belongs_to(Lobby, foreign_key=lobby_id) , belongs_to(User, foreign_key=user_id))]
pub struct LobbyParticipant {
    pub id: i32,
    pub user_id: i32,
    pub lobby_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=lobby_participant)]
pub struct CreateLobbyParticipant {
    pub user_id: i32,
    pub lobby_id: String,
}

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=lobby_participant)]
pub struct UpdateLobbyParticipant {
    pub user_id: Option<i32>,
    pub lobby_id: Option<String>,
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

impl LobbyParticipant {

    pub fn create(db: &mut Connection, item: &CreateLobbyParticipant) -> QueryResult<Self> {
        use crate::schema::lobby_participant::dsl::*;

        insert_into(lobby_participant).values(item).get_result::<Self>(db)
    }

    pub fn read(db: &mut Connection, param_id: i32) -> QueryResult<Self> {
        use crate::schema::lobby_participant::dsl::*;

        lobby_participant.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut Connection, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::lobby_participant::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = lobby_participant.count().get_result(db)?;
        let items = lobby_participant.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    pub fn update(db: &mut Connection, param_id: i32, item: &UpdateLobbyParticipant) -> QueryResult<Self> {
        use crate::schema::lobby_participant::dsl::*;

        diesel::update(lobby_participant.filter(id.eq(param_id))).set(item).get_result(db)
    }

    pub fn delete(db: &mut Connection, param_id: i32) -> QueryResult<usize> {
        use crate::schema::lobby_participant::dsl::*;

        diesel::delete(lobby_participant.filter(id.eq(param_id))).execute(db)
    }

}