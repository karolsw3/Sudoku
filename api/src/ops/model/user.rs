use diesel::expression_methods::ExpressionMethods;
use self::super::super::tables::{self, users};
use diesel::query_dsl::methods::FilterDsl;
use diesel::sqlite::SqliteConnection;
use diesel::query_dsl::RunQueryDsl;
use chrono::{NaiveDateTime, Utc};
use diesel;


/// Refer to [`doc/user.md`](../doc/user/) for more details.
#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[table_name="users"]
pub struct User {
    /// Unique user ID.
    ///
    /// Actually not optional, but this allows us to get an ID from the database.
    pub id: Option<i32>,

    /// User's name or "login" or whatever.
    pub username: String,

    /// Doubly scrypted password, see `doc/user.md` for details.
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

    /// Pull out a user with the specified ID from the specified database.
    pub fn get_by_id(id: i32, db: &SqliteConnection) -> Result<User, &'static str> {
        tables::users::table.filter(tables::users::id.eq(&id)).first(db).map_err(|_| "couldn't get user")
    }

    /// Insert this user into the specified database, if possible.
    pub fn add(&self, db: &SqliteConnection) -> Result<(), &'static str> {
        diesel::insert_into(tables::users::table).values(self).execute(db).map(|_| ()).map_err(|_| "insert failed")
    }
}
