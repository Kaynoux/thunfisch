use crate::{prelude::*, types::unmake_info::UnmakeInfo};

impl Board {
    pub fn make_move(&mut self, mv: &DecodedMove) {
        let mv_type = mv.mv_type;
        let color = self.current_color();
        let from_idx = mv.from;
        let to_idx = mv.to;
        let from_pos = from_idx.to_bit();
        let to_pos = to_idx.to_bit();

        let from_figure = self.figures(from_idx);
        let to_figure = self.figures(to_idx);

        // Unmake Info if move needs to be undone
        self.push_unmake_info_stack(UnmakeInfo {
            mv: *mv,
            white_queen_castle: self.white_queen_castle(),
            white_king_castle: self.white_king_castle(),
            black_queen_castle: self.black_queen_castle(),
            black_king_castle: self.black_king_castle(),
            capture: to_figure,
            ep_target: self.ep_target(),
            halfmove_clock: self.halfmove_clock(),
        });
        // Seems wrong at first because if ep than the target piece is a pawn
        // but here it will be empty but unmaking handles ep as a special case and
        // always restores the to position so this is correct

        // Remove start piece from bitboard
        self.bbs[from_figure as usize] &= !from_pos;
        self.pieces[from_idx.0] = Figure::Empty;

        // Remove target piece from bitboard
        self.bbs[to_figure as usize] &= !to_pos;
        self.pieces[to_idx.0] = Figure::Empty;

        // Revoking castling rights
        const WHITE_ROOK_QUEEN_POS: Square = Square(0);
        const WHITE_ROOK_KING_POS: Square = Square(7);
        const BLACK_ROOK_KING_POS: Square = Square(63);
        const BLACK_ROOK_QUEEN_POS: Square = Square(56);
        const WHITE_KING_POS: Square = Square(4);
        const BLACK_KING_POS: Square = Square(60);

        // Revoke castling rights if
        // - Rook on the relevant side has moved
        // - King has moved
        // - Rook on the relevant side was captured
        self.update_castling(color, from_figure.piece(), mv, to_figure.piece());

        // Castling: King gets moved normally by default logic but rook needs to be moved aswelll

        match (mv_type, color) {
            (MoveType::QueenCastle, White) => {
                // This workaround to perform not needs to be done because rust not trait is not const for whatever reason
                const ROOK_FROM_POS_INVERSE: Bitboard = Bitboard(!(Square(0).to_bit().0));
                const ROOK_TO_POS: Bit = Square(3).to_bit();

                self.bbs[Figure::WhiteRook as usize] &= ROOK_FROM_POS_INVERSE;
                self.bbs[Figure::WhiteRook as usize] |= ROOK_TO_POS;

                self.pieces[Square(0).0] = Figure::Empty;
                self.pieces[Square(3).0] = Figure::WhiteRook;
            }
            (MoveType::KingCastle, White) => {
                const ROOK_FROM_POS_INVERSE: Bitboard = Bitboard(!(Square(7).to_bit().0));
                const ROOK_TO_POS: Bit = Square(5).to_bit();

                self.bbs[Figure::WhiteRook as usize] &= ROOK_FROM_POS_INVERSE;
                self.bbs[Figure::WhiteRook as usize] |= ROOK_TO_POS;

                self.pieces[Square(7).0] = Figure::Empty;
                self.pieces[Square(5).0] = Figure::WhiteRook;
            }
            (MoveType::QueenCastle, Black) => {
                const ROOK_FROM_POS_INVERSE: Bitboard = Bitboard(!(Square(56).to_bit().0));
                const ROOK_TO_POS: Bit = Square(59).to_bit();

                self.bbs[Figure::BlackRook as usize] &= ROOK_FROM_POS_INVERSE;
                self.bbs[Figure::BlackRook as usize] |= ROOK_TO_POS;

                self.pieces[Square(56).0] = Figure::Empty;
                self.pieces[Square(59).0] = Figure::BlackRook;
            }
            (MoveType::KingCastle, Black) => {
                const ROOK_FROM_POS_INVERSE: Bitboard = Bitboard(!(Square(63).to_bit().0));
                const ROOK_TO_POS: Bit = Square(61).to_bit();

                self.bbs[Figure::BlackRook as usize] &= ROOK_FROM_POS_INVERSE;
                self.bbs[Figure::BlackRook as usize] |= ROOK_TO_POS;

                self.pieces[Square(63).0] = Figure::Empty;
                self.pieces[Square(61).0] = Figure::BlackRook;
            }
            _ => {}
        }

        // Remove pawn if En-passant happened
        if mv_type == MoveType::EpCapture {
            match color {
                White => {
                    let pawn_mask = !to_pos.get_offset_pos(0, -1);
                    self.bbs[Figure::BlackPawn as usize] &= pawn_mask;
                    self.pieces[to_idx.0 - 8] = Figure::Empty;
                }
                Black => {
                    let pawn_mask = !to_pos.get_offset_pos(0, 1);
                    self.bbs[Figure::WhitePawn as usize] &= pawn_mask;
                    self.pieces[to_idx.0 + 8] = Figure::Empty;
                }
            }
        }

        // Set En-passant target
        self.ep_target = if mv_type == MoveType::DoubleMove {
            let offset_dir: isize = match color {
                White => -1,
                Black => 1,
            };
            Some(to_pos.get_offset_pos(0, offset_dir))
        } else {
            None
        };

        let moved_piece = match mv_type.to_promotion_color_piece(color) {
            Some(piece) => piece,
            None => from_figure,
        };

        self.bbs[moved_piece as usize] |= to_pos;
        self.pieces[to_idx.0] = moved_piece;

        self.total_halfmove_counter += 1;
        self.current_color() = !color;
        self.recalculate_genereal_bitboards();
    }
}
