mod ota;
mod model;

pub mod setup;
pub mod routes;
pub mod tables;
pub mod constraints;

pub use self::model::{Session, User};
pub use self::ota::{SanitisedUserData, BoardMessage, LoginForm};

pub mod errors {
    //! Various API errors.

    pub use self::super::ota::{GenericErrorSeverity, GenericError, LoginError};
}
