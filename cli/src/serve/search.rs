use std::collections::HashMap;

use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    web::{Data, Path, Query},
    HttpResponse, Responder, ResponseError,
};
use derive_more::{Display, Error};
use odict::{json::ToJSON, search::SearchOptions, DictionaryFile};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    query: String,
    limit: Option<usize>,
}

#[derive(Debug, Display, Error)]
enum SearchError {
    #[display(fmt = "Dictionary not found: {}", name)]
    DictionaryNotFound { name: String },

    #[display(fmt = "Failed to read dictionary: {}", name)]
    DictionaryReadError { name: String },

    #[display(fmt = "Search error: {}", message)]
    SearchError { message: String },

    #[display(fmt = "Failed to serialize response")]
    SerializeError,
}

impl ResponseError for SearchError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            SearchError::DictionaryNotFound { .. } => StatusCode::NOT_FOUND,
            SearchError::DictionaryReadError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            SearchError::SearchError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            SearchError::SerializeError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[get("/{name}/search")]
async fn handle_search(
    params: Query<SearchRequest>,
    dict: Path<String>,
    dictionary_map: Data<HashMap<String, DictionaryFile>>,
) -> Result<impl Responder, SearchError> {
    let SearchRequest { query, limit } = params.0;

    let dictionary_name = dict.into_inner();

    let file = dictionary_map
        .get(&dictionary_name)
        .ok_or(SearchError::DictionaryNotFound {
            name: dictionary_name.to_string(),
        })?;

    let dictionary = file
        .to_archive()
        .map_err(|_e| SearchError::DictionaryReadError {
            name: dictionary_name.to_string(),
        })?;

    let entries = dictionary
        .search(&query, SearchOptions::default().limit(limit.unwrap_or(10)))
        .map_err(|e| SearchError::SearchError {
            message: e.to_string(),
        })?;

    let json = entries
        .to_json(true)
        .map_err(|_e| SearchError::SerializeError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json))
}

// #[cfg(test)]
// mod tests {
//     use actix_web::{http::header::ContentType, test, App};

//     use crate::serve::get_dictionary_map;

//     use super::handle_search;

//     #[actix_web::test]
//     async fn test_index_get() {
//         let app = test::init_service(App::new().service(handle_search)).await;
//         let req = test::TestRequest::default()
//         // get_dictionary_map(reader, alias_manager, dictionaries)
//             .insert_header(ContentType::plaintext())
//             .to_request();
//         let resp = test::call_service(&app, req).await;
//         assert!(resp.status().is_success());
//     }
// }
