use self::super::super::tables::users;
use chrono::{NaiveDateTime, Utc};


/// Refer to `doc/user.md` for more details.
#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[table_name="users"]
pub struct User {
    /// Unique user ID.
    ///
    /// Actually not optional, but this allows us to get an ID from the database.
    pub id: Option<i32>,

    /// User's name or "login" or whatever.
    pub username: String,

    /// Doubly scrypted password, see doc/user.md for details.
    pub password: String,

    /// User's contact e-mail
    pub email: String,

    /// Time user was created.
    pub created_at: NaiveDateTime,

    /// Whether the user has administrative privileges.
    pub is_admin: bool,

    /// Sum total of the user's points, calculated according to `doc/scoring.md#endgame-formula`, `CHECK`ed to nonnegativity.
    pub points_total: i32,
}

impl User {
    /// Create a defaulted user with the specified parameters.
    pub fn new<U: Into<String>, P: Into<String>, E: Into<String>>(username: U, password: P, email: E) -> User {
        User::new_impl(username.into(), password.into(), email.into())
    }

    fn new_impl(username: String, password: String, email: String) -> User {
        User {
            id: None,
            username: username,
            password: password,
            email: email,
            created_at: NaiveDateTime::from_timestamp(Utc::now().naive_utc().timestamp(), 0),
            is_admin: false,
            points_total: 0,
        }
    }
}
