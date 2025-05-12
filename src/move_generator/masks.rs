use super::between::IN_BETWEEN;
use super::normal_targets::KING_TARGETS;
use super::normal_targets::KNIGHT_TARGETS;
use super::normal_targets::PAWN_ATTACK_TARGETS;
use super::sliding_targets::get_bishop_targets;
use super::sliding_targets::get_rook_targets;
use crate::prelude::*;

pub fn calc_check_mask(board: &Board) -> (Bitboard, usize) {
    let friendly = board.current_color;
    let enemy = !friendly;
    let mut checkmask = Bitboard::EMPTY;
    let mut sliding_attackers = Bitboard::EMPTY;
    let occ = board.occupied;
    let king = board.get_king(friendly).to_square();

    let enemy_pawns = board.get_pieces_by_color(enemy, Pawn);
    let enemy_knights = board.get_pieces_by_color(enemy, Knight);
    let enemy_bishops_queens =
        board.get_pieces_by_color(enemy, Bishop) | board.get_pieces_by_color(enemy, Queen);
    let enemy_rooks_queens =
        board.get_pieces_by_color(enemy, Rook) | board.get_pieces_by_color(enemy, Queen);
    // enemy king is ignored because it cannot give check

    // friendly here is correct because we are pretending our king is a pawn and is attacking
    // if it finds an enemy pawn on its attack squares then this pawn could attack our king
    checkmask |= PAWN_ATTACK_TARGETS[friendly as usize][king] & enemy_pawns;
    checkmask |= KNIGHT_TARGETS[king] & enemy_knights;
    sliding_attackers |= get_bishop_targets(king, occ) & enemy_bishops_queens;
    sliding_attackers |= get_rook_targets(king, occ) & enemy_rooks_queens;

    // No check all positions are valid fill the board
    if (checkmask | sliding_attackers).is_empty() {
        return (Bitboard::FULL, 0);
    }
    let mut check_counter = 0usize;
    check_counter += checkmask.0.count_ones() as usize;
    for attacker in sliding_attackers.iter_mut() {
        checkmask |= IN_BETWEEN[king][attacker.to_square()] | attacker;
        check_counter += 1;
    }
    (checkmask, check_counter)
}

/// The captured removed_pawn should be Bit(0) by default
/// It is only needed when a pawn should be removed from the enemy pawns to cover an happening ep move
/// See this edge case here https://lichess.org/editor/8/8/8/1Ppp3r/RK3p1k/8/4P1P1/8_w_-_c6_0_5?color=white
pub fn calculate_attackmask(
    board: &Board,
    occupied: Bitboard,
    attacker: Color,
    removed_pawn: Option<Bit>,
) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;

    let mut enemy_pawns = board.get_pieces_by_color(attacker, Pawn);
    if let Some(bit) = removed_pawn {
        enemy_pawns &= !bit; // Handle ep edge case
    }
    let mut enemy_knights = board.get_pieces_by_color(attacker, Knight);
    let mut enemy_bishops_queens =
        board.get_pieces_by_color(attacker, Bishop) | board.get_pieces_by_color(attacker, Queen);
    let mut enemy_rooks_queens =
        board.get_pieces_by_color(attacker, Rook) | board.get_pieces_by_color(attacker, Queen);
    let enemy_king = board.get_king(attacker);

    for pawn in enemy_pawns.iter_mut() {
        attacks |= PAWN_ATTACK_TARGETS[attacker as usize][pawn.to_square()];
    }

    for knight in enemy_knights.iter_mut() {
        attacks |= KNIGHT_TARGETS[knight.to_square()];
    }

    for bishop_queen in enemy_bishops_queens.iter_mut() {
        attacks |= get_bishop_targets(bishop_queen.to_square(), occupied);
    }

    for rook_queen in enemy_rooks_queens.iter_mut() {
        attacks |= get_rook_targets(rook_queen.to_square(), occupied);
    }

    attacks |= KING_TARGETS[enemy_king.to_square()];

    attacks
}
