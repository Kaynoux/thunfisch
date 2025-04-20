use crate::prelude::*;

impl Board {
    pub fn make_move(&mut self, mv: &ChessMove) {
        let start_pos = mv.from;
        let target_pos = mv.to;

        let (start_piece, start_color) = self.get_piece_and_color_at_position(start_pos);
        let (target_piece, target_color) = self.get_piece_and_color_at_position(target_pos);

        // Revoking castling rights
        const WHITE_ROOK_LEFT_POS: Position = Position::from_idx(0);
        const WHITE_ROOK_RIGHT_POS: Position = Position::from_idx(7);
        const BLACK_ROOK_LEFT_POS: Position = Position::from_idx(56);
        const BLACK_ROOK_RIGHT_POS: Position = Position::from_idx(63);
        const WHITE_KING_POS: Position = Position::from_idx(4);
        const BLACK_KING_POS: Position = Position::from_idx(60);

        match (start_piece, mv.from) {
            (Piece::Rook, WHITE_ROOK_LEFT_POS) => self.white_castle_left = false,
            (Piece::Rook, WHITE_ROOK_RIGHT_POS) => self.white_castle_right = false,
            (Piece::Rook, BLACK_ROOK_LEFT_POS) => self.black_castle_left = false,
            (Piece::Rook, BLACK_ROOK_RIGHT_POS) => self.black_castle_right = false,
            (Piece::King, WHITE_KING_POS) => {
                self.white_castle_left = false;
                self.white_castle_right = false;
            }
            (Piece::King, BLACK_KING_POS) => {
                self.black_castle_left = false;
                self.black_castle_right = false;
            }
            (_, _) => {}
        }

        // Handling castling
        const WHITE_CASTLE_LEFT_POS: Position = Position::from_idx(2);
        const WHITE_CASTLE_RIGHT_POS: Position = Position::from_idx(6);
        const BLACK_CASTLE_LEFT_POS: Position = Position::from_idx(58);
        const BLACK_CASTLE_RIGHT_POS: Position = Position::from_idx(62);
        if mv.is_castle {
            match mv.to {
                WHITE_CASTLE_LEFT_POS => {
                    let inverse_rook_position = !Position::from_idx(0);
                    self.black_rooks &= inverse_rook_position;
                    self.black_pieces &= inverse_rook_position;
                    let rook_target_position = Position::from_idx(3);
                    self.black_rooks |= rook_target_position;
                    self.black_pieces |= rook_target_position;
                }
                WHITE_CASTLE_RIGHT_POS => {
                    let inverse_rook_position = !Position::from_idx(7);
                    self.black_rooks &= inverse_rook_position;
                    self.black_pieces &= inverse_rook_position;
                    let rook_target_position = Position::from_idx(5);
                    self.black_rooks |= rook_target_position;
                    self.black_pieces |= rook_target_position;
                }
                BLACK_CASTLE_LEFT_POS => {
                    let inverse_rook_position = !Position::from_idx(56);
                    self.black_rooks &= inverse_rook_position;
                    self.black_pieces &= inverse_rook_position;
                    let rook_target_position = Position::from_idx(59);
                    self.black_rooks |= rook_target_position;
                    self.black_pieces |= rook_target_position;
                }
                BLACK_CASTLE_RIGHT_POS => {
                    let inverse_rook_position = !Position::from_idx(63);
                    self.black_rooks &= inverse_rook_position;
                    self.black_pieces &= inverse_rook_position;
                    let rook_target_position = Position::from_idx(61);
                    self.black_rooks |= rook_target_position;
                    self.black_pieces |= rook_target_position;
                }
                _ => {}
            }
        }

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
                    if mv.is_promotion {
                        self.black_pieces |= target_pos;
                        self.black_queens |= target_pos;
                    } else {
                        self.black_pieces |= target_pos;
                        self.black_pawns |= target_pos;
                    }
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
                    if mv.is_promotion {
                        self.white_pieces |= target_pos;
                        self.white_queens |= target_pos;
                    } else {
                        self.white_pieces |= target_pos;
                        self.white_pawns |= target_pos;
                    }
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
}
