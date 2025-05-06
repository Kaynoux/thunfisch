use crate::{move_generation, prelude::*, types::unmake_info::UnmakeInfo};
/// Each piece type gets its own 64bits where
#[derive(Clone)]
pub struct Board {
    pub white_positions: Bitboard,
    pub black_positions: Bitboard,
    pub occupied: Bitboard,
    pub bbs: [Bitboard; 13],
    pub pieces: [Figure; 64],
    pub black_king_castle: bool,
    pub black_queen_castle: bool,
    pub white_queen_castle: bool,
    pub white_king_castle: bool,
    pub ep_target: Option<Bit>,
    pub current_color: Color,
    pub halfmove_clock: usize,
    pub total_halfmove_counter: usize,
    pub unmake_info_stack: Vec<UnmakeInfo>,
}

impl Board {
    #[inline(always)]
    pub fn get_pieces_by_color(&self, color: Color) -> Bitboard {
        match color {
            Color::Black => self.black_positions,
            Color::White => self.white_positions,
        }
    }

    #[inline(always)]
    pub fn get_non_friendly_pieces(&self, color: Color) -> Bitboard {
        match color {
            Color::Black => !self.black_positions,
            Color::White => !self.white_positions,
        }
    }

    #[inline(always)]
    pub fn get_enemy_pieces(&self, color: Color) -> Bitboard {
        match color {
            Color::Black => self.white_positions,
            Color::White => self.black_positions,
        }
    }

    #[inline(always)]
    pub fn get_empty_pieces(&self) -> Bitboard {
        self.bbs[Figure::Empty as usize]
    }

    #[inline(always)]
    pub fn get_piece_and_color_at_position(&self, pos: Bit) -> (Piece, Color) {
        if self.bbs[Figure::WhitePawn as usize].is_position_set(pos) {
            (Piece::Pawn, Color::White)
        } else if self.bbs[Figure::WhiteKnight as usize].is_position_set(pos) {
            (Piece::Knight, Color::White)
        } else if self.bbs[Figure::WhiteBishop as usize].is_position_set(pos) {
            (Piece::Bishop, Color::White)
        } else if self.bbs[Figure::WhiteRook as usize].is_position_set(pos) {
            (Piece::Rook, Color::White)
        } else if self.bbs[Figure::WhiteQueen as usize].is_position_set(pos) {
            (Piece::Queen, Color::White)
        } else if pos == Bit(self.bbs[Figure::WhiteKing as usize].0) {
            (Piece::King, Color::White)
        } else if self.bbs[Figure::BlackPawn as usize].is_position_set(pos) {
            (Piece::Pawn, Color::Black)
        } else if self.bbs[Figure::BlackKnight as usize].is_position_set(pos) {
            (Piece::Knight, Color::Black)
        } else if self.bbs[Figure::BlackBishop as usize].is_position_set(pos) {
            (Piece::Bishop, Color::Black)
        } else if self.bbs[Figure::BlackRook as usize].is_position_set(pos) {
            (Piece::Rook, Color::Black)
        } else if self.bbs[Figure::BlackQueen as usize].is_position_set(pos) {
            (Piece::Queen, Color::Black)
        } else if pos == Bit(self.bbs[Figure::BlackKing as usize].0) {
            (Piece::King, Color::Black)
        } else {
            // Color does not matter for empty
            (Piece::Empty, Color::White)
        }
    }

    #[inline(always)]
    pub fn get_piece_at_position(&self, pos: Square) -> Piece {
        match self.pieces[pos.0] {
            Figure::Empty => Piece::Empty,
            Figure::WhitePawn => Piece::Pawn,
            Figure::BlackPawn => Piece::Pawn,
            Figure::WhiteKnight => Piece::Knight,
            Figure::BlackKnight => Piece::Knight,
            Figure::WhiteBishop => Piece::Bishop,
            Figure::BlackBishop => Piece::Bishop,
            Figure::WhiteRook => Piece::Rook,
            Figure::BlackRook => Piece::Rook,
            Figure::WhiteQueen => Piece::Queen,
            Figure::BlackQueen => Piece::Queen,
            Figure::WhiteKing => Piece::King,
            Figure::BlackKing => Piece::King,
        }
    }

