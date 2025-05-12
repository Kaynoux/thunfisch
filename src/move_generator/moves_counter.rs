use super::masks;
use super::normal_targets;
use super::sliding_targets;
use crate::prelude::*;

pub fn generate_pawn_moves(
    board: &Board,
    friendly: Color,
    hv_pinmask: Bitboard,
    diag_pinmask: Bitboard,
    check_mask: Bitboard,
) -> usize {
    let mut counter = 0;
    let empty = board.get_empty();
    let enemy = board.get_pieces_without_king(!friendly);

    // pawns on the second last row need to promote
    let promotion_row = match friendly {
        Color::White => Bitboard(0xFF000000000000),
        Color::Black => Bitboard(0xFF00),
    };

    // pawns on the second row can double push
    let double_row = match friendly {
        Color::White => Bitboard(0xff00),
        Color::Black => Bitboard(0xff000000000000),
    };

    let all_pawns = board.get_pieces_by_color(friendly, Pawn);
    let pawns_promotion = all_pawns & promotion_row;
    let pawns_double = all_pawns & double_row;
    let normal_pawns = all_pawns & !promotion_row;
    let mut pawns_hv_pinned = normal_pawns & hv_pinmask & !diag_pinmask;
    let mut pawns_diag_pinned = normal_pawns & !hv_pinmask & diag_pinmask;
    let mut pawns_not_pinned = normal_pawns & !hv_pinmask & !diag_pinmask;
    let mut pawns_double_not_pinned = pawns_double & !hv_pinmask & !diag_pinmask;
    let mut pawns_double_hv_pinned = pawns_double & hv_pinmask & !diag_pinmask;
    let mut pawns_promotion_hv_pinned = pawns_promotion & hv_pinmask & !diag_pinmask;
    let mut pawns_promotion_diag_pinned = pawns_promotion & !hv_pinmask & diag_pinmask;
    let mut pawns_promotion_not_pinned = pawns_promotion & !hv_pinmask & !diag_pinmask;

    for from_bit in pawns_not_pinned.iter_mut() {
        let from = from_bit.to_square();

        let potential_quiet_target_1 = normal_targets::pawn_quiet_single_target(from_bit, friendly);
        let quiet_target_1 = potential_quiet_target_1 & empty & check_mask;

        if quiet_target_1 != Bit(0) {
            counter += 1;
        }

        let mut capture_targets =
            normal_targets::PAWN_ATTACK_TARGETS[friendly as usize][from] & enemy & check_mask;
        for to_bit in capture_targets.iter_mut() {
            counter += 1;
        }
    }

    for from_bit in pawns_hv_pinned.iter_mut() {
        let from = from_bit.to_square();

        let potential_target = normal_targets::pawn_quiet_single_target(from_bit, friendly);
        let target_1 = potential_target & empty & check_mask & hv_pinmask;

        if target_1 != Bit(0) {
            counter += 1;
        }
    }

    for from_bit in pawns_diag_pinned.iter_mut() {
        let from = from_bit.to_square();
        let mut capture_targets = normal_targets::PAWN_ATTACK_TARGETS[friendly as usize][from]
            & enemy
            & check_mask
            & diag_pinmask;
        for to_bit in capture_targets.iter_mut() {
            counter += 1;
        }
    }
    for from_bit in pawns_double_not_pinned.iter_mut() {
        let from = from_bit.to_square();

        let between_pos = normal_targets::pawn_quiet_single_target(from_bit, friendly) & empty;

        let to_pos =
            normal_targets::pawn_quiet_double_target(from_bit, friendly) & empty & check_mask;

        if between_pos != Bit(0) && to_pos != Bit(0) {
            counter += 1;
        }
    }

    for from_bit in pawns_double_hv_pinned.iter_mut() {
        let from = from_bit.to_square();

        let between_pos = normal_targets::pawn_quiet_single_target(from_bit, friendly) & empty;

        let to_pos = normal_targets::pawn_quiet_double_target(from_bit, friendly)
            & empty
            & check_mask
            & hv_pinmask;

        if between_pos != Bit(0) && to_pos != Bit(0) {
            counter += 1;
        }
    }

    for from_bit in pawns_promotion_not_pinned.iter_mut() {
        let from = from_bit.to_square();

        let potential_quiet_target_1 = normal_targets::pawn_quiet_single_target(from_bit, friendly);
        let quiet_target_1 = potential_quiet_target_1 & empty & check_mask;

        if quiet_target_1 != Bit(0) {
            let to_1 = quiet_target_1.to_square();
            counter += 4;
        }

        let mut capture_targets =
            normal_targets::PAWN_ATTACK_TARGETS[friendly as usize][from] & enemy & check_mask;
        for to_bit in capture_targets.iter_mut() {
            counter += 4;
        }
    }
    for from_bit in pawns_promotion_hv_pinned.iter_mut() {
        let from = from_bit.to_square();

        let potential_target = normal_targets::pawn_quiet_single_target(from_bit, friendly);
        let target_1 = potential_target & empty & check_mask & hv_pinmask;

        if target_1 != Bit(0) {
            let to_1 = target_1.to_square();
            counter += 4;
        }
    }

    for from_bit in pawns_promotion_diag_pinned.iter_mut() {
        let from = from_bit.to_square();
        let mut capture_targets = normal_targets::PAWN_ATTACK_TARGETS[friendly as usize][from]
            & enemy
            & check_mask
            & diag_pinmask;
        for to_bit in capture_targets.iter_mut() {
            counter += 4;
        }
    }

    counter
}

