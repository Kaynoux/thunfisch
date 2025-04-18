use crate::prelude::*;
/// Each piece type gets its own 64bits where
pub struct Board {
    pub white_pieces: Bitboard,
    pub black_pieces: Bitboard,
    pub empty_pieces: Bitboard,
    pub white_pawns: Bitboard,
    pub white_knights: Bitboard,
    pub white_rooks: Bitboard,
    pub white_bishops: Bitboard,
    pub white_queen: Position,
    pub white_king: Position,
    pub black_pawns: Bitboard,
    pub black_knights: Bitboard,
    pub black_rooks: Bitboard,
    pub black_bishops: Bitboard,
    pub black_queen: Position,
    pub black_king: Position,
}

impl Board {
    /// Converts a FEN to a Board
    /// FEN describes the position of all pieces on the board
    /// lowercase = black and uppercase = white
    pub fn new(fen: &str) -> Self {
        let mut board: Board = Board {
            white_pieces: Bitboard(0),
            black_pieces: Bitboard(0),
            empty_pieces: Bitboard(0),
            white_pawns: Bitboard(0),
            white_knights: Bitboard(0),
            white_rooks: Bitboard(0),
            white_bishops: Bitboard(0),
            white_queen: Position(0),
            white_king: Position(0),
            black_pawns: Bitboard(0),
            black_knights: Bitboard(0),
            black_rooks: Bitboard(0),
            black_bishops: Bitboard(0),
            black_queen: Position(0),
            black_king: Position(0),
        };

        // fen begins top left
        let mut index: usize = 56;

        for c in fen.chars() {
            match c {
                '/' => {
                    index = index.saturating_sub(16);
                }

                '1'..='8' => {
                    let skip = c.to_digit(10).unwrap() as usize;
                    index += skip;
                }

                ch => {
                    let bit = Position(1u64 << index as u64);
                    match ch {
                        'p' => board.black_pawns |= bit,
                        'n' => board.black_knights |= bit,
                        'b' => board.black_bishops |= bit,
                        'r' => board.black_rooks |= bit,
                        'q' => board.black_queen = bit,
                        'k' => board.black_king = bit,

                        'P' => board.white_pawns |= bit,
                        'N' => board.white_knights |= bit,
                        'B' => board.white_bishops |= bit,
                        'R' => board.white_rooks |= bit,
                        'Q' => board.white_queen = bit,
                        'K' => board.white_king = bit,

                        _ => {}
                    }
                    index += 1;
                }
            }
        }

        board.white_pieces = board.white_pawns
            | board.white_knights
            | board.white_bishops
            | board.white_rooks
            | board.white_queen
            | board.white_king;

        board.black_pieces = board.black_pawns
            | board.black_knights
            | board.black_bishops
            | board.black_rooks
            | board.black_queen
            | board.black_king;

        board.empty_pieces = !(board.white_pieces | board.black_pieces);
        board
    }

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
        if pos == self.white_queen {
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
        if pos == self.black_queen {
            return (Piece::Queen, Color::Black);
        }
        if pos == self.black_king {
            return (Piece::King, Color::Black);
        }

        // Color does not matter for empty
        (Piece::Empty, Color::White)
    }
}
