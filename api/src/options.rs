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


use self::super::util::ACTIVITY_TIMEOUT_DEFAULT;
use std::path::{PathBuf, Path};
use clap::{AppSettings, Arg};
use chrono::Duration;
use std::fs;


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
    /// File containing the main database.
    ///
    /// Parent directory must exist.
    ///
    /// Default: `"./sudoku-backend.db"`
    pub database_file: (String, PathBuf),

    /// Optional file containing the leaderboard settings.
    ///
    /// Parent directory must exist, if specified.
    ///
    /// Default: `"./leaderboard.toml"` if exists, otherwise `None`.
    pub leaderboard_settings_file: Option<(String, PathBuf)>,

    /// Amount of time after last request a user is considered to have "left the site".
    ///
    /// Default: 10 minutes.
    pub activity_timeout: Duration,
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let matches = app_from_crate!("\n")
            .setting(AppSettings::ColoredHelp)
            .arg(Arg::from_usage("[DATABASE_FILE] 'File containing the main database. Default: ./sudoku-backend.db'")
                .validator(|s| Options::file_validator("Database", s)))
            .arg(Arg::from_usage("[LEADERBOARD_SETTINGS_FILE] 'Optional file containing the leaderboard settings. \
                                                               Default: ./leaderboard.toml, then hard defaults'")
                .validator(|s| Options::file_validator("Leaderboard settings", s)))
            .arg(Arg::from_usage("[ACTIVITY_TIMEOUT] 'Amount of time in milliseconds after last request a user is considered to have \"left the site\". \
                                  Default: 60000'")
                .validator(Options::positive_integer_validator))
            .get_matches();

        Options {
            database_file: matches.value_of("DATABASE_FILE")
                .map(|s| if let Ok(f) = fs::canonicalize(s) {
                    (s.to_string(), f)
                } else {
                    (s.to_string(), fs::canonicalize(Path::new(s).parent().unwrap_or_else(|| Path::new("."))).unwrap().join("sudoku-backend.db"))
                })
                .unwrap_or_else(|| ("./sudoku-backend.db".to_string(), fs::canonicalize(".").unwrap().join("sudoku-backend.db"))),
            leaderboard_settings_file: matches.value_of("LEADERBOARD_SETTINGS_FILE")
                .map(|s| if let Ok(f) = fs::canonicalize(s) {
                    (s.to_string(), f)
                } else {
                    (s.to_string(), fs::canonicalize(Path::new(s).parent().unwrap_or_else(|| Path::new("."))).unwrap().join("leaderboard.toml"))
                })
                .or_else(|| {
                    fs::metadata("./leaderboard.toml").ok().and_then(|m| if m.is_file() {
                        fs::canonicalize("./leaderboard.toml").ok().map(|f| ("./leaderboard.toml".to_string(), f))
                    } else {
                        None
                    })
                }),
            activity_timeout: matches.value_of("ACTIVITY_TIMEOUT")
                .map(|at| Duration::milliseconds(at.parse().unwrap()))
                .unwrap_or_else(|| *ACTIVITY_TIMEOUT_DEFAULT),
        }
    }

    fn file_validator(whom: &str, s: String) -> Result<(), String> {
        let mut p = PathBuf::from(&s);
        if let Ok(f) = fs::canonicalize(&p) {
            if !f.is_file() {
                return Err(format!("{} file \"{}\" not actually a file", whom, s));
            }
        }

        p.pop();
        if p == Path::new("") {
            p = PathBuf::from(".");
        }
        fs::canonicalize(&p).map_err(|_| format!("{} parent directory \"{}\" nonexistant", whom, p.display())).and_then(|f| if !f.is_file() {
            Ok(())
        } else {
            Err(format!("{} file \"{}\" actually a file", whom, p.display()))
        })
    }

    fn positive_integer_validator(s: String) -> Result<(), String> {
        match s.parse::<u64>().map_err(|e| format!("{} is not a valid integer: {}", s, e))? {
            0 => Err("0 is not positive".to_string()),
            _ => Ok(()),
        }
    }
}
