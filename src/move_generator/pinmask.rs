use crate::move_generator::between::IN_BETWEEN;
use crate::move_generator::sliding_targets::{get_bishop_targets, get_rook_targets};
use crate::prelude::*;

/// Calculates two Pin mask one for horizotal and vertical (hv) and one for diagnoals (diag)
/// Explained here unter section pinmask: https://www.codeproject.com/Articles/5313417/Worlds-fastest-Bitboard-Chess-Movegenerator
/// https://www.chessprogramming.org/Pin
pub fn generate_pin_masks(board: &Board) -> (Bitboard, Bitboard) {
    let mut pin_hv = Bitboard::EMPTY;
    let mut pin_diag = Bitboard::EMPTY;

    let friendly = board.current_color();
    let enemy = !friendly;
    let king_bb = board.king(friendly);
    let king_sq = king_bb.to_square();
    let occ = board.occupied();
    let friendly_bb = board.color_bbs(friendly);

    // all enemy rooks and queens
    let enemy_rq = board.figure_bb(enemy, Rook) | board.figure_bb(enemy, Queen);
    // all enemy bishops and queens
    let enemy_bq = board.figure_bb(enemy, Bishop) | board.figure_bb(enemy, Queen);

    // which sliders can see the own king through xray
    let mut hv_sliders = get_rook_xray_targets(king_sq, occ, friendly_bb) & enemy_rq;
    let mut diag_sliders = get_bishop_xray_targets(king_sq, occ, friendly_bb) & enemy_bq;

    // for these sliders add themself and der path to king (exclusive) to the pinmask
    for slider_bb in hv_sliders.iter_mut() {
        let sq = slider_bb.to_square();
        let between = IN_BETWEEN[sq.i()][king_sq.i()];
        pin_hv |= between | slider_bb;
    }
    for slider_bb in diag_sliders.iter_mut() {
        let sq = slider_bb.to_square();
        let between = IN_BETWEEN[sq.i()][king_sq.i()];
        pin_diag |= between | slider_bb;
    }

    (pin_hv, pin_diag)
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
