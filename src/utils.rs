use crate::types::{Bit, Bitboard, Board, Color, Direction, Piece};

/// Checks if pos >= 0 and <= 7
pub fn is_axis_in_bounds(pos: i32) -> bool {
    pos >= 0 && pos <= 7
}

/// Checks if position index is in bounds
pub fn is_pos_in_bounds<T>(pos: T) -> bool
where
    // T must be comparable and u8 convertable
    T: PartialOrd + From<u8>,
{
    pos >= T::from(0) && pos < T::from(64)
}

pub fn is_next_pos_in_bounce(pos: usize, dir: Direction) -> bool {
    match dir {
        Direction::Up => (pos + 8) <= 63,
        Direction::Down => (pos - 8) >= 0,
        Direction::Left => pos % 8 != 0,
        Direction::Right => pos % 8 != 7,
        Direction::UpLeft => pos % 8 != 0 && (pos + 8) <= 63,
        Direction::UpRight => pos % 8 != 7 && (pos + 8) <= 63,
        Direction::DownLeft => pos % 8 != 0 && (pos - 8) >= 0,
        Direction::DownRight => pos % 8 != 7 && (pos - 8) >= 0,
    }
}

/// Is target position in a different row
pub fn is_position_in_diff_row(pos: usize, target: usize) -> bool {
    pos / 8 == target / 8
}

/// Converts x and y index to Bitboard
pub fn xy_to_bit(x: i32, y: i32) -> Bitboard {
    idx_to_bit(x_y_to_idx(x, y))
}

pub fn x_y_to_idx(x: i32, y: i32) -> usize {
    ((y * 8) + x) as usize
}

// Checks if the bit is set on the Bitboard
pub fn is_bit_set(bitboard: Bitboard, bit: Bitboard) -> bool {
    (bitboard & bit) != 0
}

pub fn is_pos_friendly(board: &Board, pos: Bit, color: Color) -> bool {
    (color == Color::Black && is_bit_set(board.black_pieces, pos))
        || (color == Color::White && is_bit_set(board.white_pieces, pos))
}

pub fn is_pos_enemy(board: &Board, pos: Bit, color: Color) -> bool {
    (color == Color::White && is_bit_set(board.black_pieces, pos))
        || (color == Color::Black && is_bit_set(board.white_pieces, pos))
}

pub fn is_pos_empty(board: &Board, pos: Bit, color: Color) -> bool {
    is_bit_set(board.empty_pieces, pos)
}

pub fn pos_to_color_and_piece(board: &Board, pos: Bit) -> (Color, Piece) {
    if board.empty_pieces & pos != 0 {
        (Color::None, Piece::Empty)
    } else if board.black_pawns & pos != 0 {
        (Color::Black, Piece::Pawn)
    } else if board.black_knights & pos != 0 {
        (Color::Black, Piece::Knight)
    } else if board.black_bishops & pos != 0 {
        (Color::Black, Piece::Bishop)
    } else if board.black_rooks & pos != 0 {
        (Color::Black, Piece::Rook)
    } else if board.black_queen & pos != 0 {
        (Color::Black, Piece::Queen)
    } else if board.black_king & pos != 0 {
        (Color::Black, Piece::King)
    } else if board.white_pawns & pos != 0 {
        (Color::White, Piece::Pawn)
    } else if board.white_knights & pos != 0 {
        (Color::White, Piece::Knight)
    } else if board.white_bishops & pos != 0 {
        (Color::White, Piece::Bishop)
    } else if board.white_rooks & pos != 0 {
        (Color::White, Piece::Rook)
    } else if board.white_queen & pos != 0 {
        (Color::White, Piece::Queen)
    } else if board.white_king & pos != 0 {
        (Color::White, Piece::King)
    } else {
        let (x, y) = bit_to_xy(pos);
        panic!("{} {} trying to get piece by wrong bitboard", x, y);
    }
}

pub fn idx_to_bit(idx: usize) -> Bit {
    1u64 << idx
}

pub fn bit_to_xy(pos: Bit) -> (usize, usize) {
    let idx = pos.trailing_zeros() as usize;
    let x = idx % 8;
    let y = idx / 8;
    (x, y)
}

pub fn bitboard_to_xy_list(bitboard: Bitboard) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    let mut bitboard = bitboard;
    while bitboard != 0 {
        let bit_index = bitboard.trailing_zeros() as usize;
        let x = bit_index % 8;
        let y = bit_index / 8;
        positions.push((x, y));
        bitboard &= bitboard - 1;
    }
    positions
}
