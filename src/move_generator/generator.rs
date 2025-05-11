use super::masks;
use super::moves;
use super::pinmask;
use crate::prelude::*;

impl Board {
    pub fn generate_moves(&mut self) -> Vec<EncodedMove> {
        let friendly = self.current_color;
        let mut quiet_moves: Vec<EncodedMove> = Vec::new();
        let mut special_moves: Vec<EncodedMove> = Vec::new();

        // test pin and checkmask = 5rk1/7b/8/r1PP1K2/8/5P2/8/5q2 w - - 0 1
        let (hv_pinmask, diag_pinmask) = pinmask::generate_pin_masks(self);
        let pinmask = hv_pinmask | diag_pinmask;

        let (check_mask, check_counter) = masks::calc_check_mask(self);

        // if check count > 2
        // than only the king can move also calc king evasions
        // return
        if check_counter == 2 {
            moves::generate_king_move(&mut quiet_moves, &mut special_moves, friendly, self);
            quiet_moves.extend_from_slice(&special_moves);
            // early return only king moves if 2 checks occured
            return quiet_moves;
        }

        moves::generate_pawn_moves(
            &mut quiet_moves,
            &mut special_moves,
            self,
            friendly,
            hv_pinmask,
            diag_pinmask,
            check_mask,
        );
        moves::generate_knight_moves(
            &mut quiet_moves,
            &mut special_moves,
            pinmask,
            friendly,
            self,
            check_mask,
        );
        moves::generate_bishop_moves(
            &mut quiet_moves,
            &mut special_moves,
            hv_pinmask,
            diag_pinmask,
            friendly,
            self,
            check_mask,
        );

        moves::generate_rook_moves(
            &mut quiet_moves,
            &mut special_moves,
            hv_pinmask,
            diag_pinmask,
            friendly,
            self,
            check_mask,
        );

        moves::generate_queen_moves(
            &mut quiet_moves,
            &mut special_moves,
            hv_pinmask,
            diag_pinmask,
            friendly,
            self,
            check_mask,
        );

        moves::generate_king_move(&mut quiet_moves, &mut special_moves, friendly, self);

        moves::generate_castle_moves(&mut quiet_moves, check_counter, friendly, self);
        moves::generate_ep_moves(self, &mut special_moves, friendly, hv_pinmask, diag_pinmask);

        // let mut all_moves = Vec::with_capacity(special_moves.len() + quiet_moves.len());
        // all_moves.extend_from_slice(&special_moves);
        // all_moves.extend_from_slice(&quiet_moves);

        quiet_moves.extend_from_slice(&special_moves);
        quiet_moves
    }
}
