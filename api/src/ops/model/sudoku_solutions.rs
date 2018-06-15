use diesel::query_dsl::methods::{FilterDsl, OrderDsl, FindDsl};
use self::super::sudoku_difficulty::BoardDifficulty;
use diesel::expression_methods::ExpressionMethods;
use self::super::super::tables::sudoku_solutions;
use self::super::sudoku_board::SudokuBoard;
use diesel::sqlite::SqliteConnection;
use diesel::query_dsl::RunQueryDsl;
use chrono::{NaiveDateTime, Utc};
use self::super::super::tables;
use chrono::Duration;
use sudoku::Sudoku;
use diesel;


/// Refer to [`doc/sudoku.md`](../doc/sudoku/) for more details.
#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[changeset_options(treat_none_as_null="true")]
#[table_name="sudoku_solutions"]
pub struct SudokuSolution {
    /// Unique board ID.
    ///
    /// Actually not optional, but this allows us to get an ID from the database.
    pub id: Option<i32>,

    /// Solver's display name
    pub display_name: String,

    /// The solved board ID
    pub board_id: i32,

    /// The solved board skeleton
    pub skeleton: String,

    /// Board "difficulty", between one and three
    pub difficulty: i32,

    /// Time in seconds taken to achieve the solution
    pub solution_duration_secs: i32,

    /// Score achieved for the solve
    pub score: i32,

    /// Time the solution occured at
    pub solution_time: NaiveDateTime,
}

impl SudokuSolution {
    /// Create a ready-to-insert solution out of the specified data.
    ///
    /// Does not validate.
    pub fn new<Sn: Into<String>, Sk: Into<String>>(solver_name: Sn, skeleton: Sk, solved_board: &SudokuBoard, solution_duration: &Duration)
                                                   -> Option<SudokuSolution> {
        SudokuSolution::new_impl(solver_name.into(), skeleton.into(), solved_board, solution_duration)
    }

    fn new_impl(solver_name: String, skeleton: String, solved_board: &SudokuBoard, solution_duration: &Duration) -> Option<SudokuSolution> {
        Some(SudokuSolution {
            id: None,
            display_name: solver_name,
            board_id: solved_board.id?,
            skeleton: skeleton,
            difficulty: solved_board.difficulty,
            solution_duration_secs: solution_duration.num_seconds() as i32,
            score: BoardDifficulty::from_numeric(solved_board.difficulty as u64)?.score(solution_duration)? as i32,
            solution_time: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
        })
    }

    /*/// Retrieve the board with the specified ID.
    ///
    /// # Examples
    ///
    /// Given:
    ///
    /// ```sql
    /// INSERT INTO "sudoku_solutions" VALUES(1,
    /// '269574813534918726781263594395846271478129365126357948857491632913682457642735189', 3, '2018-08-01 23:50:14');
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # extern crate sudoku_backend;
    /// # extern crate chrono;
    /// # use sudoku_backend::ops::setup::DatabaseConnection;
    /// # use sudoku_backend::ops::SudokuSolution;
    /// # use std::env::temp_dir;
    /// # use chrono::NaiveDate;
    /// # use std::fs;
    /// # let database_file =
    /// #    ("$ROOT/sudoku-backend.db".to_string(),
    /// #     temp_dir().join("sudoku-backend-doctest").join("ops-model-sudoku_board-SudokuSolution-insert").join("sudoku-backend.db"));
    /// # let _ = fs::remove_file(&database_file.1);
    /// # fs::create_dir_all(database_file.1.parent().unwrap()).unwrap();
    /// # let db = DatabaseConnection::initialise(&database_file);
    /// # let db = &db.get().unwrap();
    /// # let mut solution = SudokuSolution {
    /// #     id: None,
    /// #     full_board: "269574813534918726781263594395846271478129365126357948857491632913682457642735189".to_string(),
    /// #     difficulty: 3,
    /// #     creation_time: NaiveDate::from_ymd(2018, 8, 1).and_hms(23, 50, 14),
    /// # };
    /// # solution.insert(&db).unwrap();
    /// let solution = SudokuSolution::get(1, &db).unwrap();
    /// assert_eq!(solution, SudokuSolution {
    ///     id: Some(1),
    ///     full_board: "269574813534918726781263594395846271478129365126357948857491632913682457642735189".to_string(),
    ///     difficulty: 3,
    ///     creation_time: NaiveDate::from_ymd(2018, 8, 1).and_hms(23, 50, 14),
    /// });
    /// ```
    pub fn get(id: i32, db: &SqliteConnection) -> Result<SudokuSolution, &'static str> {
        tables::sudoku_solutions::table.find(id).first::<SudokuSolution>(db).map_err(|_| "couldn't acquire board solution")
    }*/

