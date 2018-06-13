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
        .mount("/api/v1/auth",
               routes![ops::routes::v1::auth::login,
                       ops::routes::v1::auth::logout,
                       ops::routes::v1::auth::create_account])
        .mount("/api/v1/play",
               routes![ops::routes::v1::play::new,
                       ops::routes::v1::play::replay])
        .launch();
}
