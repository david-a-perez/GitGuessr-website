use crate::models::user_answer::{CreateUserAnswer, UserAnswer};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get, post,
    web::{Data, Json, Path, Query},
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

        UserAnswer::paginate(&mut conn, info.page, info.page_size)
    })
    .await
    .map(|result| match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(err) => Err(ErrorInternalServerError(err)),
    })
}

#[get("/{user_id}/{question_id}")]
async fn read(db: Data<Database>, user_id: Path<i32>, question_id: Path<i32>) -> impl Responder {
    actix_web::web::block(move || {
        let mut conn = db.get_connection();

        UserAnswer::read(&mut conn, user_id.into_inner(), question_id.into_inner())
    })
    .await
    .map(|result| match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(err) => Err(ErrorNotFound(err)),
    })
}

#[post("")]
async fn create(db: Data<Database>, Json(item): Json<CreateUserAnswer>) -> impl Responder {
    actix_web::web::block(move || {
        let mut conn = db.get_connection();

        UserAnswer::create(&mut conn, &item)
    })
    .await
    .map(|result| match result {
        Ok(result) => Ok(HttpResponse::Created().json(result)),
        Err(err) => Err(ErrorInternalServerError(err)),
    })
}

// #[put("/{user_id}/{question_id}")]
// async fn update(
//     db: Data<Database>,
//     user_id: Path<i32>,
//     question_id: Path<i32>,
//     Json(item): Json<UpdateUserAnswer>,
// ) -> impl Responder {
//     actix_web::web::block(move || {
//         let mut conn = db.get_connection();

//         UserAnswer::update(
//             &mut conn,
//             user_id.into_inner(),
//             question_id.into_inner(),
//             &item,
//         )
//     })
//     .await
//     .map(|result| match result {
//         Ok(result) => Ok(HttpResponse::Ok().json(result)),
//         Err(err) => Err(ErrorInternalServerError(err)),
//     })
// }

// #[delete("/{user_id}/{question_id}")]
// async fn destroy(db: Data<Database>, user_id: Path<i32>, question_id: Path<i32>) -> impl Responder {
//     actix_web::web::block(move || {
//         let mut conn = db.get_connection();

//         UserAnswer::delete(&mut conn, user_id.into_inner(), question_id.into_inner())
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
