use crate::move_generator::blockers;
use crate::move_generator::magics;
use crate::move_generator::shifts;
use crate::move_generator::sliding_targets;
use crate::prelude::*;

const TOTAL_POSITIONS: usize = calculate_total_positions();

const fn calculate_total_positions() -> usize {
    let mut total_size: usize = 0;
    let mut sq_idx: usize = 0;

    // Berechne Größe für Rook-Tabellen
    while sq_idx < 64 {
        // Greife auf deine Blocker-Konstante zu
        let mask = blockers::ROOK_BLOCKERS[sq_idx];
        let bits = mask.0.count_ones() as usize;
        // Größe für dieses Feld = 2^bits
        let size_for_sq = 1usize << bits;
        // Addiere zur Gesamtsumme (verwende checked_add für Sicherheit gegen Overflow)
        total_size = match total_size.checked_add(size_for_sq) {
            Some(s) => s,
            None => panic!("Overflow calculating total magic table size!"),
        };
        sq_idx += 1;
    }

    // Reset und berechne Größe für Bishop-Tabellen
    sq_idx = 0;
    while sq_idx < 64 {
        // Greife auf deine Blocker-Konstante zu
        let mask = blockers::BISHOP_BLOCKERS[sq_idx];
        let bits = mask.0.count_ones() as usize;
        let size_for_sq = 1usize << bits;
        total_size = match total_size.checked_add(size_for_sq) {
            Some(s) => s,
            None => panic!("Overflow calculating total magic table size!"),
        };
        sq_idx += 1;
    }

    total_size
}

const RAW_TABLE: ([Bitboard; TOTAL_POSITIONS], [usize; 64], [usize; 64]) = {
    let mut positions = [Bitboard(0); TOTAL_POSITIONS];
    let mut rook_offsets = [0usize; 64];
    let mut bishop_offsets = [0usize; 64];
    let mut current_offset: usize = 0;

    let mut pos = IndexPosition(0);
    while pos.0 < 64 {
        rook_offsets[pos.0] = current_offset;
        let blocker = blockers::ROOK_BLOCKERS[pos.0];
        let blocker_count = blocker.0.count_ones() as usize;
        let shift = shifts::ROOK_SHIFTS[pos.0];
        let magic = magics::ROOK_MAGICS[pos.0];

        let mut subset_idx: usize = 0;
        let mut blockers = Bitboard(0);
        while subset_idx < blocker_count {
            let attack = sliding_targets::get_rook_positions(pos, blockers);
            let magic_index = ((blockers.0).wrapping_mul(magic) >> shift) as usize;
            positions[current_offset + magic_index] = attack;
            blockers = Bitboard(blockers.0.wrapping_sub(blocker.0) & blocker.0);
            subset_idx += 1;
        }
        current_offset += blocker_count;
        pos.0 += 1;
    }

    pos = IndexPosition(0);
    while pos.0 < 64 {
        bishop_offsets[pos.0] = current_offset;
        let blocker = blockers::BISHOP_BLOCKERS[pos.0];
        let blocker_count = blocker.0.count_ones() as usize;
        let shift = shifts::BISHOP_SHIFTS[pos.0];
        let magic = magics::BISHOP_MAGICS[pos.0];

        let mut subset_idx: usize = 0;
        let mut blockers = Bitboard(0);
        while subset_idx < blocker_count {
            let attack = sliding_targets::get_bishop_positions(pos, blockers);
            let magic_index = ((blockers.0).wrapping_mul(magic) >> shift) as usize;
            positions[current_offset + magic_index] = attack;
            blockers = Bitboard(blockers.0.wrapping_sub(blocker.0) & blocker.0);
            subset_idx += 1;
        }
        current_offset += blocker_count;
        pos.0 += 1;
    }

    (positions, rook_offsets, bishop_offsets)
};

pub const POSITIONS_TABLE: [Bitboard; TOTAL_POSITIONS] = RAW_TABLE.0;
pub const ROOK_OFFSETS_DATA: [usize; 64] = RAW_TABLE.1;
pub const BISHOP_OFFSETS_DATA: [usize; 64] = RAW_TABLE.2;
