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
    ///
    /// Negative duration is out of domain and will return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate sudoku_backend;
    /// # extern crate chrono;
    /// # use sudoku_backend::ops::BoardDifficulty;
    /// # use chrono::Duration;
    /// assert_eq!(BoardDifficulty::Easy.score(&Duration::seconds(120)), Some(55));
    /// assert_eq!(BoardDifficulty::Hard.score(&Duration::seconds(250)), Some(126));
    ///
    /// assert_eq!(BoardDifficulty::Medium.score(&Duration::seconds(-65)), None);
    /// ```
    pub fn score(&self, solve_time: &Duration) -> Option<u64> {
        if solve_time < &Duration::zero() {
            None
        } else {
            Some(self.to_numeric() * (30 + (3000 / solve_time.num_seconds())) as u64)
        }
    }

    /// Parse a numerical representation of the difficulty.
    ///
    /// Guaranteed to validly round-trip through `to_numeric()`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sudoku_backend::ops::BoardDifficulty;
    /// assert_eq!(BoardDifficulty::from_numeric(1), Some(BoardDifficulty::Easy));
    /// assert_eq!(BoardDifficulty::from_numeric(2), Some(BoardDifficulty::Medium));
    /// assert_eq!(BoardDifficulty::from_numeric(3), Some(BoardDifficulty::Hard));
    ///
    /// assert_eq!(BoardDifficulty::from_numeric(0), None);
    /// assert_eq!(BoardDifficulty::from_numeric(4), None);
    /// assert_eq!(BoardDifficulty::from_numeric(25), None);
    /// ```
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
    /// Guaranteed to validly round-trip through `from_numeric()`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sudoku_backend::ops::BoardDifficulty;
    /// assert_eq!(BoardDifficulty::Easy.to_numeric(), 1);
    /// assert_eq!(BoardDifficulty::Medium.to_numeric(), 2);
    /// assert_eq!(BoardDifficulty::Hard.to_numeric(), 3);
    ///
    /// assert_eq!(BoardDifficulty::from_numeric(BoardDifficulty::Easy.to_numeric()),
    ///            Some(BoardDifficulty::Easy));
    /// ```
    pub fn to_numeric(&self) -> u64 {
        match self {
            BoardDifficulty::Easy => 1,
            BoardDifficulty::Medium => 2,
            BoardDifficulty::Hard => 3,
        }
    }

    /// How many more squares to compensate difficulty with.
    ///
    /// A good approximation of Sudoku difficulty is square sparsity,
    /// so to make a board easier one can simply add more squares to the default unique solution.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sudoku_backend::ops::BoardDifficulty;
    /// assert_eq!(BoardDifficulty::Easy.additional_squares(), 5);
    /// assert_eq!(BoardDifficulty::Medium.additional_squares(), 2);
    /// assert_eq!(BoardDifficulty::Hard.additional_squares(), 0);
    /// ```
    pub fn additional_squares(&self) -> usize {
        match self {
            BoardDifficulty::Easy => 5,
            BoardDifficulty::Medium => 2,
            BoardDifficulty::Hard => 0,
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
