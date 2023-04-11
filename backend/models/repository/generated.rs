/* This file is generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};


type Connection = create_rust_app::Connection;

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Selectable)]
#[diesel(table_name=repository, primary_key(name))]
pub struct Repository {
    pub name: String,
    pub filename: String,
    pub url: String,
    pub description: String,
}

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=repository)]
pub struct CreateRepository {
    pub name: String,
    pub filename: String,
    pub url: String,
    pub description: String,
}

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=repository)]
pub struct UpdateRepository {
    pub filename: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
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

impl Repository {

    pub fn create(db: &mut Connection, item: &CreateRepository) -> QueryResult<Self> {
        use crate::schema::repository::dsl::*;

        insert_into(repository).values(item).get_result::<Self>(db)
    }

    pub fn read(db: &mut Connection, param_name: String) -> QueryResult<Self> {
        use crate::schema::repository::dsl::*;

        repository.filter(name.eq(param_name)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut Connection, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::repository::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = repository.count().get_result(db)?;
        let items = repository.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    pub fn update(db: &mut Connection, param_name: String, item: &UpdateRepository) -> QueryResult<Self> {
        use crate::schema::repository::dsl::*;

        diesel::update(repository.filter(name.eq(param_name))).set(item).get_result(db)
    }

    pub fn delete(db: &mut Connection, param_name: String) -> QueryResult<usize> {
        use crate::schema::repository::dsl::*;

        diesel::delete(repository.filter(name.eq(param_name))).execute(db)
    }

}