use crate::{prelude::*, types::unmake_info::UnmakeInfo};

impl Board {
    /// Converts a FEN to a Board
    /// FEN describes the position of all pieces on the board
    /// lowercase = black and uppercase = white
    pub fn from_fen(fen: &str) -> Self {
        let mut board = Board::EMPTY;
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
                    let bit = Bit(1u64 << index as u64);
                    match ch {
                        'p' => {
                            board.bbs[Figure::BlackPawn as usize] |= bit;
                            board.pieces[index] = Figure::BlackPawn;
                        }
                        'n' => {
                            board.bbs[Figure::BlackKnight as usize] |= bit;
                            board.pieces[index] = Figure::BlackKnight;
                        }
                        'b' => {
                            board.bbs[Figure::BlackBishop as usize] |= bit;
                            board.pieces[index] = Figure::BlackBishop;
                        }
                        'r' => {
                            board.bbs[Figure::BlackRook as usize] |= bit;
                            board.pieces[index] = Figure::BlackRook;
                        }
                        'q' => {
                            board.bbs[Figure::BlackQueen as usize] |= bit;
                            board.pieces[index] = Figure::BlackQueen;
                        }
                        'k' => {
                            board.bbs[Figure::BlackKing as usize] |= bit;
                            board.pieces[index] = Figure::BlackKing;
                        }

                        'P' => {
                            board.bbs[Figure::WhitePawn as usize] |= bit;
                            board.pieces[index] = Figure::WhitePawn;
                        }
                        'N' => {
                            board.bbs[Figure::WhiteKnight as usize] |= bit;
                            board.pieces[index] = Figure::WhiteKnight;
                        }
                        'B' => {
                            board.bbs[Figure::WhiteBishop as usize] |= bit;
                            board.pieces[index] = Figure::WhiteBishop;
                        }
                        'R' => {
                            board.bbs[Figure::WhiteRook as usize] |= bit;
                            board.pieces[index] = Figure::WhiteRook;
                        }
                        'Q' => {
                            board.bbs[Figure::WhiteQueen as usize] |= bit;
                            board.pieces[index] = Figure::WhiteQueen;
                        }
                        'K' => {
                            board.bbs[Figure::WhiteKing as usize] |= bit;
                            board.pieces[index] = Figure::WhiteKing;
                        }
                        _ => {}
                    }
                    index += 1;
                }
            }
        }

        // All individual bitboards set now calculate the generell bitboards
        board.recalculate_genereal_bitboards();

        // Set Active Color Part
        board.current_color() = match active_color {
            "w" => White,
            "b" => Black,
            _ => panic!("Ungültige aktive Farbe in FEN"),
        };

        // Set Castling bools
        board.white_queen_castle = castling.contains('Q');
        board.white_king_castle = castling.contains('K');
        board.black_king_castle = castling.contains('q');
        board.black_queen_castle = castling.contains('k');

        // Set En passant target
        board.ep_target = if ep_target == "-" {
            None
        } else {
            Bit::from_coords(ep_target)
        };

        board.set_halfmove_clock(halfmove.parse().expect("Invalid halfmove clock"));
        board.set_total_halfmove_counter(fullmove.parse().expect("Invalid fullmove counter"));

        board
    }
}
