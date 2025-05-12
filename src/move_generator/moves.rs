use super::generator::ARRAY_LENGTH;
use super::masks;
use super::normal_targets;
use super::sliding_targets;
use crate::prelude::*;
use arrayvec::ArrayVec;

pub fn generate_pawn_moves(
    quiet_moves: &mut ArrayVec<EncodedMove, ARRAY_LENGTH>,

    board: &Board,
    friendly: Color,
    hv_pinmask: Bitboard,
    diag_pinmask: Bitboard,
    check_mask: Bitboard,
) {
    let empty = board.empty();
    let enemy = board.color_bbs_without_king(!friendly);

    // pawns on the second last row need to promote
    let promotion_row = match friendly {
        White => Bitboard(0xFF000000000000),
        Black => Bitboard(0xFF00),
    };

    // pawns on the second row can double push
    let double_row = match friendly {
        White => Bitboard(0xff00),
        Black => Bitboard(0xff000000000000),
    };

    let all_pawns = board.figure_bb(friendly, Pawn);
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
            quiet_moves.push(EncodedMove::encode(
                from,
                quiet_target_1.to_square(),
                MoveType::Quiet,
            ));
        }

        let mut capture_targets =
            normal_targets::PAWN_ATTACK_TARGETS[friendly as usize][from] & enemy & check_mask;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
        }
    }

    for from_bit in pawns_hv_pinned.iter_mut() {
        let from = from_bit.to_square();

        let potential_target = normal_targets::pawn_quiet_single_target(from_bit, friendly);
        let target_1 = potential_target & empty & check_mask & hv_pinmask;

        if target_1 != Bit(0) {
            quiet_moves.push(EncodedMove::encode(
                from,
                target_1.to_square(),
                MoveType::Quiet,
            ));
        }
    }

    for from_bit in pawns_diag_pinned.iter_mut() {
        let from = from_bit.to_square();
        let mut capture_targets = normal_targets::PAWN_ATTACK_TARGETS[friendly as usize][from]
            & enemy
            & check_mask
            & diag_pinmask;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
        }
    }
    for from_bit in pawns_double_not_pinned.iter_mut() {
        let from = from_bit.to_square();

        let between_pos = normal_targets::pawn_quiet_single_target(from_bit, friendly) & empty;

        let to_pos =
            normal_targets::pawn_quiet_double_target(from_bit, friendly) & empty & check_mask;

        if between_pos != Bit(0) && to_pos != Bit(0) {
            quiet_moves.push(EncodedMove::encode(
                from,
                to_pos.to_square(),
                MoveType::DoubleMove,
            ));
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
            quiet_moves.push(EncodedMove::encode(
                from,
                to_pos.to_square(),
                MoveType::DoubleMove,
            ));
        }
    }

    for from_bit in pawns_promotion_not_pinned.iter_mut() {
        let from = from_bit.to_square();

        let potential_quiet_target_1 = normal_targets::pawn_quiet_single_target(from_bit, friendly);
        let quiet_target_1 = potential_quiet_target_1 & empty & check_mask;

        if quiet_target_1 != Bit(0) {
            let to_1 = quiet_target_1.to_square();
            quiet_moves.push(EncodedMove::encode(from, to_1, MoveType::QueenPromo));
            quiet_moves.push(EncodedMove::encode(from, to_1, MoveType::RookPromo));
            quiet_moves.push(EncodedMove::encode(from, to_1, MoveType::BishopPromo));
            quiet_moves.push(EncodedMove::encode(from, to_1, MoveType::KnightPromo));
        }

        let mut capture_targets =
            normal_targets::PAWN_ATTACK_TARGETS[friendly as usize][from] & enemy & check_mask;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::QueenPromoCapture));
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::RookPromoCapture));
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::BishopPromoCapture));
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::KnightPromoCapture));
        }
    }
    for from_bit in pawns_promotion_hv_pinned.iter_mut() {
        let from = from_bit.to_square();

        let potential_target = normal_targets::pawn_quiet_single_target(from_bit, friendly);
        let target_1 = potential_target & empty & check_mask & hv_pinmask;

        if target_1 != Bit(0) {
            let to_1 = target_1.to_square();
            quiet_moves.push(EncodedMove::encode(from, to_1, MoveType::QueenPromo));
            quiet_moves.push(EncodedMove::encode(from, to_1, MoveType::RookPromo));
            quiet_moves.push(EncodedMove::encode(from, to_1, MoveType::BishopPromo));
            quiet_moves.push(EncodedMove::encode(from, to_1, MoveType::KnightPromo));
        }
    }

    for from_bit in pawns_promotion_diag_pinned.iter_mut() {
        let from = from_bit.to_square();
        let mut capture_targets = normal_targets::PAWN_ATTACK_TARGETS[friendly as usize][from]
            & enemy
            & check_mask
            & diag_pinmask;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::QueenPromoCapture));
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::RookPromoCapture));
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::BishopPromoCapture));
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::KnightPromoCapture));
        }
    }
}

