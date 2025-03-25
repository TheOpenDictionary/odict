use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Either<T, U> {
    Left(T),
    Right(U),
}
