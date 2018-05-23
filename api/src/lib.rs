#![feature(plugin)]
#![plugin(rocket_codegen)]

// #[macro_use]
// extern crate diesel_codegen;
extern crate r2d2_diesel;
// #[macro_use]
extern crate rocket;
extern crate diesel;
#[macro_use]
extern crate clap;
extern crate r2d2;

mod options;

pub mod ops;
pub mod util;

pub use options::Options;