pub fn generate_knight_moves(
    quiet_moves: &mut ArrayVec<EncodedMove, ARRAY_LENGTH>,

    pinmask: Bitboard,
    friendly: Color,
    board: &Board,
    check_mask: Bitboard,
) {
    // test-fen https://lichess.org/editor/8/8/8/3n4/8/2N5/8/8_w_-_-_0_1?color=white
    let mut knights = board.figure_bb(friendly, Knight);
    for from_bit in knights.iter_mut() {
        let from = from_bit.to_square();
        if pinmask.is_position_set(from_bit) {
            continue;
        }
        let potential_targets = normal_targets::KNIGHT_TARGETS[from.i()];
        let targets = potential_targets & check_mask; // & checkmask

        let mut quiet_targets = targets & board.empty();
        for to_bit in quiet_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Quiet));
        }

        let mut capture_targets = targets & board.color_bbs_without_king(!friendly);
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
        }
    }
}

pub fn generate_bishop_moves(
    quiet_moves: &mut ArrayVec<EncodedMove, ARRAY_LENGTH>,

    hv_pinmask: Bitboard,
    diag_pinmask: Bitboard,
    friendly: Color,
    board: &Board,
    check_mask: Bitboard,
) {
    //test-fen https://lichess.org/editor/8/8/8/3b4/8/2B5/8/8_w_-_-_0_1?color=white
    let empty = board.empty();
    let enemy = board.color_bbs_without_king(!friendly);
    let occupied = board.occupied();

    let bishops = board.figure_bb(friendly, Bishop);
    let mut bishops_not_pinned = bishops & !diag_pinmask & !hv_pinmask;
    let mut bishops_diag_pinned = bishops & diag_pinmask & !hv_pinmask;

    for from_bit in bishops_not_pinned.iter_mut() {
        let from = from_bit.to_square();
        let potential_targets = sliding_targets::get_bishop_targets(from, occupied);
        let targets = potential_targets & check_mask;

        let mut quiet_targets = targets & empty;
        for to_bit in quiet_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Quiet));
        }

        let mut capture_targets = targets & enemy;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
        }
    }

    for from_bit in bishops_diag_pinned.iter_mut() {
        let from = from_bit.to_square();
        let potential_targets = sliding_targets::get_bishop_targets(from, occupied);
        let targets = potential_targets & check_mask & diag_pinmask;

        let mut quiet_targets = targets & empty;
        for to_bit in quiet_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Quiet));
        }

        let mut capture_targets = targets & enemy;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
        }
    }
}

pub fn generate_rook_moves(
    quiet_moves: &mut ArrayVec<EncodedMove, ARRAY_LENGTH>,

    hv_pinmask: Bitboard,
    diag_pinmask: Bitboard,
    friendly: Color,
    board: &Board,
    check_mask: Bitboard,
) {
    // test-fen https://lichess.org/editor/8/8/8/3r4/8/2R5/8/8_w_-_-_0_1?color=white
    let empty = board.empty();
    let enemy = board.color_bbs_without_king(!friendly);
    let occupied = board.occupied();

    let rooks = board.figure_bb(friendly, Rook);
    let mut rooks_not_pinned = rooks & !hv_pinmask & !diag_pinmask;
    let mut rooks_hv_pinned = rooks & hv_pinmask & !diag_pinmask;

    for from_bit in rooks_not_pinned.iter_mut() {
        let from = from_bit.to_square();
        let potential_targets = sliding_targets::get_rook_targets(from, occupied);
        let targets = potential_targets & check_mask;

        let mut quiet_targets = targets & empty;
        for to_bit in quiet_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Quiet));
        }

        let mut capture_targets = targets & enemy;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
        }
    }

    for from_bit in rooks_hv_pinned.iter_mut() {
        let from = from_bit.to_square();
        let potential_targets = sliding_targets::get_rook_targets(from, occupied);
        let targets = potential_targets & check_mask & hv_pinmask;

        let mut quiet_targets = targets & empty;
        for to_bit in quiet_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Quiet));
        }

        let mut capture_targets = targets & enemy;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
        }
    }
}

