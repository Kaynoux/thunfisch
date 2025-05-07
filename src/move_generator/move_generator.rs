use crate::move_generator::normal_targets;
use crate::move_generator::pinmask;
use crate::move_generator::sliding_targets;
use crate::prelude::*;
impl Board {
    pub fn generate_all_moves(&self) -> Vec<EncodedMove> {
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

    fn generate_evasion_moves(&self, moves: Vec<EncodedMove>) {
        let king_pos = self.get_king(self.current_color);
    }

    pub fn generate_capture_moves(&self, moves: &mut Vec<EncodedMove>) {
        let friendly = self.current_color;
        let enemy = !friendly;
        let enemy_bb = self.get_pieces(enemy);
        //let pin_masks = self.generate_pin_masks2();
        // for (idx, i) in pin_masks.iter().enumerate() {
        //     println!("{} {:?}", idx, self.pieces[idx]);
        //     println!("{:?}", i);
        // }
        println!(" OCCU {:?}", self.occupied);

        for bit in self
            .get_bitboard_by_piece_color(friendly, Knight)
            .iter_mut()
        {
            let from = bit.to_square();
            if self.pieces[from.i()] == Figure::WhiteKnight {
                //let pin_mask = pin_masks[from.i()];
                // println!("{:?}", pin_mask);
                // if pin_mask != Bitboard(0) {
                //     continue;
                // }
                let potential_targets = normal_targets::KNIGHT_TARGETS[from.i()];
                let mut targets = potential_targets & enemy_bb;

                for to_bit in targets.iter_mut() {
                    let to = to_bit.to_square();

                    moves.push(EncodedMove::encode(from, to, MoveType::Capture));
                }
            }
        }
    }

    pub fn generate_quiet_moves(&self, moves: &mut Vec<EncodedMove>, color: Color) {
        let empty = self.get_empty();
        let occupied = self.occupied;
        // println!("Occupied");
        // println!("{:?}", occupied);
        let (hv_pinmask, diag_pinmask) = pinmask::generate_pin_masks(self);
        let pinmask = hv_pinmask | diag_pinmask;
        println!("Pin Mask");
        println!("{:?}", hv_pinmask | diag_pinmask);

        // test-fen https://lichess.org/editor/8/8/8/3n4/8/2N5/8/8_w_-_-_0_1?color=white
        let mut knights = self.get_bitboard_by_piece_color(color, Knight);
        for from_bit in knights.iter_mut() {
            let from = from_bit.to_square();
            if pinmask.is_position_set(from_bit) {
                continue;
            }
            let potential_targets = normal_targets::KNIGHT_TARGETS[from.i()];
            let mut targets = potential_targets & empty;

            for to_bit in targets.iter_mut() {
                let to = to_bit.to_square();
                moves.push(EncodedMove::encode(from, to, MoveType::Capture));
            }
        }

        //test-fen https://lichess.org/editor/8/8/8/3b4/8/2B5/8/8_w_-_-_0_1?color=white
        let mut bishops = self.get_bitboard_by_piece_color(color, Bishop);
        for from_bit in bishops.iter_mut() {
            let from = from_bit.to_square();
            let potential_targets = sliding_targets::get_bishop_targets(from, occupied);
            let mut targets = potential_targets & empty;

            for to_bit in targets.iter_mut() {
                let to = to_bit.to_square();
                moves.push(EncodedMove::encode(from, to, MoveType::Quiet));
            }
        }

        // test-fen https://lichess.org/editor/8/8/8/3r4/8/2R5/8/8_w_-_-_0_1?color=white
        let mut rooks = self.get_bitboard_by_piece_color(color, Rook);
        println!("Rooks");
        println!("{:?}", rooks);
        for from_bit in rooks.iter_mut() {
            println!("Rook");
            println!("{:?}", from_bit);
            let from = from_bit.to_square();
            let potential_targets = sliding_targets::get_rook_targets(from, occupied);
            let mut targets = potential_targets & empty;
            println!("Rook targets");
            println!("{:?}", targets);
            for to_bit in targets.iter_mut() {
                let to = to_bit.to_square();
                moves.push(EncodedMove::encode(from, to, MoveType::Quiet));
            }
        }

        // test-fen https://lichess.org/editor/8/8/8/3q4/8/2Q5/8/8_w_-_-_0_1?color=white
        let mut queens = self.get_bitboard_by_piece_color(color, Queen);
        for from_bit in queens.iter_mut() {
            let from = from_bit.to_square();
            let potential_targets = sliding_targets::get_rook_targets(from, occupied)
                | sliding_targets::get_bishop_targets(from, occupied);
            let mut targets = potential_targets & empty;

            for to_bit in targets.iter_mut() {
                let to = to_bit.to_square();
                moves.push(EncodedMove::encode(from, to, MoveType::Capture));
            }
        }

        crate::debug::print_board(self, Some(&moves));
    }
}
