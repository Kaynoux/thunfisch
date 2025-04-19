use crate::prelude::*;

pub fn get_all_black_moves(board: &Board, moves: &mut Vec<ChessMove>) -> Bitboard {
    let mut moves_bitboard = Bitboard(0);

    moves_bitboard |= get_moves_for_piece_type(
        board,
        board.black_pawns,
        Color::Black,
        moves,
        get_pawn_positions,
    );

    moves_bitboard |= get_moves_for_piece_type(
        board,
        board.black_knights,
        Color::Black,
        moves,
        get_knight_positions,
    );
    moves_bitboard |= get_moves_for_piece_type(
        board,
        board.black_bishops,
        Color::Black,
        moves,
        get_bishop_positions,
    );
    moves_bitboard |= get_moves_for_piece_type(
        board,
        board.black_rooks,
        Color::Black,
        moves,
        get_rook_positions,
    );

    moves_bitboard |= get_moves_for_piece_type(
        board,
        board.black_queens,
        Color::Black,
        moves,
        get_queen_positions,
    );

    let king_pos = board.black_king;
    if king_pos != Position(0) {
        moves_bitboard |= get_king_moves(board, king_pos, Color::Black, moves, get_king_positions);
    }
    moves_bitboard
}

pub fn get_all_white_moves(board: &Board, moves: &mut Vec<ChessMove>) -> Bitboard {
    let mut moves_bitboard = Bitboard(0);
    moves_bitboard |= get_moves_for_piece_type(
        board,
        board.white_pawns,
        Color::White,
        moves,
        get_pawn_positions,
    );
    moves_bitboard |= get_moves_for_piece_type(
        board,
        board.white_knights,
        Color::White,
        moves,
        get_knight_positions,
    );
    moves_bitboard |= get_moves_for_piece_type(
        board,
        board.white_bishops,
        Color::White,
        moves,
        get_bishop_positions,
    );
    moves_bitboard |= get_moves_for_piece_type(
        board,
        board.white_rooks,
        Color::White,
        moves,
        get_rook_positions,
    );

    moves_bitboard |= get_moves_for_piece_type(
        board,
        board.white_queens,
        Color::White,
        moves,
        get_queen_positions,
    );

    let king_pos = board.white_king;
    if king_pos != Position(0) {
        moves_bitboard |= get_king_moves(board, king_pos, Color::White, moves, get_king_positions);
    }
    moves_bitboard
}

/// Calculate all moves for each instance of a piece type exept the king because it is unique
pub fn get_moves_for_piece_type(
    board: &Board,
    mut piece_positions: Bitboard,
    color: Color,
    moves: &mut Vec<ChessMove>,
    f: fn(board: &Board, pos: Position, color: Color) -> Bitboard,
) -> Bitboard {
    let mut target_positions = Bitboard(0);

    while piece_positions != Bitboard(0) {
        let current_pos = piece_positions.pop_lsb_position();

        let mut target_positions_for_one_piece = f(board, current_pos, color);
        target_positions |= target_positions_for_one_piece;

        while target_positions_for_one_piece != Bitboard(0) {
            let target_pos = target_positions_for_one_piece.pop_lsb_position();
            moves.push(ChessMove(current_pos, target_pos));
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
        moves.push(ChessMove(current_pos, target_pos));
    }
    target_positions
}

pub fn get_pawn_positions(board: &Board, pos: Position, color: Color) -> Bitboard {
    let mut moves_to_empty = Bitboard(0);
    let mut moves_to_enemy = Bitboard(0);
    let move_direction_y = match color {
        Color::Black => -1,
        Color::White => 1,
    };

    // Add possible move by 2 when pawn has not moved in the match
    match (color, pos.to_index() / 8) {
        (Color::Black, 6) => {
            moves_to_empty |= pos.get_offset_pos(0, -1) | pos.get_offset_pos(0, -2)
        }
        (Color::White, 1) => moves_to_empty |= pos.get_offset_pos(0, 1) | pos.get_offset_pos(0, 2),
        (_, _) => {}
    }

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
