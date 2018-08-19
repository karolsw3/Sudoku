#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate sudoku_backend;
extern crate rocket;
extern crate toml;

use sudoku_backend::{ops, Options};


fn main() {
    let opts = Options::parse();
    println!("{:#?}", opts);

    rocket::ignite()
        .manage(ops::setup::DatabaseConnection::initialise(&opts.database_file))
        .manage({
            let leaderboard_settings = opts.leaderboard_settings_file
                .as_ref()
                .and_then(|f| ops::setup::LeaderboardSettings::load(f).map_err(|e| eprintln!("{}\nReverting to defaults.", e)).ok())
                .unwrap_or_else(|| {
                    ops::setup::LeaderboardSettings {
                        default: ops::setup::LeaderboardConfig::DEFAULT_DEFAULT,
                        max: ops::setup::LeaderboardConfig::DEFAULT_MAX,
                    }
                });
            println!("Leaderboard settings:\n{}",
                     toml::to_string_pretty(&leaderboard_settings).expect("leaderboard settings reserialisation error"));
            leaderboard_settings
        })
        .catch(catchers![ops::routes::catchers::not_found,
                         ops::routes::catchers::internal_server_error])
        .mount("/api/v1/auth",
               routes![ops::routes::v1::auth::login,
                       ops::routes::v1::auth::logout,
                       ops::routes::v1::auth::create_account])
        .mount("/api/v1/play",
               routes![ops::routes::v1::play::new,
                       ops::routes::v1::play::replay,
                       ops::routes::v1::play::submit])
        .mount("/api/v1/check",
               routes![ops::routes::v1::check::leaderboard_specless,
                       ops::routes::v1::check::leaderboard])
        .launch();
}