pub fn generate_knight_moves(
    pinmask: Bitboard,
    friendly: Color,
    board: &Board,
    check_mask: Bitboard,
) -> usize {
    // test-fen https://lichess.org/editor/8/8/8/3n4/8/2N5/8/8_w_-_-_0_1?color=white
    let mut counter = 0;
    let mut knights = board.get_pieces_by_color(friendly, Knight);
    for from_bit in knights.iter_mut() {
        let from = from_bit.to_square();
        if pinmask.is_position_set(from_bit) {
            continue;
        }
        let potential_targets = normal_targets::KNIGHT_TARGETS[from.i()];
        let targets = potential_targets & check_mask; // & checkmask

        let mut quiet_targets = targets & board.get_empty();
        for to_bit in quiet_targets.iter_mut() {
            let to = to_bit.to_square();
            counter += 1;
        }

        let mut capture_targets = targets & board.get_pieces_without_king(!friendly);
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            counter += 1;
        }
    }

    counter
}

pub fn generate_bishop_moves(
    hv_pinmask: Bitboard,
    diag_pinmask: Bitboard,
    friendly: Color,
    board: &Board,
    check_mask: Bitboard,
) -> usize {
    //test-fen https://lichess.org/editor/8/8/8/3b4/8/2B5/8/8_w_-_-_0_1?color=white
    let mut counter = 0;
    let empty = board.get_empty();
    let enemy = board.get_pieces_without_king(!friendly);
    let occupied = board.occupied;

    let bishops = board.get_pieces_by_color(friendly, Bishop);
    let mut bishops_not_pinned = bishops & !diag_pinmask & !hv_pinmask;
    let mut bishops_diag_pinned = bishops & diag_pinmask & !hv_pinmask;

    for from_bit in bishops_not_pinned.iter_mut() {
        let from = from_bit.to_square();
        let potential_targets = sliding_targets::get_bishop_targets(from, occupied);
        let targets = potential_targets & check_mask;

        let mut quiet_targets = targets & empty;
        for to_bit in quiet_targets.iter_mut() {
            let to = to_bit.to_square();
            counter += 1;
        }

        let mut capture_targets = targets & enemy;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            counter += 1;
        }
    }

    for from_bit in bishops_diag_pinned.iter_mut() {
        let from = from_bit.to_square();
        let potential_targets = sliding_targets::get_bishop_targets(from, occupied);
        let targets = potential_targets & check_mask & diag_pinmask;

        let mut quiet_targets = targets & empty;
        for to_bit in quiet_targets.iter_mut() {
            let to = to_bit.to_square();
            counter += 1;
        }

        let mut capture_targets = targets & enemy;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            counter += 1;
        }
    }
    counter
}

