// #![feature(plugin)]
// #![plugin(rocket_codegen)]

extern crate sudoku_backend;
// extern crate rocket;

// use sudoku_backend::{ops, Options};
use sudoku_backend::Options;
// use std::sync::Mutex;


fn main() {
    let opts = Options::parse();
    println!("{:#?}", opts);

    // rocket::ignite()
    //     .manage(ops::setup::database_connection(&opts.database_file))
    //     .mount("/",
    //            routes![ops::routes::assets::generic,
    //                    ops::routes::assets::favicon])
    //     .launch();
}
