use crate::prelude::Position;
use std::fmt;

#[derive(Copy, Clone)]
pub struct ChessMove(pub Position, pub Position);

impl fmt::Debug for ChessMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}->{:?}", self.0, self.1)
    }
}

// impl ChessMove {
//     pub fn encode(
//         from: u8,
//         to: u8
//         is_capture: bool,

//     )
// }
