use crate::{move_picker::MoveList, prelude::*, settings};
use std::sync::atomic::{AtomicI32, Ordering};

// These values or known to perform well
const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 300;
const BISHOP_VALUE: i32 = 320;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;
const KING_VALUE: i32 = 10000;

const ORDERING_OFFSET: i32 = 10000;
const ORDERING_MULTIPLIER: i32 = 100;

const PIECE_VALUES: [i32; 6] = [
    PAWN_VALUE,
    KNIGHT_VALUE,
    BISHOP_VALUE,
    ROOK_VALUE,
    QUEEN_VALUE,
    KING_VALUE,
];

/// Constants for the gravity history increase.
/// See history.rs for details on usage.
/// Values are inspired by viridithas.
pub const HISTORY_BONUS_MUL: i32 = 355;
pub const HISTORY_BONUS_OFFS: i32 = 230;
pub const HISTORY_BONUS_MAX: i32 = 2222;
pub const HISTORY_MALUSE_MUL: i32 = 110;
pub const HISTORY_MALUSE_OFFS: i32 = 515;
pub const HISTORY_MALUSE_MAX: i32 = 900;

/// Transposition Table shared between all search threads
pub static HISTORY_TABLE: std::sync::LazyLock<HistoryTable> =
    std::sync::LazyLock::new(HistoryTable::new);

const fn calculate_mvv_lva_score(victim_idx: usize, attacker_idx: usize) -> i32 {
    // King cannot be captured
    if victim_idx >= 5 {
        return 0;
    }

    let victim_value = PIECE_VALUES[victim_idx];
    let attacker_value = PIECE_VALUES[attacker_idx];

    victim_value * ORDERING_MULTIPLIER - attacker_value + ORDERING_OFFSET
}

/// <https://www.chessprogramming.org/MVV-LVA>
const MVV_LVA_TABLE: [[i32; 6]; 6] = {
    let mut table = [[0i32; 6]; 6];
    let mut attacker_idx = 0;
    while attacker_idx < 6 {
        let mut victim_idx = 0;
        // < 5 because king can be ignored
        while victim_idx < 5 {
            table[attacker_idx][victim_idx] = calculate_mvv_lva_score(victim_idx, attacker_idx);
            victim_idx += 1;
        }
        attacker_idx += 1;
    }
    table
};

const CAPTURE_BONUS: i32 = 1024;

/// Score capture moves based on the value of the capturing piece to the captured piece
/// For example a pawn capturing a queen gets a higher score than a queen capturing a rook
/// <https://www.chessprogramming.org/Move_Ordering>
pub fn mvv_lva(move_list: &mut MoveList, board: &Board) {
    if !settings::MVV_LVA {
        return;
    }
    for entry in &mut move_list.list {
        let mv = entry.mv.decode();
        let mv_type = mv.mv_type;
        entry.score = match mv_type {
            MoveType::Capture => {
                let attacker_idx = (board.figures(mv.from) as usize) / 2;
                let victim_idx = (board.figures(mv.to) as usize) / 2;
                MVV_LVA_TABLE[attacker_idx][victim_idx] + CAPTURE_BONUS
            }
            MoveType::EpCapture => {
                let attacker_idx = (board.figures(mv.from) as usize) / 2;
                MVV_LVA_TABLE[attacker_idx][Pawn as usize] + CAPTURE_BONUS
            }
            MoveType::QueenPromoCapture => {
                let attacker_idx = (board.figures(mv.from) as usize) / 2;
                let victim_idx = (board.figures(mv.to) as usize) / 2;
                MVV_LVA_TABLE[attacker_idx][victim_idx]
                    + CAPTURE_BONUS
                    + MVV_LVA_TABLE[Pawn as usize][Queen as usize]
            }
            MoveType::RookPromoCapture => {
                let attacker_idx = (board.figures(mv.from) as usize) / 2;
                let victim_idx = (board.figures(mv.to) as usize) / 2;
                MVV_LVA_TABLE[attacker_idx][victim_idx]
                    + CAPTURE_BONUS
                    + MVV_LVA_TABLE[Pawn as usize][Rook as usize]
            }
            MoveType::BishopPromoCapture => {
                let attacker_idx = (board.figures(mv.from) as usize) / 2;
                let victim_idx = (board.figures(mv.to) as usize) / 2;
                MVV_LVA_TABLE[attacker_idx][victim_idx]
                    + CAPTURE_BONUS
                    + MVV_LVA_TABLE[Pawn as usize][Bishop as usize]
            }
            MoveType::KnightPromoCapture => {
                let attacker_idx = (board.figures(mv.from) as usize) / 2;
                let victim_idx = (board.figures(mv.to) as usize) / 2;
                MVV_LVA_TABLE[attacker_idx][victim_idx]
                    + CAPTURE_BONUS
                    + MVV_LVA_TABLE[Pawn as usize][Knight as usize]
            }
            MoveType::QueenPromo => MVV_LVA_TABLE[Pawn as usize][Queen as usize],
            MoveType::RookPromo => MVV_LVA_TABLE[Pawn as usize][Rook as usize],
            MoveType::BishopPromo => MVV_LVA_TABLE[Pawn as usize][Bishop as usize],
            MoveType::KnightPromo => MVV_LVA_TABLE[Pawn as usize][Knight as usize],
            _ => 0,
        }
    }
}

