use crate::types::{Bit, Bitboard, Board, Color, Direction};
use crate::utils::{
    idx_to_bit, is_axis_in_bounds, is_bit_set, is_next_pos_in_bounce, is_pos_empty, is_pos_enemy,
    is_pos_friendly, is_pos_in_bounds, xy_to_bit,
};

pub fn generate_pseudo_legal_pawn_moves(board: &Board, pos: usize, color: Color) -> Bitboard {
    let mut pseudo_legal_positions: Bitboard = 0;
    let move_direction = match color {
        Color::Black => Direction::Down,
        Color::White => Direction::Up,
        Color::None => panic!("Pawn provided is not valid"),
    };

    // Stop if move forward is out of bounds
    if !is_next_pos_in_bounce(pos, move_direction) {
        pseudo_legal_positions
    }

    let forward_by_one_pos = pos + move_direction as i32;

    // Add the move forward to possible options if empty
    if is_pos_empty(board, forward_by_one_pos, color) {
        pseudo_legal_positions |= forward_by_one_pos
    }

    // Handle strike to the left
    let forward_left_pos = forward_by_one_pos + Direction::Left as i32;
    if is_next_pos_in_bounce(forward_by_one_pos, Direction::Left)
        && is_pos_enemy(board, forward_left_pos, color)
    {
        pseudo_legal_positions |= forward_left_pos
    }

    // Handle strike to the right
    let forward_right_pos = forward_by_one_pos + Direction::Right as i32;
    if is_next_pos_in_bounce(forward_by_one_pos, Direction::Right)
        && is_pos_enemy(board, forward_right_pos, color)
    {
        pseudo_legal_positions |= forward_right_pos
    }

    pseudo_legal_positions
}

// Check if a move by 2 is valid
fn is_pawn_move_2_valid(target_pos: usize) -> bool {
    // Skip if out of bounds
    if !target_pos.is_within_bounds() {
        false
    }

    // Skip if target field is not empty
    let forward_field: Field = get_field(board, *target_pos);
    if forward_field.color != Color::None {
        false
    }

    // Skip if field in between is not free
    if get_field(
        board,
        Position {
            x: target_pos.x,
            y: target_pos.y - move_direction,
        },
    )
    .color
        != Color::None
    {
        false
    }

    true
}

fn get_offset_moves(
    board: &Board,
    pos: usize,
    color: Color,
    move_offsets: &[(i32, i32)],
) -> Bitboard {
    let mut moves: Bitboard = 0u64;
    let pos_x = (pos % 8) as i32;
    let pos_y = (pos / 8) as i32;

    for (off_x, off_y) in move_offsets.iter() {
        let new_x = pos_x + off_x;
        let new_y = pos_y + off_y;

        // Skip if out of bounds
        if !is_axis_in_bounds(new_x) || !is_axis_in_bounds(new_y) {
            continue;
        }

        // Skip if friendly pice already there
        let new_pos: Bit = xy_to_bit(new_x, new_y);
        if is_pos_friendly(board, new_pos, color) {
            continue;
        }

        // Add new pos to list of pseudo legal moves
        moves |= new_pos;
    }
    moves
}

pub fn get_pseudo_legal_king_moves(board: &Board, pos: usize, color: Color) -> Bitboard {
    static KING_MOVE_OFFSETS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    get_offset_moves(board, pos, color, &KING_MOVE_OFFSETS)
}

pub fn get_pseudo_legal_knight_moves(board: &Board, pos: usize, color: Color) -> Bitboard {
    static KNIGHT_MOVE_OFFSETS: [(i32, i32); 8] = [
        (-2, -1),
        (-2, 1),
        (-1, -2),
        (-1, 2),
        (1, -2),
        (1, 2),
        (2, -1),
        (2, 1),
    ];
    get_offset_moves(board, pos, color, &KNIGHT_MOVE_OFFSETS)
}

pub fn get_sliding_moves(board: &Board, pos: usize, color: Color, dir: Direction) -> Bitboard {
    let mut moves: Bitboard = 0;
    // Could be negative at first before bounds check
    let mut next_pos_idx_i = pos;
    loop {
        // This takes the current position and checks if the next one will be valid
        if !is_next_pos_in_bounce(next_pos_idx_i, dir) {
            break;
        }
        // So this will only trigger if the move was not out of bounds
        next_pos_idx_i += dir as isize;
        println!("{:?}", next_pos_idx_i);

        // Cannot be negative now
        let next_pos_idx = next_pos_idx_i as usize;

        let next_pos_bit: Bit = idx_to_bit(next_pos_idx);

        if is_pos_empty(board, next_pos_bit, color) {
            // Hit an empty or enemy piece and continue
            moves |= next_pos_bit;
            continue;
        } else if is_pos_enemy(board, next_pos_bit, color) {
            // Hit a enemy piece and stop searching because it blocks
            moves |= next_pos_bit;
            break;
        } else {
            // Hit a friendly piece stop searching but it is not a valid target
            break;
        }
    }

    moves
}

pub fn get_pseudo_legal_queen_moves(board: &Board, pos: usize, color: Color) -> Bitboard {
    let mut moves = get_sliding_moves(board, pos, color, Direction::Up);
    moves |= get_sliding_moves(board, pos, color, Direction::Down);
    moves |= get_sliding_moves(board, pos, color, Direction::Left);
    moves |= get_sliding_moves(board, pos, color, Direction::Right);
    moves |= get_sliding_moves(board, pos, color, Direction::UpLeft);
    moves |= get_sliding_moves(board, pos, color, Direction::UpRight);
    moves |= get_sliding_moves(board, pos, color, Direction::UpLeft);
    moves |= get_sliding_moves(board, pos, color, Direction::DownLeft);
    moves |= get_sliding_moves(board, pos, color, Direction::DownRight);
    moves
}

pub fn get_pseudo_legal_rook_moves(board: &Board, pos: usize, color: Color) -> Bitboard {
    let mut moves = get_sliding_moves(board, pos, color, Direction::Up);
    moves |= get_sliding_moves(board, pos, color, Direction::Down);
    moves |= get_sliding_moves(board, pos, color, Direction::Left);
    moves |= get_sliding_moves(board, pos, color, Direction::Right);
    moves
}

pub fn get_pseudo_legal_bishop_moves(board: &Board, pos: usize, color: Color) -> Bitboard {
    let mut moves = get_sliding_moves(board, pos, color, Direction::UpLeft);
    moves |= get_sliding_moves(board, pos, color, Direction::UpRight);
    moves |= get_sliding_moves(board, pos, color, Direction::UpLeft);
    moves |= get_sliding_moves(board, pos, color, Direction::DownLeft);
    moves |= get_sliding_moves(board, pos, color, Direction::DownRight);
    moves
}
