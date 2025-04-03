use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    web::{Data, Path, Query},
    HttpResponse, Responder, ResponseError,
};
use derive_more::{Display, Error};
use odict::{format::json::ToJSON, lookup::TokenizeOptions};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TokenizeRequest {
    text: String,
    follow: Option<bool>,
}

#[derive(Debug, Display, Error)]
enum TokenizeError {
    #[display("Dictionary not found: {}", name)]
    DictionaryNotFound { name: String },

    #[display("Failed to read dictionary: {}", name)]
    DictionaryReadError { name: String },

    #[display("Tokenize error: {}", message)]
    TokenizeError { message: String },

    #[display("Failed to serialize response")]
    SerializeError,
}

impl ResponseError for TokenizeError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            TokenizeError::DictionaryNotFound { .. } => StatusCode::NOT_FOUND,
            TokenizeError::DictionaryReadError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            TokenizeError::TokenizeError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            TokenizeError::SerializeError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[get("/{name}/tokenize")]
async fn handle_tokenize(
    params: Query<TokenizeRequest>,
    dict: Path<String>,
    dictionary_cache: Data<crate::serve::DictionaryCache>,
) -> Result<impl Responder, TokenizeError> {
    let TokenizeRequest { text, follow } = params.0;

    let dictionary_name = dict.into_inner();

    let file = dictionary_cache
        .get(&dictionary_name)
        .map_err(|_e| TokenizeError::DictionaryReadError {
            name: dictionary_name.to_string(),
        })?
        .ok_or(TokenizeError::DictionaryNotFound {
            name: dictionary_name.to_string(),
        })?;

    let dictionary = file
        .to_archive()
        .map_err(|_e| TokenizeError::DictionaryReadError {
            name: dictionary_name.to_string(),
        })?;

    let opts = TokenizeOptions::default().follow(follow.unwrap_or(false));

    let tokens = dictionary
        .tokenize(&text, opts)
        .map_err(|e| TokenizeError::TokenizeError {
            message: e.to_string(),
        })?;

    // Use the ToJSON trait to serialize tokens
    let json = tokens
        .to_json(false)
        .map_err(|_| TokenizeError::SerializeError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json))
}
