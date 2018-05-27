use self::super::super::User;
use chrono::{DateTime, Utc};


/// Data returned to authenticating client, with a free conversion from a [`User`](struct.User.html).
///
/// Consult `doc/user.md` for more details.
///
/// # Examples
///
/// ```
/// # extern crate sudoku_backend;
/// # extern crate chrono;
/// # use sudoku_backend::ops::{SanitisedUserData, User};
/// # use chrono::naive::NaiveDate;
/// # use chrono::DateTime;
/// assert_eq!(User {
///                id: Some(12),
///                username: "karolsw3".to_string(),
///                password: "$rscrypt$0$EAgC$p9qwIwAVjdqhKvTR+&c.".to_string(),
///                email: "karol.sw3@gmail.com".to_string(),
///                created_at: NaiveDate::from_ymd(2018, 7, 7).and_hms(17, 6, 29),
///                is_admin: true,
///                points_total: 1750,
///            }.into(),
///            SanitisedUserData {
///                username: "karolsw3".to_string(),
///                email: "karol.sw3@gmail.com".to_string(),
///                created_at: DateTime::parse_from_rfc3339("2018-07-07T17-06-29Z").unwrap(),
///                is_admin: true,
///                points_total: 1750,
///            });
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SanitisedUserData {
    /// User's name or "login" or whatever.
    pub username: String,

    /// User's contact e-mail
    pub email: String,

    /// Time user was created.
    pub created_at: DateTime<Utc>,

    /// Whether the user has administrative privileges.
    pub is_admin: bool,

    /// Sum total of the user's points, calculated according to `doc/scoring.md#endgame-formula`.
    pub points_total: u64,
}

impl From<User> for SanitisedUserData {
    fn from(u: User) -> SanitisedUserData {
        SanitisedUserData {
            username: u.username,
            email: u.email,
            created_at: DateTime::from_utc(u.created_at, Utc),
            is_admin: u.is_admin,
            points_total: u.points_total as u64, // CHECKed to be >=0
        }
    }
}
