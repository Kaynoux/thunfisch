use crate::position_generation::*;
use crate::prelude::*;

impl Board {
    #[inline(always)]
    pub fn get_moves(&mut self, only_captures: bool) -> Vec<EncodedMove> {
        let color = self.current_color;
        let mut moves: Vec<EncodedMove> = Vec::new();

        // Pawns
        get_pawn_moves(self, color, &mut moves, only_captures);

        // Knight
        get_moves_for_piece_type(
            self,
            Piece::Knight,
            color,
            &mut moves,
            get_knight_positions,
            only_captures,
        );

        // Bishop
        get_moves_for_piece_type(
            self,
            Piece::Bishop,
            color,
            &mut moves,
            get_bishop_positions,
            only_captures,
        );

        // Rook
        get_moves_for_piece_type(
            self,
            Piece::Rook,
            color,
            &mut moves,
            get_rook_positions,
            only_captures,
        );

        // Queen
        get_moves_for_piece_type(
            self,
            Piece::Queen,
            color,
            &mut moves,
            get_queen_positions,
            only_captures,
        );

        // King
        get_king_moves(
            self,
            self.get_king_bit(color),
            color,
            &mut moves,
            get_king_positions,
            only_captures,
        );

        if !only_captures {
            get_castle_moves(self, color, &mut moves);
        }

        get_en_passant_moves(self, color, &mut moves);

        moves
    }
}

#[inline(always)]
pub fn push_if_legal(moves: &mut Vec<EncodedMove>, board: &mut Board, mv: DecodedMove) {
    if mv.is_legal(board) {
        moves.push(mv.encode());
    }
}

/// Calculate all moves for each instance of a piece type exept the king because it is unique and pawns because they are
#[inline(always)]
pub fn get_moves_for_piece_type(
    board: &mut Board,
    piece: Piece,
    color: Color,
    moves: &mut Vec<EncodedMove>,
    f: fn(board: &Board, pos: Bit, color: Color) -> Bitboard,
    only_captures: bool,
) -> Bitboard {
    let mut piece_positions = board.get_bitboard_by_piece_color(color, piece);
    let mut target_positions = Bitboard(0);

    while piece_positions != Bitboard(0) {
        let current_pos = piece_positions.pop_lsb_position().unwrap();
        let from = current_pos.to_square();

        let mut target_positions_for_one_piece = f(board, current_pos, color);

        if only_captures {
            target_positions_for_one_piece &= board.get_non_friendly_pieces(color);
        }

        target_positions |= target_positions_for_one_piece;

        while target_positions_for_one_piece != Bitboard(0) {
            let target_pos = target_positions_for_one_piece.pop_lsb_position().unwrap();

            let to = target_pos.to_square();
            match target_pos.is_enemy(board, color) {
                true => push_if_legal(
                    moves,
                    board,
                    DecodedMove {
                        from: from,
                        to: to,
                        mv_type: MoveType::Capture,
                    },
                ),
                false => push_if_legal(
                    moves,
                    board,
                    DecodedMove {
                        from: from,
                        to: to,
                        mv_type: MoveType::Quiet,
                    },
                ),
            };
        }
    }
    target_positions
}

