//! Data used for general application setup: third-party connexions, et caetera.


use self::super::super::ops::{SolutionOrdering, LeaderboardOf};
use rocket::request::{FromRequest, Outcome as RequestOutcome};
use self::super::super::util::INITIALISE_DATABASE;
use diesel::r2d2::{self, ConnectionManager};
use rocket::request::{FormItems, FromForm};
use diesel::connection::SimpleConnection;
use rocket::{Request, Outcome, State};
use diesel::sqlite::SqliteConnection;
use rocket::Error as RocketError;
use rocket::http::Status;
use std::path::PathBuf;
use std::ops::Deref;
use std::fs::File;
use std::io::Read;
use std::cmp;
use toml;


/// Connection request guard type: a wrapper around an r2d2 pooled connection.
///
/// Use this as an argument to a Rocket request handler after having it `manage()`d to gain access to the database.
///
/// # Examples
///
/// ```no_run
/// # #![feature(plugin)]
/// # #![plugin(rocket_codegen)]
/// # extern crate sudoku_backend;
/// # #[macro_use]
/// # extern crate rocket;
/// # use std::fs;
/// # use std::env::temp_dir;
/// # use sudoku_backend::ops::setup::DatabaseConnection;
/// #[get("/databased")]
/// fn databased(db: DatabaseConnection) -> String {
///     // Do some database ops, which are outside the scope of this document
/// #   let funxion_result = "henlo".to_string();
///     funxion_result
/// }
///
/// fn main() {
/// #   let database_file =
/// #     ("$ROOT/sudoku-backend.db".to_string(),
/// #      temp_dir().join("sudoku-backend-doctest").join("ops-setup-DatabaseConnection-initialise").join("sudoku-backend.db"));
/// #   fs::create_dir_all(database_file.1.parent().unwrap()).unwrap();
/// #   /*
///     let database_file: (String, PathBuf) = /* obtained elsewhere */;
/// #   */
///     rocket::ignite()
///         .manage(DatabaseConnection::initialise(&database_file))
///         .mount("/", routes![databased])
///         .launch();
/// }
/// ```
pub struct DatabaseConnection(r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);

impl DatabaseConnection {
    /// Set up a connection to the main database located in the specified file and initialise it with
    /// [`util::INITIALISE_DATABASE`](../../../util/static.INITIALISE_DATABASE.html).
    pub fn initialise(db_file: &(String, PathBuf)) -> r2d2::Pool<ConnectionManager<SqliteConnection>> {
        let mgr = ConnectionManager::new(db_file.1.display().to_string().replace('\\', "/"));
        let pool: r2d2::Pool<ConnectionManager<SqliteConnection>> = r2d2::Pool::new(mgr).expect("Failed to open database");
        pool.get().unwrap().batch_execute(INITIALISE_DATABASE).unwrap();
        pool
    }
}

/// Attempts to retrieve a single connection from the managed database pool.
///
/// If no pool is currently managed, fails with an `InternalServerError` status.
/// If no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DatabaseConnection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> RequestOutcome<DatabaseConnection, ()> {
        match request.guard::<State<r2d2::Pool<ConnectionManager<SqliteConnection>>>>()?.get() {
            Ok(conn) => Outcome::Success(DatabaseConnection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DatabaseConnection {
    type Target = SqliteConnection;

    fn deref(&self) -> &SqliteConnection {
        &self.0
    }
}


/// Configuration of a leaderboard request.
//
/// Refer to [`doc/check.rs`](../../doc/check/) for more details.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct SpecificLeaderboardConfig {
    /// How many entries to return
    pub for_whom: LeaderboardOf,
    /// How many entries to return
    pub cfg: LeaderboardConfig,
}

#[derive_FromForm]
struct SpecificLeaderboardConfigData {
    pub count: Option<usize>,
    pub ordering: Option<SolutionOrdering>,
    pub of: Option<LeaderboardOf>,
}

impl<'f> FromForm<'f> for SpecificLeaderboardConfig {
    // Ideally this would be
    // type Error = <LeaderboardConfigData as FromForm<'f>>::Error;
    // But that's "leaking a private type" so
    type Error = RocketError;

    fn from_form(items: &mut FormItems<'f>, strict: bool) -> Result<Self, Self::Error> {
        SpecificLeaderboardConfigData::from_form(items, strict).map(|slcd| {
            let for_whom = slcd.of.unwrap_or_else(|| LeaderboardOf::Default);
            let defaults = match for_whom {
                LeaderboardOf::Solutions => &LeaderboardConfig::BOARD_DEFAULT_DEFAULT,
                LeaderboardOf::Users => &LeaderboardConfig::PLAYER_DEFAULT_DEFAULT,
            };

            SpecificLeaderboardConfig {
                for_whom: for_whom,
                cfg: LeaderboardConfig {
                    count: slcd.count.unwrap_or_else(|| defaults.count),
                    ordering: slcd.ordering.unwrap_or_else(|| defaults.ordering),
                },
            }
        })
    }
}

/// Configuration of a leaderboard request.
//
/// Refer to [`doc/check.rs`](../../doc/check/) for more details.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, Eq, PartialEq, Ord)]
pub struct LeaderboardConfig {
    /// How many entries to return
    pub count: usize,
    /// How to order the returned entries
    pub ordering: SolutionOrdering,
}

impl LeaderboardConfig {
    /// Default no-spec return config for solutions.
    pub const BOARD_DEFAULT_DEFAULT: LeaderboardConfig = LeaderboardConfig {
        count: 10,
        ordering: SolutionOrdering::Default,
    };

    /// Default maximal unexceedable return config for solutions.
    pub const BOARD_DEFAULT_MAX: LeaderboardConfig = LeaderboardConfig {
        count: 42,
        ordering: SolutionOrdering::Default,
    };

    /// Default no-spec return config for players.
    pub const PLAYER_DEFAULT_DEFAULT: LeaderboardConfig = LeaderboardConfig {
        count: 3,
        ordering: SolutionOrdering::Default,
    };

    /// Default maximal unexceedable return config for players.
    pub const PLAYER_DEFAULT_MAX: LeaderboardConfig = LeaderboardConfig {
        count: 10,
        ordering: SolutionOrdering::Default,
    };
}

impl cmp::PartialOrd for LeaderboardConfig {
    fn partial_cmp(&self, other: &LeaderboardConfig) -> Option<cmp::Ordering> {
        self.count.partial_cmp(&other.count)
    }
}

/// Amalgam of max and default leaderboard configurations.
//
/// Refer to [`doc/check.rs`](../../doc/check/) for more details.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct LeaderboardGroupSettings {
    /// Default config to backfill from
    pub default: LeaderboardConfig,
    /// Unexceedable config
    pub max: LeaderboardConfig,
}

/// Amalgam of max and default leaderboard configurations.
//
/// Refer to [`doc/check.rs`](../../doc/check/) for more details.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct LeaderboardSettings {
    /// Settings for solves
    pub board: LeaderboardGroupSettings,
    /// Settings for people
    pub person: LeaderboardGroupSettings,
}

impl LeaderboardSettings {
    /// Load the settings from the ones specified in the specified file.
    pub fn load(settings_file: &(String, PathBuf)) -> Result<LeaderboardSettings, String> {
        let mut buf = String::new();
        File::open(&settings_file.1).map_err(|e| format!("Couldn't open leaderboard settings file: {}", e))?
            .read_to_string(&mut buf)
            .map_err(|e| format!("Couldn't read leaderboard settings file: {}", e))?;
        toml::from_str(&buf).map_err(|e| format!("Failed to parse leaderboard settings: {}", e))
    }
}
