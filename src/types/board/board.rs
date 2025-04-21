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
    pub white_king: Position,
    pub black_pawns: Bitboard,
    pub black_knights: Bitboard,
    pub black_rooks: Bitboard,
    pub black_bishops: Bitboard,
    pub black_queens: Bitboard,
    pub black_king: Position,
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
    pub fn get_friendly_pieces(&self, color: Color) -> Bitboard {
        match color {
            Color::Black => self.black_pieces,
            Color::White => self.white_pieces,
        }
    }

    pub fn get_enemy_pieces(&self, color: Color) -> Bitboard {
        match color {
            Color::Black => self.white_pieces,
            Color::White => self.black_pieces,
        }
    }

    pub fn get_non_friendly_pieces(&self, color: Color) -> Bitboard {
        match color {
            Color::Black => !self.black_pieces,
            Color::White => !self.white_pieces,
        }
    }

    pub fn get_empty_pieces(&self) -> Bitboard {
        self.empty_pieces
    }

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
        if pos == self.white_king {
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
        if pos == self.black_king {
            return (Piece::King, Color::Black);
        }

        // Color does not matter for empty
        (Piece::Empty, Color::White)
    }

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

    pub fn get_king_pos(&self, color: Color) -> Position {
        match color {
            Color::Black => self.black_king,
            Color::White => self.white_king,
        }
    }

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
}
