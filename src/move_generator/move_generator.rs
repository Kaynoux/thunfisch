use crate::move_generator::between::LINE_THROUGH;
use crate::move_generator::normal_targets;
use crate::move_generator::pinmask;
use crate::move_generator::sliding_targets;
use crate::position_generation::get_king_positions;
use crate::prelude::*;

use super::between::IN_BETWEEN;
use super::normal_targets::KING_TARGETS;
use super::normal_targets::KNIGHT_TARGETS;
use super::normal_targets::PAWN_ATTACK_TARGETS;
use super::sliding_targets::get_bishop_targets;
use super::sliding_targets::get_rook_targets;
impl Board {
    pub fn calc_all_moves(&self) -> Vec<EncodedMove> {
        let mut moves: Vec<EncodedMove> = Vec::new();
        let mut color = self.current_color;

        self.generate_quiet_moves(&mut moves, color);
        if self.is_in_check() {
            println!("info: not in check");
            //self.generate_evasion_moves(&mut moves);
        } else {
            //self.generate_capture_moves(&mut moves);
        }
        moves
    }

    fn calc_evasion_moves(&self, moves: Vec<EncodedMove>) {
        let king_pos = self.get_king(self.current_color);
    }

    pub fn calc_check_mask(&self) -> (Bitboard, usize) {
        let friendly = self.current_color;
        let enemy = !friendly;
        let mut normal_attackers = Bitboard::EMPTY;
        let mut sliding_attackers = Bitboard::EMPTY;
        let occ = self.occupied;
        let king = self.get_king(friendly).to_square();

        let enemy_pawns = self.get_pieces_by_color(enemy, Pawn);
        let enemy_knights = self.get_pieces_by_color(enemy, Knight);
        let enemy_bishops_queens = self.get_pieces_by_color(enemy, Bishop);
        let enemy_rooks_queens = self.get_pieces_by_color(enemy, Rook);
        // enemy king is ignored because it cannot give check

        normal_attackers |= PAWN_ATTACK_TARGETS[friendly as usize][king] & enemy_pawns;
        normal_attackers |= KNIGHT_TARGETS[king] & enemy_knights;
        normal_attackers |= KING_TARGETS[king] & enemy_knights; // not needed for checkmaask but 
        sliding_attackers |= get_bishop_targets(king, occ) & enemy_bishops_queens;
        sliding_attackers |= get_rook_targets(king, occ) & enemy_rooks_queens;

        // No check all positions are valid fill the board
        if (normal_attackers | sliding_attackers).is_empty() {
            return (Bitboard::FULL, 0);
        }
        let mut check_counter = 0usize;
        let mut checkmask = Bitboard::EMPTY;
        for attacker in normal_attackers.iter_mut() {
            checkmask |= attacker;
            check_counter += 1;
        }

        for attacker in sliding_attackers.iter_mut() {
            checkmask |= IN_BETWEEN[king][attacker.to_square()] | attacker;
            check_counter += 1;
        }
        (checkmask, check_counter)
    }

    pub fn generate_quiet_moves(&self, moves: &mut Vec<EncodedMove>, color: Color) {
        let mut quiet_moves: Vec<EncodedMove> = Vec::new();
        let mut capture_moves: Vec<EncodedMove> = Vec::new();

        let empty = self.get_empty();
        let occupied = self.occupied;
        let enemy = self.get_pieces(!color);
        let frienldy_king = self.get_king(color).to_square();
        // println!("Occupied");
        // println!("{:?}", occupied);

        // test pin and checkmask = 5rk1/7b/8/r1PP1K2/8/5P2/8/5q2 w - - 0 1
        let (hv_pinmask, diag_pinmask) = pinmask::generate_pin_masks(self);
        let pinmask = hv_pinmask | diag_pinmask;
        println!("Pin Mask");
        println!("{:?}", hv_pinmask | diag_pinmask);

        // if check count > 2
        // than only the king can move also calc king evasions
        // return

        // wenn check count 1 dann normale moves mit checkmask und king evasion moves extr

        // check mask berechnen
        let (check_mask, check_counter) = self.calc_check_mask();
        println!("Check counter: {}", check_counter);
        println!("{:?}", check_mask);

        calc_quiet_pawn_moves(moves, self, color, hv_pinmask, diag_pinmask, check_mask);

        // test-fen https://lichess.org/editor/8/8/8/3n4/8/2N5/8/8_w_-_-_0_1?color=white
        let mut knights = self.get_pieces_by_color(color, Knight);
        for from_bit in knights.iter_mut() {
            let from = from_bit.to_square();
            if pinmask.is_position_set(from_bit) {
                continue;
            }
            let potential_targets = normal_targets::KNIGHT_TARGETS[from.i()];
            let targets = potential_targets & check_mask; // & checkmask

            let mut quiet_targets = targets & empty;
            for to_bit in quiet_targets.iter_mut() {
                let to = to_bit.to_square();
                quiet_moves.push(EncodedMove::encode(from, to, MoveType::Quiet));
            }

            let mut capture_targets = targets & enemy;
            for to_bit in capture_targets.iter_mut() {
                let to = to_bit.to_square();
                capture_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
            }
        }

        //test-fen https://lichess.org/editor/8/8/8/3b4/8/2B5/8/8_w_-_-_0_1?color=white
        let mut bishops = self.get_pieces_by_color(color, Bishop);
        for from_bit in bishops.iter_mut() {
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
                capture_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
            }
        }

