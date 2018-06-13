use diesel::query_dsl::methods::{FilterDsl, OrderDsl, FindDsl};
use self::super::sudoku_difficulty::BoardDifficulty;
use diesel::expression_methods::ExpressionMethods;
use self::super::super::tables::sudoku_boards;
use diesel::sqlite::SqliteConnection;
use diesel::query_dsl::RunQueryDsl;
use chrono::{NaiveDateTime, Utc};
use self::super::super::tables;
use sudoku::Sudoku;
use diesel;


/// Refer to [`doc/sudoku.md`](../doc/sudoku/) for more details.
///
/// TODO: doc full workflow with creation, database access, etc.
#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[changeset_options(treat_none_as_null="true")]
#[table_name="sudoku_boards"]
pub struct SudokuBoard {
    /// Unique board ID.
    ///
    /// Actually not optional, but this allows us to get an ID from the database.
    pub id: Option<i32>,

    /// The full "solved" board repr.
    pub full_board: String,

    /// Board "difficulty", between one and three.
    pub difficulty: i32,

    /// Time the board was generated.
    pub creation_time: NaiveDateTime,
}

impl SudokuBoard {
    /// Create a new, unique, sudoku board.
    ///
    /// Put it in the database to get an ID.
    pub fn new(difficulty: BoardDifficulty) -> SudokuBoard {
        SudokuBoard {
            id: None,
            full_board: Sudoku::generate_filled().to_str_line().to_string(),
            difficulty: difficulty.to_numeric() as i32,
            creation_time: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
        }
    }

    /// Retrieve the board with the specified ID.
    pub fn get(id: i32, db: &SqliteConnection) -> Result<SudokuBoard, &'static str> {
        tables::sudoku_boards::table.find(id).first::<SudokuBoard>(db).map_err(|_| "couldn't acquire sudoku board")
    }

    /// Insert this board into the specified DB, updating its id.
    pub fn insert(&mut self, db: &SqliteConnection) -> Result<(), &'static str> {
        if self.id.is_some() {
            return Ok(());
        }

        diesel::insert_into(tables::sudoku_boards::table).values(&*self).execute(db).map_err(|_| "couldn't create new sudoku board")?;

        // We need to round-trip to get an id
        let board = tables::sudoku_boards::table.filter(tables::sudoku_boards::full_board.eq(&self.full_board))
            .order(tables::sudoku_boards::id.desc())
            .first::<SudokuBoard>(db)
            .map_err(|_| "couldn't re-acquire new sudoku board")?;
        self.id = board.id;

        Ok(())
    }

    /*// pub fn set_product(&mut self, pid: i32, db: &SqliteConnection) -> Result<(), &'static str> {
    // self.product_id = Some(pid);
    // diesel::update(sudoku_boards::table.filter(sudoku_boards::id.eq(self.id.unwrap()))).set(&*self).execute(db).map(|_| ()).map_err(|_|
    // "update failed")
    // }

    /// Update the in-memory and in-DB model to be logged in a the specified user with the specified permissions.
    ///
    /// # Examples
    ///
    /// Before:
    ///
    /// ```sql
    /// INSERT INTO "sudoku_boards" VALUES(32, '2018-07-09 12:40:26', 0, NULL);
    /// ```
    ///
    /// Update:
    ///
    /// ```
    /// # extern crate sudoku_backend;
    /// # extern crate chrono;
    /// # use sudoku_backend::ops::setup::DatabaseConnection;
    /// # use sudoku_backend::ops::SudokuBoard;
    /// # use std::env::temp_dir;
    /// # use chrono::NaiveDate;
    /// # use std::fs;
    /// # let database_file =
    /// #    ("$ROOT/sudoku-backend.db".to_string(),
    /// #     temp_dir().join("sudoku-backend-doctest").join("ops-model-sudoku_board-SudokuBoard-log_in").join("sudoku-backend.db"));
    /// # fs::create_dir_all(database_file.1.parent().unwrap()).unwrap();
    /// # let db = DatabaseConnection::initialise(&database_file);
    /// # let db = &db.get().unwrap();
    /// # // TODO: actually insert the bloody thing first.
    /// let mut sess = SudokuBoard {
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
    /// assert_eq!(sess, SudokuBoard {
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
    /// INSERT INTO "sudoku_boards" VALUES(32, '2018-07-09 12:40:26', 1, 18);
    /// ```
    pub fn log_in(&mut self, uid: i32, is_admin: bool, db: &SqliteConnection) -> Result<(), &'static str> {
        self.user_id = Some(uid);
        self.is_admin = is_admin;
        diesel::update(sudoku_boards::table.filter(sudoku_boards::id.eq(self.id.unwrap()))).set(&*self).execute(db).map(|_| ()).map_err(|_| "update failed")
    }

    /// Update the in-memory and in-DB model to not be logged in nor be admin.
    ///
    /// # Examples
    ///
    /// Before:
    ///
    /// ```sql
    /// INSERT INTO "sudoku_boards" VALUES(32, '2018-07-09 12:40:26', 1, 18);
    /// ```
    ///
    /// Update:
    ///
    /// ```
    /// # extern crate sudoku_backend;
    /// # extern crate chrono;
    /// # use sudoku_backend::ops::setup::DatabaseConnection;
    /// # use sudoku_backend::ops::SudokuBoard;
    /// # use std::env::temp_dir;
    /// # use chrono::NaiveDate;
    /// # use std::fs;
    /// # let database_file =
    /// #    ("$ROOT/sudoku-backend.db".to_string(),
    /// #     temp_dir().join("sudoku-backend-doctest").join("ops-model-sudoku_board-SudokuBoard-log_out").join("sudoku-backend.db"));
    /// # fs::create_dir_all(database_file.1.parent().unwrap()).unwrap();
    /// # let db = DatabaseConnection::initialise(&database_file);
    /// # let db = &db.get().unwrap();
    /// # // TODO: actually insert the bloody thing first.
    /// let mut sess = SudokuBoard {
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
    /// assert_eq!(sess, SudokuBoard {
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
    /// INSERT INTO "sudoku_boards" VALUES(32, '2018-07-09 12:40:26', 0, NULL);
    /// ```
    pub fn log_out(&mut self, db: &SqliteConnection) -> Result<(), &'static str> {
        self.user_id = None;
        self.is_admin = false;
        diesel::update(sudoku_boards::table.filter(sudoku_boards::id.eq(self.id.unwrap()))).set(&*self).execute(db).map(|_| ()).map_err(|_| "update failed")
    }*/
}
