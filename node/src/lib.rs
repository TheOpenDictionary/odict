#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[cfg(not(feature = "node"))]
mod browser;

#[cfg(feature = "node")]
mod node;

mod shared;
mod types;
mod utils;
