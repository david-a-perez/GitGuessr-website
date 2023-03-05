use crate::models::answer_choice::{AnswerChoice, CreateAnswerChoice, UpdateAnswerChoice};
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
        AnswerChoice::paginate(&mut conn, info.page, info.page_size)
    })
    .await
    .map(|result| match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(err) => Err(ErrorNotFound(err)),
    })
}

#[get("/{question_id}/{answer}")]
async fn read(db: Data<Database>, question_id: Path<i32>, answer: Path<String>) -> impl Responder {
    actix_web::web::block(move || {
        let mut conn = db.get_connection();
        AnswerChoice::read(&mut conn, question_id.into_inner(), answer.into_inner())
    })
    .await
    .map(|result| match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(err) => Err(ErrorNotFound(err)),
    })
}

// #[post("")]
// async fn create(db: Data<Database>, Json(item): Json<CreateAnswerChoice>) -> impl Responder {
//     actix_web::web::block(move || {
//         let mut conn = db.get_connection();
//         AnswerChoice::create(&mut conn, &item)
//     })
//     .await
//     .map(|result| match result {
//         Ok(result) => Ok(HttpResponse::Ok().json(result)),
//         Err(err) => Err(ErrorNotFound(err)),
//     })
// }

// #[put("/{question_id}/{answer}")]
// async fn update(
//     db: Data<Database>,
//     question_id: Path<i32>,
//     answer: Path<String>,
//     Json(item): Json<UpdateAnswerChoice>,
// ) -> impl Responder {
//     actix_web::web::block(move || {
//         let mut conn = db.get_connection();
//         AnswerChoice::update(
//             &mut conn,
//             question_id.into_inner(),
//             answer.into_inner(),
//             &item,
//         )
//     })
//     .await
//     .map(|result| match result {
//         Ok(result) => Ok(HttpResponse::Ok().json(result)),
//         Err(err) => Err(ErrorNotFound(err)),
//     })
// }

// #[delete("/{question_id}/{answer}")]
// async fn destroy(
//     db: Data<Database>,
//     question_id: Path<i32>,
//     answer: Path<String>,
// ) -> impl Responder {
//     actix_web::web::block(move || {
//         let mut conn = db.get_connection();
//         AnswerChoice::delete(&mut conn, question_id.into_inner(), answer.into_inner())
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
