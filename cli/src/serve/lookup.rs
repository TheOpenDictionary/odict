use std::collections::HashMap;

use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    web::{Data, Path, Query},
    HttpRequest, HttpResponse, Responder, ResponseError,
};
use derive_more::{Display, Error};
use odict::{DictionaryFile, LookupOptions, ToJSON};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LookupRequest {
    queries: Vec<String>,
    follow: Option<bool>,
    split: Option<usize>,
}

#[derive(Debug, Display, Error)]
enum LookupError {
    #[display(fmt = "Dictionary not found: {}", name)]
    DictionaryNotFound { name: String },

    #[display(fmt = "Failed to read dictionary: {}", name)]
    DictionaryReadError { name: String },

    #[display(fmt = "Lookup error: {}", message)]
    LookupError { message: String },
}

impl ResponseError for LookupError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            LookupError::DictionaryNotFound { .. } => StatusCode::NOT_FOUND,
            LookupError::DictionaryReadError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[get("/lookup/{name}")]
async fn handle_lookup(
    req: HttpRequest,
    params: Query<LookupRequest>,
    dict: Path<String>,
    dictionary_map: Data<HashMap<String, DictionaryFile>>,
) -> Result<impl Responder, LookupError> {
    let LookupRequest {
        queries,
        follow,
        split,
    } = params.0;

    let dictionary_name = &dict.into_inner();

    if let Some(file) = dictionary_map.get(dictionary_name) {
        let dictionary = file
            .to_archive()
            .map_err(|_e| LookupError::DictionaryReadError {
                name: dictionary_name.to_string(),
            })?;

        let entries = dictionary
            .lookup(
                &queries,
                LookupOptions::default()
                    .follow(follow.unwrap_or(false))
                    .split(split.unwrap_or(0)),
            )
            .map_err(|e| LookupError::LookupError {
                message: e.to_string(),
            })?;

        entries.to_json(true);

        return Ok(HttpResponse::Ok().body(format!("{:?}", params)));
    } else {
        return Ok(HttpResponse::NotFound().body("Dictionary not found"));
    }
}
