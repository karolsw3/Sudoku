mod ota;
mod model;

pub mod setup;
pub mod routes;
pub mod tables;
pub mod constraints;

pub use self::model::{BoardDifficulty, SudokuSolution, SudokuBoard, Session, User};
pub use self::ota::{NewBoardRequestForm, OldBoardRequestForm, SanitisedUserData, SolutionOrdering, BoardMessage, LoginForm};

pub mod errors {
    //! Various API errors.

    pub use self::super::ota::{GenericErrorSeverity, GenericError, LoginError};
}
