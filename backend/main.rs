extern crate diesel;

// use actix_files::Files;
use actix_web::middleware::{Compress, Logger, NormalizePath};
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use diesel::connection::SimpleConnection;

// use crate::models::lobby::{CreateLobby, Lobby};

mod mail;
mod models;
mod schema;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    create_rust_app::setup_development().await;
    let app_data = create_rust_app::setup();

    let mut conn = app_data.database.get_connection();

    conn.batch_execute(&std::fs::read_to_string("db/data.sql")?)
        .unwrap();

    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(Compress::default())
            .wrap(NormalizePath::trim())
            .wrap(Logger::default());

        app = app.app_data(Data::new(app_data.database.clone()));
        app = app.app_data(Data::new(app_data.mailer.clone()));

        let mut api_scope = web::scope("/api");
        api_scope = api_scope.service(services::answer_choice::endpoints(web::scope(
            "/answer_choice",
        )));
        api_scope = api_scope.service(services::correct_answer::endpoints(web::scope(
            "/correct_answer",
        )));
        api_scope = api_scope.service(services::git_guessr_game_format_config::endpoints(
            web::scope("/git_guessr_game_format_config"),
        ));
        api_scope = api_scope.service(services::lobby::endpoints(web::scope("/lobby")));
        api_scope = api_scope.service(services::lobby_participant::endpoints(web::scope(
            "/lobby_participant",
        )));
        api_scope = api_scope.service(services::obfuscated_game_format_config::endpoints(
            web::scope("/obfuscated_game_format_config"),
        ));
        api_scope = api_scope.service(services::question::endpoints(web::scope("/question")));
        api_scope = api_scope.service(services::repository::endpoints(web::scope("/repository")));
        api_scope = api_scope.service(services::todo::endpoints(web::scope("/todos")));
        api_scope = api_scope.service(services::user_answer::endpoints(web::scope(
            "/user_answers",
        )));
        api_scope = api_scope.service(gitguessr_auth::endpoints(web::scope("/auth")));

        // #[cfg(debug_assertions)]
        // {
        //     /* Development-only routes */
        //     // Mount development-only API routes
        //     api_scope = api_scope.service(create_rust_app::dev::endpoints(web::scope("/development")));
        //     // Mount the admin dashboard on /admin
        //     app = app.service(web::scope("/admin").service(Files::new("/", ".cargo/admin/dist/").index_file("admin.html")));
        // }

        app = app.service(api_scope);
        app = app.default_service(web::get().to(create_rust_app::render_views));
        app
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
