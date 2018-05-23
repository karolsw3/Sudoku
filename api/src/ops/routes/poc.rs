//! To be mounted on "/sudoku/api/poc".


use rocket::http::Cookies;


/// Get a list of all cookies
#[get("/ping")]
pub fn ping(cookies: Cookies) -> String {
    format!("{:#?}", cookies)
}
