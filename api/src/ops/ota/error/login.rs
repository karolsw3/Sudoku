/// Security ⇒ no data.
///
/// Refer to [`doc/user.md`](../../doc/user/) for more details.
///
/// # Examples
///
/// ```no_run
/// # #![feature(plugin)]
/// # #![plugin(rocket_codegen)]
/// # extern crate sudoku_backend;
/// # #[macro_use]
/// # extern crate rocket;
/// # use sudoku_backend::ops::errors::LoginError;
/// # use rocket::response::content::Json;
/// # fn do_log_in() -> Option<String> {
/// #     None
/// # }
/// #[get("/login")]
/// fn login() -> Result<String, Json<LoginError>> {
///     // Specificities of the log-in process are outside the scope of the document
///     do_log_in().ok_or(Json(LoginError {}))
/// }
///
/// fn main() {
///     rocket::ignite()
///         .mount("/", routes![login])
///         .launch();
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct LoginError {}

impl From<()> for LoginError {
    fn from(_: ()) -> LoginError {
        LoginError {}
    }
}
