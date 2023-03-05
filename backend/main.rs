extern crate diesel;

// use actix_files::Files;
use actix_web::middleware::{Compress, Logger, NormalizePath};
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};

use crate::models::lobby::{CreateLobby, Lobby};
use crate::models::repository::{CreateRepository, Repository};

mod mail;
mod models;
mod schema;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    create_rust_app::setup_development().await;
    let app_data = create_rust_app::setup();
    {
        use crate::diesel::RunQueryDsl;
        use crate::schema::repository::dsl::*;
        let mut conn = app_data.database.get_connection();
        diesel::insert_into(repository)
            .values(CreateRepository {
                name: "GitOxide".to_string(),
                filename: "gitoxide".to_string(),
            })
            .on_conflict(name)
            .do_update()
            .set(CreateRepository {
                name: "GitOxide".to_string(),
                filename: "gitoxide".to_string(),
            })
            .get_results::<Repository>(&mut conn)
            .unwrap();
    }

    {
        use crate::diesel::RunQueryDsl;
        use crate::schema::lobby::dsl::*;
        let mut conn = app_data.database.get_connection();
        diesel::insert_into(lobby)
            .values(CreateLobby {
                repository: "GitOxide".to_string(),
            })
            .get_results::<Lobby>(&mut conn)
            .unwrap();
    }

    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(Compress::default())
            .wrap(NormalizePath::trim())
            .wrap(Logger::default());

        app = app.app_data(Data::new(app_data.database.clone()));
        app = app.app_data(Data::new(app_data.mailer.clone()));

        let mut api_scope = web::scope("/api");
        api_scope = api_scope.service(services::repository::endpoints(web::scope("/repository")));
        api_scope = api_scope.service(services::lobby::endpoints(web::scope("/lobby")));
        api_scope = api_scope.service(gitguessr_auth::endpoints(web::scope("/auth")));
        api_scope = api_scope.service(services::todo::endpoints(web::scope("/todos")));

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
