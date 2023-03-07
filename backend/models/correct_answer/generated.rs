/* This file is generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};
use crate::models::answer_choice::AnswerChoice;
use crate::models::lobby::Lobby;
use crate::models::question::Question;

type Connection = create_rust_app::Connection;

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Identifiable, Associations, Selectable)]
#[diesel(table_name=correct_answer, primary_key(id), belongs_to(AnswerChoice, foreign_key=answer_choice_id) , belongs_to(Lobby, foreign_key=lobby_id) , belongs_to(Question, foreign_key=question_id))]
pub struct CorrectAnswer {
    pub id: i32,
    pub answer_choice_id: i32,
    pub question_id: i32,
    pub lobby_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=correct_answer)]
pub struct CreateCorrectAnswer {
    pub answer_choice_id: i32,
    pub question_id: i32,
    pub lobby_id: String,
}

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=correct_answer)]
pub struct UpdateCorrectAnswer {
    pub answer_choice_id: Option<i32>,
    pub question_id: Option<i32>,
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

impl CorrectAnswer {

    pub fn create(db: &mut Connection, item: &CreateCorrectAnswer) -> QueryResult<Self> {
        use crate::schema::correct_answer::dsl::*;

        insert_into(correct_answer).values(item).get_result::<Self>(db)
    }

    pub fn read(db: &mut Connection, param_id: i32) -> QueryResult<Self> {
        use crate::schema::correct_answer::dsl::*;

        correct_answer.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut Connection, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::correct_answer::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = correct_answer.count().get_result(db)?;
        let items = correct_answer.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    pub fn update(db: &mut Connection, param_id: i32, item: &UpdateCorrectAnswer) -> QueryResult<Self> {
        use crate::schema::correct_answer::dsl::*;

        diesel::update(correct_answer.filter(id.eq(param_id))).set(item).get_result(db)
    }

    pub fn delete(db: &mut Connection, param_id: i32) -> QueryResult<usize> {
        use crate::schema::correct_answer::dsl::*;

        diesel::delete(correct_answer.filter(id.eq(param_id))).execute(db)
    }

}