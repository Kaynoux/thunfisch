use crate::move_generator::generator::MAX_MOVES_COUNT;
use crate::prelude::*;
use crate::search::mvv_lva::MVV_LVA_TABLE;
use crate::settings::settings;
use arrayvec::ArrayVec;
use std::cmp::Reverse;

const CAPTURE_BONUS: i32 = 1024;
// I've experimented a bit with placing killer moves within capture moves
// However just throwing them at the end seems to work best
const KILLER_SCORE: i32 = 0 + CAPTURE_BONUS;

/// If we sort good moves to the beginning we can increase cut offs in alpha beta
/// https://www.chessprogramming.org/Move_Ordering
/// TODO replace sort completly by a move picker instead
pub fn order_moves(
    moves: &mut ArrayVec<EncodedMove, MAX_MOVES_COUNT>,
    board: &Board,
    tt_mv: Option<EncodedMove>,
    killer_mv: Option<EncodedMove>,
) {
    if settings::MVV_LVA {
        moves.sort_unstable_by_key(|encoded_mv| {
            if settings::ORDER_TT_MV_FIRST {
                // give highest score if mv is the tt mv
                if Some(*encoded_mv) == tt_mv {
                    return Reverse(i32::MAX);
                }
            }

            if settings::KILLERS {
                if Some(*encoded_mv) == killer_mv {
                    return Reverse(KILLER_SCORE);
                }
            }

            let mv = encoded_mv.decode();
            let mv_type = mv.mv_type;
            let score = match mv_type {
                MoveType::Quiet => 0i32,

                MoveType::Capture => {
                    let attacker_idx = (board.figures(mv.from) as usize) / 2;
                    let victim_idx = (board.figures(mv.to) as usize) / 2;
                    MVV_LVA_TABLE[attacker_idx][victim_idx] + CAPTURE_BONUS
                }
                MoveType::DoubleMove => 0i32,
                MoveType::KingCastle => 0i32,
                MoveType::QueenCastle => 0i32,
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
                MoveType::RookPromo => MVV_LVA_TABLE[Pawn as usize][Queen as usize],
                MoveType::BishopPromo => MVV_LVA_TABLE[Pawn as usize][Queen as usize],
                MoveType::KnightPromo => MVV_LVA_TABLE[Pawn as usize][Queen as usize],
            };

            // sort descending by highest value first
            Reverse(score)
        });
    } else {
        if settings::ORDER_TT_MV_FIRST
            && let Some(tt_mv) = tt_mv
        {
            if let Some(pos) = moves.iter().position(|&m| m == tt_mv) {
                moves[0..=pos].rotate_right(1);
            }
        }
        if settings::KILLERS
            && let Some(killer_mv) = killer_mv
        {
            if let Some(pos) = moves.iter().position(|&m| m == killer_mv) {
                let target_pos = if settings::ORDER_TT_MV_FIRST { 1 } else { 0 };
                moves[target_pos..=pos].rotate_right(1);
            }
        }
    }
}

/// RE-implementation of order_moves above, which reverts it back to ONLY doing MVV_LVA.
/// Returns: true if the TT move is sorted first, false if the TT move wasn't in `moves` (necessary to avoid yielding the TT move twice in the move picker)
pub fn mvv_lva(
    moves: &mut ArrayVec<EncodedMove, MAX_MOVES_COUNT>,
    board: &Board,
    tt_mv: Option<EncodedMove>,
) -> bool {
    if !settings::MVV_LVA {
        return false;
    }
    let mut tt_move_occurred = false;
    moves.sort_unstable_by_key(|encoded_mv| {
        // give highest score if mv is the tt mv
        if Some(*encoded_mv) == tt_mv {
            tt_move_occurred = true;
            return Reverse(i32::MAX);
        }
        let mv = encoded_mv.decode();
        let mv_type = mv.mv_type;
        let score = match mv_type {
            MoveType::Quiet => {
                #[cfg(debug_assertions)]
                panic!("Quiet moves should not be scored in mvv_lva");
                #[cfg(not(debug_assertions))]
                0i32
            }

            MoveType::Capture => {
                let attacker_idx = (board.figures(mv.from) as usize) / 2;
                let victim_idx = (board.figures(mv.to) as usize) / 2;
                MVV_LVA_TABLE[attacker_idx][victim_idx] + CAPTURE_BONUS
            }
            MoveType::DoubleMove => 0i32,
            MoveType::KingCastle => 0i32,
            MoveType::QueenCastle => 0i32,
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
            MoveType::RookPromo => MVV_LVA_TABLE[Pawn as usize][Queen as usize],
            MoveType::BishopPromo => MVV_LVA_TABLE[Pawn as usize][Queen as usize],
            MoveType::KnightPromo => MVV_LVA_TABLE[Pawn as usize][Queen as usize],
        };

        // sort descending by highest value first
        Reverse(score)
    });
    return tt_move_occurred;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tt_move_is_first() {
        // Skip test if setting is disabled
        if !settings::ORDER_TT_MV_FIRST {
            return;
        }

        // Setup a simple position
        let mut board = Board::from_fen("4k3/8/8/8/8/8/4P3/4K3 w - - 0 1");
        let mut moves = board.generate_moves_legacy::<false>();

        // Grab a move that is not currently at the front (e.g., the last one)
        let tt_move = moves.last().copied();
        assert!(tt_move.is_some());

        let original_last = tt_move.unwrap();

        // Order moves with the TT move
        order_moves(&mut moves, &board, tt_move, None);

        // The TT move should now be the very first move
        assert_eq!(moves[0], original_last);
    }

    #[test]
    fn test_mvv_lva_captures() {
        // Skip test if setting is disabled, otherwise it naturally fails because no sorting happens
        if !settings::MVV_LVA {
            return;
        }

        // Setup a position where White has multiple valid captures:
        // 1. Pawn on d3 can capture Queen on e4 (PxQ - high priority)
        // 2. Rook on e2 can capture Queen on e4 (RxQ - lower priority)
        let mut board = Board::from_fen("4k3/8/8/8/4q3/3P4/4R3/4K3 w - - 0 1");
        let mut moves = board.generate_moves_legacy::<false>();

        // If MVV_LVA setting is enabled globally/by default, this will run the sort rules
        order_moves(&mut moves, &board, None, None);

        let best_move = moves[0].decode();
        let second_best = moves[1].decode();

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
}
