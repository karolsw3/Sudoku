use diesel::sqlite::{SqliteConnection, Sqlite as SqliteBackend};
use self::super::super::tables::{self, sudoku_solutions};
use self::super::sudoku_difficulty::BoardDifficulty;
use diesel::query_builder::{QueryFragment, QueryId};
use diesel::expression_methods::ExpressionMethods;
use self::super::super::setup::LeaderboardConfig;
use diesel::query_dsl::{RunQueryDsl, QueryDsl};
use diesel::{self, AppearsOnTable, Expression};
use self::super::sudoku_board::SudokuBoard;
use self::super::super::SolutionOrdering;
use chrono::{NaiveDateTime, Utc};
use chrono::Duration;


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

    /// Retrieve the board with the specified ID.
    ///
    /// # Examples
    ///
    /// Given:
    ///
    /// ```sql
    /// INSERT INTO "sudoku_solutions"
    ///     VALUES(1, 'benlo', 104,
    ///            '.79.2.48...5..........3762.....4627....85......4.9....48....7..5......3.1.....9..',
    ///            3, 26, 435, '2018-08-03 18:26:35');
    ///
    /// INSERT INTO "sudoku_solutions"
    ///     VALUES(2, 'EasyStudentBegonia', 104,
    ///            '3.9..54..............937.2.9...4.27...6......21....56..8.56....5...1.8..1..3.....',
    ///            3, 22, 498, '2018-08-03 18:42:56');
    ///
    /// INSERT INTO "sudoku_solutions"
    ///     VALUES(3, 'WithoutHighBirch', 104,
    ///            '3.9..54.1.25.8.....4.9......581...7...68.....2....35...8....7.2.......3.1......54',
    ///            3, 14, 732, '2018-08-03 18:44:06');
    ///
    /// INSERT INTO "sudoku_solutions"
    ///     VALUES(4, 'LateBrokenAppendix', 104,
    ///            '.7...5..1...4.1...8....762.95......3..68.....21.............71....2......6.3..95.',
    ///            3, 14, 732, '2018-08-03 18:44:25');
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # extern crate sudoku_backend;
    /// # extern crate chrono;
    /// # use sudoku_backend::ops::setup::{DatabaseConnection, LeaderboardConfig};
    /// # use sudoku_backend::ops::{SolutionOrdering, SudokuSolution};
    /// # use std::env::temp_dir;
    /// # use chrono::NaiveDate;
    /// # use std::fs;
    /// # let database_file =
    /// #    ("$ROOT/sudoku-backend.db".to_string(),
    /// #     temp_dir().join("sudoku-backend-doctest").join("ops-model-sudoku_board-SudokuSolution-leaders")
    /// #               .join("sudoku-backend.db"));
    /// # let _ = fs::remove_file(&database_file.1);
    /// # fs::create_dir_all(database_file.1.parent().unwrap()).unwrap();
    /// # let db = DatabaseConnection::initialise(&database_file);
    /// # let db = &db.get().unwrap();
    /// # let mut solutions =
    /// #     [SudokuSolution {
    /// #          id: None,
    /// #          display_name: "benlo".to_string(),
    /// #          board_id: 104,
    /// #          skeleton: ".79.2.48...5..........3762.....4627....85......4.9....48....7..5......3.1.....9..".to_string(),
    /// #          difficulty: 3,
    /// #          solution_duration_secs: 26,
    /// #          score: 435,
    /// #          solution_time: NaiveDate::from_ymd(2018, 8, 3).and_hms(18, 26, 35),
    /// #      },
    /// #      SudokuSolution {
    /// #          id: None,
    /// #          display_name: "EasyStudentBegonia".to_string(),
    /// #          board_id: 104,
    /// #          skeleton: "3.9..54..............937.2.9...4.27...6......21....56..8.56....5...1.8..1..3.....".to_string(),
    /// #          difficulty: 3,
    /// #          solution_duration_secs: 22,
    /// #          score: 498,
    /// #          solution_time: NaiveDate::from_ymd(2018, 8, 3).and_hms(18, 42, 56),
    /// #      },
    /// #      SudokuSolution {
    /// #          id: None,
    /// #          display_name: "WithoutHighBirch".to_string(),
    /// #          board_id: 104,
    /// #          skeleton: "3.9..54.1.25.8.....4.9......581...7...68.....2....35...8....7.2.......3.1......54".to_string(),
    /// #          difficulty: 3,
    /// #          solution_duration_secs: 14,
    /// #          score: 732,
    /// #          solution_time: NaiveDate::from_ymd(2018, 8, 3).and_hms(18, 44, 06),
    /// #      },
    /// #      SudokuSolution {
    /// #          id: None,
    /// #          display_name: "LateBrokenAppendix".to_string(),
    /// #          board_id: 104,
    /// #          skeleton: ".7...5..1...4.1...8....762.95......3..68.....21.............71....2......6.3..95.".to_string(),
    /// #          difficulty: 3,
    /// #          solution_duration_secs: 14,
    /// #          score: 732,
    /// #          solution_time: NaiveDate::from_ymd(2018, 8, 3).and_hms(18, 44, 25),
    /// #      }];
    /// # for solution in &mut solutions {
    /// #     solution.insert(&db).unwrap();
    /// # }
    /// let solutions = SudokuSolution::leaders(&LeaderboardConfig {
    ///     count: 3,
    ///     ordering: SolutionOrdering::BestToWorst,
    /// }, &db).unwrap();
    /// assert_eq!(
    ///     solutions,
    ///     &[SudokuSolution {
    ///           id: Some(3),
    ///           display_name: "WithoutHighBirch".to_string(),
    ///           board_id: 104,
    ///           skeleton: "3.9..54.1.25.8.....4.9......581...7...68.....2....35...8....7.2.......3.1......54".to_string(),
    ///           difficulty: 3,
    ///           solution_duration_secs: 14,
    ///           score: 732,
    ///           solution_time: NaiveDate::from_ymd(2018, 8, 3).and_hms(18, 44, 06),
    ///       },
    ///       SudokuSolution {
    ///           id: Some(4),
    ///           display_name: "LateBrokenAppendix".to_string(),
    ///           board_id: 104,
    ///           skeleton: ".7...5..1...4.1...8....762.95......3..68.....21.............71....2......6.3..95.".to_string(),
    ///           difficulty: 3,
    ///           solution_duration_secs: 14,
    ///           score: 732,
    ///           solution_time: NaiveDate::from_ymd(2018, 8, 3).and_hms(18, 44, 25),
    ///       },
    ///       SudokuSolution {
    ///           id: Some(2),
    ///           display_name: "EasyStudentBegonia".to_string(),
    ///           board_id: 104,
    ///           skeleton: "3.9..54..............937.2.9...4.27...6......21....56..8.56....5...1.8..1..3.....".to_string(),
    ///           difficulty: 3,
    ///           solution_duration_secs: 22,
    ///           score: 498,
    ///           solution_time: NaiveDate::from_ymd(2018, 8, 3).and_hms(18, 42, 56),
    ///       }]);
    /// ```
    pub fn leaders(cfg: &LeaderboardConfig, db: &SqliteConnection) -> Result<Vec<SudokuSolution>, &'static str> {
        match cfg.ordering {
            SolutionOrdering::BestToWorst => SudokuSolution::leaders_impl(cfg, db, tables::sudoku_solutions::score.desc()),
            SolutionOrdering::WorstToBest => SudokuSolution::leaders_impl(cfg, db, tables::sudoku_solutions::score.asc()),
        }
    }

    fn leaders_impl<OrderExpr>(cfg: &LeaderboardConfig, db: &SqliteConnection, ord: OrderExpr) -> Result<Vec<SudokuSolution>, &'static str>
        where OrderExpr: Expression + AppearsOnTable<tables::sudoku_solutions::table> + QueryId + QueryFragment<SqliteBackend>
    {
        tables::sudoku_solutions::table.order(tables::sudoku_solutions::score.desc())
            .limit(cfg.count as i64)
            .order(ord)
            .load(db)
            .map_err(|_| "couldn't load leaderboard")
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
    /// # use sudoku_backend::ops::{BoardDifficulty, SudokuSolution, SudokuBoard};
    /// # use sudoku_backend::ops::setup::DatabaseConnection;
    /// # use chrono::{NaiveDate, Duration};
    /// # use std::env::temp_dir;
    /// # use std::fs;
    /// # let database_file =
    /// #    ("$ROOT/sudoku-backend.db".to_string(),
    /// #     temp_dir().join("sudoku-backend-doctest").join("ops-model-sudoku_board-SudokuSolution-insert")
    /// #               .join("sudoku-backend.db"));
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
