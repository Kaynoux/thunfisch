use crate::prelude::*;
use crate::search::mvv_lva::MVV_LVA_TABLE;
use std::cmp::Reverse;

const CAPTURE_BONUS: i32 = 1024;

pub fn order_moves(moves: &mut Vec<EncodedMove>, board: &Board) {
    moves.sort_unstable_by_key(|encoded_mv| {
        let mv = encoded_mv.decode();
        let mv_type = mv.mv_type;
        let score = match mv_type {
            MoveType::Quiet => 0i32,

            MoveType::Capture => {
                let attacker_idx = (board.pieces[mv.from.0] as usize) / 2;
                let victim_idx = (board.pieces[mv.to.0] as usize) / 2;
                MVV_LVA_TABLE[attacker_idx][victim_idx] + CAPTURE_BONUS
            }
            MoveType::DoubleMove => 0i32,
            MoveType::KingCastle => 0i32,
            MoveType::QueenCastle => 0i32,
            MoveType::EpCapture => {
                let attacker_idx = (board.pieces[mv.from.0] as usize) / 2;
                MVV_LVA_TABLE[attacker_idx][Pawn as usize] + CAPTURE_BONUS
            }
            MoveType::QueenPromoCapture => {
                let attacker_idx = (board.pieces[mv.from.0] as usize) / 2;
                let victim_idx = (board.pieces[mv.to.0] as usize) / 2;
                MVV_LVA_TABLE[attacker_idx][victim_idx]
                    + CAPTURE_BONUS
                    + MVV_LVA_TABLE[Pawn as usize][Queen as usize]
            }
            MoveType::RookPromoCapture => {
                let attacker_idx = (board.pieces[mv.from.0] as usize) / 2;
                let victim_idx = (board.pieces[mv.to.0] as usize) / 2;
                MVV_LVA_TABLE[attacker_idx][victim_idx]
                    + CAPTURE_BONUS
                    + MVV_LVA_TABLE[Pawn as usize][Rook as usize]
            }
            MoveType::BishopPromoCapture => {
                let attacker_idx = (board.pieces[mv.from.0] as usize) / 2;
                let victim_idx = (board.pieces[mv.to.0] as usize) / 2;
                MVV_LVA_TABLE[attacker_idx][victim_idx]
                    + CAPTURE_BONUS
                    + MVV_LVA_TABLE[Pawn as usize][Bishop as usize]
            }
            MoveType::KnightPromoCapture => {
                let attacker_idx = (board.pieces[mv.from.0] as usize) / 2;
                let victim_idx = (board.pieces[mv.to.0] as usize) / 2;
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
}
