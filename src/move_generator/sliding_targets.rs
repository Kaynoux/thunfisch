use crate::move_generator::magics::BISHOP_MAGICS;
use crate::move_generator::magics::ROOK_MAGICS;
use crate::prelude::*;

/// Precalculates Black Magic Bitboards which are indexed with a hash key and return the possible sliding target positions
/// https://www.chessprogramming.org/Magic_Bitboards
static SLIDING_TARGETS: [Bitboard; 88772] = {
    let mut targets = [Bitboard::EMPTY; 88772];
    let mut square = 0;
    while square < 64 {
        let magic = &ROOK_MAGICS[square as usize];
        let range = magic.mask;
        let mut subset = 0;
        loop {
            let attack = get_sliding_targets(square, subset, true);
            let idx = (magic.factor.wrapping_mul(subset) >> (64 - 12)) as usize + magic.offset;
            targets[idx] = Bitboard(attack);
            subset = subset.wrapping_sub(range) & range;
            if subset == 0 {
                break;
            }
        }

        let magic = &BISHOP_MAGICS[square as usize];
        let range = magic.mask;
        let mut subset = 0;
        loop {
            let attack = get_sliding_targets(square, subset, false);
            let idx = (magic.factor.wrapping_mul(subset) >> (64 - 9)) as usize + magic.offset;
            targets[idx] = Bitboard(attack);
            subset = subset.wrapping_sub(range) & range;
            if subset == 0 {
                break;
            }
        }

        square += 1;
    }
    targets
};

/// Returns all hv or diag positions from the given square to the border of board
const fn get_sliding_targets(square: i32, occupied: u64, is_hv: bool) -> u64 {
    // uses the offsets which correspond to diagonal or vertical moves
    let offsets = if is_hv {
        [8, 1, -1, -8]
    } else {
        [9, 7, -7, -9]
    };
    let mut targets = 0;

    let mut i = 0;
    while i < 4 {
        let mut prev = square;
        loop {
            let sq = prev + offsets[i];
            let dy = (sq & 7) - (prev & 7);
            if dy > 2 || dy < -2 || sq < 0 || sq > 63 {
                break;
            }
            let bb = 1 << sq;
            targets |= bb;
            if occupied & bb != 0 {
                break;
            }
            prev = sq;
        }
        i += 1;
    }

    targets
}

/// Returns all possible bishop target posiotns given its current position and all occupied Squares as a Bitboard
/// https://www.chessprogramming.org/Magic_Bitboards
pub const fn get_bishop_targets(square: Square, occupied: Bitboard) -> Bitboard {
    let magic = BISHOP_MAGICS[square.0];

    // Uses magic bitboards to return the precalculated positions
    let idx =
        (magic.factor.wrapping_mul(occupied.0 & magic.mask) >> (64 - 9)) as usize + magic.offset;

    SLIDING_TARGETS[idx]
}

/// Returns all possible bishop target posiotns given its current position and all occupied Squares as a Bitboard
/// https://www.chessprogramming.org/Magic_Bitboards
pub const fn get_rook_targets(square: Square, occupied: Bitboard) -> Bitboard {
    let magic = ROOK_MAGICS[square.0];

    // Uses magic bitboards to return the precalculated positions
    let idx =
        (magic.factor.wrapping_mul(occupied.0 & magic.mask) >> (64 - 12)) as usize + magic.offset;

    SLIDING_TARGETS[idx]
}
