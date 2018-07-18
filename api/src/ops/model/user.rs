use diesel::sqlite::{SqliteConnection, Sqlite as SqliteBackend};
use diesel::query_builder::{QueryFragment, QueryId};
use diesel::expression_methods::ExpressionMethods;
use self::super::super::setup::LeaderboardConfig;
use diesel::query_dsl::{RunQueryDsl, QueryDsl};
use diesel::{self, AppearsOnTable, Expression};
use self::super::super::tables::{self, users};
use self::super::super::SolutionOrdering;
use self::super::BoardDifficulty;
use chrono::{NaiveDateTime, Utc};


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

    /// Total amount of games played, `CHECK`ed to nonnegativity.
    pub games_total: i32,

    /// Amount of easy games played, `CHECK`ed to nonnegativity.
    pub games_total_easy: i32,

    /// Amount of medium games played, `CHECK`ed to nonnegativity.
    pub games_total_medium: i32,

    /// Amount of hard games played, `CHECK`ed to nonnegativity.
    pub games_total_hard: i32,
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
            games_total: 0,
            games_total_easy: 0,
            games_total_medium: 0,
            games_total_hard: 0,
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

    /// Update in-memory and in-DB repr by the specified point count.
    pub fn solve(&mut self, for_points: usize, for_difficulty: BoardDifficulty, db: &SqliteConnection) -> Result<(), &'static str> {
        self.points_total += for_points as i32;
        self.games_total += 1;
        *match for_difficulty {
            BoardDifficulty::Easy => &mut self.games_total_easy,
            BoardDifficulty::Medium => &mut self.games_total_medium,
            BoardDifficulty::Hard => &mut self.games_total_hard,
        } += 1;
        diesel::update(tables::users::table.filter(tables::users::id.eq(self.id.unwrap()))).set(&*self).execute(db).map(|_| ()).map_err(|_| "update failed")
    }

    /// Retrieve the board with the specified ID.
    ///
    /// # Examples
    ///
    /// Given:
    ///
    /// ```sql
    /// INSERT INTO "users"
    ///     VALUES(1, 'karolsw3',       'password', 'email', '2018-07-23 18:18:24', 0, 435, 1, 1, 0, 0);
    /// INSERT INTO "users"
    ///     VALUES(2, 'nabijaczleweli', 'password', 'email', '2018-07-23 19:08:09', 1, 732, 1, 0, 1, 0);
    /// INSERT INTO "users"
    ///     VALUES(3, 'sehe',           'password', 'email', '2018-07-23 19:08:56', 0, 1230, 2, 0, 0, 2);
    /// INSERT INTO "users"
    ///     VALUES(4, 'skorezore',      'password', 'email', '2018-07-23 19:11:06', 0, 222, 1, 0, 0, 1);
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # extern crate sudoku_backend;
    /// # extern crate chrono;
    /// # use sudoku_backend::ops::setup::{DatabaseConnection, LeaderboardConfig};
    /// # use sudoku_backend::ops::{SolutionOrdering, User};
    /// # use std::env::temp_dir;
    /// # use chrono::NaiveDate;
    /// # use std::fs;
    /// # let database_file =
    /// #    ("$ROOT/sudoku-backend.db".to_string(),
    /// #     temp_dir().join("sudoku-backend-doctest").join("ops-model-user-User-leaders")
    /// #               .join("sudoku-backend.db"));
    /// # let _ = fs::remove_file(&database_file.1);
    /// # fs::create_dir_all(database_file.1.parent().unwrap()).unwrap();
    /// # let db = DatabaseConnection::initialise(&database_file);
    /// # let db = &db.get().unwrap();
    /// # let mut users =
    /// #     [User {
    /// #          id: None,
    /// #          username: "karolsw3".to_string(),
    /// #          password: "password".to_string(),
    /// #          email: "karolsw3@e.mail".to_string(),
    /// #          created_at: NaiveDate::from_ymd(2018, 07, 23).and_hms(18, 18, 24),
    /// #          is_admin: false,
    /// #          points_total: 435,
    /// #          games_total: 1,
    /// #          games_total_easy: 1,
    /// #          games_total_medium: 0,
    /// #          games_total_hard: 0,
    /// #      },
    /// #      User {
    /// #          id: None,
    /// #          username: "nabijaczleweli".to_string(),
    /// #          password: "password".to_string(),
    /// #          email: "nabijaczleweli@e.mail".to_string(),
    /// #          created_at: NaiveDate::from_ymd(2018, 07, 23).and_hms(19, 08, 09),
    /// #          is_admin: true,
    /// #          points_total: 732,
    /// #          games_total: 1,
    /// #          games_total_easy: 0,
    /// #          games_total_medium: 1,
    /// #          games_total_hard: 0,
    /// #      },
    /// #      User {
    /// #          id: None,
    /// #          username: "sehe".to_string(),
    /// #          password: "password".to_string(),
    /// #          email: "sehe@e.mail".to_string(),
    /// #          created_at: NaiveDate::from_ymd(2018, 07, 23).and_hms(19, 08, 56),
    /// #          is_admin: false,
    /// #          points_total: 1230,
    /// #          games_total: 2,
    /// #          games_total_easy: 0,
    /// #          games_total_medium: 0,
    /// #          games_total_hard: 2,
    /// #      },
    /// #      User {
    /// #          id: None,
    /// #          username: "skorezore".to_string(),
    /// #          password: "password".to_string(),
    /// #          email: "skorezore@e.mail".to_string(),
    /// #          created_at: NaiveDate::from_ymd(2018, 07, 23).and_hms(19, 11, 06),
    /// #          is_admin: false,
    /// #          points_total: 222,
    /// #          games_total: 1,
    /// #          games_total_easy: 0,
    /// #          games_total_medium: 0,
    /// #          games_total_hard: 1,
    /// #      }];
    /// # for user in &mut users {
    /// #     user.add(&db).unwrap();
    /// # }
    /// let users = User::leaders(&LeaderboardConfig {
    ///     count: 3,
    ///     ordering: SolutionOrdering::WorstToBest,
    /// }, &db).unwrap();
    /// assert_eq!(
    ///     users,
    ///     &[User {
    ///           id: Some(4),
    ///           username: "skorezore".to_string(),
    ///           password: "password".to_string(),
    ///           email: "skorezore@e.mail".to_string(),
    ///           created_at: NaiveDate::from_ymd(2018, 07, 23).and_hms(19, 11, 06),
    ///           is_admin: false,
    ///           points_total: 222,
    ///           games_total: 1,
    ///           games_total_easy: 0,
    ///           games_total_medium: 0,
    ///           games_total_hard: 1,
    ///       },
    ///       User {
    ///           id: Some(1),
    ///           username: "karolsw3".to_string(),
    ///           password: "password".to_string(),
    ///           email: "karolsw3@e.mail".to_string(),
    ///           created_at: NaiveDate::from_ymd(2018, 07, 23).and_hms(18, 18, 24),
    ///           is_admin: false,
    ///           points_total: 435,
    ///           games_total: 1,
    ///           games_total_easy: 1,
    ///           games_total_medium: 0,
    ///           games_total_hard: 0,
    ///       },
    ///       User {
    ///           id: Some(2),
    ///           username: "nabijaczleweli".to_string(),
    ///           password: "password".to_string(),
    ///           email: "nabijaczleweli@e.mail".to_string(),
    ///           created_at: NaiveDate::from_ymd(2018, 07, 23).and_hms(19, 08, 09),
    ///           is_admin: true,
    ///           points_total: 732,
    ///           games_total: 1,
    ///           games_total_easy: 0,
    ///           games_total_medium: 1,
    ///           games_total_hard: 0,
    ///       }]);
    /// ```
    pub fn leaders(cfg: &LeaderboardConfig, db: &SqliteConnection) -> Result<Vec<User>, &'static str> {
        match cfg.ordering {
            SolutionOrdering::BestToWorst => User::leaders_impl(cfg, db, tables::users::points_total.desc()),
            SolutionOrdering::WorstToBest => User::leaders_impl(cfg, db, tables::users::points_total.asc()),
        }
    }

    fn leaders_impl<OrderExpr>(cfg: &LeaderboardConfig, db: &SqliteConnection, ord: OrderExpr) -> Result<Vec<User>, &'static str>
        where OrderExpr: Expression + AppearsOnTable<tables::users::table> + QueryId + QueryFragment<SqliteBackend>
    {
        tables::users::table.order(ord)
            .limit(cfg.count as i64)
            .load(db)
            .map_err(|_| "couldn't load leaderboard")
    }
}
