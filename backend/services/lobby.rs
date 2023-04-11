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
        obfuscated_question::{CreateObfuscatedQuestion, ObfuscatedQuestion},
        obfuscated_user_answer::ObfuscatedUserAnswer,
        repository::Repository,
    },
};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get, post, put,
    web::{Data, Json, Path, Query},
    HttpResponse, Responder,
};
use create_rust_app::Database;
use diesel::{
    debug_query, dsl::IntervalDsl, pg::Pg, sql_types::Interval, BelongingToDsl,
    BoolExpressionMethods, ExpressionMethods, GroupedBy, IntoSql, QueryDsl, QueryResult,
    RunQueryDsl, SelectableHelper,
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
struct FullLobby {
    lobby: Lobby,
}

#[get("/{id}")]
async fn read(db: Data<Database>, item_id: Path<String>, auth: Auth) -> impl Responder {
    let lobby_id = item_id.into_inner();
    actix_web::web::block(move || {
        let mut conn = db.get_connection();
        let l/*: Vec<(
            Lobby,
            LobbyParticipant,
            ObfuscatedQuestion,
            ObfuscatedCorrectAnswer,
            Option<ObfuscatedUserAnswer>,
        )> */ = {
            use crate::schema::{obfuscated_correct_answer, lobby, lobby_participant, obfuscated_question, obfuscated_user_answer, git_guessr_correct_answer, git_guessr_question, git_guessr_user_answer};

            // let all_lobby_participants = lobby_participant::table
            //     .filter(lobby_participant::lobby_id.eq(&lobby_id))
            //     .select(LobbyParticipant::as_select())
            //     .load(&mut conn)?;

            // // all_lobby_participants

            let the_lobby = lobby::table.filter(lobby::id.eq(lobby_id)).first::<Lobby>(&mut conn)?;

            let a = ObfuscatedQuestion::belonging_to(&the_lobby)
                .inner_join(obfuscated_correct_answer::table)
                .left_join(obfuscated_user_answer::table)
                .select((
                    ObfuscatedQuestion::as_select(),
                    ObfuscatedCorrectAnswer::as_select(),
                    Option::<ObfuscatedUserAnswer>::as_select())).load::<(
                        ObfuscatedQuestion,
                        ObfuscatedCorrectAnswer,
                        Option<ObfuscatedUserAnswer>,
                        // ObfuscatedUserAnswer,
                    )>(&mut conn)?;

            let b = GitGuessrQuestion::belonging_to(&the_lobby)
                    .inner_join(git_guessr_correct_answer::table)
                    .left_join(git_guessr_user_answer::table)
                    .select((
                        GitGuessrQuestion::as_select(),
                        GitGuessrCorrectAnswer::as_select(),
                        Option::<GitGuessrUserAnswer>::as_select()))
                    .load::<(
                            GitGuessrQuestion,
                            GitGuessrCorrectAnswer,
                            Option<GitGuessrUserAnswer>,
                            // ObfuscatedUserAnswer,
                        )>(&mut conn)?;

            // println!("{a}");
            // println!("{b}");

            // let all_data = lobby::table
            //     .inner_join(lobby_participant::table)
            //     .inner_join(
            //         question::table
            //             .inner_join(correct_answer::table)
            //             // .inner_join(user_answer::table),
            //             .left_join(user_answer::table),
            //     )
            //     .filter(
            //         lobby::id
            //             .eq(&lobby_id)
            //             .and(lobby_participant::user_id.eq(auth.user_id)),
            //     )
            //     .select((
            //         Lobby::as_select(),
            //         LobbyParticipant::as_select(),
            //         ObfuscatedQuestion::as_select(),
            //         ObfuscatedCorrectAnswer::as_select(),
            //         Option::<ObfuscatedUserAnswer>::as_select(),
            //         // ObfuscatedUserAnswer::as_select(),
            //     ))
            //     .load::<(
            //         Lobby,
            //         LobbyParticipant,
            //         ObfuscatedQuestion,
            //         ObfuscatedCorrectAnswer,
            //         Option<ObfuscatedUserAnswer>,
            //         // ObfuscatedUserAnswer,
            //     )>(&mut conn)?;
            //     all_data.grouped_by()
            a
        };

        println!("{:?}", l);

        QueryResult::Ok(l)

        // Lobby::read(&mut conn, lobby_id)
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

            let chosen_entries: Vec<_> = get_random_entries(&entries, 5);

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
                        question_num: i as i32,
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
        // for i in 1..5 {
        //     let question = ObfuscatedQuestion::create(
        //         &mut conn,
        //         &CreateObfuscatedQuestion {
        //             lobby_id: lobby.id.clone(),
        //             question_num: i,
        //             question_text: format!("What is {i} + {i}?"),
        //             big_answer_choices: false,
        //             start_time: None,
        //             end_time: None,
        //         },
        //     )?;

        //     ObfuscatedAnswerChoice::create(
        //         &mut conn,
        //         &CreateObfuscatedAnswerChoice {
        //             lobby_id: lobby.id.clone(),
        //             question_id: question.id,
        //             answer: i.to_string(),
        //         },
        //     )?;
        //     let answer_choice = ObfuscatedAnswerChoice::create(
        //         &mut conn,
        //         &CreateObfuscatedAnswerChoice {
        //             lobby_id: lobby.id.clone(),
        //             question_id: question.id,
        //             answer: (i * 2).to_string(),
        //         },
        //     )?;

        //     ObfuscatedCorrectAnswer::create(
        //         &mut conn,
        //         &CreateObfuscatedCorrectAnswer {
        //             lobby_id: lobby.id.clone(),
        //             question_id: question.id,
        //             answer_choice_id: answer_choice.id,
        //         },
        //     )?;
        // }

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
            use crate::schema::obfuscated_question::dsl::*;

            println!(
                "{}",
                debug_query::<Pg, _>(
                    &diesel::update(
                        obfuscated_question.filter(
                            lobby_id
                                .eq(lobby_id_param.clone())
                                .and(start_time.is_null()),
                        ),
                    )
                    .set((
                        start_time.eq((new_start_time.as_sql::<diesel::sql_types::Timestamptz>()
                            + 20.seconds().as_sql::<Interval>() * (question_num - 1))
                            .nullable()),
                        end_time.eq((new_start_time.as_sql::<diesel::sql_types::Timestamptz>()
                            + 20.seconds().as_sql::<Interval>() * question_num
                            - 10.seconds())
                        .nullable()),
                    ))
                )
            );

            diesel::update(
                obfuscated_question.filter(
                    lobby_id
                        .eq(lobby_id_param.clone())
                        .and(start_time.is_null()),
                ),
            )
            .set((
                start_time.eq((new_start_time.as_sql::<diesel::sql_types::Timestamptz>()
                    + 20.seconds().as_sql::<Interval>() * (question_num - 1))
                    .nullable()),
                end_time.eq((new_start_time.as_sql::<diesel::sql_types::Timestamptz>()
                    + 20.seconds().as_sql::<Interval>() * question_num
                    - 10.seconds())
                .nullable()),
            ))
            // .set(start_time.eq(new_start_time + question_num.seconds() + 10.seconds()))
            .get_result::<ObfuscatedQuestion>(&mut conn)?;
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
    scope
        .service(index)
        .service(read)
        .service(create)
        .service(update)
    // .service(destroy)
}
