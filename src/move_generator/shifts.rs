use crate::move_generator::blockers;
use crate::prelude::*;

pub const ROOK: [u8; 64] = calculate_shifts(blockers::ROOK);
pub const BISHOP: [u8; 64] = calculate_shifts(blockers::BISHOP);

const fn calculate_shifts(blockers: [Bitboard; 64]) -> [u8; 64] {
    let mut shifts = [0u8; 64];
    let mut pos = Square(0);

    while pos.0 < 64 {
        let blocker = blockers[pos.0];
        let count_of_ones = blocker.0.count_ones();
        shifts[pos.0] = 64u8 - (count_of_ones as u8);
        pos.0 += 1;
    }

    shifts
}
