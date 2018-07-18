//! Auth-related routes.
//!
//! To be mounted on "/api/v1/play".
//!
//! Refer to [doc/sudoku.md](../../../doc/sudoku/) for the specifics.


use self::super::super::super::{NewBoardRequestForm, OldBoardRequestForm, BoardDifficulty, SudokuSolution, BoardMessage, SudokuBoard, Session, User};
use self::super::super::super::super::util::{random_username, board_includes};
use self::super::super::super::errors::{GenericErrorSeverity, GenericError};
use self::super::super::super::setup::{DatabaseConnection, ActivityCache};
use rocket::response::status::Custom;
use rocket::http::{Cookies, Status};
use rocket::request::{State, Form};
use chrono::{DateTime, Utc};
use rocket_contrib::Json;
use std::borrow::Cow;
use sudoku::Sudoku;


/// Get a new board of the specified difficulty and update the session.
#[get("/new?<difficulty>")]
pub fn new(db: DatabaseConnection, ac: State<ActivityCache>, mut cookies: Cookies, difficulty: NewBoardRequestForm)
           -> Result<Custom<Json<BoardMessage>>, Custom<Json<GenericError>>> {
    let mut session = Session::get(&db, &mut cookies).map_err(|e| Custom(Status::InternalServerError, Json((e, GenericErrorSeverity::Danger).into())))?;
    ac.register_activity(session.id.unwrap() as usize);
    let difficulty = difficulty.difficulty
        .map_err(|e| {
            Custom(Status::BadRequest,
                   Json((e.map(Cow::Owned).unwrap_or_else(|| "specified difficulty out of domain".into()), GenericErrorSeverity::Warning).into()))
        })?;

    let mut board = SudokuBoard::new(difficulty);
    board.insert(&db).map_err(|e| Custom(Status::InternalServerError, Json((e, GenericErrorSeverity::Danger).into())))?;

    let skeleton = board.generate_skeleton().map_err(|e| Custom(Status::InternalServerError, Json((e.to_string(), GenericErrorSeverity::Danger).into())))?;
    session.start_game(board.id
                        .ok_or_else(|| {
                            Custom(Status::InternalServerError,
                                   Json(("missing roundtripped ID", GenericErrorSeverity::Danger).into()))
                        })?, // Should never happen since the insert would fail first, but y'know
                    skeleton.clone(),
                    &db)
        .map_err(|e| Custom(Status::InternalServerError, Json((e, GenericErrorSeverity::Danger).into())))?;

    Ok(Custom(Status::Created,
              Json(BoardMessage {
                  board_id: board.id.unwrap(), // Verified above
                  board_skeleton: skeleton.into(),
                  solved_board: None,
              })))
}

/// Get the specified board and update the session.
#[get("/replay?<board_id>")]
pub fn replay(db: DatabaseConnection, ac: State<ActivityCache>, mut cookies: Cookies, board_id: OldBoardRequestForm)
              -> Result<Custom<Json<BoardMessage>>, Custom<Json<GenericError>>> {
    let mut session = Session::get(&db, &mut cookies).map_err(|e| Custom(Status::InternalServerError, Json((e, GenericErrorSeverity::Danger).into())))?;
    ac.register_activity(session.id.unwrap() as usize);
    let board_id = board_id.board_id
        .map_err(|e| {
            Custom(Status::BadRequest,
                   Json((e.map(Cow::Owned).unwrap_or_else(|| "specified board_id nonpositive".into()), GenericErrorSeverity::Warning).into()))
        })?
        .0;

    let board = SudokuBoard::get(board_id, &db).map_err(|e| Custom(Status::InternalServerError, Json((e, GenericErrorSeverity::Danger).into())))?;
    let skeleton = board.generate_skeleton().map_err(|e| Custom(Status::InternalServerError, Json((e.to_string(), GenericErrorSeverity::Danger).into())))?;
    session.start_game(board.id.unwrap(), skeleton.clone(), &db) // Verified above
        .map_err(|e| Custom(Status::InternalServerError, Json((e, GenericErrorSeverity::Danger).into())))?;

    Ok(Custom(Status::Created,
              Json(BoardMessage {
                  board_id: board.id.unwrap(), // Verified above
                  board_skeleton: skeleton.into(),
                  solved_board: None,
              })))
}

/// Submit a completed board and update user/leaderboard
#[post("/submit", data="<submitted_board>")]
pub fn submit(db: DatabaseConnection, ac: State<ActivityCache>, mut cookies: Cookies, submitted_board: Form<BoardMessage>)
              -> Result<Json<SudokuSolution>, Custom<Json<GenericError>>> {
    let mut session = Session::get(&db, &mut cookies).map_err(|e| Custom(Status::InternalServerError, Json((e, GenericErrorSeverity::Danger).into())))?;
    ac.register_activity(session.id.unwrap() as usize);

    let submitted_board = submitted_board.into_inner();
    if submitted_board.solved_board.is_none() {
        return Err(Custom(Status::BadRequest,
                          Json(("solved_board field missing in submission form", GenericErrorSeverity::Warning).into())));
    }

    if !session.game_started() {
        return Err(Custom(Status::PreconditionFailed, Json(("no game in progress", GenericErrorSeverity::Warning).into())));
    }

    if !(session.board_skeleton.as_ref().unwrap() == &submitted_board.board_skeleton &&
         board_includes(submitted_board.solved_board.as_ref().unwrap(), &submitted_board.board_skeleton) &&
         Sudoku::from_str_line(submitted_board.solved_board.as_ref().unwrap())
        .map_err(|e| Custom(Status::InternalServerError, Json((e.to_string(), GenericErrorSeverity::Danger).into())))?
        .is_solved()) {
        return Err(Custom(Status::PreconditionFailed, Json(("solution invalid", GenericErrorSeverity::Warning).into())));
    }

    let mut user = if let Some(uid) = session.user_id {
        Some(User::get_by_id(uid, &db).map_err(|e| Custom(Status::InternalServerError, Json((e.to_string(), GenericErrorSeverity::Danger).into())))?)
    } else {
        None
    };

    let mut solution = SudokuSolution::new(user.as_ref().map(|u| u.username.clone()).unwrap_or_else(random_username),
                                           submitted_board.board_skeleton,
                                           &SudokuBoard::get(submitted_board.board_id, &db)
                                               .map_err(|e| Custom(Status::InternalServerError, Json((e, GenericErrorSeverity::Danger).into())))?,
                                           &(Utc::now() - DateTime::from_utc(session.solve_start.unwrap(), Utc)))
        .ok_or_else(|| Custom(Status::InternalServerError, Json(("could not create solution", GenericErrorSeverity::Danger).into())))?;

    if let Some(ref mut user) = user {
        user.solve(solution.score as usize,
                   BoardDifficulty::from_numeric(solution.difficulty as u64).ok_or_else(|| {
                           Custom(Status::InternalServerError,
                                  Json(("invalid board difficulty", GenericErrorSeverity::Danger).into()))
                       })?,
                   &db)
            .map_err(|e| Custom(Status::InternalServerError, Json((e, GenericErrorSeverity::Danger).into())))?;
    }

    solution.insert(&db).map_err(|e| Custom(Status::InternalServerError, Json((e, GenericErrorSeverity::Danger).into())))?;
    session.stop_game(&db).map_err(|e| Custom(Status::InternalServerError, Json((e, GenericErrorSeverity::Danger).into())))?;

    Ok(Json(solution))
}