#[inline(always)]
pub fn get_pawn_moves(
    board: &mut Board,
    color: Color,
    moves: &mut Vec<EncodedMove>,
    only_captures: bool,
) -> Bitboard {
    let mut target_positions = Bitboard(0);
    let mut pawn_positions = board.get_bitboard_by_piece_color(color, Piece::Pawn);

    while pawn_positions != Bitboard(0) {
        let current_pos = pawn_positions.pop_lsb_position().unwrap();
        let from = current_pos.to_square();

        let mut target_positions_for_one_piece =
            get_pawn_positions(board, current_pos, color, only_captures);
        target_positions |= target_positions_for_one_piece;

        while target_positions_for_one_piece != Bitboard(0) {
            let target_pos = target_positions_for_one_piece.pop_lsb_position().unwrap();

            let cy = current_pos.to_y();
            let ty = target_pos.to_y();
            let is_double_move = if cy.abs_diff(ty) == 2 { true } else { false };
            let is_capture = target_pos.is_enemy(board, color);

            let to = target_pos.to_square();

            if ty == 0 || ty == 7 {
                match is_capture {
                    true => {
                        push_if_legal(
                            moves,
                            board,
                            DecodedMove {
                                from: from,
                                to: to,
                                mv_type: MoveType::QueenPromoCapture,
                            },
                        );
                        push_if_legal(
                            moves,
                            board,
                            DecodedMove {
                                from: from,
                                to: to,
                                mv_type: MoveType::RookPromoCapture,
                            },
                        );
                        push_if_legal(
                            moves,
                            board,
                            DecodedMove {
                                from: from,
                                to: to,
                                mv_type: MoveType::BishopPromoCapture,
                            },
                        );
                        push_if_legal(
                            moves,
                            board,
                            DecodedMove {
                                from: from,
                                to: to,
                                mv_type: MoveType::KnightPromoCapture,
                            },
                        );
                    }

                    false => {
                        push_if_legal(
                            moves,
                            board,
                            DecodedMove {
                                from: from,
                                to: to,
                                mv_type: MoveType::QueenPromo,
                            },
                        );
                        push_if_legal(
                            moves,
                            board,
                            DecodedMove {
                                from: from,
                                to: to,
                                mv_type: MoveType::RookPromo,
                            },
                        );
                        push_if_legal(
                            moves,
                            board,
                            DecodedMove {
                                from: from,
                                to: to,
                                mv_type: MoveType::BishopPromo,
                            },
                        );
                        push_if_legal(
                            moves,
                            board,
                            DecodedMove {
                                from: from,
                                to: to,
                                mv_type: MoveType::KnightPromo,
                            },
                        );
                    }
                };
            } else {
                match (is_double_move, is_capture) {
                    (true, false) => {
                        push_if_legal(
                            moves,
                            board,
                            DecodedMove {
                                from: from,
                                to: to,
                                mv_type: MoveType::DoubleMove,
                            },
                        );
                    }
                    (false, true) => {
                        push_if_legal(
                            moves,
                            board,
                            DecodedMove {
                                from: from,
                                to: to,
                                mv_type: MoveType::Capture,
                            },
                        );
                    }
                    _ => {
                        push_if_legal(
                            moves,
                            board,
                            DecodedMove {
                                from: from,
                                to: to,
                                mv_type: MoveType::Quiet,
                            },
                        );
                    }
                };
            }
        }
    }
    target_positions
}

/// Calculates all possible moves for the king which is the only unique piece
#[inline(always)]
pub fn get_king_moves(
    board: &mut Board,
    current_pos: Bit,
    color: Color,
    moves: &mut Vec<EncodedMove>,
    f: fn(board: &Board, pos: Bit, color: Color) -> Bitboard,
    only_captures: bool,
) -> Bitboard {
    let mut target_positions = f(board, current_pos, color);

    if only_captures {
        target_positions &= board.get_non_friendly_pieces(color);
    }

    let from = current_pos.to_square();
    while target_positions != Bitboard(0) {
        let target_pos = target_positions.pop_lsb_position().unwrap();
        let to = target_pos.to_square();

        match target_pos.is_enemy(board, color) {
            true => push_if_legal(
                moves,
                board,
                DecodedMove {
                    from: from,
                    to: to,
                    mv_type: MoveType::Capture,
                },
            ),
            false => push_if_legal(
                moves,
                board,
                DecodedMove {
                    from: from,
                    to: to,
                    mv_type: MoveType::Quiet,
                },
            ),
        }
    }
    target_positions
}