    /// Insert this board into the specified DB, updating its id.
    ///
    /// # Examples
    ///
    /// Assuming empty table:
    ///
    /// ```
    /// # extern crate sudoku_backend;
    /// # extern crate chrono;
    /// # use sudoku_backend::ops::{BoardDifficulty, SudokuSolution, SudokuBoard};
    /// # use sudoku_backend::ops::setup::DatabaseConnection;
    /// # use chrono::{NaiveDate, Duration};
    /// # use std::env::temp_dir;
    /// # use std::fs;
    /// # let database_file =
    /// #    ("$ROOT/sudoku-backend.db".to_string(),
    /// #     temp_dir().join("sudoku-backend-doctest").join("ops-model-sudoku_board-SudokuSolution-insert").
    /// join("sudoku-backend.db"));
    /// # let _ = fs::remove_file(&database_file.1);
    /// # fs::create_dir_all(database_file.1.parent().unwrap()).unwrap();
    /// # let db = DatabaseConnection::initialise(&database_file);
    /// # let db = &db.get().unwrap();
    /// # /*
    /// let mut board =
    ///     SudokuSolution::new(username, board_skeleton, &SudokuBoard::get(board_id, &db).unwrap(), Duration::seconds(25));
    /// # */
    /// # let mut solution = SudokuSolution {
    /// #     id: None,
    /// #     display_name: "benlo".to_string(),
    /// #     board_id: 1,
    /// #     skeleton: "2....4....34.18.........5......4..71.7....36..2.3......5...1.32.13........2..5.89".into(),
    /// #     difficulty: BoardDifficulty::Hard.to_numeric() as i32,
    /// #     solution_duration_secs: 25,
    /// #     score: BoardDifficulty::Hard.score(&Duration::seconds(25)).unwrap() as i32,
    /// #     solution_time: NaiveDate::from_ymd(2018, 8, 1).and_hms(23, 50, 14),
    /// # };
    /// # let mut board = SudokuBoard {
    /// #     id: None,
    /// #     full_board: "269574813534918726781263594395846271478129365126357948857491632913682457642735189".to_string(),
    /// #     difficulty: 3,
    /// #     creation_time: NaiveDate::from_ymd(2018, 8, 1).and_hms(23, 50, 14),
    /// # };
    /// # board.insert(&db).unwrap();
    /// assert!(solution.id.is_none());
    ///
    /// solution.insert(&db).unwrap();
    /// assert_eq!(solution.id, Some(1));
    /// # assert_eq!(solution, SudokuSolution {
    /// #     id: Some(1),
    /// #     display_name: "benlo".to_string(),
    /// #     board_id: 1,
    /// #     skeleton: "2....4....34.18.........5......4..71.7....36..2.3......5...1.32.13........2..5.89".into(),
    /// #     difficulty: BoardDifficulty::Hard.to_numeric() as i32,
    /// #     solution_duration_secs: 25,
    /// #     score: BoardDifficulty::Hard.score(&Duration::seconds(25)).unwrap() as i32,
    /// #     solution_time: NaiveDate::from_ymd(2018, 8, 1).and_hms(23, 50, 14),
    /// # });
    /// ```
    pub fn insert(&mut self, db: &SqliteConnection) -> Result<(), &'static str> {
        if self.id.is_some() {
            return Ok(());
        }

        diesel::insert_into(tables::sudoku_solutions::table).values(&*self).execute(db).map_err(|_| "couldn't create new board solution")?;

        // We need to round-trip to get an id
        let board = tables::sudoku_solutions::table.filter(tables::sudoku_solutions::solution_time.eq(&self.solution_time))
            .filter(tables::sudoku_solutions::display_name.eq(&self.display_name))
            .order(tables::sudoku_solutions::id.desc())
            .first::<SudokuSolution>(db)
            .map_err(|_| "couldn't re-acquire new board solution")?;
        self.id = board.id;

        Ok(())
    }
}