/// Score quiet moves using history heuristics (and in the future potentially more)
pub fn score_quiets(move_list: &mut MoveList, board: &Board) {
    if !settings::HISTORIES {
        return;
    }
    let current_color = board.current_color();
    for m in &mut move_list.list {
        m.score = HISTORY_TABLE.get_score(m.mv.decode(), current_color);
    }
}
///////////////////////////////////////////////////////////////////////////////////////////////////
// Histories

const MAX_HISTORY_VALUE: i32 = i16::MAX as i32;

/// vectors are two-dimensional arrays indexed by `[from_square][to_square]`
pub struct HistoryTable([[AtomicI32; 64]; 64], [[AtomicI32; 64]; 64]);

// impl Default for HistoryTable {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl HistoryTable {
    pub fn new() -> Self {
        Self(
            std::array::from_fn(|_| std::array::from_fn(|_| AtomicI32::new(0))),
            std::array::from_fn(|_| std::array::from_fn(|_| AtomicI32::new(0))),
        )
    }

    /// Update the history value for `mv` at `depth` for `color` by `bonus`.
    /// The bonus should be calculated by either `history_bonus` for bonuses, and
    /// `history_maluse` for history maluse punishments.
    pub fn update_history(&self, color: Color, mv: DecodedMove, bonus: i32) {
        let fro = mv.from.0;
        let to = mv.to.0;
        match color {
            White => {
                let old = self.0[fro][to].load(Ordering::Relaxed);
                let new = gravity(old, bonus);
                self.0[fro][to].store(new, Ordering::Relaxed);
            }
            Black => {
                let old = self.1[fro][to].load(Ordering::Relaxed);
                let new = gravity(old, bonus);
                self.1[fro][to].store(new, Ordering::Relaxed);
            }
        }
    }

    pub fn get_score(&self, mv: DecodedMove, color: Color) -> i32 {
        let fro = mv.from.0;
        let to = mv.to.0;
        match color {
            White => self.0[fro][to].load(Ordering::Relaxed),
            Black => self.1[fro][to].load(Ordering::Relaxed),
        }
    }

    #[allow(dead_code)]
    pub fn get_relative_history(&self, _mv: DecodedMove, _color: Color) -> i32 {
        todo!()
    }

    /// Age history values between search iterations
    /// I have no idea why this is useful, but the Relative History Paper (Winands et. al.) suggests it
    /// and apparently Histories lose ELO without it
    pub fn age(&self) {
        for inner in &self.0 {
            for h in inner {
                let old = h.load(Ordering::Relaxed);
                h.store(old / 2, Ordering::Relaxed);
            }
        }
        for inner in &self.1 {
            for h in inner {
                let old = h.load(Ordering::Relaxed);
                h.store(old / 2, Ordering::Relaxed);
            }
        }
    }

    pub fn clear(&self) {
        for inner in &self.0 {
            for h in inner {
                h.store(0, Ordering::Relaxed);
            }
        }
        for inner in &self.1 {
            for h in inner {
                h.store(0, Ordering::Relaxed);
            }
        }
    }
}

