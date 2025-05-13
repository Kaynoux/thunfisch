use super::masks;
use super::moves;
use super::pinmask;
use crate::prelude::*;
use arrayvec::ArrayVec;

// 218 is the limit: https://www.chessprogramming.org/Chess_Position
pub const ARRAY_LENGTH: usize = 256;

impl Board {
    pub fn generate_moves<const SPECIAL_MOVES_ONLY: bool>(
        &mut self,
    ) -> ArrayVec<EncodedMove, ARRAY_LENGTH> {
        let friendly = self.current_color();
        let mut moves = ArrayVec::<EncodedMove, ARRAY_LENGTH>::new();
        //let mut special_moves = ArrayVec::<EncodedMove, 128>::new();
        // test pin and checkmask = 5rk1/7b/8/r1PP1K2/8/5P2/8/5q2 w - - 0 1
        let (hv_pinmask, diag_pinmask) = pinmask::generate_pin_masks(self);
        let pinmask = hv_pinmask | diag_pinmask;

        let (check_mask, check_counter) = masks::calc_check_mask(self);

        // if check count > 2
        // than only the king can move also calc king evasions
        // return
        if check_counter == 2 {
            moves::generate_king_move::<false>(&mut moves, friendly, self);
            //quiet_moves.extend_from_slice(&special_moves);
            // early return only king moves if 2 checks occured
            return moves;
        }

        moves::generate_pawn_moves::<SPECIAL_MOVES_ONLY>(
            &mut moves,
            self,
            friendly,
            hv_pinmask,
            diag_pinmask,
            check_mask,
        );
        moves::generate_knight_moves::<SPECIAL_MOVES_ONLY>(
            &mut moves, pinmask, friendly, self, check_mask,
        );
        moves::generate_bishop_moves::<SPECIAL_MOVES_ONLY>(
            &mut moves,
            hv_pinmask,
            diag_pinmask,
            friendly,
            self,
            check_mask,
        );

        moves::generate_rook_moves::<SPECIAL_MOVES_ONLY>(
            &mut moves,
            hv_pinmask,
            diag_pinmask,
            friendly,
            self,
            check_mask,
        );

        moves::generate_queen_moves::<SPECIAL_MOVES_ONLY>(
            &mut moves,
            hv_pinmask,
            diag_pinmask,
            friendly,
            self,
            check_mask,
        );

        moves::generate_king_move::<SPECIAL_MOVES_ONLY>(&mut moves, friendly, self);

        if !SPECIAL_MOVES_ONLY {
            moves::generate_castle_moves(&mut moves, check_counter, friendly, self);
        }

        moves::generate_ep_moves(self, &mut moves, friendly, hv_pinmask, diag_pinmask);

        moves
    }

    pub fn is_in_check(&self) -> bool {
        let check_count = masks::calc_check_mask(self).1;
        if check_count == 0 {
            return false;
        }
        true
    }
}
