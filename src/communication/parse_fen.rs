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
                    match ch {
                        'p' => {
                            board.toggle(Color::Black, Figure::BlackPawn, Square(index));
                        }
                        'n' => {
                            board.toggle(Color::Black, Figure::BlackKnight, Square(index));
                        }
                        'b' => {
                            board.toggle(Color::Black, Figure::BlackBishop, Square(index));
                        }
                        'r' => {
                            board.toggle(Color::Black, Figure::BlackRook, Square(index));
                        }
                        'q' => {
                            board.toggle(Color::Black, Figure::BlackQueen, Square(index));
                        }
                        'k' => {
                            board.toggle(Color::Black, Figure::BlackKing, Square(index));
                        }
                        'P' => {
                            board.toggle(Color::White, Figure::WhitePawn, Square(index));
                        }
                        'N' => {
                            board.toggle(Color::White, Figure::WhiteKnight, Square(index));
                        }
                        'B' => {
                            board.toggle(Color::White, Figure::WhiteBishop, Square(index));
                        }
                        'R' => {
                            board.toggle(Color::White, Figure::WhiteRook, Square(index));
                        }
                        'Q' => {
                            board.toggle(Color::White, Figure::WhiteQueen, Square(index));
                        }
                        'K' => {
                            board.toggle(Color::White, Figure::WhiteKing, Square(index));
                        }
                        _ => {}
                    }
                    index += 1;
                }
            }
        }

        // Set Active Color Part
        board.set_current_color(match active_color {
            "w" => White,
            "b" => Black,
            _ => panic!("Ungültige aktive Farbe in FEN"),
        });

        // Set Castling bools
        let white_queen_castle = castling.contains('Q');
        let white_king_castle = castling.contains('K');
        let black_king_castle = castling.contains('q');
        let black_queen_castle = castling.contains('k');

        board.set_castling_rights(
            white_queen_castle,
            white_king_castle,
            black_queen_castle,
            black_king_castle,
        );

        // Set En passant target
        board.set_ep_target(if ep_target == "-" {
            None
        } else {
            Bit::from_coords(ep_target)
        });

        board.set_halfmove_clock(halfmove.parse().expect("Invalid halfmove clock"));
        board.set_total_halfmove_counter(fullmove.parse().expect("Invalid fullmove counter"));

        board
    }
}
