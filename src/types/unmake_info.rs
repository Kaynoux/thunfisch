use crate::prelude::*;

#[derive(Clone, Copy)]
#[allow(clippy::struct_excessive_bools)]
pub struct UnmakeInfo {
    pub(crate) mv: EncodedMove,
    pub(crate) capture: Figure,
    pub(crate) black_king_castle: bool,
    pub(crate) black_queen_castle: bool,
    pub(crate) white_queen_castle: bool,
    pub(crate) white_king_castle: bool,
    pub(crate) ep_target: Option<Bit>,
    pub(crate) halfmove_clock: usize,
    pub(crate) hash: u64,
    pub(crate) attackmask: Bitboard,
    pub(crate) checkmask: Bitboard,
    pub(crate) check_counter: usize,
    pub(crate) hv_pinmask: Bitboard,
    pub(crate) diag_pinmask: Bitboard,
}
