use std::borrow::Cow;


/// Use this for most non-specific errors.
///
/// Refer to [`doc/errors.md`](../../doc/errors/) for more details.
///
/// # Examples
///
/// ```no_run
/// # #![feature(plugin)]
/// # #![plugin(rocket_codegen)]
/// # extern crate sudoku_backend;
/// # #[macro_use]
/// # extern crate rocket;
/// # use rocket::response::content::Json;
/// # use sudoku_backend::ops::errors::{GenericErrorSeverity, GenericError};
/// # fn work() -> Result<String, &'static str> {
/// #     Err("henlo")
/// # }
/// #[get("/endpoint")]
/// fn endpoint() -> Result<String, Json<GenericError>> {
///     work().map_err(|e| Json(GenericError {
///         reason: format!("couldn't finish work: {}", e).into(),
///         severity: GenericErrorSeverity::Warning,
///     }))
/// }
///
/// fn main() {
///     rocket::ignite()
///         .mount("/", routes![endpoint])
///         .launch();
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct GenericError {
    /// In all-lowercase past-tense finishing-punctuation-free form.
    ///
    /// For example: "failed to apply diff", "user with that name exists".
    pub reason: Cow<'static, str>,

    /// How bad the error is.
    pub severity: GenericErrorSeverity,
}

/// How severe an error is.
///
/// Refer to [`doc/errors.md`](../../doc/errors/) for more details.
#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum GenericErrorSeverity {
    /// Something went wrong, usually user's fault.
    Warning,

    /// Some invariant failed, usually internal DB failure or a malicious API call.
    Danger,
}


impl<R: Into<Cow<'static, str>>> From<(R, GenericErrorSeverity)> for GenericError {
    fn from((r, s): (R, GenericErrorSeverity)) -> GenericError {
        GenericError {
            reason: r.into(),
            severity: s,
        }
    }
}
