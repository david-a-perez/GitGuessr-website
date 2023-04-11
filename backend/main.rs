extern crate diesel;

// use actix_files::Files;
use actix_web::middleware::{Compress, Logger, NormalizePath};
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use diesel::connection::SimpleConnection;
use gitguessr_auth::middleware::RequireAuth;

mod gitguessr;
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

        let api_scope = web::scope("/api")
            .wrap(RequireAuth)
            .service(services::git_guessr_correct_answer::endpoints(web::scope(
                "/git_guessr_correct_answer",
            )))
            .service(services::git_guessr_game_format_config::endpoints(
                web::scope("/git_guessr_game_format_config"),
            ))
            .service(services::git_guessr_paths::endpoints(web::scope(
                "/git_guessr_paths",
            )))
            .service(services::git_guessr_question::endpoints(web::scope(
                "/git_guessr_question",
            )))
            .service(services::git_guessr_user_answer::endpoints(web::scope(
                "/git_guessr_user_answer",
            )))
            .service(services::lobby::endpoints(web::scope("/lobby")))
            .service(services::lobby_participant::endpoints(web::scope(
                "/lobby_participant",
            )))
            .service(services::obfuscated_answer_choice::endpoints(web::scope(
                "/obfuscated_answer_choice",
            )))
            .service(services::obfuscated_correct_answer::endpoints(web::scope(
                "/obfuscated_correct_answer",
            )))
            .service(services::obfuscated_game_format_config::endpoints(
                web::scope("/obfuscated_game_format_config"),
            ))
            .service(services::obfuscated_question::endpoints(web::scope(
                "/obfuscated_question",
            )))
            .service(services::obfuscated_user_answer::endpoints(web::scope(
                "/obfuscated_user_answer",
            )))
            .service(services::repository::endpoints(web::scope("/repository")))
            .service(services::todo::endpoints(web::scope("/todos")));

        let auth_scope =
            web::scope("/auth_api").service(gitguessr_auth::endpoints(web::scope("/auth")));

        // #[cfg(debug_assertions)]
        // {
        //     /* Development-only routes */
        //     // Mount development-only API routes
        //     api_scope = api_scope.service(create_rust_app::dev::endpoints(web::scope("/development")));
        //     // Mount the admin dashboard on /admin
        //     app = app.service(web::scope("/admin").service(Files::new("/", ".cargo/admin/dist/").index_file("admin.html")));
        // }

        app = app.service(api_scope);
        app = app.service(auth_scope);
        app = app.default_service(web::get().to(create_rust_app::render_views));
        app
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
