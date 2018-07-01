use serde_json::{self, Value as JsonValue};
use rocket::request::FromFormValue;
use rocket::http::RawStr;


/// Which leaderboard to acquire.
///
/// Refer to [`doc/check.rs`](../doc/check/) for more details.
///
/// The string reprs are `snake_case`.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum LeaderboardOf {
    Solutions,
    Users,
}

impl LeaderboardOf {
    /// The default leaderboard target.
    #[allow(non_upper_case_globals)]
    pub const Default: LeaderboardOf = LeaderboardOf::Solutions;
}

impl<'v> FromFormValue<'v> for LeaderboardOf {
    type Error = ();

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        serde_json::from_value(JsonValue::String(form_value.to_string())).map_err(|_| ())
    }
}
