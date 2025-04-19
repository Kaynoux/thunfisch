use crate::prelude::*;
pub fn get_all_moves(board: &Board, color: Color, moves: &mut Vec<ChessMove>) -> Bitboard {
    let mut moves_bitboard = Bitboard(0);

    // Pawn by 1
    moves_bitboard |= get_moves_for_piece_type(
        board,
        board.get_positions_by_piece_color(color, Piece::Pawn),
        color,
        moves,
        false,
        get_pawn_positions,
    );

    // Knight
    moves_bitboard |= get_moves_for_piece_type(
        board,
        board.get_positions_by_piece_color(color, Piece::Knight),
        color,
        moves,
        false,
        get_knight_positions,
    );

    // Bishop
    moves_bitboard |= get_moves_for_piece_type(
        board,
        board.get_positions_by_piece_color(color, Piece::Bishop),
        color,
        moves,
        false,
        get_bishop_positions,
    );

    // Rook
    moves_bitboard |= get_moves_for_piece_type(
        board,
        board.get_positions_by_piece_color(color, Piece::Rook),
        color,
        moves,
        false,
        get_rook_positions,
    );

    // Queen
    moves_bitboard |= get_moves_for_piece_type(
        board,
        board.get_positions_by_piece_color(color, Piece::Queen),
        color,
        moves,
        false,
        get_queen_positions,
    );

    // King
    let king_pos = board.get_king_pos(color);
    if king_pos != Position(0) {
        moves_bitboard |= get_king_moves(board, king_pos, color, moves, get_king_positions);
    }

    // Double pawn moves
    moves_bitboard |= get_moves_for_piece_type(
        board,
        board.black_pawns,
        Color::Black,
        moves,
        true,
        get_pawn_double_positions,
    );

    get_castle_moves(board, Color::Black, moves);
    //get_promotions_moves();
    //get_en_passant_moves();
    moves_bitboard
}

/// Calculate all moves for each instance of a piece type exept the king because it is unique
pub fn get_moves_for_piece_type(
    board: &Board,
    mut piece_positions: Bitboard,
    color: Color,
    moves: &mut Vec<ChessMove>,
    is_double_move: bool,
    f: fn(board: &Board, pos: Position, color: Color) -> Bitboard,
) -> Bitboard {
    let mut target_positions = Bitboard(0);

    while piece_positions != Bitboard(0) {
        let current_pos = piece_positions.pop_lsb_position();

        let mut target_positions_for_one_piece = f(board, current_pos, color);
        target_positions |= target_positions_for_one_piece;

        while target_positions_for_one_piece != Bitboard(0) {
            let target_pos = target_positions_for_one_piece.pop_lsb_position();
            let (target_piece, _) = board.get_piece_and_color_at_position(target_pos);
            let is_capture = match target_piece {
                Piece::Empty => false,
                _ => true,
            };
            moves.push(ChessMove {
                from: current_pos,
                to: target_pos,
                is_capture: is_capture,
                is_double_move: is_double_move,
                is_promotion: false,
                is_en_passant: false,
                is_castle: false,
                promotion: Piece::Empty,
                captured: target_piece,
            });
        }
    }
    target_positions
}

/// Calculates all possible moves for the king which is the only unique piece
pub fn get_king_moves(
    board: &Board,
    current_pos: Position,
    color: Color,
    moves: &mut Vec<ChessMove>,
    f: fn(board: &Board, pos: Position, color: Color) -> Bitboard,
) -> Bitboard {
    let mut target_positions = f(board, current_pos, color);
    while target_positions != Bitboard(0) {
        let target_pos = target_positions.pop_lsb_position();
        let (target_piece, _) = board.get_piece_and_color_at_position(target_pos);
        let is_capture = match target_piece {
            Piece::Empty => false,
            _ => true,
        };
        moves.push(ChessMove {
            from: current_pos,
            to: target_pos,
            is_capture: is_capture,
            is_double_move: false,
            is_promotion: false,
            is_en_passant: false,
            is_castle: false,
            promotion: Piece::Empty,
            captured: target_piece,
        });
    }
    target_positions
}

