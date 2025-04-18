use crate::prelude::*;

pub fn get_all_black_moves(board: &Board) -> Vec<ChessMove> {
    let mut moves = Vec::new();
    moves.extend(get_all_moves_for_one_piece_type_not_unique(
        board,
        board.black_pawns,
        Color::Black,
        get_pawn_moves,
    ));
    moves.extend(get_all_moves_for_one_piece_type_not_unique(
        board,
        board.black_knights,
        Color::Black,
        get_knight_moves,
    ));
    moves.extend(get_all_moves_for_one_piece_type_not_unique(
        board,
        board.black_bishops,
        Color::Black,
        get_bishop_moves,
    ));
    moves.extend(get_all_moves_for_one_piece_type_not_unique(
        board,
        board.black_rooks,
        Color::Black,
        get_rook_moves,
    ));
    moves.extend(get_all_moves_for_one_piece_type_unique(
        board,
        board.black_queen,
        Color::Black,
        get_queen_moves,
    ));
    moves.extend(get_all_moves_for_one_piece_type_unique(
        board,
        board.black_king,
        Color::Black,
        get_king_moves,
    ));

    moves
}

pub fn get_all_white_moves(board: &Board) -> Vec<ChessMove> {
    let mut moves = Vec::new();
    moves.extend(get_all_moves_for_one_piece_type_not_unique(
        board,
        board.white_pawns,
        Color::White,
        get_pawn_moves,
    ));
    moves.extend(get_all_moves_for_one_piece_type_not_unique(
        board,
        board.white_knights,
        Color::White,
        get_knight_moves,
    ));
    moves.extend(get_all_moves_for_one_piece_type_not_unique(
        board,
        board.white_bishops,
        Color::White,
        get_bishop_moves,
    ));
    moves.extend(get_all_moves_for_one_piece_type_not_unique(
        board,
        board.white_rooks,
        Color::White,
        get_rook_moves,
    ));
    moves.extend(get_all_moves_for_one_piece_type_unique(
        board,
        board.white_queen,
        Color::White,
        get_queen_moves,
    ));
    moves.extend(get_all_moves_for_one_piece_type_unique(
        board,
        board.white_king,
        Color::White,
        get_king_moves,
    ));

    moves
}

pub fn get_all_moves_for_one_piece_type_not_unique(
    board: &Board,
    mut piece_positions: Bitboard,
    color: Color,
    f: fn(board: &Board, pos: Position, color: Color) -> Bitboard,
) -> Vec<ChessMove> {
    let mut moves = Vec::new();
    while piece_positions != Bitboard(0) {
        let current_pos = piece_positions.pop_lsb_position();

        let mut target_positions = f(board, current_pos, color);

        while target_positions != Bitboard(0) {
            let target_pos = target_positions.pop_lsb_position();
            moves.push(ChessMove(current_pos, target_pos));
        }
    }
    moves
}

pub fn get_all_moves_for_one_piece_type_unique(
    board: &Board,
    current_pos: Position,
    color: Color,
    f: fn(board: &Board, pos: Position, color: Color) -> Bitboard,
) -> Vec<ChessMove> {
    let mut moves = Vec::new();
    let mut target_positions = f(board, current_pos, color);
    while target_positions != Bitboard(0) {
        let target_pos = target_positions.pop_lsb_position();
        moves.push(ChessMove(current_pos, target_pos))
    }
    moves
}

pub fn get_pawn_moves(board: &Board, pos: Position, color: Color) -> Bitboard {
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

pub fn get_king_moves(board: &Board, pos: Position, color: Color) -> Bitboard {
    let mut moves = Bitboard(0);
    let non_friendly_pieces = board.get_non_friendly_pieces(color);
    moves |= pos.get_offset_pos(-1, 1) & non_friendly_pieces;
    moves |= pos.get_offset_pos(0, 1) & non_friendly_pieces;
    moves |= pos.get_offset_pos(1, 1) & non_friendly_pieces;
    moves |= pos.get_offset_pos(-1, 0) & non_friendly_pieces;
    moves |= pos.get_offset_pos(1, 0) & non_friendly_pieces;
    moves |= pos.get_offset_pos(-1, -1) & non_friendly_pieces;
    moves |= pos.get_offset_pos(0, -1) & non_friendly_pieces;
    moves |= pos.get_offset_pos(1, -1) & non_friendly_pieces;
    moves
}

pub fn get_knight_moves(board: &Board, pos: Position, color: Color) -> Bitboard {
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

pub fn get_sliding_moves(
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

pub fn get_queen_moves(board: &Board, pos: Position, color: Color) -> Bitboard {
    let mut moves = Bitboard(0);
    moves |= get_sliding_moves(board, pos, color, 1, -1);
    moves |= get_sliding_moves(board, pos, color, 1, 0);
    moves |= get_sliding_moves(board, pos, color, 1, 1);
    moves |= get_sliding_moves(board, pos, color, 0, -1);
    moves |= get_sliding_moves(board, pos, color, 0, 1);
    moves |= get_sliding_moves(board, pos, color, -1, -1);
    moves |= get_sliding_moves(board, pos, color, -1, 0);
    moves |= get_sliding_moves(board, pos, color, -1, 1);
    moves
}

pub fn get_bishop_moves(board: &Board, pos: Position, color: Color) -> Bitboard {
    let mut moves = Bitboard(0);
    moves |= get_sliding_moves(board, pos, color, 1, -1);
    moves |= get_sliding_moves(board, pos, color, 1, -1);
    moves |= get_sliding_moves(board, pos, color, -1, -1);
    moves |= get_sliding_moves(board, pos, color, -1, 1);
    moves
}

pub fn get_rook_moves(board: &Board, pos: Position, color: Color) -> Bitboard {
    let mut moves = Bitboard(0);
    moves |= get_sliding_moves(board, pos, color, 1, 0);
    moves |= get_sliding_moves(board, pos, color, 0, -1);
    moves |= get_sliding_moves(board, pos, color, 0, 1);
    moves |= get_sliding_moves(board, pos, color, -1, 0);
    moves
}
