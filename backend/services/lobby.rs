use std::string::FromUtf8Error;

use crate::{
    gitguessr::{
        get_all_file_entries, get_random_entries, get_snippet_from_file, get_text_from_entry,
        FilteredRecorder, GitGuessrError,
    },
    models::{
        git_guessr_correct_answer::{CreateGitGuessrCorrectAnswer, GitGuessrCorrectAnswer},
        git_guessr_game_format_config::GitGuessrGameFormatConfig,
        git_guessr_question::{CreateGitGuessrQuestion, GitGuessrQuestion},
        git_guessr_user_answer::GitGuessrUserAnswer,
        lobby::{CreateLobby, Lobby},
        lobby_participant::LobbyParticipant,
        obfuscated_answer_choice::{CreateObfuscatedAnswerChoice, ObfuscatedAnswerChoice},
        obfuscated_correct_answer::{CreateObfuscatedCorrectAnswer, ObfuscatedCorrectAnswer},
        obfuscated_game_format_config::ObfuscatedGameFormatConfig,
        obfuscated_question::{CreateObfuscatedQuestion, ObfuscatedQuestion},
        obfuscated_user_answer::ObfuscatedUserAnswer,
        repository::Repository,
    },
    obfuscated::{obfuscate, ObfuscatedError},
};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get, post, put,
    web::{Data, Json, Path, Query},
    HttpResponse, Responder,
};
use create_rust_app::Database;
use diesel::{
    dsl::IntervalDsl,sql_types::Interval, BelongingToDsl,
    ExpressionMethods, GroupedBy, IntoSql, QueryDsl, QueryResult,
    RunQueryDsl,
};
use gitguessr_auth::Auth;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[tsync::tsync]
#[derive(serde::Deserialize)]
pub struct PaginationParams {
    pub page: i64,
    pub page_size: i64,
}

#[tsync::tsync]
#[derive(Deserialize)]
struct LobbyFilters {
    repository_id: Option<String>,
}

