use crate::types::bitboard::Bitboard;
use crate::types::position::Position;
/// Each piece type gets its own 64bits where
pub struct Board {
    pub white_pieces: Bitboard,
    pub black_pieces: Bitboard,
    pub empty_pieces: Bitboard,
    pub white_pawns: Bitboard,
    pub white_knights: Bitboard,
    pub white_rooks: Bitboard,
    pub white_bishops: Bitboard,
    pub white_queen: Bitboard,
    pub white_king: Bitboard,
    pub black_pawns: Bitboard,
    pub black_knights: Bitboard,
    pub black_rooks: Bitboard,
    pub black_bishops: Bitboard,
    pub black_queen: Bitboard,
    pub black_king: Bitboard,
}

impl Board {
    /// Converts a FEN to a Board
    /// FEN describes the position of all pieces on the board
    /// lowercase = black and uppercase = white
    pub fn new(fen: Option<&str>) -> Self {
        let mut board: Board = Board {
            white_pieces: Bitboard(0),
            black_pieces: Bitboard(0),
            empty_pieces: Bitboard(0),
            white_pawns: Bitboard(0),
            white_knights: Bitboard(0),
            white_rooks: Bitboard(0),
            white_bishops: Bitboard(0),
            white_queen: Bitboard(0),
            white_king: Bitboard(0),
            black_pawns: Bitboard(0),
            black_knights: Bitboard(0),
            black_rooks: Bitboard(0),
            black_bishops: Bitboard(0),
            black_queen: Bitboard(0),
            black_king: Bitboard(0),
        };
        if let None = fen {
            return board;
        }
        let fen_str = fen.unwrap();
        let mut index: usize = 0;

        for c in fen_str.chars() {
            // Shift 1 as u64 by index amount of bits to the left
            let current_bit: Position = Position(1u64 << index);
            // Sets the current_bit in the bitmap of the corresponding field
            match c {
                '/' => index = (index + 7) / 8 * 8,
                '1'..'9' => index += c.to_digit(10).unwrap_or(0) as usize,
                'p' => {
                    board.black_pawns |= current_bit;
                    index += 1
                }
                'n' => {
                    board.black_knights |= current_bit;
                    index += 1
                }
                'b' => {
                    board.black_bishops |= current_bit;
                    index += 1
                }
                'r' => {
                    board.black_rooks |= current_bit;
                    index += 1
                }
                'q' => {
                    board.black_queen |= current_bit;
                    index += 1
                }
                'k' => {
                    board.black_king |= current_bit;
                    index += 1
                }
                'P' => {
                    board.white_pawns |= current_bit;
                    index += 1
                }
                'N' => {
                    board.white_knights |= current_bit;
                    index += 1
                }
                'B' => {
                    board.white_bishops |= current_bit;
                    index += 1
                }
                'R' => {
                    board.white_rooks |= current_bit;
                    index += 1
                }
                'Q' => {
                    board.white_queen |= current_bit;
                    index += 1
                }
                'K' => {
                    board.white_king |= current_bit;
                    index += 1
                }

                _ => {
                    println!("Invalid Character");
                }
            }
        }

        board.white_pieces = board.white_pawns
            | board.white_knights
            | board.white_bishops
            | board.white_rooks
            | board.white_queen
            | board.white_king;

        board.black_pieces = board.black_pawns
            | board.black_knights
            | board.black_bishops
            | board.black_rooks
            | board.black_queen
            | board.black_king;

        board.empty_pieces = !(board.white_pieces | board.black_pieces);
        board
    }
}
