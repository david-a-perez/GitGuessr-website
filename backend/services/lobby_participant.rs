use crate::models::lobby_participant::{CreateLobbyParticipant, LobbyParticipant};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get, post,
    web::{Data, Json, Path, Query},
    HttpResponse, Responder,
};
use create_rust_app::Database;
use gitguessr_auth::Auth;

#[tsync::tsync]
#[derive(serde::Deserialize)]
pub struct PaginationParams {
    pub page: i64,
    pub page_size: i64,
}

#[get("")]
async fn index(db: Data<Database>, Query(info): Query<PaginationParams>) -> impl Responder {
    actix_web::web::block(move || {
        let mut conn = db.get_connection();

        LobbyParticipant::paginate(&mut conn, info.page, info.page_size)
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

// #[put("/{id}")]
// async fn update(
//     db: Data<Database>,
//     item_id: Path<i32>,
//     Json(item): Json<UpdateLobbyParticipant>,
// ) -> impl Responder {
//     actix_web::web::block(move || {
//         let mut conn = db.get_connection();

//         LobbyParticipant::update(&mut conn, item_id.into_inner(), &item)
//     })
//     .await
//     .map(|result| match result {
//         Ok(result) => Ok(HttpResponse::Ok().json(result)),
//         Err(err) => Err(ErrorInternalServerError(err)),
//     })
// }

// #[delete("/{id}")]
// async fn destroy(db: Data<Database>, item_id: Path<i32>) -> impl Responder {
//     actix_web::web::block(move || {
//         let mut conn = db.get_connection();

//         LobbyParticipant::delete(&mut conn, item_id.into_inner())
//     })
//     .await
//     .map(|result| match result {
//         Ok(result) => Ok(HttpResponse::Ok().json(result)),
//         Err(err) => Err(ErrorInternalServerError(err)),
//     })
// }

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    scope.service(index).service(read).service(create)
    // .service(update)
    // .service(destroy)
}
