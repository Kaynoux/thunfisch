use crate::prelude::*;

impl Board {
    pub fn make_move(&mut self, mv: &DecodedMove) {
        let current_color = self.current_color;
        let from_idx = mv.from;
        let to_idx = mv.to;
        let from_pos = from_idx.to_position();
        let to_pos = to_idx.to_position();

        let from_piece = self.pieces[from_idx.0];
        let to_piece = self.pieces[to_idx.0];

        // Revoking castling rights
        const WHITE_ROOK_QUEEN_POS: IndexPosition = IndexPosition(0);
        const WHITE_ROOK_KING_POS: IndexPosition = IndexPosition(7);
        const BLACK_ROOK_KING_POS: IndexPosition = IndexPosition(56);
        const BLACK_ROOK_QUEEN_POS: IndexPosition = IndexPosition(63);
        const WHITE_KING_POS: IndexPosition = IndexPosition(4);
        const BLACK_KING_POS: IndexPosition = IndexPosition(60);

        // Revoke castling rights if
        // - Rook on the relevant side has moved
        // - King has moved
        // - Rook on the relevant side was captured
        if self.white_queen_castle
            && (from_idx == WHITE_ROOK_QUEEN_POS
                || from_idx == WHITE_KING_POS
                || to_idx == WHITE_ROOK_QUEEN_POS)
        {
            self.white_queen_castle = false;
        }

        if self.white_king_castle
            && (from_idx == WHITE_ROOK_KING_POS
                || from_idx == WHITE_KING_POS
                || to_idx == WHITE_ROOK_KING_POS)
        {
            self.white_king_castle = false;
        }

        if self.black_queen_castle
            && (from_idx == BLACK_ROOK_QUEEN_POS
                || from_idx == BLACK_KING_POS
                || to_idx == BLACK_ROOK_QUEEN_POS)
        {
            self.black_queen_castle = false;
        }

        if self.black_king_castle
            && (from_idx == BLACK_ROOK_KING_POS
                || from_idx == BLACK_KING_POS
                || to_idx == BLACK_ROOK_KING_POS)
        {
            self.black_king_castle = false;
        }

        // Handling castling
        const WHITE_QUEEN_CASTLE_POS: Position = IndexPosition(2).to_position();
        const WHITE_KING_CASTLE_POS: Position = IndexPosition(6).to_position();
        const BLACK_KING_CASTLE_POS: Position = IndexPosition(58).to_position();
        const BLACK_QUEEN_CASTLE_POS: Position = IndexPosition(62).to_position();

        if mv.is_queen_castle && current_color == Color::White {
            let inverse_rook_position = !IndexPosition(0).to_position();
            self.bbs[ColorPiece::WhiteRook as usize] &= inverse_rook_position;
            let rook_target_position = IndexPosition(3).to_position();
            self.bbs[ColorPiece::WhiteRook as usize] |= rook_target_position;
        } else if mv.is_king_castle && current_color == Color::White {
            let inverse_rook_position = !IndexPosition(7).to_position();
            self.bbs[ColorPiece::WhiteRook as usize] &= inverse_rook_position;
            let rook_target_position = IndexPosition(5).to_position();
            self.bbs[ColorPiece::WhiteRook as usize] |= rook_target_position;
        } else if mv.is_queen_castle && current_color == Color::Black {
            let inverse_rook_position = !IndexPosition(56).to_position();
            self.bbs[ColorPiece::BlackRook as usize] &= inverse_rook_position;
            let rook_target_position = IndexPosition(59).to_position();
            self.bbs[ColorPiece::BlackRook as usize] |= rook_target_position;
        } else if mv.is_king_castle && current_color == Color::Black {
            let inverse_rook_position = !IndexPosition(63).to_position();
            self.bbs[ColorPiece::BlackRook as usize] &= inverse_rook_position;
            let rook_target_position = IndexPosition(61).to_position();
            self.bbs[ColorPiece::BlackRook as usize] |= rook_target_position;
        }

        // Remove pawn if En-passant happened
        if mv.is_ep_capture {
            match current_color {
                Color::White => {
                    let pawn_mask = !to_pos.get_offset_pos(0, -1);
                    self.bbs[ColorPiece::BlackPawn as usize] &= pawn_mask;
                }
                Color::Black => {
                    let pawn_mask = !to_pos.get_offset_pos(0, 1);
                    self.bbs[ColorPiece::WhitePawn as usize] &= pawn_mask;
                }
            }
        }

        // Set En-passant target

        if mv.is_double_move {
            let offset_dir: isize = match current_color {
                Color::White => -1,
                Color::Black => 1,
            };
            self.en_passant_target = Some(to_pos.get_offset_pos(0, offset_dir));
        } else {
            self.en_passant_target = None;
        }

        // Remove start piece from bitboard
        self.bbs[from_piece as usize] &= !from_pos;

        // Remove target piece from bitboard
        self.bbs[to_piece as usize] &= !to_pos;

        // Add the start piece to the target position
        match (mv.promotion, current_color) {
            (None, _) => {
                self.bbs[from_piece as usize] |= to_pos;
            }
            (Some(piece), color) => self.bbs[(piece as usize) * 2 + (color as usize)] |= to_pos,
        }

        if current_color == Color::White {
            self.fullmove_counter += 1
        }
        self.current_color = !current_color;
        self.recalculate_black_white_empty_pieces();
    }
}
