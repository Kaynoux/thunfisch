use crate::{
    move_generator::{
        between::IN_BETWEEN,
        normal_targets::{KING_TARGETS, KNIGHT_TARGETS, PAWN_ATTACK_TARGETS},
        sliding_targets::{get_bishop_targets, get_rook_targets},
    },
    prelude::*,
};

/// NOTE: is all ones when there are no checks
pub fn calc_check_mask(board: &Board) -> (Bitboard, usize) {
    let friendly = board.current_color();
    let enemy = !friendly;
    let mut checkmask = Bitboard::EMPTY;
    let mut sliding_attackers = Bitboard::EMPTY;
    let occ = board.occupied();
    let king = board.king(friendly).to_square();

    let enemy_pawns = board.figure_bb(enemy, Pawn);
    let enemy_knights = board.figure_bb(enemy, Knight);
    let enemy_bishops_queens = board.figure_bb(enemy, Bishop) | board.figure_bb(enemy, Queen);
    let enemy_rooks_queens = board.figure_bb(enemy, Rook) | board.figure_bb(enemy, Queen);
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

/// The captured `removed_pawn` should be None by default
/// It is only needed when a pawn should be removed from the enemy pawns to cover an happening ep move
/// See this edge case here <https://lichess.org/editor/8/8/8/1Ppp3r/RK3p1k/8/4P1P1/8_w>_-_`c6_0_5?color=white`
pub fn calculate_attackmask(
    board: &Board,
    occupied: Bitboard,
    attacker: Color,
    removed_pawn: Option<Bit>,
) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;

    let mut enemy_pawns = board.figure_bb(attacker, Pawn);
    if let Some(bit) = removed_pawn {
        enemy_pawns &= !bit; // Handle ep edge case
    }
    let mut enemy_knights = board.figure_bb(attacker, Knight);
    let mut enemy_bishops_queens =
        board.figure_bb(attacker, Bishop) | board.figure_bb(attacker, Queen);
    let mut enemy_rooks_queens = board.figure_bb(attacker, Rook) | board.figure_bb(attacker, Queen);
    let enemy_king = board.king(attacker);

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

/// Note: unlike the regular attackmask, if a piece can take a piece of its own colour, this is NOT counted as an attack
/// it would yield incorrect results for mobility evaluation (which this is used for)
/// For the generic attackmask this makes sense however, as an occupied square is still dangerous for the king
pub fn calculate_attackmask_by_figure(
    board: &Board,
    occupied: Bitboard,
    figure: usize,
    removed_pawn: Option<Bit>,
) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;

    let color = Color::from_usize(figure & 1);

    match figure {
        // pawns
        0 | 1 => {
            let mut pawns = board.figure_bb(color, Pawn);
            if let Some(bit) = removed_pawn {
                pawns &= !bit;
            }
            for pawn in pawns.iter_mut() {
                attacks |= PAWN_ATTACK_TARGETS[color as usize][pawn.to_square()];
            }
        }
        // knights
        2 | 3 => {
            for knight in board.figure_bb(color, Knight).iter_mut() {
                attacks |= KNIGHT_TARGETS[knight.to_square()];
            }
        }
        // bishops
        4 | 5 => {
            for bishop in board.figure_bb(color, Bishop).iter_mut() {
                attacks |= get_bishop_targets(bishop.to_square(), occupied);
            }
        }
        // rooks
        6 | 7 => {
            for rook in board.figure_bb(color, Rook).iter_mut() {
                attacks |= get_rook_targets(rook.to_square(), occupied);
            }
        }
        // queens
        8 | 9 => {
            for queen in board.figure_bb(color, Queen).iter_mut() {
                attacks |= get_bishop_targets(queen.to_square(), occupied);
                attacks |= get_rook_targets(queen.to_square(), occupied);
            }
        }
        // kings
        10 | 11 => {
            let king = board.king(color);
            attacks |= KING_TARGETS[king.to_square()];
        }
        _ => unreachable!(),
    }

    attacks & !board.color_bbs(color)
}

// TODO: not pass entire board
/// i'm not entirely sure whether this type of mask actually is what CPW means
/// I think it's a bit too narrow, maybe I should try giving it one more file towards the center as well and then compare the results
/// but that seems like a lot of effort given i'd likely have to tune both
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn king_safety_mask(board: &Board, color: Color) -> Bitboard {
    // let mut mask = board.figure_bb(color, Piece::King);
    let king = board.king(color);
    let (x, y) = (king.to_x() as i16, king.to_y() as u16);
    let mut mask = Bitboard::EMPTY;
    let file_template = Bitboard(0xff)
        & (Bitboard::file(x) | Bitboard::file((x - 1).max(0)) | Bitboard::file((x + 1).min(7)));
    mask |= file_template << (8 * y);
    mask |= file_template << (8 * (y.saturating_sub(1)));
    mask |= file_template << (8 * (y + 1).min(7));
    // duplicate the created mask 2 ranks toward the center
    mask |= match y {
        0..=3 => mask << 8,
        4..=7 => mask >> 8,
        _ => unreachable!(),
    };

    mask
}
