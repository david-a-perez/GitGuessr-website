use actix_web::{delete, Error, get, HttpResponse, post, put, Result, web::{Data, Json, Path, Query}};
use create_rust_app::Database;
use crate::models::repository::{CreateRepository, Repository, UpdateRepository};

#[tsync::tsync]
#[derive(serde::Deserialize)]
pub struct PaginationParams {
    pub page: i64,
    pub page_size: i64,
}

#[get("")]
async fn index(
    db: Data<Database>,
    Query(info): Query<PaginationParams>,
) -> HttpResponse {
    let mut con = db.get_connection();

    let result = Repository::paginate(&mut con, info.page, info.page_size);

    if result.is_ok() {
        HttpResponse::Ok().json(result.unwrap())
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[get("/{id}")]
async fn read(
    db: Data<Database>,
    item_id: Path<String>,
) -> HttpResponse {
    let mut con = db.get_connection();

    let result = Repository::read(&mut con, item_id.into_inner());

    if result.is_ok() {
        let todo = result.unwrap();

        HttpResponse::Ok().json(todo)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[post("")]
async fn create(
    db: Data<Database>,
    Json(item): Json<CreateRepository>,
) -> Result<HttpResponse, Error> {
    let mut con = db.get_connection();

    let result = Repository::create(&mut con, &item).expect("Creation error");

    Ok(HttpResponse::Created().json(result))
}

#[put("/{id}")]
async fn update(
    db: Data<Database>,
    item_id: Path<String>,
    Json(item): Json<UpdateRepository>,
) -> HttpResponse {
    let mut con = db.get_connection();

    let result = Repository::update(&mut con, item_id.into_inner(), &item);

    if result.is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[delete("/{id}")]
async fn destroy(
    db: Data<Database>,
    item_id: Path<String>,
) -> HttpResponse {
    let mut con = db.get_connection();

    let result = Repository::delete(&mut con, item_id.into_inner());

    if result.is_ok() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(index)
        .service(read)
        .service(create)
        .service(update)
        .service(destroy);
}
