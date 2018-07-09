use diesel::{self, ExpressionMethods, ExecuteDsl, FilterDsl};
use chrono::{NaiveDateTime, Duration, Utc};
use self::super::super::tables::sessions;
use diesel::sqlite::SqliteConnection;


/// Refer to `doc/session.md` for more details.
///
/// TODO: doc full workflow with creation, database access, etc.
#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[changeset_options(treat_none_as_null="true")]
#[table_name="sessions"]
pub struct Session {
    /// Unique user ID.
    ///
    /// Actually not optional, but this allows us to get an ID from the database.
    pub id: Option<i32>,

    /// Time the cookie expires, defaults to a day from creation time.
    pub expiry: NaiveDateTime,

    /// Whether the user is logged in as admin (always `false` if `user_id == None`).
    pub is_admin: bool,

    /// The logged-in user ID, or `None` for unauthed access.
    pub user_id: Option<i32>,

    // pub product_id: Option<i32>,
}

impl Session {
    /// Create an empty session expiring a day from the creation datetime.
    ///
    /// Put it in the database to get an ID to put back in the cookie.
    pub fn new() -> Session {
        Session {
            id: None,
            expiry: NaiveDateTime::from_timestamp((Utc::now() + Duration::days(1)).naive_utc().timestamp(), 0),
            is_admin: false,
            user_id: None,
            // product_id: None,
        }
    }

    // pub fn set_product(&mut self, pid: i32, db: &SqliteConnection) -> Result<(), &'static str> {
    // self.product_id = Some(pid);
    // diesel::update(sessions::table.filter(sessions::id.eq(self.id.unwrap()))).set(&*self).execute(db).map(|_| ()).map_err(|_|
    // "update failed")
    // }

    /// Update the in-memory and in-DB model to be logged in a the specified user with the specified permissions.
    ///
    /// # Examples
    ///
    /// Before:
    ///
    /// ```sql
    /// INSERT INTO "sessions" VALUES(32, '2018-07-09 12:40:26', 0, NULL);
    /// ```
    ///
    /// Update:
    ///
    /// ```
    /// # extern crate sudoku_backend;
    /// # extern crate chrono;
    /// # use sudoku_backend::ops::setup::DatabaseConnection;
    /// # use sudoku_backend::ops::Session;
    /// # use std::env::temp_dir;
    /// # use chrono::NaiveDate;
    /// # use std::fs;
    /// # let database_file =
    /// #    ("$ROOT/sudoku-backend.db".to_string(),
    /// #     temp_dir().join("sudoku-backend-doctest").join("ops-model-session-Session-log_in").join("sudoku-backend.db"));
    /// # fs::create_dir_all(database_file.1.parent().unwrap()).unwrap();
    /// # let db = DatabaseConnection::initialise(&database_file);
    /// # let db = &db.get().unwrap();
    /// # // TODO: actually insert the bloody thing first.
    /// let mut sess = Session {
    ///     id: Some(32),
    ///     expiry: NaiveDate::from_ymd(2018, 7, 9).and_hms(12, 40, 26),
    ///     is_admin: false,
    ///     user_id: None,
    /// };
    ///
    /// sess.log_in(18, true, &db).unwrap();
    ///
    /// assert_eq!(sess, Session {
    ///     id: Some(32),
    ///     expiry: NaiveDate::from_ymd(2018, 7, 9).and_hms(12, 40, 26),
    ///     is_admin: true,
    ///     user_id: Some(18),
    /// });
    /// ```
    ///
    /// After:
    ///
    /// ```sql
    /// INSERT INTO "sessions" VALUES(32, '2018-07-09 12:40:26', 1, 18);
    /// ```
    pub fn log_in(&mut self, uid: i32, is_admin: bool, db: &SqliteConnection) -> Result<(), &'static str> {
        self.user_id = Some(uid);
        self.is_admin = is_admin;
        diesel::update(sessions::table.filter(sessions::id.eq(self.id.unwrap()))).set(&*self).execute(db).map(|_| ()).map_err(|_| "update failed")
    }

    /// Update the in-memory and in-DB model to not be logged in nor be admin.
    ///
    /// # Examples
    ///
    /// Before:
    ///
    /// ```sql
    /// INSERT INTO "sessions" VALUES(32, '2018-07-09 12:40:26', 1, 18);
    /// ```
    ///
    /// Update:
    ///
    /// ```
    /// # extern crate sudoku_backend;
    /// # extern crate chrono;
    /// # use sudoku_backend::ops::setup::DatabaseConnection;
    /// # use sudoku_backend::ops::Session;
    /// # use std::env::temp_dir;
    /// # use chrono::NaiveDate;
    /// # use std::fs;
    /// # let database_file =
    /// #    ("$ROOT/sudoku-backend.db".to_string(),
    /// #     temp_dir().join("sudoku-backend-doctest").join("ops-model-session-Session-log_out").join("sudoku-backend.db"));
    /// # fs::create_dir_all(database_file.1.parent().unwrap()).unwrap();
    /// # let db = DatabaseConnection::initialise(&database_file);
    /// # let db = &db.get().unwrap();
    /// # // TODO: actually insert the bloody thing first.
    /// let mut sess = Session {
    ///     id: Some(32),
    ///     expiry: NaiveDate::from_ymd(2018, 7, 9).and_hms(12, 40, 26),
    ///     is_admin: true,
    ///     user_id: Some(18),
    /// };
    ///
    /// sess.log_out(&db).unwrap();
    ///
    /// assert_eq!(sess, Session {
    ///     id: Some(32),
    ///     expiry: NaiveDate::from_ymd(2018, 7, 9).and_hms(12, 40, 26),
    ///     is_admin: false,
    ///     user_id: None,
    /// });
    /// ```
    ///
    /// After:
    ///
    /// ```sql
    /// INSERT INTO "sessions" VALUES(32, '2018-07-09 12:40:26', 0, NULL);
    /// ```
    pub fn log_out(&mut self, db: &SqliteConnection) -> Result<(), &'static str> {
        self.user_id = None;
        self.is_admin = false;
        diesel::update(sessions::table.filter(sessions::id.eq(self.id.unwrap()))).set(&*self).execute(db).map(|_| ()).map_err(|_| "update failed")
    }
}
