mod ota;
mod model;

pub mod setup;
pub mod routes;
pub mod tables;
pub mod constraints;

pub use self::model::{BoardDifficulty, SudokuBoard, Session, User};
pub use self::ota::{NewBoardRequestForm, OldBoardRequestForm, SanitisedUserData, BoardMessage, LoginForm};

pub mod errors {
    //! Various API errors.

    pub use self::super::ota::{GenericErrorSeverity, GenericError, LoginError};
}
