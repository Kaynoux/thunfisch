use crate::prelude::*;

#[inline(always)]
pub fn get_pawn_positions(board: &Board, pos: Position, color: Color) -> Bitboard {
    let mut moves_to_empty = Bitboard(0);
    let mut moves_to_enemy = Bitboard(0);
    let move_direction_y = match color {
        Color::Black => -1,
        Color::White => 1,
    };

    moves_to_empty |= pos.get_offset_pos(0, move_direction_y);

    // Add possible move by 2 when pawn has not moved in the match and position in front is empty
    match (color, pos.to_index() / 8) {
        (Color::Black, 6) => {
            if board
                .empty_pieces
                .is_position_set(pos.get_offset_pos(0, -1))
            {
                moves_to_empty |= pos.get_offset_pos(0, -2)
            }
        }
        (Color::White, 1) => {
            if board.empty_pieces.is_position_set(pos.get_offset_pos(0, 1)) {
                moves_to_empty |= pos.get_offset_pos(0, 2)
            }
        }
        (_, _) => {}
    }

    // Positions need to be empty to be valid
    moves_to_empty &= board.empty_pieces;

    // Add the to possible Strike moves
    moves_to_enemy |= pos.get_offset_pos(-1, move_direction_y);
    moves_to_enemy |= pos.get_offset_pos(1, move_direction_y);

    // Positions need to be enemy to be valid
    moves_to_enemy &= board.get_pieces_by_color(!color);

    moves_to_empty | moves_to_enemy
}

#[inline(always)]
pub fn get_pawn_attack_positions(board: &Board, pos: Position, color: Color) -> Bitboard {
    let mut moves_to_enemy = Bitboard(0);
    let non_friendly_pieces = board.get_non_friendly_pieces(color);
    let move_direction_y = match color {
        Color::Black => -1,
        Color::White => 1,
    };
    // Add the to possible Strike moves
    moves_to_enemy |= pos.get_offset_pos(-1, move_direction_y);
    moves_to_enemy |= pos.get_offset_pos(1, move_direction_y);
    moves_to_enemy &= non_friendly_pieces;

    moves_to_enemy
}

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
pub fn get_bishop_positions(board: &Board, pos: Position, color: Color) -> Bitboard {
    let mut moves = Bitboard(0);
    moves |= get_sliding_positions(board, pos, color, 1, -1);
    moves |= get_sliding_positions(board, pos, color, 1, 1);
    moves |= get_sliding_positions(board, pos, color, -1, -1);
    moves |= get_sliding_positions(board, pos, color, -1, 1);
    moves
}

#[inline(always)]
pub fn get_rook_positions(board: &Board, pos: Position, color: Color) -> Bitboard {
    let mut moves = Bitboard(0);
    moves |= get_sliding_positions(board, pos, color, 1, 0);
    moves |= get_sliding_positions(board, pos, color, 0, -1);
    moves |= get_sliding_positions(board, pos, color, 0, 1);
    moves |= get_sliding_positions(board, pos, color, -1, 0);
    moves
}
