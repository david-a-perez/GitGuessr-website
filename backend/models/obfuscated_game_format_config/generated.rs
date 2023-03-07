/* This file is generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};
use crate::models::repository::Repository;

type Connection = create_rust_app::Connection;

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Identifiable, Associations, Selectable)]
#[diesel(table_name=obfuscated_game_format_config, primary_key(repository_id), belongs_to(Repository, foreign_key=repository_id))]
pub struct ObfuscatedGameFormatConfig {
    pub repository_id: String,
    pub filenames: String,
}

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=obfuscated_game_format_config)]
pub struct CreateObfuscatedGameFormatConfig {
    pub repository_id: String,
    pub filenames: String,
}

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=obfuscated_game_format_config)]
pub struct UpdateObfuscatedGameFormatConfig {
    pub filenames: Option<String>,
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

impl ObfuscatedGameFormatConfig {

    pub fn create(db: &mut Connection, item: &CreateObfuscatedGameFormatConfig) -> QueryResult<Self> {
        use crate::schema::obfuscated_game_format_config::dsl::*;

        insert_into(obfuscated_game_format_config).values(item).get_result::<Self>(db)
    }

    pub fn read(db: &mut Connection, param_repository_id: String) -> QueryResult<Self> {
        use crate::schema::obfuscated_game_format_config::dsl::*;

        obfuscated_game_format_config.filter(repository_id.eq(param_repository_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut Connection, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::obfuscated_game_format_config::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = obfuscated_game_format_config.count().get_result(db)?;
        let items = obfuscated_game_format_config.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    pub fn update(db: &mut Connection, param_repository_id: String, item: &UpdateObfuscatedGameFormatConfig) -> QueryResult<Self> {
        use crate::schema::obfuscated_game_format_config::dsl::*;

        diesel::update(obfuscated_game_format_config.filter(repository_id.eq(param_repository_id))).set(item).get_result(db)
    }

    pub fn delete(db: &mut Connection, param_repository_id: String) -> QueryResult<usize> {
        use crate::schema::obfuscated_game_format_config::dsl::*;

        diesel::delete(obfuscated_game_format_config.filter(repository_id.eq(param_repository_id))).execute(db)
    }

}