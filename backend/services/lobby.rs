use crate::models::{
    answer_choice::{AnswerChoice, CreateAnswerChoice},
    correct_answer::{CorrectAnswer, CreateCorrectAnswer},
    lobby::{CreateLobby, Lobby},
    question::{CreateQuestion, Question},
};
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

        Lobby::paginate(&mut conn, info.page, info.page_size)
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

        Lobby::read(&mut conn, item_id.into_inner())
    })
    .await
    .map(|result| match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(err) => Err(ErrorNotFound(err)),
    })
}

#[post("")]
async fn create(db: Data<Database>, Json(item): Json<CreateLobby>) -> impl Responder {
    actix_web::web::block(move || {
        let mut conn = db.get_connection();

        let lobby = Lobby::create(&mut conn, &item)?;

        let question = Question::create(
            &mut conn,
            &CreateQuestion {
                lobby_id: lobby.id.clone(),
                question_text: "What is 1+1".to_string(),
            },
        )?;

        for i in 0..4 {
            AnswerChoice::create(
                &mut conn,
                &CreateAnswerChoice {
                    question_id: question.id,
                    answer: i.to_string(),
                },
            )?;
        }

        CorrectAnswer::create(
            &mut conn,
            &CreateCorrectAnswer {
                question_id: question.id,
                answer: "2".to_string(),
            },
        )?;

        Ok::<Lobby, diesel::result::Error>(lobby)
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
//     item_id: Path<String>,
//     Json(item): Json<UpdateLobby>,
// ) -> impl Responder {
//     actix_web::web::block(move || {
//         let mut conn = db.get_connection();

//         Lobby::update(&mut conn, item_id.into_inner(), &item)
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

//         Lobby::delete(&mut conn, item_id.into_inner())
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