        // test-fen https://lichess.org/editor/8/8/8/3r4/8/2R5/8/8_w_-_-_0_1?color=white
        let mut rooks = self.get_pieces_by_color(color, Rook);
        for from_bit in rooks.iter_mut() {
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
                capture_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
            }
        }

        // test-fen https://lichess.org/editor/8/8/8/3q4/8/2Q5/8/8_w_-_-_0_1?color=white
        let mut queens = self.get_pieces_by_color(color, Queen);
        for from_bit in queens.iter_mut() {
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
                capture_moves.push(EncodedMove::encode(from, to, MoveType::Capture));
            }
        }

        let friendly_king = self.get_king(color);
        let potential_targets = normal_targets::KING_TARGETS[friendly_king.to_square()];
        let targets = potential_targets & check_mask;

        let mut quiet_targets = targets & empty;
        for to_bit in quiet_targets.iter_mut() {
            let to = to_bit.to_square();
            quiet_moves.push(EncodedMove::encode(frienldy_king, to, MoveType::Quiet));
        }

        let mut capture_targets = targets & enemy;
        for to_bit in capture_targets.iter_mut() {
            let to = to_bit.to_square();
            capture_moves.push(EncodedMove::encode(frienldy_king, to, MoveType::Capture));
        }

        crate::debug::print_board(self, Some(&moves));
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

pub fn calc_quiet_pawn_moves(
    moves: &mut Vec<EncodedMove>,
    board: &Board,
    color: Color,
    hv_pinmask: Bitboard,
    diag_pinmask: Bitboard,
    check_mask: Bitboard,
) {
    let friendly_king = board.get_king(color).to_square();
    let empty = board.get_empty();

    let all_pawns = board.get_pieces_by_color(color, Pawn);
    let mut pawns_hv_pinned = all_pawns & hv_pinmask & !diag_pinmask; // Diag pinned pawns cannot move so they are removed
    let mut pawns_not_pinned = all_pawns & !hv_pinmask & !diag_pinmask;
    for from_bit in pawns_not_pinned.iter_mut() {
        let from = from_bit.to_square();

        let from = from_bit.to_square();
        // println!("from ");
        // println!("{:?}", from.to_bitboard());
        let potential_target = normal_targets::pawn_quiet_single_target(from_bit, color);
        let target_1 = potential_target & empty & check_mask;
        // println!("to");
        // println!("{:?}", target_1.to_bb());

        //println!("Single: from: {} to: {}", from.0, target_1.to_square().0);
        moves.push(EncodedMove::encode(
            from,
            target_1.to_square(),
            MoveType::Quiet,
        ));

        // generates a mask corresponding to the 3 y layer depending on the color
        // not using the 2. because we use the already moved by one pos
        let double_push_mask: Bitboard = match color {
            Color::White => {
                Bitboard(0xff000000) // y3
            }
            Color::Black => {
                Bitboard(0xff00000000) // y4
            }
        };

        let target_2 = normal_targets::pawn_quiet_single_target(target_1, color)
            & double_push_mask
            & empty
            & check_mask;

        if target_2 == Bit(0) {
            continue;
        }

        //println!("Double: from: {} to: {}", from.0, target_2.to_square().0);
        moves.push(EncodedMove::encode(
            from,
            target_2.to_square(),
            MoveType::DoubleMove,
        ));
    }
}
