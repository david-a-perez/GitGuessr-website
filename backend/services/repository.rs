use crate::models::repository::Repository;
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get,
    web::{Data, Path, Query},
    HttpResponse, Responder,
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

        Repository::paginate(&mut conn, info.page, info.page_size)
    })
    .await
    .map(|result| match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(err) => Err(ErrorInternalServerError(err)),
    })
}

#[get("/{id}")]
async fn read(db: Data<Database>, item_id: Path<String>) -> impl Responder {
    actix_web::web::block(move || {
        let mut conn = db.get_connection();

        Repository::read(&mut conn, item_id.into_inner())
    })
    .await
    .map(|result| match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(err) => Err(ErrorNotFound(err)),
    })
}

// #[post("")]
// async fn create(db: Data<Database>, Json(item): Json<CreateRepository>) -> impl Responder {
//     actix_web::web::block(move || {
//         let mut conn = db.get_connection();

//         Repository::create(&mut conn, &item)
//     })
//     .await
//     .map(|result| match result {
//         Ok(result) => Ok(HttpResponse::Created().json(result)),
//         Err(err) => Err(ErrorInternalServerError(err)),
//     })
// }

// #[put("/{id}")]
// async fn update(
//     db: Data<Database>,
//     item_id: Path<String>,
//     Json(item): Json<UpdateRepository>,
// ) -> impl Responder {
//     actix_web::web::block(move || {
//         let mut conn = db.get_connection();

//         Repository::update(&mut conn, item_id.into_inner(), &item)
//     })
//     .await
//     .map(|result| match result {
//         Ok(result) => Ok(HttpResponse::Ok().json(result)),
//         Err(err) => Err(ErrorInternalServerError(err)),
//     })
// }

// #[delete("/{id}")]
// async fn destroy(db: Data<Database>, item_id: Path<String>) -> impl Responder {
//     actix_web::web::block(move || {
//         let mut conn = db.get_connection();

//         Repository::delete(&mut conn, item_id.into_inner())
//     })
//     .await
//     .map(|result| match result {
//         Ok(result) => Ok(HttpResponse::Ok().json(result)),
//         Err(err) => Err(ErrorInternalServerError(err)),
//     })
// }

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    scope.service(index).service(read)
    // .service(create)
    // .service(update)
    // .service(destroy)
}
