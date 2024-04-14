use std::collections::HashMap;

use actix_web::{
    get,
    web::{Path, Query},
    HttpRequest, HttpResponse, Responder,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LookupRequest {
    query: String,
    follow: Option<bool>,
    split: Option<u32>,
}

#[get("/lookup/{name}")]
async fn handle_lookup(
    req: HttpRequest,
    params: Query<LookupRequest>,
    dict: Path<String>,
) -> impl Responder {
    let dictionary_map = req.app_data::<HashMap<String, String>>().unwrap();

    if let Some(dictionary_path) = dictionary_map.get(&dict.into_inner()) {
        // let dictionary =
        return HttpResponse::Ok().body(format!("{:?}", params));
    } else {
        return HttpResponse::NotFound().body("Dictionary not found");
    }
}
