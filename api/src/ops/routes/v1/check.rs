//! Data retrieval related routes.
//!
//! To be mounted on "/api/v1/check".


use self::super::super::super::setup::{SpecificLeaderboardConfig, LeaderboardGroupSettings, LeaderboardSettings, DatabaseConnection, LeaderboardConfig,
                                       ActivityCache};
use self::super::super::super::{SanitisedUserData, SudokuSolution, LeaderboardOf, User};
use self::super::super::super::errors::{GenericErrorSeverity, GenericError};
use rocket::response::status::Custom;
use rocket::request::State;
use rocket_contrib::Json;
use rocket::http::Status;


/// Get default scores
#[get("/leaderboard")]
pub fn leaderboard_specless(db: DatabaseConnection, settings: State<LeaderboardSettings>)
                            -> Result<Result<Json<Vec<SudokuSolution>>, Json<Vec<SanitisedUserData>>>, Custom<Json<GenericError>>> {
    leaderboard(db, settings, None)
}

/// Get scores
#[get("/leaderboard?<spec>")]
pub fn leaderboard(db: DatabaseConnection, settings: State<LeaderboardSettings>, spec: Option<SpecificLeaderboardConfig>)
                   -> Result<Result<Json<Vec<SudokuSolution>>, Json<Vec<SanitisedUserData>>>, Custom<Json<GenericError>>> {
    match spec.as_ref().map(|s| s.for_whom).unwrap_or_else(|| LeaderboardOf::Default) {
        LeaderboardOf::Solutions => leaderboard_solutions(db, &settings.board, spec.map(|s| s.cfg)),
        LeaderboardOf::Users => leaderboard_users(db, &settings.person, spec.map(|s| s.cfg)),
    }
}

/// Get active user count
#[get("/active_users")]
pub fn active_users(ac: State<ActivityCache>) -> Json<usize> {
    Json(ac.active_users())
}

fn leaderboard_solutions(db: DatabaseConnection, settings: &LeaderboardGroupSettings, spec: Option<LeaderboardConfig>)
                         -> Result<Result<Json<Vec<SudokuSolution>>, Json<Vec<SanitisedUserData>>>, Custom<Json<GenericError>>> {
    SudokuSolution::leaders(&if let Some(spec) = spec {
                                if spec > settings.max {
                                    LeaderboardConfig { count: settings.max.count, ..spec }
                                } else {
                                    spec
                                }
                            } else {
                                settings.default
                            },
                            &db)
        .map(Json)
        .map(Ok)
        .map_err(|e| Custom(Status::InternalServerError, Json((e, GenericErrorSeverity::Danger).into())))
}


fn leaderboard_users(db: DatabaseConnection, settings: &LeaderboardGroupSettings, spec: Option<LeaderboardConfig>)
                     -> Result<Result<Json<Vec<SudokuSolution>>, Json<Vec<SanitisedUserData>>>, Custom<Json<GenericError>>> {
    User::leaders(&if let Some(spec) = spec {
                      if spec > settings.max {
                          LeaderboardConfig { count: settings.max.count, ..spec }
                      } else {
                          spec
                      }
                  } else {
                      settings.default
                  },
                  &db)
        .map(|ls| ls.into_iter().map(Into::into).collect())
        .map(Json)
        .map(Err)
        .map_err(|e| Custom(Status::InternalServerError, Json((e, GenericErrorSeverity::Danger).into())))
}
