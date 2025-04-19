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
            white_queens: Bitboard(0),
            white_king: Position(0),
            black_pawns: Bitboard(0),
            black_knights: Bitboard(0),
            black_rooks: Bitboard(0),
            black_bishops: Bitboard(0),
            black_queens: Bitboard(0),
            black_king: Position(0),
            black_castle_left: true,
            black_castle_right: true,
            white_castle_left: true,
            white_castle_right: true,
            en_passant_target: None,

            current_color: Color::Black,
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
                        'q' => board.black_queens |= bit,
                        'k' => board.black_king = bit,

                        'P' => board.white_pawns |= bit,
                        'N' => board.white_knights |= bit,
                        'B' => board.white_bishops |= bit,
                        'R' => board.white_rooks |= bit,
                        'Q' => board.white_queens |= bit,
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
            | board.white_queens
            | board.white_king;

        board.black_pieces = board.black_pawns
            | board.black_knights
            | board.black_bishops
            | board.black_rooks
            | board.black_queens
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

    pub fn make_move(&mut self, chess_move: &ChessMove) {
        /// !!NEED TO ALSO MOVE ROOK ON CASTLE
        let start_pos = chess_move.from;
        let target_pos = chess_move.to;

        let (start_piece, start_color) = self.get_piece_and_color_at_position(start_pos);
        let (target_piece, target_color) = self.get_piece_and_color_at_position(target_pos);

        // Remove start piece from bitboard
        let start_mask = !start_pos;
        match start_color {
            Color::Black => match start_piece {
                Piece::Empty => self.empty_pieces &= start_mask,
                Piece::Pawn => {
                    self.black_pieces &= start_mask;
                    self.black_pawns &= start_mask;
                }
                Piece::Knight => {
                    self.black_pieces &= start_mask;
                    self.black_knights &= start_mask;
                }
                Piece::Bishop => {
                    self.black_pieces &= start_mask;
                    self.black_bishops &= start_mask;
                }
                Piece::Rook => {
                    self.black_pieces &= start_mask;
                    self.black_rooks &= start_mask;
                }
                Piece::Queen => {
                    self.black_pieces &= start_mask;
                    self.black_queens &= start_mask;
                }
                Piece::King => {
                    self.black_pieces &= start_mask;
                    self.black_king &= start_mask;
                }
            },
            Color::White => match start_piece {
                Piece::Empty => self.empty_pieces &= start_mask,
                Piece::Pawn => {
                    self.white_pieces &= start_mask;
                    self.white_pawns &= start_mask;
                }
                Piece::Knight => {
                    self.white_pieces &= start_mask;
                    self.white_knights &= start_mask;
                }
                Piece::Bishop => {
                    self.white_pieces &= start_mask;
                    self.white_bishops &= start_mask;
                }
                Piece::Rook => {
                    self.white_pieces &= start_mask;
                    self.white_rooks &= start_mask;
                }
                Piece::Queen => {
                    self.white_pieces &= start_mask;
                    self.white_queens &= start_mask;
                }
                Piece::King => {
                    self.white_pieces &= start_mask;
                    self.white_king &= start_mask;
                }
            },
        }

        // Remove target piece from bitboard
        let target_mask = !target_pos;
        match target_color {
            Color::Black => match target_piece {
                Piece::Empty => self.empty_pieces &= target_mask,
                Piece::Pawn => {
                    self.black_pieces &= target_mask;
                    self.black_pawns &= target_mask;
                }
                Piece::Knight => {
                    self.black_pieces &= target_mask;
                    self.black_knights &= target_mask;
                }
                Piece::Bishop => {
                    self.black_pieces &= target_mask;
                    self.black_bishops &= target_mask;
                }
                Piece::Rook => {
                    self.black_pieces &= target_mask;
                    self.black_rooks &= target_mask;
                }
                Piece::Queen => {
                    self.black_pieces &= target_mask;
                    self.black_king &= target_mask;
                }
                Piece::King => {
                    self.black_pieces &= target_mask;
                    self.black_king &= target_mask;
                }
            },
            Color::White => match target_piece {
                Piece::Empty => self.empty_pieces &= target_mask,
                Piece::Pawn => {
                    self.white_pieces &= target_mask;
                    self.white_pawns &= target_mask;
                }
                Piece::Knight => {
                    self.white_pieces &= target_mask;
                    self.white_knights &= target_mask;
                }
                Piece::Bishop => {
                    self.white_pieces &= target_mask;
                    self.white_bishops &= target_mask;
                }
                Piece::Rook => {
                    self.white_pieces &= target_mask;
                    self.white_rooks &= target_mask;
                }
                Piece::Queen => {
                    self.white_pieces &= target_mask;
                    self.white_king &= target_mask;
                }
                Piece::King => {
                    self.white_pieces &= target_mask;
                    self.white_king &= target_mask;
                }
            },
        }

        // Add the start piece to the target position
        match start_color {
            Color::Black => match start_piece {
                Piece::Empty => {}
                Piece::Pawn => {
                    self.black_pieces |= target_pos;
                    self.black_pawns |= target_pos;
                }
                Piece::Knight => {
                    self.black_pieces |= target_pos;
                    self.black_knights |= target_pos;
                }
                Piece::Bishop => {
                    self.black_pieces |= target_pos;
                    self.black_bishops |= target_pos;
                }
                Piece::Rook => {
                    self.black_pieces |= target_pos;
                    self.black_rooks |= target_pos;
                }
                Piece::Queen => {
                    self.black_pieces |= target_pos;
                    self.black_king |= target_pos;
                }
                Piece::King => {
                    self.black_pieces |= target_pos;
                    self.black_king |= target_pos;
                }
            },
            Color::White => match start_piece {
                Piece::Empty => {}
                Piece::Pawn => {
                    self.white_pieces |= target_pos;
                    self.white_pawns |= target_pos;
                }
                Piece::Knight => {
                    self.white_pieces |= target_pos;
                    self.white_knights |= target_pos;
                }
                Piece::Bishop => {
                    self.white_pieces |= target_pos;
                    self.white_bishops |= target_pos;
                }
                Piece::Rook => {
                    self.white_pieces |= target_pos;
                    self.white_rooks |= target_pos;
                }
                Piece::Queen => {
                    self.white_pieces |= target_pos;
                    self.white_king |= target_pos;
                }
                Piece::King => {
                    self.white_pieces |= target_pos;
                    self.white_king |= target_pos;
                }
            },
        }
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
}
