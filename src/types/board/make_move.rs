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

        // Remove start piece from bitboard
        self.bbs[from_piece as usize] &= !from_pos;
        self.pieces[from_idx.0] = ColorPiece::Empty;

        // Remove target piece from bitboard
        self.bbs[to_piece as usize] &= !to_pos;
        self.pieces[to_idx.0] = ColorPiece::Empty;

        // Revoking castling rights
        const WHITE_ROOK_QUEEN_POS: IndexPosition = IndexPosition(0);
        const WHITE_ROOK_KING_POS: IndexPosition = IndexPosition(7);
        const BLACK_ROOK_KING_POS: IndexPosition = IndexPosition(63);
        const BLACK_ROOK_QUEEN_POS: IndexPosition = IndexPosition(56);
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

        // Castling: King gets moved normally by default logic but rook needs to be moved aswelll
        if mv.is_queen_castle && current_color == Color::White {
            // This workaround to perform not needs to be done because rust not trait is not const for whatever reason
            const ROOK_FROM_POS_INVERSE: Bitboard = Bitboard(!(IndexPosition(0).to_position().0));
            const ROOK_TO_POS: Position = IndexPosition(3).to_position();

            self.bbs[ColorPiece::WhiteRook as usize] &= ROOK_FROM_POS_INVERSE;
            self.bbs[ColorPiece::WhiteRook as usize] |= ROOK_TO_POS;

            self.pieces[IndexPosition(0).0] = ColorPiece::Empty;
            self.pieces[IndexPosition(3).0] = ColorPiece::WhiteRook;
        } else if mv.is_king_castle && current_color == Color::White {
            const ROOK_FROM_POS_INVERSE: Bitboard = Bitboard(!(IndexPosition(7).to_position().0));
            const ROOK_TO_POS: Position = IndexPosition(5).to_position();

            self.bbs[ColorPiece::WhiteRook as usize] &= ROOK_FROM_POS_INVERSE;
            self.bbs[ColorPiece::WhiteRook as usize] |= ROOK_TO_POS;

            self.pieces[IndexPosition(7).0] = ColorPiece::Empty;
            self.pieces[IndexPosition(5).0] = ColorPiece::WhiteRook;
        } else if mv.is_queen_castle && current_color == Color::Black {
            const ROOK_FROM_POS_INVERSE: Bitboard = Bitboard(!(IndexPosition(56).to_position().0));
            const ROOK_TO_POS: Position = IndexPosition(59).to_position();

            self.bbs[ColorPiece::BlackRook as usize] &= ROOK_FROM_POS_INVERSE;
            self.bbs[ColorPiece::BlackRook as usize] |= ROOK_TO_POS;

            self.pieces[IndexPosition(56).0] = ColorPiece::Empty;
            self.pieces[IndexPosition(59).0] = ColorPiece::BlackRook;
        } else if mv.is_king_castle && current_color == Color::Black {
            const ROOK_FROM_POS_INVERSE: Bitboard = Bitboard(!(IndexPosition(63).to_position().0));
            const ROOK_TO_POS: Position = IndexPosition(61).to_position();

            self.bbs[ColorPiece::BlackRook as usize] &= ROOK_FROM_POS_INVERSE;
            self.bbs[ColorPiece::BlackRook as usize] |= ROOK_TO_POS;

            self.pieces[IndexPosition(63).0] = ColorPiece::Empty;
            self.pieces[IndexPosition(61).0] = ColorPiece::BlackRook;
        }

        // Remove pawn if En-passant happened
        if mv.is_ep_capture {
            match current_color {
                Color::White => {
                    let pawn_mask = !to_pos.get_offset_pos(0, -1);
                    self.bbs[ColorPiece::BlackPawn as usize] &= pawn_mask;
                    self.pieces[to_idx.0 - 8] = ColorPiece::Empty;
                }
                Color::Black => {
                    let pawn_mask = !to_pos.get_offset_pos(0, 1);
                    self.bbs[ColorPiece::WhitePawn as usize] &= pawn_mask;
                    self.pieces[to_idx.0 + 8] = ColorPiece::Empty;
                }
            }
        }

        // Set En-passant target
        self.ep_target = if mv.is_double_move {
            let offset_dir: isize = match current_color {
                Color::White => -1,
                Color::Black => 1,
            };
            Some(to_pos.get_offset_pos(0, offset_dir))
        } else {
            None
        };

        // Add the from piece to the to position
        match mv.promotion {
            None => {
                self.bbs[from_piece as usize] |= to_pos;
                self.pieces[to_idx.0] = from_piece;
            }
            Some(piece) => {
                self.bbs[(piece as usize) * 2 + (current_color as usize)] |= to_pos;
                self.pieces[to_idx.0] = piece.to_color_piece(current_color);
            }
        }

        self.total_halfmove_counter += 1;
        self.current_color = !current_color;
        self.recalculate_black_white_empty_pieces();
    }
}
