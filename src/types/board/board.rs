use crate::prelude::*;
/// Each piece type gets its own 64bits where
#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub white_pieces: Bitboard,
    pub black_pieces: Bitboard,
    pub empty_pieces: Bitboard,
    pub white_pawns: Bitboard,
    pub white_knights: Bitboard,
    pub white_rooks: Bitboard,
    pub white_bishops: Bitboard,
    pub white_queens: Bitboard,
    pub white_king: Bitboard,
    pub black_pawns: Bitboard,
    pub black_knights: Bitboard,
    pub black_rooks: Bitboard,
    pub black_bishops: Bitboard,
    pub black_queens: Bitboard,
    pub black_king: Bitboard,
    pub black_castle_left: bool,
    pub black_castle_right: bool,
    pub white_castle_left: bool,
    pub white_castle_right: bool,
    pub en_passant_target: Option<Position>,
    pub current_color: Color,
    pub halfmove_clock: isize,
    pub fullmove_counter: isize,
}

impl Board {
    #[inline(always)]
    pub fn get_pieces_by_color(&self, color: Color) -> Bitboard {
        match color {
            Color::Black => self.black_pieces,
            Color::White => self.white_pieces,
        }
    }

    #[inline(always)]
    pub fn get_non_friendly_pieces(&self, color: Color) -> Bitboard {
        match color {
            Color::Black => !self.black_pieces,
            Color::White => !self.white_pieces,
        }
    }

    #[inline(always)]
    pub fn get_empty_pieces(&self) -> Bitboard {
        self.empty_pieces
    }

    #[inline(always)]
    pub fn get_piece_and_color_at_position(&self, pos: Position) -> (Piece, Color) {
        if self.white_pawns.is_position_set(pos) {
            return (Piece::Pawn, Color::White);
        }
        if self.white_knights.is_position_set(pos) {
            return (Piece::Knight, Color::White);
        }
        if self.white_bishops.is_position_set(pos) {
            return (Piece::Bishop, Color::White);
        }
        if self.white_rooks.is_position_set(pos) {
            return (Piece::Rook, Color::White);
        }
        if self.white_queens.is_position_set(pos) {
            return (Piece::Queen, Color::White);
        }
        if pos == Position(self.white_king.0) {
            return (Piece::King, Color::White);
        }

        if self.black_pawns.is_position_set(pos) {
            return (Piece::Pawn, Color::Black);
        }
        if self.black_knights.is_position_set(pos) {
            return (Piece::Knight, Color::Black);
        }
        if self.black_bishops.is_position_set(pos) {
            return (Piece::Bishop, Color::Black);
        }
        if self.black_rooks.is_position_set(pos) {
            return (Piece::Rook, Color::Black);
        }
        if self.black_queens.is_position_set(pos) {
            return (Piece::Queen, Color::Black);
        }
        if pos == Position(self.black_king.0) {
            return (Piece::King, Color::Black);
        }

        // Color does not matter for empty
        (Piece::Empty, Color::White)
    }

    #[inline(always)]
    pub fn get_positions_by_piece_color(&self, color: Color, piece: Piece) -> Bitboard {
        match color {
            Color::Black => match piece {
                Piece::Empty => self.empty_pieces,
                Piece::Pawn => self.black_pawns,
                Piece::Knight => self.black_knights,
                Piece::Bishop => self.black_bishops,
                Piece::Rook => self.black_rooks,
                Piece::Queen => self.black_queens,
                Piece::King => Bitboard(self.black_king.0),
            },
            Color::White => match piece {
                Piece::Empty => self.empty_pieces,
                Piece::Pawn => self.white_pawns,
                Piece::Knight => self.white_knights,
                Piece::Bishop => self.white_bishops,
                Piece::Rook => self.white_rooks,
                Piece::Queen => self.white_queens,
                Piece::King => Bitboard(self.white_king.0),
            },
        }
    }

    #[inline(always)]
    pub fn get_king_pos(&self, color: Color) -> Position {
        match color {
            Color::Black => Position(self.black_king.0),
            Color::White => Position(self.white_king.0),
        }
    }

    #[inline(always)]
    pub fn recalculate_black_white_empty_pieces(&mut self) {
        self.white_pieces = self.white_pawns
            | self.white_knights
            | self.white_bishops
            | self.white_rooks
            | self.white_queens
            | self.white_king;

        self.black_pieces = self.black_pawns
            | self.black_knights
            | self.black_bishops
            | self.black_rooks
            | self.black_queens
            | self.black_king;

        self.empty_pieces = !(self.white_pieces | self.black_pieces);
    }

    #[inline(always)]
    pub fn get_count_of_piece(&self, color: Color, piece: Piece) -> u32 {
        match color {
            Color::Black => match piece {
                Piece::Empty => self.empty_pieces.get_count(),
                Piece::Pawn => self.black_pawns.get_count(),
                Piece::Knight => self.black_knights.get_count(),
                Piece::Bishop => self.black_bishops.get_count(),
                Piece::Rook => self.black_rooks.get_count(),
                Piece::Queen => self.black_queens.get_count(),
                Piece::King => Bitboard(self.black_king.0).get_count(),
            },
            Color::White => match piece {
                Piece::Empty => self.empty_pieces.get_count(),
                Piece::Pawn => self.white_pawns.get_count(),
                Piece::Knight => self.white_knights.get_count(),
                Piece::Bishop => self.white_bishops.get_count(),
                Piece::Rook => self.white_rooks.get_count(),
                Piece::Queen => self.white_queens.get_count(),
                Piece::King => Bitboard(self.white_king.0).get_count(),
            },
        }
    }

    pub fn all_piece_bitboards(&self) -> [Bitboard; 12] {
        [
            self.white_pawns,
            self.black_pawns,
            self.white_knights,
            self.black_knights,
            self.white_bishops,
            self.black_bishops,
            self.white_rooks,
            self.black_rooks,
            self.white_queens,
            self.black_queens,
            self.white_king,
            self.black_king,
        ]
    }
}
