use diesel::query_dsl::methods::{FilterDsl, OrderDsl, FindDsl};
use diesel::expression_methods::ExpressionMethods;
use self::super::super::tables::{self, sessions};
use chrono::{NaiveDateTime, Duration, Utc};
use diesel::sqlite::SqliteConnection;
use rocket::http::{Cookies, Cookie};
use diesel::query_dsl::RunQueryDsl;
use time::{self, Timespec};
use std::str::FromStr;
use std::borrow::Cow;
use diesel;


/// Refer to [`doc/session.md`](../doc/session/) for more details.
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

    /// ID of board currently being solved
    pub sudoku_board_id: Option<i32>,

    /// The board skeleton sent to the user
    pub board_skeleton: Option<Cow<'static, str>>,

    /// Time the solving started
    pub solve_start: Option<NaiveDateTime>,
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
            sudoku_board_id: None,
            board_skeleton: None,
            solve_start: None,
        }
    }

    /// Get/create a session for the specified cookieset.
    pub fn get(db: &SqliteConnection, cookies: &mut Cookies) -> Result<Session, &'static str> {
        if let Some(ssid) = cookies.get_private("session_id") {
            if let Ok(s) = tables::sessions::table.find(i32::from_str(ssid.value()).map_err(|_| "session_id cookie not an int")?).first::<Session>(db) {
                if s.expiry > Utc::now().naive_utc() {
                    return Ok(s);
                } else {
                    cookies.remove_private(ssid);
                }
            }
        }

        let sess = Session::new();
        diesel::insert_into(tables::sessions::table).values(&sess).execute(db).map_err(|_| "couldn't create new session")?;

        // We need to round-trip to get an id
        let sess = tables::sessions::table.filter(tables::sessions::expiry.eq(&sess.expiry))
            .order(tables::sessions::id.desc())
            .first::<Session>(db)
            .map_err(|_| "couldn't re-acquire new session")?;
        cookies.add_private(Cookie::build("session_id", sess.id.unwrap().to_string())
            .expires(time::at_utc(Timespec::new(sess.expiry.timestamp(), sess.expiry.timestamp_subsec_nanos() as i32)))
            .http_only(true)
            .finish());
        Ok(sess)
    }

    /// Update the in-memory and in-DB model to start playing the specified game.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate sudoku_backend;
    /// # extern crate chrono;
    /// # use sudoku_backend::ops::setup::DatabaseConnection;
    /// # use chrono::{NaiveDateTime, NaiveDate, Utc};
    /// # use sudoku_backend::ops::Session;
    /// # use std::env::temp_dir;
    /// # use std::fs;
    /// # let database_file =
    /// #    ("$ROOT/sudoku-backend.db".to_string(),
    /// #     temp_dir().join("sudoku-backend-doctest").join("ops-model-session-Session-log_in").join("sudoku-backend.db"));
    /// # fs::create_dir_all(database_file.1.parent().unwrap()).unwrap();
    /// # let db = DatabaseConnection::initialise(&database_file);
    /// # let db = &db.get().unwrap();
    /// let mut sess = Session {
    ///     id: Some(32),
    ///     expiry: NaiveDate::from_ymd(2018, 7, 9).and_hms(12, 40, 26),
    ///     is_admin: false,
    ///     user_id: None,
    ///     sudoku_board_id: None,
    ///     board_skeleton: None,
    ///     solve_start: None,
    /// };
    ///
    /// let skeleton = "3....5....25.81.........6......4..73.3....14..1.7......8...9.12.97........2..8.54";
    /// sess.start_game(24, skeleton, &db).unwrap();
    ///
    /// assert_eq!(sess, Session {
    ///     id: Some(32),
    ///     expiry: NaiveDate::from_ymd(2018, 7, 9).and_hms(12, 40, 26),
    ///     is_admin: false,
    ///     user_id: None,
    ///     sudoku_board_id: Some(24),
    ///     board_skeleton: Some(skeleton.into()),
    ///     solve_start: Some(NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)),
    /// });
    /// ```
    pub fn start_game<S: Into<Cow<'static, str>>>(&mut self, bid: i32, skeleton: S, db: &SqliteConnection) -> Result<(), &'static str> {
        self.start_game_impl(bid, skeleton.into(), db)
    }

    fn start_game_impl(&mut self, bid: i32, skeleton: Cow<'static, str>, db: &SqliteConnection) -> Result<(), &'static str> {
        self.sudoku_board_id = Some(bid);
        self.board_skeleton = Some(skeleton);
        self.solve_start = Some(NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0));

        diesel::update(sessions::table.filter(sessions::id.eq(self.id.unwrap()))).set(&*self).execute(db).map(|_| ()).map_err(|_| "update failed")
    }

    /// Update the in-memory and in-DB model to start playing the specified game.
    ///
    /// # Examples
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
    /// let mut sess = Session {
    ///     id: Some(32),
    ///     expiry: NaiveDate::from_ymd(2018, 7, 9).and_hms(12, 40, 26),
    ///     is_admin: false,
    ///     user_id: None,
    ///     sudoku_board_id: Some(24),
    ///     board_skeleton: Some("3....5....25.81.........6......4..73.3....14..1.7......8...9.12.97........2..8.54".into()),
    ///     solve_start: Some(NaiveDate::from_ymd(2018, 7, 9).and_hms(2, 41, 27)),
    /// };
    ///
    /// sess.stop_game(&db).unwrap();
    ///
    /// assert_eq!(sess, Session {
    ///     id: Some(32),
    ///     expiry: NaiveDate::from_ymd(2018, 7, 9).and_hms(12, 40, 26),
    ///     is_admin: false,
    ///     user_id: None,
    ///     sudoku_board_id: None,
    ///     board_skeleton: None,
    ///     solve_start: None,
    /// });
    /// ```
    pub fn stop_game(&mut self, db: &SqliteConnection) -> Result<(), &'static str> {
        self.sudoku_board_id = None;
        self.board_skeleton = None;
        self.solve_start = None;

        diesel::update(sessions::table.filter(sessions::id.eq(self.id.unwrap()))).set(&*self).execute(db).map(|_| ()).map_err(|_| "update failed")
    }

    /// Check if this session has a game going
    pub fn game_started(&self) -> bool {
        self.sudoku_board_id.is_some() && self.board_skeleton.is_some() && self.solve_start.is_some()
    }

    /// Update the in-memory and in-DB model to be logged in as the specified user with the specified permissions.
    ///
    /// # Examples
    ///
    /// Before:
    ///
    /// ```sql
    /// INSERT INTO "sessions" VALUES(32, '2018-07-09 12:40:26', 0, NULL, NULL, NULL, NULL);
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
    /// let mut sess = Session {
    ///     id: Some(32),
    ///     expiry: NaiveDate::from_ymd(2018, 7, 9).and_hms(12, 40, 26),
    ///     is_admin: false,
    ///     user_id: None,
    ///     sudoku_board_id: None,
    ///     board_skeleton: None,
    ///     solve_start: None,
    /// };
    ///
    /// sess.log_in(18, true, &db).unwrap();
    ///
    /// assert_eq!(sess, Session {
    ///     id: Some(32),
    ///     expiry: NaiveDate::from_ymd(2018, 7, 9).and_hms(12, 40, 26),
    ///     is_admin: true,
    ///     user_id: Some(18),
    ///     sudoku_board_id: None,
    ///     board_skeleton: None,
    ///     solve_start: None,
    /// });
    /// ```
    ///
    /// After:
    ///
    /// ```sql
    /// INSERT INTO "sessions" VALUES(32, '2018-07-09 12:40:26', 1, 18, NULL, NULL, NULL);
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
    /// INSERT INTO "sessions" VALUES(32, '2018-07-09 12:40:26', 1, 18, NULL, NULL, NULL);
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
    /// let mut sess = Session {
    ///     id: Some(32),
    ///     expiry: NaiveDate::from_ymd(2018, 7, 9).and_hms(12, 40, 26),
    ///     is_admin: true,
    ///     user_id: Some(18),
    ///     sudoku_board_id: None,
    ///     board_skeleton: None,
    ///     solve_start: None,
    /// };
    ///
    /// sess.log_out(&db).unwrap();
    ///
    /// assert_eq!(sess, Session {
    ///     id: Some(32),
    ///     expiry: NaiveDate::from_ymd(2018, 7, 9).and_hms(12, 40, 26),
    ///     is_admin: false,
    ///     user_id: None,
    ///     sudoku_board_id: None,
    ///     board_skeleton: None,
    ///     solve_start: None,
    /// });
    /// ```
    ///
    /// After:
    ///
    /// ```sql
    /// INSERT INTO "sessions" VALUES(32, '2018-07-09 12:40:26', 0, NULL, NULL, NULL, NULL);
    /// ```
    pub fn log_out(&mut self, db: &SqliteConnection) -> Result<(), &'static str> {
        self.user_id = None;
        self.is_admin = false;
        diesel::update(sessions::table.filter(sessions::id.eq(self.id.unwrap()))).set(&*self).execute(db).map(|_| ()).map_err(|_| "update failed")
    }
}
