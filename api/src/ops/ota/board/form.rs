use self::super::super::super::constraints::Positive;
use self::super::super::super::BoardDifficulty;


/// Form requested by the client to play a new Sudoku board
///
/// Consult [`doc/sudoku.md`](../doc/sudoku/)
#[derive_FromForm]
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct NewBoardRequestForm {
    /// Difficulty of generated board
    pub difficulty: Result<BoardDifficulty, Option<String>>,
}

/// Form requested by the client to play the specified Sudoku board
///
/// Consult [`doc/sudoku.md`](../doc/sudoku/)
#[derive_FromForm]
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct OldBoardRequestForm {
    /// ID of board to play
    pub board_id: Result<Positive<i32>, Option<String>>,
}
