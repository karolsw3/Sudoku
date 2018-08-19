//! Data retrieval related routes.
//!
//! To be mounted on "/api/v1/check".


use self::super::super::super::setup::{LeaderboardSettings, DatabaseConnection, LeaderboardConfig};
use self::super::super::super::errors::{GenericErrorSeverity, GenericError};
use self::super::super::super::SudokuSolution;
use rocket::response::status::Custom;
use rocket::request::State;
use rocket_contrib::Json;
use rocket::http::Status;


/// Get default scores
#[get("/leaderboard")]
pub fn leaderboard_specless(db: DatabaseConnection, settings: State<LeaderboardSettings>) -> Result<Json<Vec<SudokuSolution>>, Custom<Json<GenericError>>> {
    leaderboard(db, settings, None)
}

/// Get scores
#[get("/leaderboard?<spec>")]
pub fn leaderboard(db: DatabaseConnection, settings: State<LeaderboardSettings>, spec: Option<LeaderboardConfig>)
                   -> Result<Json<Vec<SudokuSolution>>, Custom<Json<GenericError>>> {
    SudokuSolution::leaders(&if let Some(spec) = spec.as_ref() {
        if spec > &settings.max {
            LeaderboardConfig { count: settings.max.count, ..*spec }
        } else {
            *spec
        }
    } else {
        settings.default
    }, &db).map(Json).map_err(|e| Custom(Status::InternalServerError, Json((e, GenericErrorSeverity::Danger).into())))
}
