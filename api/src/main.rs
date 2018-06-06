#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate sudoku_backend;
extern crate rocket;

use sudoku_backend::{ops, Options};
// use std::sync::Mutex;


fn main() {
    let opts = Options::parse();
    println!("{:#?}", opts);

    rocket::ignite()
        .manage(ops::setup::DatabaseConnection::initialise(&opts.database_file))
        .catch(catchers![ops::routes::catchers::not_found, ops::routes::catchers::internal_server_error])
        .launch();
}
