use odict::Error;

pub fn cast_error(e: Error) -> napi::Error {
    napi::Error::new(napi::Status::GenericFailure, format!("{}", e))
}
