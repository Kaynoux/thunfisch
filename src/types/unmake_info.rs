use crate::prelude::*;

#[derive(Clone, Copy)]
#[allow(clippy::struct_excessive_bools)]
pub struct UnmakeInfo {
    pub mv: EncodedMove,
    pub capture: Figure,
    pub black_king_castle: bool,
    pub black_queen_castle: bool,
    pub white_queen_castle: bool,
    pub white_king_castle: bool,
    pub ep_target: Option<Bit>,
    pub halfmove_clock: usize,
    pub hash: u64,
    pub attackmask: Bitboard,
    pub checkmask: Bitboard,
    pub check_counter: usize,
    pub hv_pinmask: Bitboard,
    pub diag_pinmask: Bitboard,
}