pub fn get_pawn_double_positions(board: &Board, pos: Position, color: Color) -> Bitboard {
    let mut moves = Bitboard(0);

    // Add possible move by 2 when pawn has not moved in the match and position in front is empty
    match (color, pos.to_index() / 8) {
        (Color::Black, 6) => {
            if board
                .empty_pieces
                .is_position_set(pos.get_offset_pos(0, -1))
            {
                moves |= pos.get_offset_pos(0, -2)
            }
        }
        (Color::White, 1) => {
            if board.empty_pieces.is_position_set(pos.get_offset_pos(0, 1)) {
                moves |= pos.get_offset_pos(0, 2)
            }
        }
        (_, _) => {}
    }

    // Target Pos also needs to be empty
    moves &= board.empty_pieces;
    moves
}

pub fn get_pawn_positions(board: &Board, pos: Position, color: Color) -> Bitboard {
    let mut moves_to_empty = Bitboard(0);
    let mut moves_to_enemy = Bitboard(0);
    let move_direction_y = match color {
        Color::Black => -1,
        Color::White => 1,
    };

    moves_to_empty |= pos.get_offset_pos(0, move_direction_y);
    // Positions need to be empty to be valid
    moves_to_empty &= board.empty_pieces;

    // Add the to possible Strike moves
    moves_to_enemy |= pos.get_offset_pos(-1, move_direction_y);
    moves_to_enemy |= pos.get_offset_pos(1, move_direction_y);

    // Positions need to be enemy to be valid
    moves_to_enemy &= board.get_enemy_pieces(color);

    // Return combination off possible empty and enemy pos
    moves_to_empty | moves_to_enemy
}

pub fn get_king_positions(board: &Board, pos: Position, color: Color) -> Bitboard {
    let mut moves = Bitboard(0);
    let non_friendly_pieces = board.get_non_friendly_pieces(color);
    moves |= pos.get_offset_pos(-1, 1);
    moves |= pos.get_offset_pos(0, 1);
    moves |= pos.get_offset_pos(1, 1);
    moves |= pos.get_offset_pos(-1, 0);
    moves |= pos.get_offset_pos(1, 0);
    moves |= pos.get_offset_pos(-1, -1);
    moves |= pos.get_offset_pos(0, -1);
    moves |= pos.get_offset_pos(1, -1);
    moves & non_friendly_pieces
}

pub fn get_knight_positions(board: &Board, pos: Position, color: Color) -> Bitboard {
    let mut moves = Bitboard(0);
    let non_friendly_pieces = board.get_non_friendly_pieces(color);
    moves |= pos.get_offset_pos(-2, 1);
    moves |= pos.get_offset_pos(-1, 2);
    moves |= pos.get_offset_pos(1, 2);
    moves |= pos.get_offset_pos(2, 1);
    moves |= pos.get_offset_pos(-2, -1);
    moves |= pos.get_offset_pos(-1, -2);
    moves |= pos.get_offset_pos(1, -2);
    moves |= pos.get_offset_pos(2, -1);
    moves & non_friendly_pieces
}

pub fn get_sliding_positions(
    board: &Board,
    pos: Position,
    color: Color,
    dx: isize,
    dy: isize,
) -> Bitboard {
    let mut moves = Bitboard(0);
    let non_friendly_pieces = board.get_non_friendly_pieces(color);
    let mut current_dx = 0isize;
    let mut current_dy = 0isize;
    loop {
        current_dx += dx;
        current_dy += dy;
        let current_pos = pos.get_offset_pos(current_dx, current_dy);
        if current_pos == Position(0) {
            break;
        }

        if current_pos.is_friendly(board, color) {
            break;
        }

        moves |= current_pos;

        if current_pos.is_enemy(board, color) {
            break;
        }
    }
    moves & non_friendly_pieces
}

