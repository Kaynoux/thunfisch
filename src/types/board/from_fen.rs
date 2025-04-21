use crate::prelude::*;

impl Board {
    /// Converts a FEN to a Board
    /// FEN describes the position of all pieces on the board
    /// lowercase = black and uppercase = white
    pub fn from_fen(fen: &str) -> Self {
        let mut board: Board = Board {
            white_pieces: Bitboard(0),
            black_pieces: Bitboard(0),
            empty_pieces: Bitboard(0),
            white_pawns: Bitboard(0),
            white_knights: Bitboard(0),
            white_rooks: Bitboard(0),
            white_bishops: Bitboard(0),
            white_queens: Bitboard(0),
            white_king: Position(0),
            black_pawns: Bitboard(0),
            black_knights: Bitboard(0),
            black_rooks: Bitboard(0),
            black_bishops: Bitboard(0),
            black_queens: Bitboard(0),
            black_king: Position(0),
            black_castle_left: true,
            black_castle_right: true,
            white_castle_left: true,
            white_castle_right: true,
            en_passant_target: None,
            current_color: Color::Black,
            fullmove_counter: 0,
            halfmove_clock: 0,
        };

        let mut parts = fen.split_whitespace();
        let placement = parts.next().expect("Placement invalid");
        let active_color = parts.next().unwrap_or("w");
        let castling = parts.next().unwrap_or("-");
        let ep_target = parts.next().unwrap_or("-");
        let halfmove = parts.next().unwrap_or("0");
        let fullmove = parts.next().unwrap_or("1");

        // Set Pieces
        // fen begins top left
        let mut index: usize = 56;
        for c in placement.chars() {
            match c {
                '/' => {
                    index = index.saturating_sub(16);
                }

                '1'..='8' => {
                    let skip = c.to_digit(10).unwrap() as usize;
                    index += skip;
                }

                ch => {
                    let bit = Position(1u64 << index as u64);
                    match ch {
                        'p' => board.black_pawns |= bit,
                        'n' => board.black_knights |= bit,
                        'b' => board.black_bishops |= bit,
                        'r' => board.black_rooks |= bit,
                        'q' => board.black_queens |= bit,
                        'k' => board.black_king = bit,

                        'P' => board.white_pawns |= bit,
                        'N' => board.white_knights |= bit,
                        'B' => board.white_bishops |= bit,
                        'R' => board.white_rooks |= bit,
                        'Q' => board.white_queens |= bit,
                        'K' => board.white_king = bit,

                        _ => {}
                    }
                    index += 1;
                }
            }
        }

        board.white_pieces = board.white_pawns
            | board.white_knights
            | board.white_bishops
            | board.white_rooks
            | board.white_queens
            | board.white_king;

        board.black_pieces = board.black_pawns
            | board.black_knights
            | board.black_bishops
            | board.black_rooks
            | board.black_queens
            | board.black_king;

        board.empty_pieces = !(board.white_pieces | board.black_pieces);

        // Set Active Color Part
        board.current_color = match active_color {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Ungültige aktive Farbe in FEN"),
        };

        // Set Castling bools
        board.white_castle_left = castling.contains('Q');
        board.white_castle_right = castling.contains('K');
        board.black_castle_left = castling.contains('q');
        board.black_castle_right = castling.contains('k');

        // Set En passant target
        board.en_passant_target = if ep_target == "-" {
            None
        } else {
            Position::from_coords(ep_target)
        };

        board.halfmove_clock = halfmove.parse().expect("Invalid halfmove clock");
        board.fullmove_counter = fullmove.parse().expect("Invalid fullmove counter");

        board
    }
}
