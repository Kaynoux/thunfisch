use crate::{move_generation, prelude::*, types::unmake_info::UnmakeInfo};
/// Each piece type gets its own 64bits where
#[derive(Clone)]
pub struct Board {
    pub white_pieces: Bitboard,
    pub black_pieces: Bitboard,
    pub bbs: [Bitboard; 13],
    pub pieces: [ColorPiece; 64],
    pub black_king_castle: bool,
    pub black_queen_castle: bool,
    pub white_queen_castle: bool,
    pub white_king_castle: bool,
    pub ep_target: Option<Position>,
    pub current_color: Color,
    pub halfmove_clock: usize,
    pub total_halfmove_counter: usize,
    pub unmake_info_stack: Vec<UnmakeInfo>,
}

impl Board {
    #[inline(always)]
    pub fn get_pieces_by_color(&self, color: Color) -> Bitboard {
        match color {
            Color::Black => self.black_pieces,
            Color::White => self.white_pieces,
        }
    }

    self.bbs[ColorPiece::BlackPawn as usize];

    #[inline(always)]
    pub fn get_non_friendly_pieces(&self, color: Color) -> Bitboard {
        match color {
            Color::Black => !self.black_pieces,
            Color::White => !self.white_pieces,
        }
    }

    #[inline(always)]
    pub fn get_enemy_pieces(&self, color: Color) -> Bitboard {
        match color {
            Color::Black => self.white_pieces,
            Color::White => self.black_pieces,
        }
    }

    #[inline(always)]
    pub fn get_empty_pieces(&self) -> Bitboard {
        self.bbs[ColorPiece::Empty as usize]
    }

    #[inline(always)]
    pub fn get_piece_and_color_at_position(&self, pos: Position) -> (Piece, Color) {
        if self.bbs[ColorPiece::WhitePawn as usize].is_position_set(pos) {
            (Piece::Pawn, Color::White)
        } else if self.bbs[ColorPiece::WhiteKnight as usize].is_position_set(pos) {
            (Piece::Knight, Color::White)
        } else if self.bbs[ColorPiece::WhiteBishop as usize].is_position_set(pos) {
            (Piece::Bishop, Color::White)
        } else if self.bbs[ColorPiece::WhiteRook as usize].is_position_set(pos) {
            (Piece::Rook, Color::White)
        } else if self.bbs[ColorPiece::WhiteQueen as usize].is_position_set(pos) {
            (Piece::Queen, Color::White)
        } else if pos == Position(self.bbs[ColorPiece::WhiteKing as usize].0) {
            (Piece::King, Color::White)
        } else if self.bbs[ColorPiece::BlackPawn as usize].is_position_set(pos) {
            (Piece::Pawn, Color::Black)
        } else if self.bbs[ColorPiece::BlackKnight as usize].is_position_set(pos) {
            (Piece::Knight, Color::Black)
        } else if self.bbs[ColorPiece::BlackBishop as usize].is_position_set(pos) {
            (Piece::Bishop, Color::Black)
        } else if self.bbs[ColorPiece::BlackRook as usize].is_position_set(pos) {
            (Piece::Rook, Color::Black)
        } else if self.bbs[ColorPiece::BlackQueen as usize].is_position_set(pos) {
            (Piece::Queen, Color::Black)
        } else if pos == Position(self.bbs[ColorPiece::BlackKing as usize].0) {
            (Piece::King, Color::Black)
        } else {
            // Color does not matter for empty
            (Piece::Empty, Color::White)
        }
    }

    #[inline(always)]
    pub fn get_piece_at_position(&self, pos: IndexPosition) -> Piece {
        match self.pieces[pos.0] {
            ColorPiece::Empty => Piece::Empty,
            ColorPiece::WhitePawn => Piece::Pawn,
            ColorPiece::BlackPawn => Piece::Pawn,
            ColorPiece::WhiteKnight => Piece::Knight,
            ColorPiece::BlackKnight => Piece::Knight,
            ColorPiece::WhiteBishop => Piece::Bishop,
            ColorPiece::BlackBishop => Piece::Bishop,
            ColorPiece::WhiteRook => Piece::Rook,
            ColorPiece::BlackRook => Piece::Rook,
            ColorPiece::WhiteQueen => Piece::Queen,
            ColorPiece::BlackQueen => Piece::Queen,
            ColorPiece::WhiteKing => Piece::King,
            ColorPiece::BlackKing => Piece::King,
        }
    }