pub fn generate_queen_moves(
    quiet_moves: &mut ArrayVec<EncodedMove, ARRAY_LENGTH>,

    hv_pinmask: Bitboard,
    diag_pinmask: Bitboard,
    friendly: Color,
    board: &Board,
    check_mask: Bitboard,
) {
    // test-fen https://lichess.org/editor/8/8/8/3q4/8/2Q5/8/8_w_-_-_0_1?color=white
    let empty = board.empty();
    let enemy = board.color_bbs_without_king(!friendly);
    let occupied = board.occupied();

    let queens = board.figure_bb(friendly, Queen);
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
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Quiet));
        }

        let mut capture_targets = targets & enemy;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
        }
    }

    for from_bit in queens_hv_pinned.iter_mut() {
        let from = from_bit.to_square();
        let potential_targets = sliding_targets::get_rook_targets(from, occupied);
        let targets = potential_targets & check_mask & hv_pinmask;

        let mut quiet_targets = targets & empty;
        for to_bit in quiet_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Quiet));
        }

        let mut capture_targets = targets & enemy;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
        }
    }

    for from_bit in queens_diag_pinned.iter_mut() {
        let from = from_bit.to_square();
        let potential_targets = sliding_targets::get_bishop_targets(from, occupied);
        let targets = potential_targets & check_mask & diag_pinmask;

        let mut quiet_targets = targets & empty;
        for to_bit in quiet_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Quiet));
        }

        let mut capture_targets = targets & enemy;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
        }
    }
}

pub fn generate_king_move(
    quiet_moves: &mut ArrayVec<EncodedMove, ARRAY_LENGTH>,

    friendly: Color,
    board: &Board,
) {
    let from_bit = board.king(friendly);
    let from = from_bit.to_square();
    let occupied_without_king = board.occupied() & !from_bit;
    let attackmask = masks::calculate_attackmask(board, occupied_without_king, !friendly, None);

    let potential_targets = normal_targets::KING_TARGETS[from.i()];
    let targets = potential_targets & !attackmask;

    let mut quiet_targets = targets & board.empty();
    for to_bit in quiet_targets.iter_mut() {
        let to = to_bit.to_square();
        quiet_moves.push(EncodedMove::encode(from, to, MoveType::Quiet));
    }

    let mut capture_targets = targets & board.color_bbs_without_king(!friendly);
    for to_bit in capture_targets.iter_mut() {
        let to = to_bit.to_square();
        quiet_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
    }
}

