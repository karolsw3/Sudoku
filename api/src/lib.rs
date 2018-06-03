#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel_codegen;
// #[macro_use]
// extern crate serde_derive;
extern crate r2d2_diesel;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate base64;
extern crate chrono;
extern crate crypto;
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate clap;
extern crate r2d2;
extern crate time;

mod options;

pub mod ops;
pub mod util;

pub use options::Options;
