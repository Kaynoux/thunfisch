use crate::move_generator::between::LINE_THROUGH;
use crate::move_generator::normal_targets;
use crate::move_generator::pinmask;
use crate::move_generator::sliding_targets;
use crate::position_generation::get_king_positions;
use crate::prelude::*;
use crate::types::board;

use super::between::IN_BETWEEN;
use super::normal_targets::KING_TARGETS;
use super::normal_targets::KNIGHT_TARGETS;
use super::normal_targets::PAWN_ATTACK_TARGETS;
use super::sliding_targets::get_bishop_targets;
use super::sliding_targets::get_rook_targets;
impl Board {
    pub fn generate_moves(&mut self, friendly: Color) -> Vec<EncodedMove> {
        let mut quiet_moves: Vec<EncodedMove> = Vec::new();
        let mut special_moves: Vec<EncodedMove> = Vec::new();
        // println!("Occupied");
        // println!("{:?}", occupied);

        // test pin and checkmask = 5rk1/7b/8/r1PP1K2/8/5P2/8/5q2 w - - 0 1
        let (hv_pinmask, diag_pinmask) = pinmask::generate_pin_masks(self);
        let pinmask = hv_pinmask | diag_pinmask;
        println!("Pin Mask");
        println!("{:?}", hv_pinmask | diag_pinmask);

        let (check_mask, check_counter) = calc_check_mask(self);
        let attackmask = calculate_attackmask(self, self.occupied);
        println!("Check Mask: {}", check_counter);
        println!("{:?}", check_mask);

        println!("Attack Mask:");
        println!("{:?}", attackmask);

        // if check count > 2
        // than only the king can move also calc king evasions
        // return
        if check_counter == 2 {
            calc_king(
                &mut quiet_moves,
                &mut special_moves,
                attackmask,
                friendly,
                self,
            );
            quiet_moves.extend_from_slice(&special_moves);
            // early return only king moves if 2 checks occured
            return quiet_moves;
        }
        // wenn check count 1 dann normale moves mit checkmask und king evasion moves extr

        // check mask berechnen

        calc_pawn_moves(
            &mut quiet_moves,
            &mut special_moves,
            self,
            friendly,
            hv_pinmask,
            diag_pinmask,
            check_mask,
        );
        calc_knights(
            &mut quiet_moves,
            &mut special_moves,
            pinmask,
            friendly,
            self,
            check_mask,
        );
        calc_bishops(
            &mut quiet_moves,
            &mut special_moves,
            hv_pinmask,
            diag_pinmask,
            friendly,
            self,
            check_mask,
        );

        calc_rooks(
            &mut quiet_moves,
            &mut special_moves,
            hv_pinmask,
            diag_pinmask,
            friendly,
            self,
            check_mask,
        );

        calc_queens(
            &mut quiet_moves,
            &mut special_moves,
            hv_pinmask,
            diag_pinmask,
            friendly,
            self,
            check_mask,
        );

        calc_king(
            &mut quiet_moves,
            &mut special_moves,
            attackmask,
            friendly,
            self,
        );

        generate_castle_moves(&mut quiet_moves, attackmask, friendly, self);
        generate_ep_moves(self, &mut special_moves, friendly, hv_pinmask, diag_pinmask);

        // let mut all_moves = Vec::with_capacity(special_moves.len() + quiet_moves.len());
        // all_moves.extend_from_slice(&special_moves);
        // all_moves.extend_from_slice(&quiet_moves);

        quiet_moves.extend_from_slice(&special_moves);
        quiet_moves
    }

    /// Gets a bitboard which contains all positions from enemy pieces which attack the given square
    /// This is very fast because with the execption of pawns all moves are symmetrical
    pub fn get_all_attackers_to(&self, to: Square, attacker: Color) -> Bitboard {
        let mut normal_attackers = Bitboard::EMPTY;
        let mut sliding_attackers = Bitboard::EMPTY;
        let occ = self.occupied;

        let enemy_pawns = self.get_pieces_by_color(attacker, Pawn);
        let enemy_knights = self.get_pieces_by_color(attacker, Knight);
        let enemy_bishops_queens = self.get_pieces_by_color(attacker, Bishop);
        let enemy_rooks_queens = self.get_pieces_by_color(attacker, Rook);
        let enemy_king = self.get_pieces_by_color(attacker, King);

        normal_attackers |= PAWN_ATTACK_TARGETS[!attacker as usize][to.i()] & enemy_pawns;
        normal_attackers |= KNIGHT_TARGETS[to.i()] & enemy_knights;
        normal_attackers |= KING_TARGETS[to.i()] & enemy_king; // not needed for checkmaask but 
        sliding_attackers |= get_bishop_targets(to, occ) & enemy_bishops_queens;
        sliding_attackers |= get_rook_targets(to, occ) & enemy_rooks_queens;

        sliding_attackers
    }
}

