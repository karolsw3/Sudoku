use sudoku::parse_errors::LineFormatParseError as SudokuLineFormatParseError;
use diesel::query_dsl::methods::{FilterDsl, OrderDsl, FindDsl};
use self::super::sudoku_difficulty::BoardDifficulty;
use diesel::expression_methods::ExpressionMethods;
use self::super::super::tables::sudoku_boards;
use diesel::sqlite::SqliteConnection;
use diesel::query_dsl::RunQueryDsl;
use chrono::{NaiveDateTime, Utc};
use self::super::super::tables;
use rand::{Rng, thread_rng};
use sudoku::Sudoku;
use diesel;


/// Refer to [`doc/sudoku.md`](../doc/sudoku/) for more details.
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
    /// INSERT INTO "sudoku_boards"
    ///     VALUES(
    ///         1,
    ///         '269574813534918726781263594395846271478129365126357948857491632913682457642735189',
    ///         3,
    ///         '2018-08-01 23:50:14');
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
    /// #  ("$ROOT/sudoku-backend.db".to_string(),
    /// #   temp_dir().join("sudoku-backend-doctest").join("ops-model-sudoku_board-SudokuBoard-get").join("sudoku-backend.db"));
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
    /// #     temp_dir().join("sudoku-backend-doctest")
    /// #               .join("ops-model-sudoku_board-SudokuBoard-insert").join("sudoku-backend.db"));
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
    /// INSERT INTO "sudoku_boards"
    ///     VALUES(
    ///         1,
    ///         '269574813534918726781263594395846271478129365126357948857491632913682457642735189',
    ///         3,
    ///         '2018-08-01 23:50:14');
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

    /// Get а board skeleton adjusted for the difficulty.
    ///
    /// Quoth <del>the Raven</del> [`Animu_x63`](https://github.com/Aarowaim):
    ///
    /// > The absolute measures of sudoku difficulty are the average sparsity of squares (over all gamestates) that can
    /// logically be resolved, and the maximum amount of logical deductions required to fill in the easiest square along states
    /// leading to the solution.
    ///
    /// But that's hard. However:
    ///
    /// > Most other measurements of difficulty are just arbitrary and you can approximate difficult by sparsity.
    ///
    /// And that is what we do here: get a unique solution, then partially fill it back in according to
    /// [`BoardDifficulty::additional_squares()`](enum.BoardDifficulty.html#method.additional_squares).
    ///
    /// # Examples
    ///
    /// ```
    /// # use sudoku_backend::ops::{BoardDifficulty, SudokuBoard};
    /// let easy_board = SudokuBoard::new(BoardDifficulty::Easy);
    /// let easy_skeleton = easy_board.generate_skeleton().unwrap();
    ///
    /// let medium_board = SudokuBoard::new(BoardDifficulty::Medium);
    /// let medium_skeleton = medium_board.generate_skeleton().unwrap();
    ///
    /// assert!(  easy_skeleton.chars().filter(|&c| c != '.').count() >
    ///         medium_skeleton.chars().filter(|&c| c != '.').count());
    ///
    /// let hard_board = SudokuBoard::new(BoardDifficulty::Hard);
    /// let hard_skeleton = hard_board.generate_skeleton().unwrap();
    ///
    /// assert!(medium_skeleton.chars().filter(|&c| c != '.').count() >
    ///           hard_skeleton.chars().filter(|&c| c != '.').count());
    /// ```
    pub fn generate_skeleton(&self) -> Result<String, SudokuLineFormatParseError> {
        let solution = Sudoku::generate_unique_from(Sudoku::from_str_line(&self.full_board)?);
        let mut filled_in = solution.n_clues() as usize;
        let mut patch_array = [false; 9 * 9];

        let target_filled = filled_in + BoardDifficulty::from_numeric(self.difficulty as u64).unwrap_or(BoardDifficulty::Hard).additional_squares();
        while filled_in < target_filled {
            let which_idx: usize = thread_rng().gen_range(0, 9 * 9 - filled_in);
            if let Some((_, patch_idx)) =
                solution.iter()
                    .enumerate()
                    .filter(|(_, sq)| sq.is_none())
                    .map(|(i, _)| i)
                    .enumerate()
                    .find(|&(empty_idx, _)| !patch_array[empty_idx] && empty_idx == which_idx) {
                patch_array[patch_idx] = true;
                filled_in += 1;
            }
        }

        Ok(solution.to_str_line()
            .chars()
            .zip(self.full_board.chars())
            .zip(patch_array.iter())
            .map(|((sol, full), patch)| if *patch { full } else { sol })
            .collect())
    }
}