#[derive(Error, Debug)]
enum LobbyError {
    #[error("GitGuessr Error: {0}")]
    GitGuessr(#[from] crate::gitguessr::GitGuessrError),

    #[error("Obfuscated Error: {0}")]
    Obfuscated(#[from] crate::obfuscated::ObfuscatedError),

    #[error("Diesel Error: {0}")]
    Diesel(#[from] diesel::result::Error),
}

#[get("")]
async fn index(
    db: Data<Database>,
    Query(PaginationParams { page, page_size }): Query<PaginationParams>,
    Query(filters): Query<LobbyFilters>,
) -> impl Responder {
    actix_web::web::block(move || {
        let mut conn = db.get_connection();

        {
            use crate::diesel::ExpressionMethods;
            use crate::diesel::QueryDsl;
            use crate::diesel::QueryResult;
            use crate::diesel::RunQueryDsl;
            use crate::models::lobby::PaginationResult;
            use crate::schema::lobby::dsl::*;

            let page_size = if page_size < 1 { 1 } else { page_size };

            let mut query = lobby.into_boxed();
            let mut count_query = lobby.into_boxed();

            if let Some(repository) = filters.repository_id {
                query = query.filter(repository_id.eq(repository.clone()));
                count_query = count_query.filter(repository_id.eq(repository));
            }

            let items = query
                .limit(page_size)
                .offset(page * page_size)
                .load::<Lobby>(&mut conn)?;

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

        // Lobby::paginate(&mut conn, info.page, info.page_size)
    })
    .await
    .map(|result| match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(err) => Err(ErrorInternalServerError(err)),
    })
}

#[tsync::tsync]
#[derive(Serialize)]
struct FullObfuscatedQuestions {
    question: ObfuscatedQuestion,
    answer_choices: Vec<ObfuscatedAnswerChoice>,
    correct_answer: ObfuscatedCorrectAnswer,
    user_answer: Option<ObfuscatedUserAnswer>,
}

#[tsync::tsync]
#[derive(Serialize)]
struct FullGitGuessrQuestions {
    question: GitGuessrQuestion,
    correct_answer: GitGuessrCorrectAnswer,
    user_answer: Option<GitGuessrUserAnswer>,
}

#[tsync::tsync]
#[derive(Serialize)]
struct FullLobby {
    lobby: Lobby,
    full_obfuscated_questions: Vec<FullObfuscatedQuestions>,
    full_git_guessr_questions: Vec<FullGitGuessrQuestions>,
}

#[get("/{id}")]
async fn read(db: Data<Database>, item_id: Path<String>, auth: Auth) -> impl Responder {
    let lobby_id = item_id.into_inner();
    actix_web::web::block(move || {
        let mut conn = db.get_connection();

        let lobby = Lobby::read(&mut conn, lobby_id)?;

        let participant = LobbyParticipant::belonging_to(&lobby)
            .filter(crate::schema::lobby_participant::user_id.eq(auth.user_id))
            .load::<LobbyParticipant>(&mut conn)?;
        let full_obfuscated_questions = {
            let obfuscated_questions =
                ObfuscatedQuestion::belonging_to(&lobby).load::<ObfuscatedQuestion>(&mut conn)?;

            let obfuscated_answer_choices =
                ObfuscatedAnswerChoice::belonging_to(&obfuscated_questions)
                    .load::<ObfuscatedAnswerChoice>(&mut conn)?;

            let obfuscated_correct_answers =
                    ObfuscatedCorrectAnswer::belonging_to(&obfuscated_questions)
                        .load::<ObfuscatedCorrectAnswer>(&mut conn)?;

            let obfuscated_user_answers = ObfuscatedUserAnswer::belonging_to(&participant)
                .load::<ObfuscatedUserAnswer>(&mut conn)?;

            let grouped_obfuscated_answer_choices =
                obfuscated_answer_choices.grouped_by(&obfuscated_questions);
            let grouped_obfuscated_correct_answers =
                obfuscated_correct_answers.grouped_by(&obfuscated_questions);
            let grouped_obfuscated_user_answers =
                obfuscated_user_answers.grouped_by(&obfuscated_questions);

            let grouped_obfuscated_answers = grouped_obfuscated_answer_choices
                .into_iter()
                .zip(grouped_obfuscated_correct_answers)
                .zip(grouped_obfuscated_user_answers);

            obfuscated_questions
                .into_iter()
                .zip(grouped_obfuscated_answers)
                .map(
                    |(question, ((answer_choices, correct_answers), user_answers))| FullObfuscatedQuestions {
                        question,
                        answer_choices,
                        correct_answer: correct_answers[0].clone(),
                        user_answer: user_answers.get(0).cloned(),
                    },
                )
                .collect::<Vec<_>>()
        };

        let full_git_guessr_questions = {
            let git_guessr_questions =
                GitGuessrQuestion::belonging_to(&lobby).load::<GitGuessrQuestion>(&mut conn)?;

            let git_guessr_correct_answers = GitGuessrCorrectAnswer::belonging_to(&git_guessr_questions)
                .load::<GitGuessrCorrectAnswer>(&mut conn)?;
            let git_guessr_user_answers = GitGuessrUserAnswer::belonging_to(&participant)
              .load::<GitGuessrUserAnswer>(&mut conn)?;

            let grouped_git_guessr_correct_answers =
                git_guessr_correct_answers.grouped_by(&git_guessr_questions);
            let grouped_git_guessr_user_answers =
                git_guessr_user_answers.grouped_by(&git_guessr_questions);

            git_guessr_questions
                .into_iter()
                .zip(grouped_git_guessr_correct_answers)
                .zip(grouped_git_guessr_user_answers)
                .map(|((question, correct_answers), user_answers)| FullGitGuessrQuestions {
                    question,
                    correct_answer: correct_answers[0].clone(),
                    user_answer: user_answers.get(0).cloned(),
                })
                .collect::<Vec<_>>()
        };

        QueryResult::Ok(FullLobby {
            lobby,
            full_obfuscated_questions,
            full_git_guessr_questions,
        })
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

        if let Some(git_guessr_game_format_config_id) = lobby.git_guessr_game_format_config_id {
            let config =
                GitGuessrGameFormatConfig::read(&mut conn, git_guessr_game_format_config_id)?;
            let repository = Repository::read(&mut conn, lobby.repository_id.clone())?;

            let repo = gix::open(repository.filename).map_err(GitGuessrError::from)?;

            let entries = get_all_file_entries(&repo, FilteredRecorder::new(&config.filenames)?)?;

            let chosen_entries: Vec<_> = get_random_entries(&entries, 3);

            for (i, entry) in chosen_entries.into_iter().enumerate() {
                let blob = get_text_from_entry(&repo, entry)?;
                let snippet = get_snippet_from_file(
                    entry.filepath.as_ref(),
                    &blob.data,
                    config.lines_shown as usize,
                )?;

                let question = GitGuessrQuestion::create(
                    &mut conn,
                    &CreateGitGuessrQuestion {
                        lobby_id: lobby.id.clone(),
                        question_num: i as i32 + 1,
                        question_text: snippet.join("\n"),
                        start_time: None,
                        end_time: None,
                    },
                )?;

                GitGuessrCorrectAnswer::create(
                    &mut conn,
                    &CreateGitGuessrCorrectAnswer {
                        lobby_id: lobby.id.clone(),
                        question_id: question.id,
                        answer: entry.filepath.to_string(),
                    },
                )?;
            }
        }

        if let Some(obfuscated_game_format_config_id) = lobby.obfuscated_game_format_config_id {
            let config =
                ObfuscatedGameFormatConfig::read(&mut conn, obfuscated_game_format_config_id)?;

            let repository = Repository::read(&mut conn, lobby.repository_id.clone())?;

            let repo = gix::open(repository.filename).map_err(GitGuessrError::from)?;

            let entries = get_all_file_entries(&repo, FilteredRecorder::new(&config.filenames)?)?;

            let chosen_entries: Vec<_> = get_random_entries(&entries, 5);

            let full_text = chosen_entries
                .into_iter()
                .map(|entry| {
                    get_text_from_entry(&repo, entry)
                        .map(|object| object.data.clone())
                        .map(|bytes| String::from_utf8(bytes))
                })
                .collect::<Result<Result<Vec<_>, FromUtf8Error>, GitGuessrError>>()?
                .map_err(ObfuscatedError::FromUtf8Error)?
                .join("\n");

            for i in 1..=3 {
                let question_data = obfuscate(&config.language, &full_text.as_bytes(), 4)?;

                let question = ObfuscatedQuestion::create(
                    &mut conn,
                    &CreateObfuscatedQuestion {
                        lobby_id: lobby.id.clone(),
                        question_num: i,
                        question_text: question_data[0].text.clone(),
                        big_answer_choices: false,
                        start_time: None,
                        end_time: None,
                    },
                )?;

                let answer_choices = question_data
                    .into_iter()
                    .map(|data| {
                        ObfuscatedAnswerChoice::create(
                            &mut conn,
                            &CreateObfuscatedAnswerChoice {
                                lobby_id: lobby.id.clone(),
                                question_id: question.id,
                                answer: data.answer,
                            },
                        )
                    })
                    .collect::<Result<Vec<_>, diesel::result::Error>>()?;

                ObfuscatedCorrectAnswer::create(
                    &mut conn,
                    &CreateObfuscatedCorrectAnswer {
                        lobby_id: lobby.id.clone(),
                        question_id: question.id,
                        answer_choice_id: answer_choices[0].id,
                    },
                )?;
            }
        }

        Ok::<Lobby, LobbyError>(lobby)
    })
    .await
    .map(|result| match result {
        Ok(result) => Ok(HttpResponse::Created().json(result)),
        Err(err) => Err(ErrorInternalServerError(err)),
    })
}

#[tsync::tsync]
#[derive(Deserialize)]
struct StartLobby {
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
}

#[put("/{id}")]
async fn update(
    db: Data<Database>,
    item_id: Path<String>,
    Json(item): Json<StartLobby>,
) -> impl Responder {
    actix_web::web::block(move || {
        let lobby_id_param = item_id.into_inner();
        let mut conn = db.get_connection();

        let new_start_time = item
            .start_time
            .unwrap_or_else(|| chrono::offset::Utc::now());

        {
            use crate::diesel::BoolExpressionMethods;
            use crate::diesel::ExpressionMethods;
            use crate::diesel::NullableExpressionMethods;
            use crate::diesel::QueryDsl;
            use crate::diesel::RunQueryDsl;
            use crate::schema::git_guessr_question::dsl::*;

            diesel::update(
                git_guessr_question.filter(
                    lobby_id
                        .eq(lobby_id_param.clone())
                        .and(start_time.is_null()),
                ),
            )
            .set((
                start_time.eq((new_start_time.as_sql::<diesel::sql_types::Timestamptz>()
                    + 25.seconds().as_sql::<Interval>() * (question_num - 1))
                    .nullable()),
                end_time.eq((new_start_time.as_sql::<diesel::sql_types::Timestamptz>()
                    + 25.seconds().as_sql::<Interval>() * question_num
                    - 5.seconds())
                .nullable()),
            ))
            .execute(&mut conn)?;
        }

        {
            use crate::diesel::BoolExpressionMethods;
            use crate::diesel::ExpressionMethods;
            use crate::diesel::NullableExpressionMethods;
            use crate::diesel::QueryDsl;
            use crate::diesel::RunQueryDsl;
            use crate::schema::obfuscated_question::dsl::*;

            diesel::update(
                obfuscated_question.filter(
                    lobby_id
                        .eq(lobby_id_param.clone())
                        .and(start_time.is_null()),
                ),
            )
            .set((
                start_time.eq((new_start_time.as_sql::<diesel::sql_types::Timestamptz>()
                    + 15.seconds().as_sql::<Interval>() * (question_num - 1))
                    .nullable()),
                end_time.eq((new_start_time.as_sql::<diesel::sql_types::Timestamptz>()
                    + 15.seconds().as_sql::<Interval>() * question_num
                    - 5.seconds())
                .nullable()),
            ))
            .execute(&mut conn)?;
        }

        {
            use crate::diesel::BoolExpressionMethods;
            use crate::diesel::ExpressionMethods;
            use crate::diesel::QueryDsl;
            use crate::diesel::RunQueryDsl;
            use crate::schema::lobby::dsl::*;

            diesel::update(lobby.filter(id.eq(lobby_id_param).and(start_time.is_null())))
                .set(start_time.eq(new_start_time))
                .get_result::<Lobby>(&mut conn)
        }
    })
    .await
    .map(|result| match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(err) => Err(ErrorInternalServerError(err)),
    })
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    scope
        .service(index)
        .service(read)
        .service(create)
        .service(update)
}
