//! Auth-related routes.
//!
//! To be mounted on "/api/v1/play".
//!
//! Refer to [doc/sudoku.md](../../../doc/sudoku/) for the specifics.


use self::super::super::super::{BoardMessage, BoardRequestForm, SudokuBoard, Session};
use self::super::super::super::errors::{GenericErrorSeverity, GenericError};
use self::super::super::super::setup::DatabaseConnection;
use rocket::response::status::Custom;
use rocket::http::{Cookies, Status};
use rocket_contrib::Json;
use std::borrow::Cow;
use sudoku::Sudoku;


/// Log the user in.
///
/// Check the received form.
/// If login exists and password verifies correct and not a dupe,
/// update session to reflect permissions.
#[get("/new?<difficulty>")]
pub fn new(db: DatabaseConnection, mut cookies: Cookies, difficulty: BoardRequestForm) -> Result<Custom<Json<BoardMessage>>, Custom<Json<GenericError>>> {
    let mut session = Session::get(&db, &mut cookies).map_err(|e| Custom(Status::InternalServerError, Json((e, GenericErrorSeverity::Danger).into())))?;
    let difficulty = difficulty.difficulty
        .map_err(|e| {
            Custom(Status::BadRequest,
                   Json((e.map(Cow::Owned).unwrap_or_else(|| "specified difficulty out of domain".into()), GenericErrorSeverity::Warning).into()))
        })?;

    let mut board = SudokuBoard::new(difficulty);
    board.insert(&db).map_err(|e| Custom(Status::InternalServerError, Json((e, GenericErrorSeverity::Danger).into())))?;

    let skeleton = Sudoku::generate_unique_from(Sudoku::from_str_line(&board.full_board)
        .map_err(|e| Custom(Status::InternalServerError, Json((e.to_string(), GenericErrorSeverity::Danger).into())))?)
        .to_str_line()
        .to_string();
    session.start_game(board.id
                        .ok_or_else(|| {
                            Custom(Status::InternalServerError,
                                   Json(("missing roundtripped ID", GenericErrorSeverity::Danger).into()))
                        })?, // Should never happen since the insert would fail first, but y'know
                    skeleton.clone(),
                    &db)
        .map_err(|e| Custom(Status::InternalServerError, Json((e, GenericErrorSeverity::Danger).into())))?;

    Ok(Custom(Status::Unauthorized,
              Json(BoardMessage {
                  board_id: board.id.unwrap(), // Verified above
                  board_skeleton: skeleton.into(),
                  solved_board: None,
              })))
}
