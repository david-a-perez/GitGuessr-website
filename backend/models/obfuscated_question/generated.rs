/* This file is generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};
use crate::models::lobby::Lobby;

type Connection = create_rust_app::Connection;

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Identifiable, Associations, Selectable)]
#[diesel(table_name=obfuscated_question, primary_key(id), belongs_to(Lobby, foreign_key=lobby_id))]
pub struct ObfuscatedQuestion {
    pub id: i32,
    pub lobby_id: String,
    pub question_num: i32,
    pub question_text: String,
    pub big_answer_choices: bool,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=obfuscated_question)]
pub struct CreateObfuscatedQuestion {
    pub lobby_id: String,
    pub question_num: i32,
    pub question_text: String,
    pub big_answer_choices: bool,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
}

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=obfuscated_question)]
pub struct UpdateObfuscatedQuestion {
    pub lobby_id: Option<String>,
    pub question_num: Option<i32>,
    pub question_text: Option<String>,
    pub big_answer_choices: Option<bool>,
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

impl ObfuscatedQuestion {

    pub fn create(db: &mut Connection, item: &CreateObfuscatedQuestion) -> QueryResult<Self> {
        use crate::schema::obfuscated_question::dsl::*;

        insert_into(obfuscated_question).values(item).get_result::<Self>(db)
    }

    pub fn read(db: &mut Connection, param_id: i32) -> QueryResult<Self> {
        use crate::schema::obfuscated_question::dsl::*;

        obfuscated_question.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut Connection, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::obfuscated_question::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = obfuscated_question.count().get_result(db)?;
        let items = obfuscated_question.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    pub fn update(db: &mut Connection, param_id: i32, item: &UpdateObfuscatedQuestion) -> QueryResult<Self> {
        use crate::schema::obfuscated_question::dsl::*;

        diesel::update(obfuscated_question.filter(id.eq(param_id))).set(item).get_result(db)
    }

    pub fn delete(db: &mut Connection, param_id: i32) -> QueryResult<usize> {
        use crate::schema::obfuscated_question::dsl::*;

        diesel::delete(obfuscated_question.filter(id.eq(param_id))).execute(db)
    }

}