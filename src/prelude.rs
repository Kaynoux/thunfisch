// Used to import always needed types into all files
pub use crate::types::{
    bit::Bit, bitboard::Bitboard, board::Board, color::Color, color::Color::*,
    decoded_move::DecodedMove, encoded_move::EncodedMove, figure::Figure, move_type::MoveType,
    piece::Piece, piece::Piece::*, search_info::SearchInfo, square::Square,
};
