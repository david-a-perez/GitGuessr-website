use crate::models::lobby_participant::{CreateLobbyParticipant, LobbyParticipant};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get, post,
    web::{Data, Json, Path, Query},
    HttpResponse, Responder,
};
use create_rust_app::Database;
use gitguessr_auth::Auth;
use serde::Deserialize;

#[tsync::tsync]
#[derive(serde::Deserialize)]
pub struct PaginationParams {
    pub page: i64,
    pub page_size: i64,
}

#[tsync::tsync]
#[derive(Deserialize)]
struct LobbyParticipantFilters {
    lobby_id: Option<String>,
    user_id: Option<i32>,
}

#[get("")]
async fn index(
    db: Data<Database>,
    Query(PaginationParams { page, page_size }): Query<PaginationParams>,
    Query(filters): Query<LobbyParticipantFilters>,
) -> impl Responder {
    actix_web::web::block(move || {
        let mut conn = db.get_connection();

        {
            use crate::diesel::ExpressionMethods;
            use crate::diesel::QueryDsl;
            use crate::diesel::QueryResult;
            use crate::diesel::RunQueryDsl;
            use crate::models::lobby_participant::PaginationResult;
            use crate::schema::lobby_participant::dsl::*;

            let page_size = if page_size < 1 { 1 } else { page_size };

            let mut query = lobby_participant.into_boxed();
            let mut count_query = lobby_participant.into_boxed();

            if let Some(lobby) = filters.lobby_id {
                query = query.filter(lobby_id.eq(lobby.clone()));
                count_query = count_query.filter(lobby_id.eq(lobby));
            }

            if let Some(user) = filters.user_id {
                query = query.filter(user_id.eq(user));
                count_query = count_query.filter(user_id.eq(user));
            }

            let items = query
                .limit(page_size)
                .offset(page * page_size)
                .load::<LobbyParticipant>(&mut conn)?;

            let total_items = count_query.count().get_result(&mut conn)?;

            QueryResult::Ok(PaginationResult {
                items,
                total_items,
                page,
                page_size,
                /* ceiling division of integers */
                num_pages: total_items / page_size + i64::from(total_items % page_size != 0),
            })
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

        LobbyParticipant::read(&mut conn, item_id.into_inner())
    })
    .await
    .map(|result| match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(err) => Err(ErrorNotFound(err)),
    })
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(mut item): Json<CreateLobbyParticipant>,
    auth: Auth,
) -> impl Responder {
    actix_web::web::block(move || {
        let mut conn = db.get_connection();

        item.user_id = auth.user_id;

        LobbyParticipant::create(&mut conn, &item)
    })
    .await
    .map(|result| match result {
        Ok(result) => Ok(HttpResponse::Created().json(result)),
        Err(err) => Err(ErrorInternalServerError(err)),
    })
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    scope.service(index).service(read).service(create)
}
