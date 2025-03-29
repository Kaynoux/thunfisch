use crate::types::{Bit, Bitboard, Board, Color, Piece};

/// Checks if pos >= 0 and <= 7
pub fn is_axis_in_bounds(pos: i32) -> bool {
    pos >= 0 && pos <= 7
}

/// Converts x and y index to Bitboard
pub fn bit_from_x_y(x: i32, y: i32) -> Bitboard {
    bit_from_idx(x_y_to_idx(x, y))
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

pub fn color_and_piece_by_pos(board: &Board, pos: Bit) -> (Color, Piece) {
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
        let (x, y) = x_y_from_bit(pos);
        panic!("{} {} trying to get piece by wrong bitboard", x, y);
    }
}

pub fn bit_from_idx(idx: usize) -> Bit {
    1u64 << idx
}

pub fn x_y_from_bit(pos: Bit) -> (usize, usize) {
    let idx = pos.trailing_zeros() as usize;
    let x = idx % 8;
    let y = idx / 8;
    (x, y)
}

pub fn x_y_list_from_bitboard(bitboard: Bitboard) -> Vec<(usize, usize)> {
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