pub fn generate_rook_moves(
    hv_pinmask: Bitboard,
    diag_pinmask: Bitboard,
    friendly: Color,
    board: &Board,
    check_mask: Bitboard,
) -> usize {
    // test-fen https://lichess.org/editor/8/8/8/3r4/8/2R5/8/8_w_-_-_0_1?color=white
    let mut counter = 0;
    let empty = board.get_empty();
    let enemy = board.get_pieces_without_king(!friendly);
    let occupied = board.occupied;

    let rooks = board.get_pieces_by_color(friendly, Rook);
    let mut rooks_not_pinned = rooks & !hv_pinmask & !diag_pinmask;
    let mut rooks_hv_pinned = rooks & hv_pinmask & !diag_pinmask;

    for from_bit in rooks_not_pinned.iter_mut() {
        let from = from_bit.to_square();
        let potential_targets = sliding_targets::get_rook_targets(from, occupied);
        let targets = potential_targets & check_mask;

        let mut quiet_targets = targets & empty;
        for to_bit in quiet_targets.iter_mut() {
            let to = to_bit.to_square();
            counter += 1;
        }

        let mut capture_targets = targets & enemy;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            counter += 1;
        }
    }

    for from_bit in rooks_hv_pinned.iter_mut() {
        let from = from_bit.to_square();
        let potential_targets = sliding_targets::get_rook_targets(from, occupied);
        let targets = potential_targets & check_mask & hv_pinmask;

        let mut quiet_targets = targets & empty;
        for to_bit in quiet_targets.iter_mut() {
            let to = to_bit.to_square();
            counter += 1;
        }

        let mut capture_targets = targets & enemy;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            counter += 1;
        }
    }

    counter
}

pub fn generate_queen_moves(
    hv_pinmask: Bitboard,
    diag_pinmask: Bitboard,
    friendly: Color,
    board: &Board,
    check_mask: Bitboard,
) -> usize {
    // test-fen https://lichess.org/editor/8/8/8/3q4/8/2Q5/8/8_w_-_-_0_1?color=white
    let mut counter = 0;
    let empty = board.get_empty();
    let enemy = board.get_pieces_without_king(!friendly);
    let occupied = board.occupied;

    let queens = board.get_pieces_by_color(friendly, Queen);
    let mut quuens_not_pinned = queens & !hv_pinmask & !diag_pinmask;
    let mut queens_hv_pinned = queens & hv_pinmask & !diag_pinmask;
    let mut queens_diag_pinned = queens & !hv_pinmask & diag_pinmask;

    for from_bit in quuens_not_pinned.iter_mut() {
        let from = from_bit.to_square();
        let potential_targets = sliding_targets::get_rook_targets(from, occupied)
            | sliding_targets::get_bishop_targets(from, occupied);
        let targets = potential_targets & check_mask;

        let mut quiet_targets = targets & empty;
        for to_bit in quiet_targets.iter_mut() {
            let to = to_bit.to_square();
            counter += 1;
        }

        let mut capture_targets = targets & enemy;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            counter += 1;
        }
    }

    for from_bit in queens_hv_pinned.iter_mut() {
        let from = from_bit.to_square();
        let potential_targets = sliding_targets::get_rook_targets(from, occupied);
        let targets = potential_targets & check_mask & hv_pinmask;

        let mut quiet_targets = targets & empty;
        for to_bit in quiet_targets.iter_mut() {
            let to = to_bit.to_square();
            counter += 1;
        }

        let mut capture_targets = targets & enemy;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            counter += 1;
        }
    }

    for from_bit in queens_diag_pinned.iter_mut() {
        let from = from_bit.to_square();
        let potential_targets = sliding_targets::get_bishop_targets(from, occupied);
        let targets = potential_targets & check_mask & diag_pinmask;

        let mut quiet_targets = targets & empty;
        for to_bit in quiet_targets.iter_mut() {
            let to = to_bit.to_square();
            counter += 1;
        }

        let mut capture_targets = targets & enemy;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            counter += 1;
        }
    }

    counter
}

pub fn generate_king_move(friendly: Color, board: &Board) -> usize {
    let mut counter = 0;
    let from_bit = board.get_king(friendly);
    let from = from_bit.to_square();
    let occupied_without_king = board.occupied & !from_bit;
    let attackmask = masks::calculate_attackmask(board, occupied_without_king, !friendly);

    let potential_targets = normal_targets::KING_TARGETS[from.i()];
    let targets = potential_targets & !attackmask;

    let mut quiet_targets = targets & board.get_empty();
    for to_bit in quiet_targets.iter_mut() {
        let to = to_bit.to_square();
        counter += 1;
    }

    let mut capture_targets = targets & board.get_pieces_without_king(!friendly);
    for to_bit in capture_targets.iter_mut() {
        let to = to_bit.to_square();
        counter += 1;
    }

    counter
}

