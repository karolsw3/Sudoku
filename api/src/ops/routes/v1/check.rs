//! Data retrieval related routes.
//!
//! To be mounted on "/api/v1/check".


use self::super::super::super::errors::{GenericErrorSeverity, GenericError};
use self::super::super::super::setup::DatabaseConnection;
use self::super::super::super::SudokuSolution;
use rocket::response::status::Custom;
use rocket_contrib::Json;
use rocket::http::Status;


/// Get top ten scores
///
/// TODO: advanced filtering
#[get("/leaderboard")]
pub fn leaderboard(db: DatabaseConnection) -> Result<Json<Vec<SudokuSolution>>, Custom<Json<GenericError>>> {
    SudokuSolution::leaders(10, &db).map(Json).map_err(|e| Custom(Status::InternalServerError, Json((e, GenericErrorSeverity::Danger).into())))
}
