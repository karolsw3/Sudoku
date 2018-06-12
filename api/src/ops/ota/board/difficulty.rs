use self::super::super::super::constraints::{SudokuBoard9x9ConciseLength, SudokuString};
use serde::de::{Deserializer, Deserialize, Error as DeserializerError};
use self::super::super::super::BoardDifficulty;
use std::str::FromStr;
use std::borrow::Cow;


/// The message sent to and from the client on acquisiion/submission of solved board
///
/// Consult [`doc/sudoku.md`](../doc/sudoku/)
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[derive_FromForm]
pub struct BoardRequestForm {
    /// Difficulty of generated board
    pub difficulty: BoardDifficulty,
}
