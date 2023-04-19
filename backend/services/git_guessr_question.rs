use crate::models::{
    git_guessr_correct_answer::GitGuessrCorrectAnswer, git_guessr_question::GitGuessrQuestion,
    git_guessr_user_answer::GitGuessrUserAnswer,
};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get,
    web::{Data, Path, Query},
    HttpResponse, Responder,
};
use chrono::Duration;
use create_rust_app::Database;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, QueryResult, RunQueryDsl};

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

        GitGuessrQuestion::paginate(&mut conn, info.page, info.page_size)
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

        let question = GitGuessrQuestion::read(&mut conn, item_id.into_inner())?;
        if let Some(start_time) = question.start_time {
            if start_time <= chrono::offset::Utc::now() {
                return Ok(Some(question));
            }
        }
        Ok::<Option<GitGuessrQuestion>, diesel::result::Error>(None)
    })
    .await
    .map(|result| match result {
        Ok(Some(result)) => Ok(HttpResponse::Ok().json(result)),
        Ok(None) => Ok(HttpResponse::Forbidden().finish()), // TODO: use error?
        Err(err) => Err(ErrorNotFound(err)),
    })
}

#[tsync::tsync]
#[derive(serde::Serialize)]
struct FullGitGuessrQuestion {
    question: GitGuessrQuestion,
    correct_answer: Option<GitGuessrCorrectAnswer>,
    user_answer: Option<GitGuessrUserAnswer>,
    next_question_start_time: Option<chrono::DateTime<chrono::Utc>>,
}

#[get("/{lobby_id}/{question_num}")]
async fn read_by_lobby_and_question_num(
    db: Data<Database>,
    params: Path<(String, i32)>,
) -> impl Responder {
    actix_web::web::block(move || {
        let (lobby_param, question_num_param) = params.into_inner();
        let mut conn = db.get_connection();

        let mut question = {
            use crate::schema::git_guessr_question::dsl::*;

            git_guessr_question
                .filter(lobby_id.eq(lobby_param.clone()))
                .filter(question_num.eq(question_num_param))
                .first::<GitGuessrQuestion>(&mut conn)?
        };

        let next_question = {
            use crate::schema::git_guessr_question::dsl::*;

            git_guessr_question
                .filter(lobby_id.eq(lobby_param))
                .filter(question_num.eq(question_num_param + 1))
                .first::<GitGuessrQuestion>(&mut conn)
                .optional()?
        };

        // Remove the text from the question and answer choices if the question hasn't started yet
        let curr_time = chrono::offset::Utc::now();
        match question.start_time {
            Some(start_time) if curr_time + Duration::milliseconds(5000) >= start_time => {}
            _ => {
                question.question_text = String::new();
            }
        }

        // Only show correct answer if the question has already ended
        let correct_answer = match question.end_time {
            Some(end_time) if curr_time + Duration::milliseconds(5000) >= end_time => Some({
                use crate::schema::git_guessr_correct_answer::dsl::*;

                git_guessr_correct_answer
                    .filter(question_id.eq(question.id))
                    .first::<GitGuessrCorrectAnswer>(&mut conn)?
            }),
            _ => None,
        };

        // Only show correct answer if the question has already ended
        let user_answer = {
            use crate::schema::git_guessr_user_answer::dsl::*;

            git_guessr_user_answer
                .filter(question_id.eq(question.id))
                .first::<GitGuessrUserAnswer>(&mut conn)
                .optional()?
        };

        QueryResult::Ok(FullGitGuessrQuestion {
            question,
            correct_answer,
            user_answer,
            next_question_start_time: next_question
            .and_then(|next_question| next_question.start_time),

        })
    })
    .await
    .map(|result| match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(err) => Err(ErrorInternalServerError(err)),
    })
}

// #[post("")]
// async fn create(db: Data<Database>, Json(item): Json<CreateQuestion>) -> impl Responder {
//     actix_web::web::block(move || {
//         let mut conn = db.get_connection();

//         GitGuessrQuestion::create(&mut conn, &item)
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
//     item_id: Path<i32>,
//     Json(item): Json<UpdateQuestion>,
// ) -> impl Responder {
//     actix_web::web::block(move || {
//         let mut conn = db.get_connection();

//         GitGuessrQuestion::update(&mut conn, item_id.into_inner(), &item)
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

//         GitGuessrQuestion::delete(&mut conn, item_id.into_inner())
//     })
//     .await
//     .map(|result| match result {
//         Ok(result) => Ok(HttpResponse::Ok().json(result)),
//         Err(err) => Err(ErrorInternalServerError(err)),
//     })
// }

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    scope
        .service(index)
        .service(read)
        .service(read_by_lobby_and_question_num)
    // .service(create)
    // .service(update)
    // .service(destroy)
}
