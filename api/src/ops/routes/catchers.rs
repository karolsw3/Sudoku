//! Handlers that catch various error states.


use self::super::super::errors::{GenericErrorSeverity, GenericError};
use rocket_contrib::Json;


lazy_static! {
    /// Lazy error value for 404s
    pub static ref RESPONSE_404: GenericError = GenericError {
        reason: "endpoint does not exist".into(),
        severity: GenericErrorSeverity::Warning,
    };

    /// Lazy error value for 500s
    pub static ref RESPONSE_500: GenericError = GenericError {
        reason: "critical internal server error".into(),
        severity: GenericErrorSeverity::Danger,
    };
}


/// Handler for 404s, returning [`RESPONSE_404`](struct.RESPONSE_404.html).
#[catch(404)]
pub fn not_found() -> Json<&'static GenericError> {
    Json(&*RESPONSE_404)
}

/// Handler for 500s, returning [`RESPONSE_500`](struct.RESPONSE_500.html).
#[catch(500)]
pub fn internal_server_error() -> Json<&'static GenericError> {
    Json(&*RESPONSE_500)
}
