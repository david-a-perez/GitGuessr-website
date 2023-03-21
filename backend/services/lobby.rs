use crate::models::{
    answer_choice::{AnswerChoice, CreateAnswerChoice},
    correct_answer::{CorrectAnswer, CreateCorrectAnswer},
    lobby::{CreateLobby, Lobby},
    lobby_participant::LobbyParticipant,
    question::{CreateQuestion, Question},
    user_answer::UserAnswer,
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
        // let l/*: Vec<(
        //     Lobby,
        //     LobbyParticipant,
        //     Question,
        //     CorrectAnswer,
        //     Option<UserAnswer>,
        // )> */ = {
        //     use crate::schema::{correct_answer, lobby, lobby_participant, question, user_answer};

        //     let all_lobby_participants = lobby_participant::table
        //         .filter(lobby_participant::lobby_id.eq(&lobby_id))
        //         .select(LobbyParticipant::as_select())
        //         .load(&mut conn)?;

        //     let all_data = lobby::table
        //         .inner_join(lobby_participant::table)
        //         .inner_join(
        //             question::table
        //                 .inner_join(correct_answer::table)
        //                 .left_join(user_answer::table),
        //         )
        //         .filter(
        //             lobby::id
        //                 .eq(&lobby_id)
        //                 .and(lobby_participant::user_id.eq(auth.user_id)),
        //         )
        //         .select((
        //             Lobby::as_select(),
        //             LobbyParticipant::as_select(),
        //             Question::as_select(),
        //             CorrectAnswer::as_select(),
        //             Option::<UserAnswer>::as_select(),
        //         ))
        //         .load::<(
        //             Lobby,
        //             LobbyParticipant,
        //             Question,
        //             CorrectAnswer,
        //             Option<UserAnswer>,
        //         )>(&mut conn)?;
        //     all_data.grouped_by(&all_participants)
        //     .into_iter()
        //     .zip(lobby_participant::table)
        //      .map(|(question_data, participant)| (participant, question_data.into_iter().map(|(_, book)| book).collect()))
        // };
        // l.group_by(|a, b| a.0.id == b.0.id && a.1.id == b.1.id).map(
        //     |(lobby, lobby_participant, question, correct_answer, user_answer)| correct_answer,
        // );

        // println!("{}", l);

        // QueryResult::Ok(l)

        Lobby::read(&mut conn, lobby_id)
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
        for i in 1..5 {
            let question = Question::create(
                &mut conn,
                &CreateQuestion {
                    lobby_id: lobby.id.clone(),
                    question_num: i,
                    question_text: format!("What is {i} + {i}?"),
                    start_time: None,
                    end_time: None,
                },
            )?;

            AnswerChoice::create(
                &mut conn,
                &CreateAnswerChoice {
                    lobby_id: lobby.id.clone(),
                    question_id: question.id,
                    answer: i.to_string(),
                },
            )?;
            let answer_choice = AnswerChoice::create(
                &mut conn,
                &CreateAnswerChoice {
                    lobby_id: lobby.id.clone(),
                    question_id: question.id,
                    answer: (i * 2).to_string(),
                },
            )?;

            CorrectAnswer::create(
                &mut conn,
                &CreateCorrectAnswer {
                    lobby_id: lobby.id.clone(),
                    question_id: question.id,
                    answer_choice_id: answer_choice.id,
                },
            )?;
        }

        Ok::<Lobby, diesel::result::Error>(lobby)
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
            use crate::schema::question::dsl::*;

            println!(
                "{}",
                debug_query::<Pg, _>(
                    &diesel::update(
                        question.filter(
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
                question.filter(
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
            .get_result::<Question>(&mut conn)?;
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