    #[inline(always)]
    pub fn get_piece_idx_at_position(&self, pos: Position) -> usize {
        if self.bbs[ColorPiece::WhitePawn as usize].is_position_set(pos) {
            return 0;
        }
        if self.bbs[ColorPiece::WhiteKnight as usize].is_position_set(pos) {
            return 1;
        }
        if self.bbs[ColorPiece::WhiteBishop as usize].is_position_set(pos) {
            return 2;
        }
        if self.bbs[ColorPiece::WhiteRook as usize].is_position_set(pos) {
            return 3;
        }
        if self.bbs[ColorPiece::WhiteQueen as usize].is_position_set(pos) {
            return 4;
        }
        if pos == Position(self.bbs[ColorPiece::WhiteKing as usize].0) {
            return 5;
        }

        if self.bbs[ColorPiece::BlackPawn as usize].is_position_set(pos) {
            return 0;
        }
        if self.bbs[ColorPiece::BlackKnight as usize].is_position_set(pos) {
            return 1;
        }
        if self.bbs[ColorPiece::BlackBishop as usize].is_position_set(pos) {
            return 2;
        }
        if self.bbs[ColorPiece::BlackRook as usize].is_position_set(pos) {
            return 3;
        }
        if self.bbs[ColorPiece::BlackQueen as usize].is_position_set(pos) {
            return 4;
        }
        if pos == Position(self.bbs[ColorPiece::BlackKing as usize].0) {
            return 5;
        }

        0
    }

    #[inline(always)]
    pub const fn get_positions_by_piece_color(&self, color: Color, piece: Piece) -> Bitboard {
        match color {
            Color::Black => match piece {
                Piece::Empty => self.bbs[ColorPiece::Empty as usize],
                Piece::Pawn => self.bbs[ColorPiece::BlackPawn as usize],
                Piece::Knight => self.bbs[ColorPiece::BlackKnight as usize],
                Piece::Bishop => self.bbs[ColorPiece::BlackBishop as usize],
                Piece::Rook => self.bbs[ColorPiece::BlackRook as usize],
                Piece::Queen => self.bbs[ColorPiece::BlackQueen as usize],
                Piece::King => self.bbs[ColorPiece::BlackKing as usize],
            },
            Color::White => match piece {
                Piece::Empty => self.bbs[ColorPiece::Empty as usize],
                Piece::Pawn => self.bbs[ColorPiece::WhitePawn as usize],
                Piece::Knight => self.bbs[ColorPiece::WhiteKnight as usize],
                Piece::Bishop => self.bbs[ColorPiece::WhiteBishop as usize],
                Piece::Rook => self.bbs[ColorPiece::WhiteRook as usize],
                Piece::Queen => self.bbs[ColorPiece::WhiteQueen as usize],
                Piece::King => self.bbs[ColorPiece::WhiteKing as usize],
            },
        }
    }

    #[inline(always)]
    pub fn get_king_pos(&self, color: Color) -> Position {
        match color {
            Color::Black => Position(self.bbs[ColorPiece::BlackKing as usize].0),
            Color::White => Position(self.bbs[ColorPiece::WhiteKing as usize].0),
        }
    }

    #[inline(always)]
    pub fn recalculate_black_white_empty_pieces(&mut self) {
        self.white_pieces = self.bbs[ColorPiece::WhitePawn as usize]
            | self.bbs[ColorPiece::WhiteKnight as usize]
            | self.bbs[ColorPiece::WhiteBishop as usize]
            | self.bbs[ColorPiece::WhiteRook as usize]
            | self.bbs[ColorPiece::WhiteQueen as usize]
            | self.bbs[ColorPiece::WhiteKing as usize];

        self.black_pieces = self.bbs[ColorPiece::BlackPawn as usize]
            | self.bbs[ColorPiece::BlackKnight as usize]
            | self.bbs[ColorPiece::BlackBishop as usize]
            | self.bbs[ColorPiece::BlackRook as usize]
            | self.bbs[ColorPiece::BlackQueen as usize]
            | self.bbs[ColorPiece::BlackKing as usize];

        self.bbs[ColorPiece::Empty as usize] = !(self.white_pieces | self.black_pieces);
    }

    // #[inline(always)]
    // pub fn get_count_of_piece(&self, color: Color, piece: Piece) -> u32 {
    //     match color {
    //         Color::Black => match piece {
    //             Piece::Empty => self.empty_pieces.get_count(),
    //             Piece::Pawn => self.black_pawns.get_count(),
    //             Piece::Knight => self.black_knights.get_count(),
    //             Piece::Bishop => self.black_bishops.get_count(),
    //             Piece::Rook => self.black_rooks.get_count(),
    //             Piece::Queen => self.black_queens.get_count(),
    //             Piece::King => Bitboard(self.black_king.0).get_count(),
    //         },
    //         Color::White => match piece {
    //             Piece::Empty => self.empty_pieces.get_count(),
    //             Piece::Pawn => self.white_pawns.get_count(),
    //             Piece::Knight => self.white_knights.get_count(),
    //             Piece::Bishop => self.white_bishops.get_count(),
    //             Piece::Rook => self.white_rooks.get_count(),
    //             Piece::Queen => self.white_queens.get_count(),
    //             Piece::King => Bitboard(self.white_king.0).get_count(),
    //         },
    //     }
    // }

    #[inline(always)]
    pub fn is_in_check(&self) -> bool {
        let opposite_attacks = move_generation::get_all_attacks(self, !self.current_color);

        (opposite_attacks & self.get_king_pos(self.current_color)) != Bitboard(0)
    }
}