    #[inline(always)]
    pub fn get_piece_idx_at_position(&self, pos: Bit) -> usize {
        if self.bbs[Figure::WhitePawn as usize].is_position_set(pos) {
            return 0;
        }
        if self.bbs[Figure::WhiteKnight as usize].is_position_set(pos) {
            return 1;
        }
        if self.bbs[Figure::WhiteBishop as usize].is_position_set(pos) {
            return 2;
        }
        if self.bbs[Figure::WhiteRook as usize].is_position_set(pos) {
            return 3;
        }
        if self.bbs[Figure::WhiteQueen as usize].is_position_set(pos) {
            return 4;
        }
        if pos == Bit(self.bbs[Figure::WhiteKing as usize].0) {
            return 5;
        }

        if self.bbs[Figure::BlackPawn as usize].is_position_set(pos) {
            return 0;
        }
        if self.bbs[Figure::BlackKnight as usize].is_position_set(pos) {
            return 1;
        }
        if self.bbs[Figure::BlackBishop as usize].is_position_set(pos) {
            return 2;
        }
        if self.bbs[Figure::BlackRook as usize].is_position_set(pos) {
            return 3;
        }
        if self.bbs[Figure::BlackQueen as usize].is_position_set(pos) {
            return 4;
        }
        if pos == Bit(self.bbs[Figure::BlackKing as usize].0) {
            return 5;
        }

        0
    }

    #[inline(always)]
    pub const fn get_bitboard_by_piece_color(&self, color: Color, piece: Piece) -> Bitboard {
        match color {
            Color::Black => match piece {
                Piece::Empty => self.bbs[Figure::Empty as usize],
                Piece::Pawn => self.bbs[Figure::BlackPawn as usize],
                Piece::Knight => self.bbs[Figure::BlackKnight as usize],
                Piece::Bishop => self.bbs[Figure::BlackBishop as usize],
                Piece::Rook => self.bbs[Figure::BlackRook as usize],
                Piece::Queen => self.bbs[Figure::BlackQueen as usize],
                Piece::King => self.bbs[Figure::BlackKing as usize],
            },
            Color::White => match piece {
                Piece::Empty => self.bbs[Figure::Empty as usize],
                Piece::Pawn => self.bbs[Figure::WhitePawn as usize],
                Piece::Knight => self.bbs[Figure::WhiteKnight as usize],
                Piece::Bishop => self.bbs[Figure::WhiteBishop as usize],
                Piece::Rook => self.bbs[Figure::WhiteRook as usize],
                Piece::Queen => self.bbs[Figure::WhiteQueen as usize],
                Piece::King => self.bbs[Figure::WhiteKing as usize],
            },
        }
    }

    #[inline(always)]
    pub fn get_king_bit(&self, color: Color) -> Bit {
        match color {
            Color::Black => Bit(self.bbs[Figure::BlackKing as usize].0),
            Color::White => Bit(self.bbs[Figure::WhiteKing as usize].0),
        }
    }

    #[inline(always)]
    pub fn recalculate_genereal_bitboards(&mut self) {
        self.white_positions = self.bbs[Figure::WhitePawn as usize]
            | self.bbs[Figure::WhiteKnight as usize]
            | self.bbs[Figure::WhiteBishop as usize]
            | self.bbs[Figure::WhiteRook as usize]
            | self.bbs[Figure::WhiteQueen as usize]
            | self.bbs[Figure::WhiteKing as usize];

        self.black_positions = self.bbs[Figure::BlackPawn as usize]
            | self.bbs[Figure::BlackKnight as usize]
            | self.bbs[Figure::BlackBishop as usize]
            | self.bbs[Figure::BlackRook as usize]
            | self.bbs[Figure::BlackQueen as usize]
            | self.bbs[Figure::BlackKing as usize];

        self.occupied = self.white_positions | self.black_positions;
        self.bbs[Figure::Empty as usize] = !!self.occupied;
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

        (opposite_attacks & self.get_king_bit(self.current_color)) != Bitboard(0)
    }
}