pub fn get_castle_moves(board: &mut Board, color: Color, moves: &mut Vec<EncodedMove>) {
    match color {
        Color::White => {
            const MASK_WHITE_QUEEN_CASTLE: Bitboard = Bitboard(1u64 << 1 | 1u64 << 2 | 1u64 << 3);
            if board.white_queen_castle
                && board.get_empty_pieces() & MASK_WHITE_QUEEN_CASTLE == MASK_WHITE_QUEEN_CASTLE
            {
                push_if_legal(
                    moves,
                    board,
                    DecodedMove {
                        from: Square(4),
                        to: Square(2),
                        mv_type: MoveType::QueenCastle,
                    },
                );
            }
            const MASK_WHITE_KING_CASTLE: Bitboard = Bitboard(1u64 << 5 | 1u64 << 6);
            if board.white_king_castle
                && board.get_empty_pieces() & MASK_WHITE_KING_CASTLE == MASK_WHITE_KING_CASTLE
            {
                push_if_legal(
                    moves,
                    board,
                    DecodedMove {
                        from: Square(4),
                        to: Square(6),
                        mv_type: MoveType::KingCastle,
                    },
                );
            }
        }
        Color::Black => {
            const MASK_BLACK_KING_CASTLE: Bitboard = Bitboard(1u64 << 61 | 1u64 << 62);
            if board.black_king_castle
                && board.get_empty_pieces() & MASK_BLACK_KING_CASTLE == MASK_BLACK_KING_CASTLE
            {
                push_if_legal(
                    moves,
                    board,
                    DecodedMove {
                        from: Square(60),
                        to: Square(62),
                        mv_type: MoveType::KingCastle,
                    },
                );
            }
            const MASK_BLACK_QUEEN_CASTLE: Bitboard =
                Bitboard(1u64 << 57 | 1u64 << 58 | 1u64 << 59);
            if board.black_queen_castle
                && board.get_empty_pieces() & MASK_BLACK_QUEEN_CASTLE == MASK_BLACK_QUEEN_CASTLE
            {
                push_if_legal(
                    moves,
                    board,
                    DecodedMove {
                        from: Square(60),
                        to: Square(58),
                        mv_type: MoveType::QueenCastle,
                    },
                );
            }
        }
    }
}

#[inline(always)]
pub fn get_en_passant_moves(board: &mut Board, color: Color, moves: &mut Vec<EncodedMove>) {
    let ep_target = match board.ep_target {
        Some(pos) => pos,
        None => return,
    };

    let to = ep_target.to_square();

    match color {
        Color::White => {
            let position_left = ep_target.get_offset_pos(-1, -1);
            let from_l = position_left.to_square();
            let position_right = ep_target.get_offset_pos(1, -1);
            let from_r = position_right.to_square();
            if board.bbs[Figure::WhitePawn as usize].is_position_set(position_left) {
                push_if_legal(
                    moves,
                    board,
                    DecodedMove {
                        from: from_l,
                        to: to,
                        mv_type: MoveType::EpCapture,
                    },
                );
            }
            if board.bbs[Figure::WhitePawn as usize].is_position_set(position_right) {
                push_if_legal(
                    moves,
                    board,
                    DecodedMove {
                        from: from_r,
                        to: to,
                        mv_type: MoveType::EpCapture,
                    },
                );
            }
        }
        Color::Black => {
            let position_left = ep_target.get_offset_pos(-1, 1);
            let from_l = position_left.to_square();
            let position_right = ep_target.get_offset_pos(1, 1);
            let from_r = position_right.to_square();
            if board.bbs[Figure::BlackPawn as usize].is_position_set(position_left) {
                push_if_legal(
                    moves,
                    board,
                    DecodedMove {
                        from: from_l,
                        to: to,
                        mv_type: MoveType::EpCapture,
                    },
                );
            }
            if board.bbs[Figure::BlackPawn as usize].is_position_set(position_right) {
                push_if_legal(
                    moves,
                    board,
                    DecodedMove {
                        from: from_r,
                        to: to,
                        mv_type: MoveType::EpCapture,
                    },
                );
            }
        }
    }
}

#[inline(always)]
pub fn get_all_attacks(board: &Board, color: Color) -> Bitboard {
    let mut attacks = Bitboard(0);
    let mut positions = match color {
        Color::Black => board.black_positions,
        Color::White => board.white_positions,
    };
    while positions != Bitboard(0) {
        let current_pos = positions.pop_lsb_position().unwrap();
        let (piece, color) = board.get_piece_and_color_at_position(current_pos);
        attacks |= match piece {
            Piece::Pawn => get_pawn_attack_positions(board, current_pos, color),
            Piece::Knight => get_knight_positions(board, current_pos, color),
            Piece::Bishop => get_bishop_positions(board, current_pos, color),
            Piece::Rook => get_rook_positions(board, current_pos, color),
            Piece::Queen => get_queen_positions(board, current_pos, color),
            Piece::King => get_king_positions(board, current_pos, color),
            _ => Bitboard(0),
        };
    }
    attacks
}