pub fn generate_castle_moves(
    quiet_moves: &mut ArrayVec<EncodedMove, ARRAY_LENGTH>,
    check_counter: usize,
    friendly: Color,
    board: &Board,
) {
    if check_counter != 0 {
        return;
    }
    let attackmask = masks::calculate_attackmask(board, board.occupied(), !friendly, None);
    let occupied = board.occupied();
    match friendly {
        White => {
            const NEED_TO_BE_EMPTY_QUEEN: Bitboard = Bitboard::from_idx([1, 2, 3]);
            const NEED_TO_BE_NOT_ATTACKED_QUEEN: Bitboard = Bitboard::from_idx([2, 3]);

            if board.white_queen_castle()
                && NEED_TO_BE_EMPTY_QUEEN & occupied == Bitboard::EMPTY
                && attackmask & NEED_TO_BE_NOT_ATTACKED_QUEEN == Bitboard::EMPTY
            {
                quiet_moves.push(EncodedMove::encode(
                    Square(4),
                    Square(2),
                    MoveType::QueenCastle,
                ));
            }
            const NEED_TO_BE_EMPTY_KING: Bitboard = Bitboard::from_idx([5, 6]);
            const NEED_TO_BE_NOT_ATTACKED_KING: Bitboard = Bitboard::from_idx([5, 6]);

            if board.white_king_castle()
                && NEED_TO_BE_EMPTY_KING & occupied == Bitboard::EMPTY
                && attackmask & NEED_TO_BE_NOT_ATTACKED_KING == Bitboard::EMPTY
            {
                quiet_moves.push(EncodedMove::encode(
                    Square(4),
                    Square(6),
                    MoveType::KingCastle,
                ));
            }
        }
        Black => {
            const NEED_TO_BE_EMPTY_QUEEN: Bitboard = Bitboard::from_idx([57, 58, 59]);
            const NEED_TO_BE_NOT_ATTACKED_QUEEN: Bitboard = Bitboard::from_idx([58, 59]);

            if board.black_queen_castle()
                && NEED_TO_BE_EMPTY_QUEEN & occupied == Bitboard::EMPTY
                && attackmask & NEED_TO_BE_NOT_ATTACKED_QUEEN == Bitboard::EMPTY
            {
                quiet_moves.push(EncodedMove::encode(
                    Square(60),
                    Square(58),
                    MoveType::QueenCastle,
                ));
            }
            const NEED_TO_BE_EMPTY_KING: Bitboard = Bitboard::from_idx([61, 62]);
            const NEED_TO_BE_NOT_ATTACKED_KING: Bitboard = Bitboard::from_idx([61, 62]);

            if board.black_king_castle()
                && NEED_TO_BE_EMPTY_KING & occupied == Bitboard::EMPTY
                && attackmask & NEED_TO_BE_NOT_ATTACKED_KING == Bitboard::EMPTY
            {
                quiet_moves.push(EncodedMove::encode(
                    Square(60),
                    Square(62),
                    MoveType::KingCastle,
                ));
            }
        }
    }
}

pub fn generate_ep_moves(
    board: &Board,
    quiet_moves: &mut ArrayVec<EncodedMove, ARRAY_LENGTH>,
    friendly: Color,
    hv_pinmask: Bitboard,
    diag_pinmask: Bitboard,
) {
    if let Some(ep_target_bit) = board.ep_target() {
        let ep_target = ep_target_bit.to_square();
        let captured_pawn_bit = match friendly {
            White => ep_target_bit >> 8,
            Black => ep_target_bit << 8,
        };

        let all_pawns = board.figure_bb(friendly, Pawn);

        let pawns_not_pinned = all_pawns & !hv_pinmask & !diag_pinmask;
        let enemy = !board.current_color();

        // Using !friendly to get the inverse position from the ep target to our pawns
        // so we can check where actually a pawn from us is there
        let mut possible_not_pinned_pawns: Bitboard =
            normal_targets::PAWN_ATTACK_TARGETS[!friendly as usize][ep_target] & pawns_not_pinned;
        for pawn in possible_not_pinned_pawns.iter_mut() {
            let occupied_after_ep = (board.occupied() & !pawn & !captured_pawn_bit) | ep_target_bit;
            let attackmask_after_ep = masks::calculate_attackmask(
                board,
                occupied_after_ep,
                enemy,
                Some(captured_pawn_bit),
            );

            // if not Ep would open open up a check from slider after being deleted
            if attackmask_after_ep & board.king(friendly) == Bitboard::EMPTY {
                quiet_moves.push(EncodedMove::encode(
                    pawn.to_square(),
                    ep_target,
                    MoveType::EpCapture,
                ));
            }
        }

        if diag_pinmask & captured_pawn_bit != Bitboard(0) {
            let pawns_diag_pinned = all_pawns & !hv_pinmask & diag_pinmask;
            let mut possible_diag_pinned_pawns: Bitboard = normal_targets::PAWN_ATTACK_TARGETS
                [!friendly as usize][ep_target]
                & pawns_diag_pinned;
            for pawn in possible_diag_pinned_pawns.iter_mut() {
                let occupied_after_ep =
                    (board.occupied() & !pawn & !captured_pawn_bit) | ep_target_bit;
                let attackmask_after_ep = masks::calculate_attackmask(
                    board,
                    occupied_after_ep,
                    enemy,
                    Some(captured_pawn_bit),
                );
                // if not Ep would open open up a check from slider after being deleted
                if attackmask_after_ep & board.king(friendly) == Bitboard::EMPTY {
                    quiet_moves.push(EncodedMove::encode(
                        pawn.to_square(),
                        ep_target,
                        MoveType::EpCapture,
                    ));
                }
            }
        }

        // hv pinned pawns are ignored because they cannot do ep capture because it needs a diagnoal move
    }
}
