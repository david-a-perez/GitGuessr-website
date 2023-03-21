/* This file is generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};
use crate::models::repository::Repository;

type Connection = create_rust_app::Connection;

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Identifiable, Associations, Selectable)]
#[diesel(table_name=git_guessr_game_format_config, primary_key(id), belongs_to(Repository, foreign_key=repository_id))]
pub struct GitGuessrGameFormatConfig {
    pub id: i32,
    pub repository_id: String,
    pub filenames: String,
    pub lines_shown: i32,
    pub allow_smaller_files: bool,
}

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=git_guessr_game_format_config)]
pub struct CreateGitGuessrGameFormatConfig {
    pub repository_id: String,
    pub filenames: String,
    pub lines_shown: i32,
    pub allow_smaller_files: bool,
}

#[tsync::tsync]
#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=git_guessr_game_format_config)]
pub struct UpdateGitGuessrGameFormatConfig {
    pub repository_id: Option<String>,
    pub filenames: Option<String>,
    pub lines_shown: Option<i32>,
    pub allow_smaller_files: Option<bool>,
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

impl GitGuessrGameFormatConfig {

    pub fn create(db: &mut Connection, item: &CreateGitGuessrGameFormatConfig) -> QueryResult<Self> {
        use crate::schema::git_guessr_game_format_config::dsl::*;

        insert_into(git_guessr_game_format_config).values(item).get_result::<Self>(db)
    }

    pub fn read(db: &mut Connection, param_id: i32) -> QueryResult<Self> {
        use crate::schema::git_guessr_game_format_config::dsl::*;

        git_guessr_game_format_config.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut Connection, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::git_guessr_game_format_config::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = git_guessr_game_format_config.count().get_result(db)?;
        let items = git_guessr_game_format_config.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    pub fn update(db: &mut Connection, param_id: i32, item: &UpdateGitGuessrGameFormatConfig) -> QueryResult<Self> {
        use crate::schema::git_guessr_game_format_config::dsl::*;

        diesel::update(git_guessr_game_format_config.filter(id.eq(param_id))).set(item).get_result(db)
    }

    pub fn delete(db: &mut Connection, param_id: i32) -> QueryResult<usize> {
        use crate::schema::git_guessr_game_format_config::dsl::*;

        diesel::delete(git_guessr_game_format_config.filter(id.eq(param_id))).execute(db)
    }

}