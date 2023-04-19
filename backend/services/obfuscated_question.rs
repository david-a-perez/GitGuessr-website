use crate::models::{
    obfuscated_answer_choice::ObfuscatedAnswerChoice,
    obfuscated_correct_answer::ObfuscatedCorrectAnswer, obfuscated_question::ObfuscatedQuestion,
    obfuscated_user_answer::ObfuscatedUserAnswer,
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

        ObfuscatedQuestion::paginate(&mut conn, info.page, info.page_size)
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

        let question = ObfuscatedQuestion::read(&mut conn, item_id.into_inner())?;
        if let Some(start_time) = question.start_time {
            if start_time <= chrono::offset::Utc::now() {
                return Ok(Some(question));
            }
        }
        Ok::<Option<ObfuscatedQuestion>, diesel::result::Error>(None)
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
struct FullObfuscatedQuestion {
    question: ObfuscatedQuestion,
    answer_choices: Vec<ObfuscatedAnswerChoice>,
    correct_answer: Option<ObfuscatedCorrectAnswer>,
    user_answer: Option<ObfuscatedUserAnswer>,
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
            use crate::schema::obfuscated_question::dsl::*;

            obfuscated_question
                .filter(lobby_id.eq(lobby_param.clone()))
                .filter(question_num.eq(question_num_param))
                .first::<ObfuscatedQuestion>(&mut conn)?
        };

        let next_question = {
            use crate::schema::obfuscated_question::dsl::*;

            obfuscated_question
                .filter(lobby_id.eq(lobby_param))
                .filter(question_num.eq(question_num_param + 1))
                .first::<ObfuscatedQuestion>(&mut conn)
                .optional()?
        };

        let mut answer_choices = {
            use crate::schema::obfuscated_answer_choice::dsl::*;

            obfuscated_answer_choice
                .filter(question_id.eq(question.id))
                .load::<ObfuscatedAnswerChoice>(&mut conn)?
        };

        // Remove the text from the question and answer choices if the question hasn't started yet
        let curr_time = chrono::offset::Utc::now();
        match question.start_time {
            Some(start_time) if curr_time + Duration::milliseconds(5000) >= start_time => {}
            _ => {
                question.question_text = String::new();
                for answer_choice in &mut answer_choices {
                    answer_choice.answer = String::new();
                }
            }
        }

        // Only show correct answer if the question has already ended
        let correct_answer = match question.end_time {
            Some(end_time) if curr_time + Duration::milliseconds(5000) >= end_time => Some({
                use crate::schema::obfuscated_correct_answer::dsl::*;

                obfuscated_correct_answer
                    .filter(question_id.eq(question.id))
                    .first::<ObfuscatedCorrectAnswer>(&mut conn)?
            }),
            _ => None,
        };

        // Only show correct answer if the question has already ended
        let user_answer = {
            use crate::schema::obfuscated_user_answer::dsl::*;

            obfuscated_user_answer
                .filter(question_id.eq(question.id))
                .first::<ObfuscatedUserAnswer>(&mut conn)
                .optional()?
        };

        QueryResult::Ok(FullObfuscatedQuestion {
            question,
            answer_choices,
            correct_answer,
            user_answer,
            next_question_start_time: next_question
                .and_then(|next_question| next_question.start_time),
        })
    })
    .await
    .map(|result| match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(err) => Err(ErrorNotFound(err)),
    })
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    scope
        .service(index)
        .service(read)
        .service(read_by_lobby_and_question_num)
}
