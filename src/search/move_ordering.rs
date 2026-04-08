use crate::prelude::*;
use crate::search::move_picker::MoveList;
use crate::search::mvv_lva::MVV_LVA_TABLE;
use crate::settings;

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

#[cfg(test)]
mod tests {
    use crate::search::move_picker::MovePicker;

    use super::*;

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
}
