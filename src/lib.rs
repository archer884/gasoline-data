#![feature(box_syntax, proc_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

extern crate dotenv;
extern crate r2d2_diesel;
extern crate r2d2;

mod entity;
mod service;

pub use entity::*;
pub use service::*;

// Must be public for `cargo doc`.
pub mod schema {
    infer_schema!("dotenv:DATABASE_URL");
}
