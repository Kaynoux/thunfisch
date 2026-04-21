// Used to import always needed types into all files
pub use crate::types::{
    bit::Bit,
    bitboard::Bitboard,
    board::Board,
    color::{Color, Color::*},
    decoded_move::DecodedMove,
    encoded_move::EncodedMove,
    figure::Figure,
    move_type::MoveType,
    piece::{Piece, Piece::*},
    search_data::SharedSearchData,
    square::Square,
};
