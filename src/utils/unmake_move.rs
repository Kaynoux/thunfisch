use crate::prelude::*;

impl Board {
    pub fn unmake_move(&mut self) {
        let prev = match self.unmake_info_stack.pop() {
            Some(info) => info,
            None => {
                println!("info: Could not undo move because there was no previous move");
                return;
            }
        };
        let mv = prev.mv;
        let mv_type = mv.mv_type;
        let color = !self.current_color;
        let from_idx = mv.from;
        let captured_idx = mv.to;
        let from_pos = from_idx.to_bit();
        let captured_pos = captured_idx.to_bit();

        let captured_piece = prev.capture;
        let moved_piece = self.pieces[captured_idx.0];

        self.current_color = color;
        self.total_halfmove_counter -= 1;
        self.halfmove_clock = prev.halfmove_clock;
        self.ep_target = prev.ep_target;

        self.white_queen_castle = prev.white_queen_castle;
        self.white_king_castle = prev.white_king_castle;
        self.black_queen_castle = prev.black_queen_castle;
        self.black_king_castle = prev.black_king_castle;

        let original_piece = if mv_type.is_promotion() {
            match color {
                Color::White => Figure::WhitePawn,
                Color::Black => Figure::BlackPawn,
            }
        } else {
            moved_piece
        };

        // Add from piece to from pos bitboard
        self.bbs[original_piece as usize] |= from_pos;
        self.pieces[from_idx.0] = original_piece;

        // Remove from piece from to pos bitboard
        self.bbs[moved_piece as usize] &= !captured_pos;

        // Add to piece back to bitboard
        self.bbs[captured_piece as usize] |= captured_pos;

        // Set pieces array to to piece
        self.pieces[captured_idx.0] = captured_piece;

        // Undo if En-passant happened
        if mv_type == MoveType::EpCapture {
            match color {
                Color::White => {
                    let pawn_pos = captured_pos.get_offset_pos(0, -1);
                    self.bbs[Figure::BlackPawn as usize] |= pawn_pos;
                    self.pieces[captured_idx.0 - 8] = Figure::BlackPawn;
                }
                Color::Black => {
                    let pawn_pos = captured_pos.get_offset_pos(0, 1);
                    self.bbs[Figure::WhitePawn as usize] |= pawn_pos;
                    self.pieces[captured_idx.0 + 8] = Figure::WhitePawn;
                }
            }
        }

        // Castling: King gets moved back normally by default logic but rook needs to be moved back aswelll
        match (mv_type, color) {
            (MoveType::QueenCastle, Color::White) => {
                // This workaround to perform not needs to be done because rust not trait is not const for whatever reason
                const ROOK_FROM_POS: Bit = Square(0).to_bit();
                const ROOK_TO_POS_INVERSE: Bitboard = Bitboard(!(Square(3).to_bit().0));

                self.bbs[Figure::WhiteRook as usize] |= ROOK_FROM_POS;
                self.bbs[Figure::WhiteRook as usize] &= ROOK_TO_POS_INVERSE;

                self.pieces[Square(0).0] = Figure::WhiteRook;
                self.pieces[Square(3).0] = Figure::Empty;
            }
            (MoveType::KingCastle, Color::White) => {
                const ROOK_FROM_POS: Bit = Square(7).to_bit();
                const ROOK_TO_POS_INVERSE: Bitboard = Bitboard(!(Square(5).to_bit().0));

                self.bbs[Figure::WhiteRook as usize] |= ROOK_FROM_POS;
                self.bbs[Figure::WhiteRook as usize] &= ROOK_TO_POS_INVERSE;

                self.pieces[Square(7).0] = Figure::WhiteRook;
                self.pieces[Square(5).0] = Figure::Empty;
            }
            (MoveType::QueenCastle, Color::Black) => {
                const ROOK_FROM_POS: Bit = Square(56).to_bit();
                const ROOK_TO_POS_INVERSE: Bitboard =
                    Bitboard(!(Square(59).to_bit().0));

                self.bbs[Figure::BlackRook as usize] |= ROOK_FROM_POS;
                self.bbs[Figure::BlackRook as usize] &= ROOK_TO_POS_INVERSE;

                self.pieces[Square(56).0] = Figure::BlackRook;
                self.pieces[Square(59).0] = Figure::Empty;
            }
            (MoveType::KingCastle, Color::Black) => {
                const ROOK_FROM_POS: Bit = Square(63).to_bit();
                const ROOK_TO_POS_INVERSE: Bitboard =
                    Bitboard(!(Square(61).to_bit().0));

                self.bbs[Figure::BlackRook as usize] |= ROOK_FROM_POS;
                self.bbs[Figure::BlackRook as usize] &= ROOK_TO_POS_INVERSE;

                self.pieces[Square(63).0] = Figure::BlackRook;
                self.pieces[Square(61).0] = Figure::Empty;
            }
            _ => {}
        }

        self.recalculate_genereal_bitboards();
    }
}