pub fn calc_pawn_moves(
    quiet_moves: &mut Vec<EncodedMove>,
    special_moves: &mut Vec<EncodedMove>,
    board: &Board,
    friendly: Color,
    hv_pinmask: Bitboard,
    diag_pinmask: Bitboard,
    check_mask: Bitboard,
) {
    let empty = board.get_empty();
    let enemy = board.get_pieces(!friendly);

    let last_row = match friendly {
        Color::White => Bitboard(0xFF00000000000000),
        Color::Black => Bitboard(0xFF),
    };

    let all_pawns = board.get_pieces_by_color(friendly, Pawn);
    let pawns_not_to_last_row = all_pawns & !last_row;
    let pawns_to_last_row = all_pawns & last_row;
    let mut pawns_hv_pinned = pawns_not_to_last_row & hv_pinmask & !diag_pinmask;
    let mut pawns_diag_pinned = pawns_not_to_last_row & !hv_pinmask & diag_pinmask;
    let mut pawns_not_pinned = pawns_not_to_last_row & !hv_pinmask & !diag_pinmask;
    let mut last_row_pawns_hv_pinned = pawns_to_last_row & hv_pinmask & !diag_pinmask;
    let mut last_row_pawns_diag_pinned = pawns_to_last_row & !hv_pinmask & diag_pinmask;
    let mut last_row_pawns_not_pinned = pawns_to_last_row & !hv_pinmask & !diag_pinmask;
    // mask corresponding to the 3 y layer depending on the color
    // not using the 2. because we use the already moved by one pos
    let double_push_mask: Bitboard = match friendly {
        Color::White => {
            Bitboard(0xff000000) // y3
        }
        Color::Black => {
            Bitboard(0xff00000000) // y4
        }
    };

    for from_bit in pawns_not_pinned.iter_mut() {
        let from = from_bit.to_square();

        let potential_quiet_target_1 = normal_targets::pawn_quiet_single_target(from_bit, friendly);
        let quiet_target_1 = potential_quiet_target_1 & empty & check_mask;

        if quiet_target_1 == Bit(0) {
            continue;
        }

        quiet_moves.push(EncodedMove::encode(
            from,
            quiet_target_1.to_square(),
            MoveType::Quiet,
        ));

        let quiet_target_2 = normal_targets::pawn_quiet_single_target(quiet_target_1, friendly)
            & double_push_mask
            & empty
            & check_mask;

        if quiet_target_2 != Bit(0) {
            quiet_moves.push(EncodedMove::encode(
                from,
                quiet_target_2.to_square(),
                MoveType::DoubleMove,
            ));
        }

        let mut capture_targets = PAWN_ATTACK_TARGETS[friendly as usize][from] & enemy & check_mask;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            special_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
        }
    }
    for from_bit in pawns_hv_pinned.iter_mut() {
        let from = from_bit.to_square();

        let potential_target = normal_targets::pawn_quiet_single_target(from_bit, friendly);
        let target_1 = potential_target & empty & check_mask & hv_pinmask;

        if target_1 == Bit(0) {
            continue;
        }

        quiet_moves.push(EncodedMove::encode(
            from,
            target_1.to_square(),
            MoveType::Quiet,
        ));

        // do net need to check hv pinmask again because if first move is on hv second one is also
        let target_2 = normal_targets::pawn_quiet_single_target(target_1, friendly)
            & double_push_mask
            & empty
            & check_mask;

        if target_2 != Bit(0) {
            quiet_moves.push(EncodedMove::encode(
                from,
                target_2.to_square(),
                MoveType::DoubleMove,
            ));
        }
    }

    for from_bit in pawns_diag_pinned.iter_mut() {
        let from = from_bit.to_square();
        let mut capture_targets = PAWN_ATTACK_TARGETS[friendly as usize][from] & enemy & check_mask;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            special_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
        }
    }

    // last row pawns
    for from_bit in last_row_pawns_not_pinned.iter_mut() {
        let from = from_bit.to_square();

        let potential_quiet_target_1 = normal_targets::pawn_quiet_single_target(from_bit, friendly);
        let quiet_target_1 = potential_quiet_target_1 & empty & check_mask;

        if quiet_target_1 == Bit(0) {
            continue;
        }

        let to_1 = quiet_target_1.to_square();
        special_moves.push(EncodedMove::encode(from, to_1, MoveType::QueenPromo));
        special_moves.push(EncodedMove::encode(from, to_1, MoveType::RookPromo));
        special_moves.push(EncodedMove::encode(from, to_1, MoveType::BishopPromo));
        special_moves.push(EncodedMove::encode(from, to_1, MoveType::KnightPromo));

        let mut capture_targets = PAWN_ATTACK_TARGETS[friendly as usize][from] & enemy & check_mask;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            special_moves.push(EncodedMove::encode(from, to, MoveType::QueenPromoCapture));
            special_moves.push(EncodedMove::encode(from, to, MoveType::RookPromoCapture));
            special_moves.push(EncodedMove::encode(from, to, MoveType::BishopPromoCapture));
            special_moves.push(EncodedMove::encode(from, to, MoveType::KnightPromoCapture));
        }
    }
    for from_bit in last_row_pawns_hv_pinned.iter_mut() {
        let from = from_bit.to_square();

        let potential_target = normal_targets::pawn_quiet_single_target(from_bit, friendly);
        let target_1 = potential_target & empty & check_mask & hv_pinmask;

        if target_1 == Bit(0) {
            continue;
        }

        let to_1 = target_1.to_square();
        special_moves.push(EncodedMove::encode(from, to_1, MoveType::QueenPromo));
        special_moves.push(EncodedMove::encode(from, to_1, MoveType::RookPromo));
        special_moves.push(EncodedMove::encode(from, to_1, MoveType::BishopPromo));
        special_moves.push(EncodedMove::encode(from, to_1, MoveType::KnightPromo));
    }

    for from_bit in last_row_pawns_diag_pinned.iter_mut() {
        let from = from_bit.to_square();
        let mut capture_targets = PAWN_ATTACK_TARGETS[friendly as usize][from] & enemy & check_mask;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            special_moves.push(EncodedMove::encode(from, to, MoveType::QueenPromoCapture));
            special_moves.push(EncodedMove::encode(from, to, MoveType::RookPromoCapture));
            special_moves.push(EncodedMove::encode(from, to, MoveType::BishopPromoCapture));
            special_moves.push(EncodedMove::encode(from, to, MoveType::KnightPromoCapture));
        }
    }
}

