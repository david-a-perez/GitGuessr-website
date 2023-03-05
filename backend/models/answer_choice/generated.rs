/* This file is generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};
use crate::models::question::Question;

type Connection = create_rust_app::Connection;

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Identifiable, Associations, Selectable)]
#[diesel(table_name=answer_choice, primary_key(question_id,answer), belongs_to(Question, foreign_key=question_id))]
pub struct AnswerChoice {
    pub question_id: i32,
    pub answer: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable)]
#[diesel(table_name=answer_choice)]
pub struct CreateAnswerChoice {
    pub question_id: i32,
    pub answer: String,
}

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=answer_choice)]
pub struct UpdateAnswerChoice {
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

impl AnswerChoice {

    pub fn create(db: &mut Connection, item: &CreateAnswerChoice) -> QueryResult<Self> {
        use crate::schema::answer_choice::dsl::*;

        insert_into(answer_choice).values(item).get_result::<Self>(db)
    }

    pub fn read(db: &mut Connection, param_question_id: i32, param_answer: String) -> QueryResult<Self> {
        use crate::schema::answer_choice::dsl::*;

        answer_choice.filter(question_id.eq(param_question_id)).filter(answer.eq(param_answer)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut Connection, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::answer_choice::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = answer_choice.count().get_result(db)?;
        let items = answer_choice.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    pub fn update(db: &mut Connection, param_question_id: i32, param_answer: String, item: &UpdateAnswerChoice) -> QueryResult<Self> {
        use crate::schema::answer_choice::dsl::*;

        diesel::update(answer_choice.filter(question_id.eq(param_question_id)).filter(answer.eq(param_answer))).set(item).get_result(db)
    }

    pub fn delete(db: &mut Connection, param_question_id: i32, param_answer: String) -> QueryResult<usize> {
        use crate::schema::answer_choice::dsl::*;

        diesel::delete(answer_choice.filter(question_id.eq(param_question_id)).filter(answer.eq(param_answer))).execute(db)
    }

}