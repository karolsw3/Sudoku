// #![feature(plugin)]
// #![plugin(rocket_codegen)]

extern crate sudoku_backend;
// extern crate rocket;

// use sudoku_backend::{ops, Options};
// use std::sync::Mutex;


fn main() {
    println!("{} v{} by {}: {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS").replace(':', ", "), env!("CARGO_PKG_DESCRIPTION"));
    println!("{}", sudoku_backend::henlo());

    // let opts = Options::parse();
    // println!("{:#?}", opts);

    // rocket::ignite()
    //     .manage(ops::setup::templates(&opts.asset_dir))
    //     .manage(ops::setup::AssetDir::new(&opts.asset_dir))
    //     .manage(ops::setup::database_connection(&opts.database_file))
    //     .manage(ops::setup::ExternalAddress(opts.external_address))
    //     .mount("/",
    //            routes![ops::routes::assets::generic,
    //                    ops::routes::assets::favicon])
    //     .launch();
}
