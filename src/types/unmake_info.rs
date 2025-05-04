use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct UnmakeInfo {
    pub mv: DecodedMove,
    pub capture: ColorPiece,
    pub black_king_castle: bool,
    pub black_queen_castle: bool,
    pub white_queen_castle: bool,
    pub white_king_castle: bool,
    pub ep_target: Option<Position>,
    pub halfmove_clock: usize,
}
