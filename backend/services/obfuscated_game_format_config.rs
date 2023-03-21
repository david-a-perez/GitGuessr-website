use crate::models::obfuscated_game_format_config::ObfuscatedGameFormatConfig;
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get,
    web::{Data, Path, Query},
    HttpResponse, Responder,
};
use create_rust_app::Database;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use serde::Deserialize;

#[derive(Deserialize)]
struct Filter {
    repository_id: String,
}

#[get("")]
async fn index(db: Data<Database>, Query(filter): Query<Filter>) -> impl Responder {
    actix_web::web::block(move || {
        let mut conn = db.get_connection();

        {
            use crate::schema::obfuscated_game_format_config::dsl::*;

            obfuscated_game_format_config
                .filter(repository_id.eq(filter.repository_id))
                .first::<ObfuscatedGameFormatConfig>(&mut conn)
                .optional()
        }
    })
    .await
    .map(|result| match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(err) => Err(ErrorInternalServerError(err)),
    })
}

#[get("/{id}")]
async fn read(db: Data<Database>, item_id: Path<i32>) -> impl Responder {
    actix_web::web::block(move || {
        let mut conn = db.get_connection();

        ObfuscatedGameFormatConfig::read(&mut conn, item_id.into_inner())
    })
    .await
    .map(|result| match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(err) => Err(ErrorNotFound(err)),
    })
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    scope.service(index).service(read)
}
