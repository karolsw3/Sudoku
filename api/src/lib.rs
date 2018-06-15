#![feature(plugin)]
#![plugin(rocket_codegen)]

// `diesel` will generate these erorrs no mater how hard we try, so
#![allow(proc_macro_derive_resolution_fallback)]

extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate base64;
extern crate chrono;
extern crate crypto;
extern crate rocket;
extern crate sudoku;
#[macro_use]
extern crate diesel;
extern crate serde;
#[macro_use]
extern crate clap;
extern crate time;
extern crate rand;

mod options;

pub mod doc;
pub mod ops;
pub mod util;

pub use options::Options;
