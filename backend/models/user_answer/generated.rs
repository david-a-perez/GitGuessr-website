/* This file is generated and managed by dsync */

use crate::diesel::*;
use crate::models::question::Question;
use crate::models::users::User;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};

type Connection = create_rust_app::Connection;

#[tsync::tsync]
#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Insertable,
    AsChangeset,
    Identifiable,
    Associations,
    Selectable,
)]
#[diesel(table_name=user_answer, primary_key(user_id,question_id), belongs_to(Question, foreign_key=question_id) , belongs_to(User, foreign_key=user_id))]
pub struct UserAnswer {
    pub user_id: i32,
    pub question_id: i32,
    pub answer: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=user_answer)]
pub struct CreateUserAnswer {
    pub user_id: i32,
    pub question_id: i32,
    pub answer: String,
}

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=user_answer)]
pub struct UpdateUserAnswer {
    pub answer: Option<String>,
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

impl UserAnswer {
    pub fn create(db: &mut Connection, item: &CreateUserAnswer) -> QueryResult<Self> {
        use crate::schema::user_answer::dsl::*;

        insert_into(user_answer).values(item).get_result::<Self>(db)
    }

    pub fn read(
        db: &mut Connection,
        param_user_id: i32,
        param_question_id: i32,
    ) -> QueryResult<Self> {
        use crate::schema::user_answer::dsl::*;

        user_answer
            .filter(user_id.eq(param_user_id))
            .filter(question_id.eq(param_question_id))
            .first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(
        db: &mut Connection,
        page: i64,
        page_size: i64,
    ) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::user_answer::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = user_answer.count().get_result(db)?;
        let items = user_answer
            .limit(page_size)
            .offset(page * page_size)
            .load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0),
        })
    }

    pub fn update(
        db: &mut Connection,
        param_user_id: i32,
        param_question_id: i32,
        item: &UpdateUserAnswer,
    ) -> QueryResult<Self> {
        use crate::schema::user_answer::dsl::*;

        diesel::update(
            user_answer
                .filter(user_id.eq(param_user_id))
                .filter(question_id.eq(param_question_id)),
        )
        .set(item)
        .get_result(db)
    }

    pub fn delete(
        db: &mut Connection,
        param_user_id: i32,
        param_question_id: i32,
    ) -> QueryResult<usize> {
        use crate::schema::user_answer::dsl::*;

        diesel::delete(
            user_answer
                .filter(user_id.eq(param_user_id))
                .filter(question_id.eq(param_question_id)),
        )
        .execute(db)
    }
}
