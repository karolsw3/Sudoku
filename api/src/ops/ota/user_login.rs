/// Data requested by client to log a user in.
///
/// Consult `doc/user.md` for more details.
///
/// # Examples
///
/// ```
/// # extern crate sudoku_backend;
/// # extern crate serde_json;
/// # use sudoku_backend::ops::UserLoginData;
/// assert_eq!(serde_json::from_str(r#"{
///                                        "login": "karolsw3",
///                                        "password": "0d0d4a3483976b409ad7 &c."
///                                    }"#).ok(),
///            Some(UserLoginData {
///                login: "karolsw3".to_string(),
///                password: "0d0d4a3483976b409ad7 &c.".to_string(),
///            }));
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct UserLoginData {
    /// User's name or "login" or whatever.
    pub login: String,

    /// Singly `scrypt`ed user password, to be lowercased before re-deriving.
    pub password: String,
}