#[inline]
fn gravity(val: i32, bonus: i32) -> i32 {
    i32::clamp(
        val + bonus - val * bonus.abs() / MAX_HISTORY_VALUE,
        -MAX_HISTORY_VALUE,
        MAX_HISTORY_VALUE,
    )
}

#[inline]
pub fn history_bonus(depth: usize) -> i32 {
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    i32::min(
        HISTORY_BONUS_MUL * depth as i32 + HISTORY_BONUS_OFFS,
        HISTORY_BONUS_MAX,
    )
}

#[inline]
pub fn history_maluse(depth: usize) -> i32 {
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    -i32::min(
        HISTORY_MALUSE_MUL * depth as i32 + HISTORY_MALUSE_OFFS,
        HISTORY_MALUSE_MAX,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{move_picker::MovePicker, settings};

    #[test]
    fn test_mvv_lva_captures() {
        // Skip test if setting is disabled, otherwise it naturally fails because no sorting happens
        if !settings::MVV_LVA {
            return;
        }

        // Setup a position where White has multiple valid captures:
        // 1. Pawn on d3 can capture Queen on e4 (PxQ - high priority)
        // 2. Rook on e2 can capture Queen on e4 (RxQ - lower priority)
        let mut board = Board::new("4k3/8/8/8/4q3/3P4/4R3/4K3 w - - 0 1");
        let mut move_picker = MovePicker::new(None, None, false);
        let best_move = move_picker.next(&mut board).unwrap().decode();
        let second_best = move_picker.next(&mut board).unwrap().decode();

        // The first move should be a capture
        assert_eq!(best_move.mv_type, MoveType::Capture);

        // Both d3->e4 and e2->e4 are captures. PxQ is strictly better than RxQ in MVV-LVA.
        // We can verify this by looking at the attacker figure (Pawn < Rook)
        let attacker_best = board.figures(best_move.from);
        let attacker_second = board.figures(second_best.from);

        // The pawn should be the attacker of the best move
        assert_eq!((attacker_best as usize) / 2, Pawn as usize);

        // Ensure the second move is also a capture but with a higher value attacker (Rook)
        assert_eq!(second_best.mv_type, MoveType::Capture);
        assert_eq!((attacker_second as usize) / 2, Rook as usize);
    }

    use std::collections::HashMap;

    #[test]
    fn print_mvva_lva_table() {
        let mut value_dist: HashMap<i32, usize> = HashMap::new();
        for &mvv_lva_val in MVV_LVA_TABLE.iter().flatten() {
            *value_dist.entry(mvv_lva_val).or_insert(0) += 1;
        }
        let mut value_dist = value_dist.iter().collect::<Vec<(&i32, &usize)>>();
        value_dist.sort_by_key(|&(key, _)| key);
        value_dist.sort_by_key(|&(_, val)| val);
        println!("{:?}", value_dist.iter());

        // This is the output:
        // [(10000, 1), (19100, 1), (19500, 1), (19680, 1), (19700, 1), (19900, 1),
        //  (30000, 1), (32000, 1), (39100, 1), (39500, 1), (39680, 1), (39700, 1),
        //  (39900, 1), (41100, 1), (41500, 1), (41680, 1), (41700, 1), (41900, 1),
        //  (50000, 1), (59100, 1), (59500, 1), (59680, 1), (59700, 1), (59900, 1),
        //  (90000, 1), (99100, 1), (99500, 1), (99680, 1), (99700, 1), (99900, 1),
        //  (0, 6)]
    }
}
