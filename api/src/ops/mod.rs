mod ota;
mod model;

pub mod setup;
pub mod routes;
pub mod tables;

pub use self::model::User;
pub use self::ota::{SanitisedUserData, UserLoginData};
