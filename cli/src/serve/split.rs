use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    web::{Data, Path, Query},
    HttpResponse, Responder, ResponseError,
};
use derive_more::{Display, Error};
use odict::{format::json::ToJSON, split::SplitOptions};
use serde::Deserialize;

use crate::get_lookup_entries;

#[derive(Debug, Deserialize)]
pub struct SplitRequest {
    q: String,
    follow: Option<bool>,
    min_length: Option<usize>,
}

#[derive(Debug, Display, Error)]
enum SplitError {
    #[display("Dictionary not found: {}", name)]
    DictionaryNotFound { name: String },

    #[display("Failed to read dictionary: {}", name)]
    DictionaryReadError { name: String },

    #[display("Split error: {}", message)]
    SplitError { message: String },

    #[display("Failed to serialize response")]
    SerializeError,
}

impl ResponseError for SplitError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            SplitError::DictionaryNotFound { .. } => StatusCode::NOT_FOUND,
            SplitError::DictionaryReadError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            SplitError::SplitError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            SplitError::SerializeError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[get("/{name}/split")]
async fn handle_split(
    params: Query<SplitRequest>,
    dict: Path<String>,
    dictionary_cache: Data<crate::serve::DictionaryCache>,
) -> Result<impl Responder, SplitError> {
    let SplitRequest {
        q: raw_queries,
        follow,
        min_length,
    } = params.0;

    let queries = raw_queries
        .split(',')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let dictionary_name = dict.into_inner();

    let file = dictionary_cache
        .get(&dictionary_name)
        .await
        .map_err(|_e| SplitError::DictionaryReadError {
            name: dictionary_name.to_string(),
        })?
        .ok_or(SplitError::DictionaryNotFound {
            name: dictionary_name.to_string(),
        })?;

    let dictionary = file
        .contents()
        .map_err(|_e| SplitError::DictionaryReadError {
            name: dictionary_name.to_string(),
        })?;

    let opts = SplitOptions::default()
        .threshold(min_length.unwrap_or(1))
        .follow(follow.unwrap_or(false));

    let entries = dictionary
        .split(&queries, &opts)
        .map_err(|e| SplitError::SplitError {
            message: e.to_string(),
        })?;

    let json = get_lookup_entries(entries)
        .to_json(true)
        .map_err(|_e| SplitError::SerializeError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json))
}
