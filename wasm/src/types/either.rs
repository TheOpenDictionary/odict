use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub enum Either<T, U> {
    Left(T),
    Right(U),
}