pub fn get_queen_positions(board: &Board, pos: Position, color: Color) -> Bitboard {
    let mut moves = Bitboard(0);
    moves |= get_sliding_positions(board, pos, color, 1, -1);
    moves |= get_sliding_positions(board, pos, color, 1, 0);
    moves |= get_sliding_positions(board, pos, color, 1, 1);
    moves |= get_sliding_positions(board, pos, color, 0, -1);
    moves |= get_sliding_positions(board, pos, color, 0, 1);
    moves |= get_sliding_positions(board, pos, color, -1, -1);
    moves |= get_sliding_positions(board, pos, color, -1, 0);
    moves |= get_sliding_positions(board, pos, color, -1, 1);
    moves
}

pub fn get_bishop_positions(board: &Board, pos: Position, color: Color) -> Bitboard {
    let mut moves = Bitboard(0);
    moves |= get_sliding_positions(board, pos, color, 1, -1);
    moves |= get_sliding_positions(board, pos, color, 1, 1);
    moves |= get_sliding_positions(board, pos, color, -1, -1);
    moves |= get_sliding_positions(board, pos, color, -1, 1);
    moves
}

pub fn get_rook_positions(board: &Board, pos: Position, color: Color) -> Bitboard {
    let mut moves = Bitboard(0);
    moves |= get_sliding_positions(board, pos, color, 1, 0);
    moves |= get_sliding_positions(board, pos, color, 0, -1);
    moves |= get_sliding_positions(board, pos, color, 0, 1);
    moves |= get_sliding_positions(board, pos, color, -1, 0);
    moves
}

pub fn get_castle_moves(board: &Board, color: Color, moves: &mut Vec<ChessMove>) {
    match color {
        Color::Black => {
            let mask_black_left = Bitboard(1u64 << 1 | 1u64 << 2 | 1u64 << 3);
            if board.black_castle_left && board.empty_pieces & mask_black_left == mask_black_left {
                let mv = ChessMove {
                    from: Position::from_idx(4),
                    to: Position::from_idx(2),
                    is_capture: false,
                    is_double_move: false,
                    is_promotion: false,
                    is_en_passant: false,
                    is_castle: true,
                    promotion: Piece::Empty,
                    captured: Piece::Empty,
                };
                moves.push(mv);
            }
            let mask_black_right = Bitboard(1u64 << 5 | 1u64 << 6);
            if board.black_castle_right && board.empty_pieces & mask_black_right == mask_black_right
            {
                let mv = ChessMove {
                    from: Position::from_idx(4),
                    to: Position::from_idx(6),
                    is_capture: false,
                    is_double_move: false,
                    is_promotion: false,
                    is_en_passant: false,
                    is_castle: true,
                    promotion: Piece::Empty,
                    captured: Piece::Empty,
                };
                moves.push(mv);
            }
        }
        Color::White => {
            let mask_white_left = Bitboard(1u64 << 57 | 1u64 << 58 | 1u64 << 59);
            if board.white_castle_left && board.empty_pieces & mask_white_left == mask_white_left {
                let mv = ChessMove {
                    from: Position::from_idx(60),
                    to: Position::from_idx(58),
                    is_capture: false,
                    is_double_move: false,
                    is_promotion: false,
                    is_en_passant: false,
                    is_castle: true,
                    promotion: Piece::Empty,
                    captured: Piece::Empty,
                };
                moves.push(mv);
            }
            let mask_white_right = Bitboard(1u64 << 61 | 1u64 << 62);
            if board.white_castle_right && board.empty_pieces & mask_white_right == mask_white_right
            {
                let mv = ChessMove {
                    from: Position::from_idx(60),
                    to: Position::from_idx(62),
                    is_capture: false,
                    is_double_move: false,
                    is_promotion: false,
                    is_en_passant: false,
                    is_castle: true,
                    promotion: Piece::Empty,
                    captured: Piece::Empty,
                };
                moves.push(mv);
            }
        }
    }
}