pub fn calc_knights(
    quiet_moves: &mut Vec<EncodedMove>,
    special_moves: &mut Vec<EncodedMove>,
    pinmask: Bitboard,
    friendly: Color,
    board: &Board,
    check_mask: Bitboard,
) {
    // test-fen https://lichess.org/editor/8/8/8/3n4/8/2N5/8/8_w_-_-_0_1?color=white
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
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Quiet));
        }

        let mut capture_targets = targets & board.get_pieces(!friendly);
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            special_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
        }
    }
}

pub fn calc_bishops(
    quiet_moves: &mut Vec<EncodedMove>,
    special_moves: &mut Vec<EncodedMove>,
    hv_pinmask: Bitboard,
    diag_pinmask: Bitboard,
    friendly: Color,
    board: &Board,
    check_mask: Bitboard,
) {
    //test-fen https://lichess.org/editor/8/8/8/3b4/8/2B5/8/8_w_-_-_0_1?color=white
    let empty = board.get_empty();
    let enemy = board.get_pieces(!friendly);
    let occupied = board.occupied;

    let bishops = board.get_pieces_by_color(friendly, Bishop);
    let mut bishops_not_pinned = bishops & !diag_pinmask & !hv_pinmask;
    let mut bishops_diag_pinned = bishops & diag_pinmask & !hv_pinmask;

    for from_bit in bishops_not_pinned.iter_mut() {
        let from = from_bit.to_square();
        let potential_targets = sliding_targets::get_bishop_targets(from, occupied);
        let targets = potential_targets & empty & check_mask;

        let mut quiet_targets = targets & empty;
        for to_bit in quiet_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Quiet));
        }

        let mut capture_targets = targets & enemy;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            special_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
        }
    }

    for from_bit in bishops_diag_pinned.iter_mut() {
        let from = from_bit.to_square();
        let potential_targets = sliding_targets::get_bishop_targets(from, occupied);
        let targets = potential_targets & empty & check_mask & diag_pinmask;

        let mut quiet_targets = targets & empty;
        for to_bit in quiet_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Quiet));
        }

        let mut capture_targets = targets & enemy;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            special_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
        }
    }
}

