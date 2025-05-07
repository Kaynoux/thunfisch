use crate::move_generator::between::IN_BETWEEN;
use crate::move_generator::sliding_targets::{get_bishop_targets, get_rook_targets};
use crate::prelude::*;

/// Calculates a mask where every square is which pinned is set
/// https://www.chessprogramming.org/Pin
pub fn generate_pin_masks(board: &Board) -> (Bitboard, Bitboard) {
    let mut pinned_hv = Bitboard::EMPTY; // pinned horizontally and vertically from rook or queen
    let mut pinned_diag = Bitboard::EMPTY; // pinned diagonally from bishop or queen
    let friendly_color = board.current_color;
    let enemy_color = !friendly_color;
    let king_bit = board.get_king(friendly_color);
    let king_sq = king_bit.to_square();
    let occ = board.occupied;
    let friendly_pieces = board.get_pieces(friendly_color);
    let enemy_rooks_queens = board.get_bitboard_by_piece_color(enemy_color, Rook)
        | board.get_bitboard_by_piece_color(enemy_color, Queen);

    let enemy_bishops_queens = board.get_bitboard_by_piece_color(enemy_color, Bishop)
        | board.get_bitboard_by_piece_color(enemy_color, Queen);

    let mut hv_pos_from_king =
        get_rook_xray_targets(king_sq, occ, friendly_pieces) & enemy_rooks_queens;

    let mut diag_pos_from_king =
        get_bishop_xray_targets(king_sq, occ, friendly_pieces) & enemy_bishops_queens;

    for bit in hv_pos_from_king.iter_mut() {
        pinned_hv |= IN_BETWEEN[bit.to_square().i()][king_sq.i()] & friendly_pieces;
    }
    for bit in diag_pos_from_king.iter_mut() {
        pinned_diag |= IN_BETWEEN[bit.to_square().i()][king_sq.i()] & friendly_pieces;
    }

    (pinned_hv, pinned_diag)
}

/// calculate squares which will be attacked by the rook when the blockers will be seen as transparent
/// https://www.chessprogramming.org/X-ray_Attacks_(Bitboards)
fn get_rook_xray_targets(square: Square, occ: Bitboard, blockers: Bitboard) -> Bitboard {
    let targets = get_rook_targets(square, occ); // normal rook targets
    targets ^ get_rook_targets(square, occ ^ (targets & blockers)) // removes the blockers from the target calc and removes all prev targets so only the new ones stay
}

/// calculate squares which will be attacked by the bishop when the blockers will be seen as transparent
/// https://www.chessprogramming.org/X-ray_Attacks_(Bitboards)
fn get_bishop_xray_targets(square: Square, occ: Bitboard, blockers: Bitboard) -> Bitboard {
    let targets = get_bishop_targets(square, occ);
    targets ^ get_bishop_targets(square, occ ^ (targets & blockers))
}
