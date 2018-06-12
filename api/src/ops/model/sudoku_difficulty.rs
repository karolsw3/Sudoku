use rocket::request::FromFormValue;
use rocket::http::RawStr;
use chrono::Duration;


/// The difficulties sudoku boards can come in.
///
/// Consult [`doc/sudoku.md`](../doc/sudoku/)
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum BoardDifficulty {
    Easy,
    Medium,
    Hard,
}

impl BoardDifficulty {
    /// Calculate the score for a board according to [`doc/sudoku.md#scoring-formula`](../doc/sudoku/#scoring-formula)
    pub fn score(&self, solve_time: Duration) -> u64 {
        if solve_time < Duration::zero() {
            0
        } else {
            self.to_numeric() * (30 + (3000 / solve_time.num_seconds())) as u64
        }
    }

    /// Parse a numerical representation of the difficulty.
    pub fn from_numeric(repr: u64) -> Option<BoardDifficulty> {
        match repr {
            1 => Some(BoardDifficulty::Easy),
            2 => Some(BoardDifficulty::Medium),
            3 => Some(BoardDifficulty::Hard),

            _ => None,
        }
    }

    /// Convert difficulty to a numeric repr
    ///
    /// Guaranteed to be a valid round-trip
    pub fn to_numeric(&self) -> u64 {
        match self {
            BoardDifficulty::Easy => 1,
            BoardDifficulty::Medium => 2,
            BoardDifficulty::Hard => 3,
        }
    }
}

impl<'v> FromFormValue<'v> for BoardDifficulty {
    type Error = Option<String>;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        match form_value.parse() {
            Ok(n) => BoardDifficulty::from_numeric(n).ok_or(None),
            Err(e) => Err(Some(e.to_string())),
        }
    }
}