pub fn calc_rooks(
    quiet_moves: &mut Vec<EncodedMove>,
    special_moves: &mut Vec<EncodedMove>,
    hv_pinmask: Bitboard,
    diag_pinmask: Bitboard,
    friendly: Color,
    board: &Board,
    check_mask: Bitboard,
) {
    // test-fen https://lichess.org/editor/8/8/8/3r4/8/2R5/8/8_w_-_-_0_1?color=white
    let empty = board.get_empty();
    let enemy = board.get_pieces(!friendly);
    let occupied = board.occupied;

    let rooks = board.get_pieces_by_color(friendly, Rook);
    let mut rooks_not_pinned = rooks & !hv_pinmask & !diag_pinmask;
    let mut rooks_hv_pinned = rooks & hv_pinmask & !diag_pinmask;

    for from_bit in rooks_not_pinned.iter_mut() {
        let from = from_bit.to_square();
        let potential_targets = sliding_targets::get_rook_targets(from, occupied);
        let targets = potential_targets & empty & check_mask;

        let mut quiet_targets = targets & empty;
        for to_bit in quiet_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Quiet));
        }

        let mut capture_targets = targets & enemy;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            special_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
        }
    }

    for from_bit in rooks_hv_pinned.iter_mut() {
        let from = from_bit.to_square();
        let potential_targets = sliding_targets::get_rook_targets(from, occupied);
        let targets = potential_targets & empty & check_mask & hv_pinmask;

        let mut quiet_targets = targets & empty;
        for to_bit in quiet_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Quiet));
        }

        let mut capture_targets = targets & enemy;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            special_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
        }
    }
}

pub fn calc_queens(
    quiet_moves: &mut Vec<EncodedMove>,
    special_moves: &mut Vec<EncodedMove>,
    hv_pinmask: Bitboard,
    diag_pinmask: Bitboard,
    friendly: Color,
    board: &Board,
    check_mask: Bitboard,
) {
    // test-fen https://lichess.org/editor/8/8/8/3q4/8/2Q5/8/8_w_-_-_0_1?color=white
    let empty = board.get_empty();
    let enemy = board.get_pieces(!friendly);
    let occupied = board.occupied;

    let queens = board.get_pieces_by_color(friendly, Queen);
    let mut quuens_not_pinned = queens & !hv_pinmask & !diag_pinmask;
    let mut queens_hv_pinned = queens & hv_pinmask & !diag_pinmask;
    let mut queens_diag_pinned = queens & !hv_pinmask & diag_pinmask;

    for from_bit in quuens_not_pinned.iter_mut() {
        let from = from_bit.to_square();
        let potential_targets = sliding_targets::get_rook_targets(from, occupied)
            | sliding_targets::get_bishop_targets(from, occupied);
        let targets = potential_targets & empty & check_mask;

        let mut quiet_targets = targets & empty;
        for to_bit in quiet_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Quiet));
        }

        let mut capture_targets = targets & enemy;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            special_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
        }
    }

    for from_bit in queens_hv_pinned.iter_mut() {
        let from = from_bit.to_square();
        let potential_targets = sliding_targets::get_rook_targets(from, occupied);
        let targets = potential_targets & empty & check_mask & hv_pinmask;

        let mut quiet_targets = targets & empty;
        for to_bit in quiet_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Quiet));
        }

        let mut capture_targets = targets & enemy;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            special_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
        }
    }

    for from_bit in queens_diag_pinned.iter_mut() {
        let from = from_bit.to_square();
        let potential_targets = sliding_targets::get_bishop_targets(from, occupied);
        let targets = potential_targets & empty & check_mask & diag_pinmask;

        let mut quiet_targets = targets & empty;
        for to_bit in quiet_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(from, to, MoveType::Quiet));
        }

        let mut capture_targets = targets & enemy;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            special_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
        }
    }
}

