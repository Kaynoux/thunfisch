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
    node_type::NodeType,
    piece::{Piece, Piece::*},
    search_info::SearchInfo,
    square::Square,
};
