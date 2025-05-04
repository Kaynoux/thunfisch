use crate::prelude::*;

impl Board {
    pub fn unmake_move(&mut self) {
        let prev = match self.unmake_info {
            Some(info) => info,
            None => {
                println!("info: Could not undo move because there was no previous move");
                return;
            }
        };

        let mv = prev.mv;

        let color = !self.current_color;
        let from_idx = mv.from;
        let to_idx = mv.to;
        let from_pos = from_idx.to_position();
        let to_pos = to_idx.to_position();

        let to_piece = prev.capture;

        self.current_color = color;
        self.total_halfmove_counter -= 1;
        self.halfmove_clock = prev.halfmove_clock;
        self.ep_target = prev.ep_target;

        self.white_queen_castle = prev.white_queen_castle;
        self.white_king_castle = prev.white_king_castle;
        self.black_queen_castle = prev.black_queen_castle;
        self.black_king_castle = prev.black_king_castle;

        match mv.promotion {
            None => {
                let arrived_piece = self.pieces[to_idx.0];
                // Add from piece to from pos bitboard
                self.bbs[arrived_piece as usize] |= from_pos;

                // Remove from piece from to pos bitboard
                self.bbs[arrived_piece as usize] &= !to_pos;

                // Add to piece back to bitboard
                self.bbs[to_piece as usize] |= to_pos;

                // Set pieces array to to piece
                self.pieces[to_idx.0] = to_piece;
            }
            Some(_) => {
                let arrived_piece = self.pieces[to_idx.0];
                let original_piece = match color {
                    Color::White => ColorPiece::WhitePawn,
                    Color::Black => ColorPiece::BlackPawn,
                };
                // Add from piece to from pos bitboard
                self.bbs[original_piece as usize] |= from_pos;

                // Remove from piece from to pos bitboard
                self.bbs[arrived_piece as usize] &= !to_pos;
            }
        }

        // Add to piece back to bitboard
        self.bbs[to_piece as usize] |= to_pos;

        // Set pieces array to to piece
        self.pieces[to_idx.0] = to_piece;

        // Undo if En-passant happened
        if mv.is_ep_capture {
            match color {
                Color::White => {
                    let pawn_pos = to_pos.get_offset_pos(0, -1);
                    self.bbs[ColorPiece::BlackPawn as usize] |= pawn_pos;
                    self.pieces[to_idx.0 - 8] = ColorPiece::BlackPawn;
                }
                Color::Black => {
                    let pawn_pos = to_pos.get_offset_pos(0, 1);
                    self.bbs[ColorPiece::WhitePawn as usize] |= pawn_pos;
                    self.pieces[to_idx.0 + 8] = ColorPiece::WhitePawn;
                }
            }
        }

        // Castling: King gets moved back normally by default logic but rook needs to be moved back aswelll
        if mv.is_queen_castle && color == Color::White {
            // This workaround to perform not needs to be done because rust not trait is not const for whatever reason
            const ROOK_FROM_POS: Position = IndexPosition(0).to_position();
            const ROOK_TO_POS_INVERSE: Bitboard = Bitboard(!(IndexPosition(3).to_position().0));

            self.bbs[ColorPiece::WhiteRook as usize] |= ROOK_FROM_POS;
            self.bbs[ColorPiece::WhiteRook as usize] &= ROOK_TO_POS_INVERSE;

            self.pieces[IndexPosition(0).0] = ColorPiece::WhiteRook;
            self.pieces[IndexPosition(3).0] = ColorPiece::Empty;
        } else if mv.is_king_castle && color == Color::White {
            const ROOK_FROM_POS: Position = IndexPosition(7).to_position();
            const ROOK_TO_POS_INVERSE: Bitboard = Bitboard(!(IndexPosition(5).to_position().0));

            self.bbs[ColorPiece::WhiteRook as usize] |= ROOK_FROM_POS;
            self.bbs[ColorPiece::WhiteRook as usize] &= ROOK_TO_POS_INVERSE;

            self.pieces[IndexPosition(7).0] = ColorPiece::WhiteRook;
            self.pieces[IndexPosition(5).0] = ColorPiece::Empty;
        } else if mv.is_queen_castle && color == Color::Black {
            const ROOK_FROM_POS: Position = IndexPosition(56).to_position();
            const ROOK_TO_POS_INVERSE: Bitboard = Bitboard(!(IndexPosition(59).to_position().0));

            self.bbs[ColorPiece::BlackRook as usize] |= ROOK_FROM_POS;
            self.bbs[ColorPiece::BlackRook as usize] &= ROOK_TO_POS_INVERSE;

            self.pieces[IndexPosition(56).0] = ColorPiece::BlackRook;
            self.pieces[IndexPosition(59).0] = ColorPiece::Empty;
        } else if mv.is_king_castle && color == Color::Black {
            const ROOK_FROM_POS: Position = IndexPosition(63).to_position();
            const ROOK_TO_POS_INVERSE: Bitboard = Bitboard(!(IndexPosition(61).to_position().0));

            self.bbs[ColorPiece::BlackRook as usize] |= ROOK_FROM_POS;
            self.bbs[ColorPiece::BlackRook as usize] &= ROOK_TO_POS_INVERSE;

            self.pieces[IndexPosition(63).0] = ColorPiece::BlackRook;
            self.pieces[IndexPosition(61).0] = ColorPiece::Empty;
        }

        self.recalculate_black_white_empty_pieces();
    }
}
