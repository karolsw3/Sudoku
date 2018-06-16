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
    ///
    /// # Examples
    ///
    /// Given:
    ///
    /// ```sql
    /// INSERT INTO "sudoku_boards" VALUES(1, '269574813534918726781263594395846271478129365126357948857491632913682457642735189', 3, '2018-08-01 23:50:14');
    /// ```
    ///
    /// The following holds:
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
    /// #     temp_dir().join("sudoku-backend-doctest").join("ops-model-sudoku_board-SudokuBoard-get").join("sudoku-backend.db"));
    /// # let _ = fs::remove_file(&database_file.1);
    /// # fs::create_dir_all(database_file.1.parent().unwrap()).unwrap();
    /// # let db = DatabaseConnection::initialise(&database_file);
    /// # let db = &db.get().unwrap();
    /// # let mut board = SudokuBoard {
    /// #     id: None,
    /// #     full_board: "269574813534918726781263594395846271478129365126357948857491632913682457642735189".to_string(),
    /// #     difficulty: 3,
    /// #     creation_time: NaiveDate::from_ymd(2018, 8, 1).and_hms(23, 50, 14),
    /// # };
    /// # board.insert(&db).unwrap();
    /// let board = SudokuBoard::get(1, &db).unwrap();
    /// assert_eq!(board, SudokuBoard {
    ///     id: Some(1),
    ///     full_board: "269574813534918726781263594395846271478129365126357948857491632913682457642735189".to_string(),
    ///     difficulty: 3,
    ///     creation_time: NaiveDate::from_ymd(2018, 8, 1).and_hms(23, 50, 14),
    /// });
    /// ```
    pub fn get(id: i32, db: &SqliteConnection) -> Result<SudokuBoard, &'static str> {
        tables::sudoku_boards::table.find(id).first::<SudokuBoard>(db).map_err(|_| "couldn't acquire sudoku board")
    }

    /// Insert this board into the specified DB, updating its id.
    ///
    /// # Examples
    ///
    /// Assuming empty table:
    ///
    /// ```
    /// # extern crate sudoku_backend;
    /// # extern crate chrono;
    /// # use sudoku_backend::ops::{BoardDifficulty, SudokuBoard};
    /// # use sudoku_backend::ops::setup::DatabaseConnection;
    /// # use std::env::temp_dir;
    /// # use chrono::NaiveDate;
    /// # use std::fs;
    /// # let database_file =
    /// #    ("$ROOT/sudoku-backend.db".to_string(),
    /// #     temp_dir().join("sudoku-backend-doctest").join("ops-model-sudoku_board-SudokuBoard-insert").join("sudoku-backend.db"));
    /// # let _ = fs::remove_file(&database_file.1);
    /// # fs::create_dir_all(database_file.1.parent().unwrap()).unwrap();
    /// # let db = DatabaseConnection::initialise(&database_file);
    /// # let db = &db.get().unwrap();
    /// # let difficulty = BoardDifficulty::Hard;
    /// let mut board = SudokuBoard::new(difficulty);
    /// # let mut board = SudokuBoard {
    /// #     id: None,
    /// #     full_board: "269574813534918726781263594395846271478129365126357948857491632913682457642735189".to_string(),
    /// #     difficulty: 3,
    /// #     creation_time: NaiveDate::from_ymd(2018, 8, 1).and_hms(23, 50, 14),
    /// # };
    /// assert!(board.id.is_none());
    ///
    /// board.insert(&db).unwrap();
    /// assert_eq!(board.id, Some(1));
    /// # assert_eq!(board, SudokuBoard {
    /// #     id: Some(1),
    /// #     full_board: "269574813534918726781263594395846271478129365126357948857491632913682457642735189".to_string(),
    /// #     difficulty: 3,
    /// #     creation_time: NaiveDate::from_ymd(2018, 8, 1).and_hms(23, 50, 14),
    /// # });
    /// ```
    ///
    /// After, example:
    ///
    /// ```sql
    /// INSERT INTO "sudoku_boards" VALUES(1, '269574813534918726781263594395846271478129365126357948857491632913682457642735189', 3, '2018-08-01 23:50:14');
    /// ```
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
}
