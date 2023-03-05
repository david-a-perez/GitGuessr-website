use crate::models::correct_answer::{CorrectAnswer, CreateCorrectAnswer, UpdateCorrectAnswer};
use actix_web::{
    delete,
    error::ErrorNotFound,
    get, post, put,
    web::{Data, Json, Path, Query},
    Error, HttpResponse, Responder, Result,
};
use create_rust_app::Database;

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
        CorrectAnswer::paginate(&mut conn, info.page, info.page_size)
    })
    .await
    .map(|result| match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(err) => Err(ErrorNotFound(err)),
    })
}

#[get("/{id}")]
async fn read(db: Data<Database>, item_id: Path<i32>) -> impl Responder {
    actix_web::web::block(move || {
        let mut conn = db.get_connection();

        CorrectAnswer::read(&mut conn, item_id.into_inner())
    })
    .await
    .map(|result| match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(err) => Err(ErrorNotFound(err)),
    })
}

// #[post("")]
// async fn create(db: Data<Database>, Json(item): Json<CreateCorrectAnswer>) -> impl Responder {
//     actix_web::web::block(move || {
//         let mut conn = db.get_connection();

//         CorrectAnswer::create(&mut conn, &item)
//     })
//     .await
//     .map(|result| match result {
//         Ok(result) => Ok(HttpResponse::Ok().json(result)),
//         Err(err) => Err(ErrorNotFound(err)),
//     })
// }

// #[put("/{id}")]
// async fn update(
//     db: Data<Database>,
//     item_id: Path<i32>,
//     Json(item): Json<UpdateCorrectAnswer>,
// ) -> impl Responder {
//     actix_web::web::block(move || {
//         let mut conn = db.get_connection();

//         CorrectAnswer::update(&mut conn, item_id.into_inner(), &item)
//     })
//     .await
//     .map(|result| match result {
//         Ok(result) => Ok(HttpResponse::Ok().json(result)),
//         Err(err) => Err(ErrorNotFound(err)),
//     })
// }

// #[delete("/{id}")]
// async fn destroy(db: Data<Database>, item_id: Path<i32>) -> impl Responder {
//     actix_web::web::block(move || {
//         let mut conn = db.get_connection();

//         CorrectAnswer::delete(&mut conn, item_id.into_inner())
//     })
//     .await
//     .map(|result| match result {
//         Ok(result) => Ok(HttpResponse::Ok().json(result)),
//         Err(err) => Err(ErrorNotFound(err)),
//     })
// }

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    scope.service(index).service(read)
    // .service(create)
    // .service(update)
    // .service(destroy)
}
