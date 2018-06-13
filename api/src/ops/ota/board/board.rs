use self::super::super::super::constraints::{SudokuBoard9x9ConciseLength, SudokuString};
use serde::de::{Deserializer, Deserialize, Error as DeserializerError};
use serde::ser::{SerializeStruct, Serializer, Serialize};
use std::str::FromStr;
use std::borrow::Cow;


/// The message sent to and from the client on acquisiion/submission of solved board
///
/// Consult [`doc/sudoku.md`](../doc/sudoku/)
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct BoardMessage {
    /// ID of the full original board
    pub board_id: i32,

    /// The skeleton to be solved by the user
    pub board_skeleton: Cow<'static, str>,

    /// The solved board from the skeleton, to be present only on board submit
    pub solved_board: Option<Cow<'static, str>>,
}

#[derive(Deserialize)]
struct BoardMessageData<'s> {
    board_id: i32,
    board_skeleton: Cow<'s, str>,
    solved_board: Option<Cow<'s, str>>,
}

impl Serialize for BoardMessage {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("BoardMessage", 2 + self.solved_board.is_some() as usize)?;
        s.serialize_field("board_id", &self.board_id)?;
        s.serialize_field("board_skeleton", &self.board_skeleton)?;
        if let Some(ref sb) = self.solved_board {
            s.serialize_field("solved_board", &sb)?;
        }
        s.end()
    }
}

impl<'de> Deserialize<'de> for BoardMessage {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let data = BoardMessageData::deserialize(deserializer)?;

        if data.board_id <= 0 {
            Err(DeserializerError::custom("id nonpositive"))?;
        }
        if let Err(e) = SudokuString::<SudokuBoard9x9ConciseLength>::from_str(&data.board_skeleton) {
            Err(board_error::<D>(e, "board skeleton"))?;
        }
        if let Some(sb) = data.solved_board.as_ref() {
            if let Err(e) = SudokuString::<SudokuBoard9x9ConciseLength>::from_str(sb) {
                Err(board_error::<D>(e, "solved board"))?;
            }
        }

        Ok(BoardMessage {
            board_id: data.board_id,
            board_skeleton: data.board_skeleton,
            solved_board: data.solved_board,
        })
    }
}

fn board_error<'de, D: Deserializer<'de>>(e: Option<Option<usize>>, name: &'static str) -> D::Error {
    match e {
        Some(Some(s)) => DeserializerError::custom(format_args!("{} of invalid length {}", name, s)),
        Some(None) => DeserializerError::custom(format_args!("{} contained non-sudoku character", name)),
        None => DeserializerError::custom(format_args!("couldn't decode {} string", name)),
    }
}
