#![feature(box_syntax, proc_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

extern crate dotenv;
extern crate iron;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_diesel;

// Must be public for `cargo doc`.
pub mod schema;

mod entity;
mod service;

pub use entity::*;
pub use service::*;
