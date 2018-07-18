use self::super::super::super::User;
use chrono::{FixedOffset, DateTime};


/// Data returned to authenticating client, with a free conversion from a [`User`](struct.User.html).
///
/// Consult [`doc/user.md`](../doc/user/) for more details.
///
/// # Examples
///
/// ```
/// # extern crate sudoku_backend;
/// # extern crate chrono;
/// # use sudoku_backend::ops::{SanitisedUserData, User};
/// # use chrono::naive::NaiveDate;
/// # use chrono::DateTime;
/// assert_eq!(SanitisedUserData::from(User {
///                id: Some(12),
///                username: "karolsw3".to_string(),
///                password: "$rscrypt$0$EAgC$p9qwIwAVjdqhKvTR+&c.".to_string(),
///                email: "karol.sw3@gmail.com".to_string(),
///                created_at: NaiveDate::from_ymd(2018, 7, 7).and_hms(17, 6, 29),
///                is_admin: true,
///                points_total: 1750,
///                games_total: 3,
///                games_total_easy: 2,
///                games_total_medium: 1,
///                games_total_hard: 0,
///            }),
///            SanitisedUserData {
///                username: "karolsw3".to_string(),
///                created_at: DateTime::parse_from_rfc3339("2018-07-07T17:06:29Z").unwrap(),
///                is_admin: true,
///                points_total: 1750,
///                games_total: 3,
///                games_total_easy: 2,
///                games_total_medium: 1,
///                games_total_hard: 0,
///            });
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct SanitisedUserData {
    /// User's name or "login" or whatever.
    pub username: String,

    /// Time user was created.
    pub created_at: DateTime<FixedOffset>,

    /// Whether the user has administrative privileges.
    pub is_admin: bool,

    /// Sum total of the user's points, calculated according to
    /// [`doc/scoring.md#endgame-formula`](../doc/scoring/#endgame-formula).
    pub points_total: u64,

    /// Total amount of games played.
    pub games_total: u64,

    /// Amount easy of games played.
    pub games_total_easy: u64,

    /// Amount medium of games played.
    pub games_total_medium: u64,

    /// Amount hard of games played.
    pub games_total_hard: u64,
}

impl From<User> for SanitisedUserData {
    fn from(u: User) -> SanitisedUserData {
        SanitisedUserData {
            username: u.username,
            created_at: DateTime::from_utc(u.created_at, FixedOffset::east(0)),
            is_admin: u.is_admin,
            points_total: u.points_total as u64, // CHECKed to be >=0
            games_total: u.games_total as u64, // CHECKed to be >=0
            games_total_easy: u.games_total_easy as u64, // CHECKed to be >=0
            games_total_medium: u.games_total_medium as u64, // CHECKed to be >=0
            games_total_hard: u.games_total_hard as u64, // CHECKed to be >=0
        }
    }
}