pub fn calc_king(
    quiet_moves: &mut Vec<EncodedMove>,
    special_moves: &mut Vec<EncodedMove>,
    attackmask: Bitboard,
    friendly: Color,
    board: &Board,
) {
    let from_bit = board.get_king(friendly);
    let from = from_bit.to_square();
    let potential_targets = normal_targets::KING_TARGETS[from.i()];
    let targets = potential_targets & !attackmask;

    let mut quiet_targets = targets & board.get_empty();
    for to_bit in quiet_targets.iter_mut() {
        let to = to_bit.to_square();
        quiet_moves.push(EncodedMove::encode(from, to, MoveType::Quiet));
    }

    let mut capture_targets = targets & board.get_pieces(!friendly);
    for to_bit in capture_targets.iter_mut() {
        let to = to_bit.to_square();
        special_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
    }
}

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

    checkmask |= PAWN_ATTACK_TARGETS[friendly as usize][king] & enemy_pawns;
    checkmask |= KNIGHT_TARGETS[king] & enemy_knights;
    checkmask |= KING_TARGETS[king] & enemy_knights; // not needed for checkmaask but 
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

pub fn generate_castle_moves(
    quiet_moves: &mut Vec<EncodedMove>,
    attackmask: Bitboard,
    friendly: Color,
    board: &Board,
) {
    let occupied = board.occupied;
    match friendly {
        Color::White => {
            const NEED_TO_BE_EMPTY_QUEEN: Bitboard = Bitboard::from_idx([1, 2, 3]);
            const NEED_TO_BE_NOT_ATTACKED_QUEEN: Bitboard = Bitboard::from_idx([2, 3]);

            if board.white_queen_castle
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

            if board.white_king_castle
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
        Color::Black => {
            const NEED_TO_BE_EMPTY_QUEEN: Bitboard = Bitboard::from_idx([57, 58, 59]);
            const NEED_TO_BE_NOT_ATTACKED_QUEEN: Bitboard = Bitboard::from_idx([58, 59]);

            if board.black_queen_castle
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

            if board.black_king_castle
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

pub fn calculate_attackmask(board: &Board, occupied: Bitboard) -> Bitboard {
    let friendly = board.current_color;
    let enemy = !friendly;

    let mut attacks = Bitboard::EMPTY;

    let mut enemy_pawns = board.get_pieces_by_color(enemy, Pawn);
    let mut enemy_knights = board.get_pieces_by_color(enemy, Knight);
    let mut enemy_bishops_queens =
        board.get_pieces_by_color(enemy, Bishop) | board.get_pieces_by_color(enemy, Queen);
    let mut enemy_rooks_queens =
        board.get_pieces_by_color(enemy, Rook) | board.get_pieces_by_color(enemy, Queen);
    let enemy_king = board.get_king(friendly);

    for pawn in enemy_pawns.iter_mut() {
        attacks |= PAWN_ATTACK_TARGETS[enemy as usize][pawn.to_square()];
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

pub fn generate_ep_moves(
    board: &Board,
    special_moves: &mut Vec<EncodedMove>,
    friendly: Color,
    hv_pinmask: Bitboard,
    diag_pinmask: Bitboard,
) {
    if let Some(ep_target_bit) = board.ep_target {
        let ep_target = ep_target_bit.to_square();
        let temp_occupied = board.occupied & !ep_target_bit;
        let temp_attackmask = calculate_attackmask(board, temp_occupied);
        if temp_attackmask & board.get_king(friendly) != Bitboard::EMPTY {
            return; // Ep would open open up a check from slider after being deleted
        }

        let empty = board.get_empty();
        let enemy = board.get_pieces(!friendly);

        let all_pawns = board.get_pieces_by_color(friendly, Pawn);

        let mut pawns_not_pinned = all_pawns & !hv_pinmask & !diag_pinmask;

        let mut possible_not_pinned_pawns: Bitboard =
            PAWN_ATTACK_TARGETS[!friendly as usize][ep_target] & pawns_not_pinned;
        for pawn in possible_not_pinned_pawns.iter_mut() {
            special_moves.push(EncodedMove::encode(
                pawn.to_square(),
                ep_target,
                MoveType::EpCapture,
            ));
        }

        if diag_pinmask & ep_target_bit != Bitboard(0) {
            let pawns_diag_pinned = all_pawns & !hv_pinmask & diag_pinmask;
            let mut possible_diag_pinned_pawns: Bitboard =
                PAWN_ATTACK_TARGETS[!friendly as usize][ep_target] & pawns_diag_pinned;
            for pawn in possible_diag_pinned_pawns.iter_mut() {
                special_moves.push(EncodedMove::encode(
                    pawn.to_square(),
                    ep_target,
                    MoveType::EpCapture,
                ));
            }
        }
    }
}
