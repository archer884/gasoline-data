#![feature(box_syntax, proc_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

extern crate dotenv;
extern crate r2d2_diesel;
extern crate r2d2;

mod entity;
mod service;

pub mod schema;

pub use entity::*;
pub use service::*;