pub fn generate_castle_moves(check_counter: usize, friendly: Color, board: &Board) -> usize {
    let mut counter = 0;
    if check_counter != 0 {
        return 0;
    }
    let attackmask = masks::calculate_attackmask(board, board.occupied, !friendly);
    let occupied = board.occupied;
    match friendly {
        Color::White => {
            const NEED_TO_BE_EMPTY_QUEEN: Bitboard = Bitboard::from_idx([1, 2, 3]);
            const NEED_TO_BE_NOT_ATTACKED_QUEEN: Bitboard = Bitboard::from_idx([2, 3]);

            if board.white_queen_castle
                && NEED_TO_BE_EMPTY_QUEEN & occupied == Bitboard::EMPTY
                && attackmask & NEED_TO_BE_NOT_ATTACKED_QUEEN == Bitboard::EMPTY
            {
                counter += 1;
            }
            const NEED_TO_BE_EMPTY_KING: Bitboard = Bitboard::from_idx([5, 6]);
            const NEED_TO_BE_NOT_ATTACKED_KING: Bitboard = Bitboard::from_idx([5, 6]);

            if board.white_king_castle
                && NEED_TO_BE_EMPTY_KING & occupied == Bitboard::EMPTY
                && attackmask & NEED_TO_BE_NOT_ATTACKED_KING == Bitboard::EMPTY
            {
                counter += 1;
            }
        }
        Color::Black => {
            const NEED_TO_BE_EMPTY_QUEEN: Bitboard = Bitboard::from_idx([57, 58, 59]);
            const NEED_TO_BE_NOT_ATTACKED_QUEEN: Bitboard = Bitboard::from_idx([58, 59]);

            if board.black_queen_castle
                && NEED_TO_BE_EMPTY_QUEEN & occupied == Bitboard::EMPTY
                && attackmask & NEED_TO_BE_NOT_ATTACKED_QUEEN == Bitboard::EMPTY
            {
                counter += 1;
            }
            const NEED_TO_BE_EMPTY_KING: Bitboard = Bitboard::from_idx([61, 62]);
            const NEED_TO_BE_NOT_ATTACKED_KING: Bitboard = Bitboard::from_idx([61, 62]);

            if board.black_king_castle
                && NEED_TO_BE_EMPTY_KING & occupied == Bitboard::EMPTY
                && attackmask & NEED_TO_BE_NOT_ATTACKED_KING == Bitboard::EMPTY
            {
                counter += 1;
            }
        }
    }
    counter
}

pub fn generate_ep_moves(
    board: &Board,

    friendly: Color,
    hv_pinmask: Bitboard,
    diag_pinmask: Bitboard,
) -> usize {
    if let Some(ep_target_bit) = board.ep_target {
        let mut counter = 0;
        let ep_target = ep_target_bit.to_square();
        let captured_pawn_bit = match friendly {
            Color::White => ep_target_bit >> 8,
            Color::Black => ep_target_bit << 8,
        };

        let all_pawns = board.get_pieces_by_color(friendly, Pawn);

        let pawns_not_pinned = all_pawns & !hv_pinmask & !diag_pinmask;
        let enemy = !board.current_color;

        // Using !friendly to get the inverse position from the ep target to our pawns
        // so we can check where actually a pawn from us is there
        let mut possible_not_pinned_pawns: Bitboard =
            normal_targets::PAWN_ATTACK_TARGETS[!friendly as usize][ep_target] & pawns_not_pinned;
        for pawn in possible_not_pinned_pawns.iter_mut() {
            let occupied_after_ep = (board.occupied & !pawn & !captured_pawn_bit) | ep_target_bit;
            let attackmask_after_ep = masks::calculate_attackmask(board, occupied_after_ep, enemy);
            // if not Ep would open open up a check from slider after being deleted
            if attackmask_after_ep & board.get_king(friendly) == Bitboard::EMPTY {
                counter += 1;
            }
        }

        if diag_pinmask & captured_pawn_bit != Bitboard(0) {
            let pawns_diag_pinned = all_pawns & !hv_pinmask & diag_pinmask;
            let mut possible_diag_pinned_pawns: Bitboard = normal_targets::PAWN_ATTACK_TARGETS
                [!friendly as usize][ep_target]
                & pawns_diag_pinned;
            for pawn in possible_diag_pinned_pawns.iter_mut() {
                let occupied_after_ep =
                    (board.occupied & !pawn & !captured_pawn_bit) | ep_target_bit;
                let attackmask_after_ep =
                    masks::calculate_attackmask(board, occupied_after_ep, enemy);
                // if not Ep would open open up a check from slider after being deleted
                if attackmask_after_ep & board.get_king(friendly) == Bitboard::EMPTY {
                    counter += 1;
                }
            }
        }
        counter
        // hv pinned pawns are ignored because they cannot do ep capture because it needs a diagnoal move
    } else {
        0
    }
}
