//! This module contains the configuration of the application.
//!
//! Use the `Options::parse()` function to get the program's configuration,
//! as parsed from the commandline.
//!
//! # Examples
//!
//! ```no_run
//! # use sudoku_backend::Options;
//! let opts = Options::parse();
//! println!("Using {} as the database", opts.database_file.0);
//! ```

use std::path::{PathBuf, Path};
use clap::{AppSettings, Arg};
use std::fs;


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// File containing the main database.
    ///
    /// Parent directory must exist.
    ///
    /// Default: `"./sudoku-backend.db"`
    pub database_file: (String, PathBuf),
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let matches = app_from_crate!("\n")
            .setting(AppSettings::ColoredHelp)
            .arg(Arg::from_usage("[DATABASE_FILE] 'File containing the main database. Default: ./sudoku-backend.db'")
                .validator(Options::database_file_validator))
            .get_matches();

        Options {
            database_file: matches.value_of("DATABASE_FILE")
                .map(|s| if let Ok(f) = fs::canonicalize(s) {
                    (s.to_string(), f)
                } else {
                    (s.to_string(), fs::canonicalize(Path::new(s).parent().unwrap()).unwrap().join("sudoku-backend.db"))
                })
                .unwrap_or_else(|| ("./sudoku-backend.db".to_string(), fs::canonicalize(".").unwrap().join("sudoku-backend.db"))),
        }
    }

    fn database_file_validator(s: String) -> Result<(), String> {
        let mut p = PathBuf::from(&s);
        if let Ok(f) = fs::canonicalize(&p) {
            if !f.is_file() {
                return Err(format!("Database file \"{}\" not actually a file", s));
            }
        }

        p.pop();
        fs::canonicalize(&p).map_err(|_| format!("Database parent directory \"{}\" nonexistant", p.display())).and_then(|f| if !f.is_file() {
            Ok(())
        } else {
            Err(format!("Database file \"{}\" actually a file", p.display()))
        })
    }
}
