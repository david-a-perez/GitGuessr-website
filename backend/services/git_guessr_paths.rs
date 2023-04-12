use actix_web::{
    error::ErrorInternalServerError,
    get,
    web::{Data, Path, Query},
    HttpResponse, Responder,
};
use create_rust_app::Database;
use thiserror::Error;

use crate::{
    gitguessr::{get_paths_at_path, GitGuessrError},
    models::{lobby::Lobby, repository::Repository},
};

#[tsync::tsync]
#[derive(serde::Deserialize)]
pub struct PaginationParams {
    pub page: i64,
    pub page_size: i64,
}

#[tsync::tsync]
#[derive(serde::Serialize)]
pub struct Entry {
    pub is_directory: bool,
    pub filename: String,
}

#[tsync::tsync]
#[derive(serde::Serialize)]
pub struct Directory {
    pub entries: Vec<Entry>,
}

#[derive(Error, Debug)]
enum PathError {
    #[error("GitGuessr Error: {0}")]
    GitGuessr(#[from] crate::gitguessr::GitGuessrError),

    #[error("Diesel Error: {0}")]
    Diesel(#[from] diesel::result::Error),
}

#[tsync::tsync]
#[derive(serde::Deserialize)]
pub struct PathQuery {
    pub path: String,
}

#[get("/{lobby_id}")]
async fn read(db: Data<Database>, params: Path<String>, Query(info): Query<PathQuery>) -> impl Responder {
    actix_web::web::block(move || {
        let mut conn = db.get_connection();

        let lobby = Lobby::read(&mut conn, params.clone())?;

        let repository = Repository::read(&mut conn, lobby.repository_id.clone())?;

        let repo = gix::open(repository.filename).map_err(GitGuessrError::from)?;

        let entries = get_paths_at_path(&repo, &info.path)?
            .iter()
            .map(|entry| -> Result<Entry, GitGuessrError> {
                let entry = entry?;
                Ok(Entry {
                    is_directory: entry.mode().is_tree(),
                    filename: entry.filename().to_string(),
                })
            })
            .collect::<Result<Vec<Entry>, GitGuessrError>>()?;
        Ok::<Directory, PathError>(Directory { entries })
    })
    .await
    .map(|result| match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(err) => Err(ErrorInternalServerError(err)),
    })
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    scope.service(read)
}
