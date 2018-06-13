use self::super::super::super::BoardDifficulty;


/// The message sent to and from the client on acquisiion/submission of solved board
///
/// Consult [`doc/sudoku.md`](../doc/sudoku/)
#[derive_FromForm]
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct BoardRequestForm {
    /// Difficulty of generated board
    pub difficulty: Result<BoardDifficulty, Option<String>>,
}
