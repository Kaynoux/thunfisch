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
            bbs: [Bitboard(0); 13],
            pieces: [ColorPiece::Empty; 64],
            black_king_castle: true,
            black_queen_castle: true,
            white_queen_castle: true,
            white_king_castle: true,
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
                        'p' => {
                            board.bbs[ColorPiece::BlackPawn as usize] |= bit;
                            board.pieces[index] = ColorPiece::BlackPawn;
                        }
                        'n' => {
                            board.bbs[ColorPiece::BlackKnight as usize] |= bit;
                            board.pieces[index] = ColorPiece::BlackKnight;
                        }
                        'b' => {
                            board.bbs[ColorPiece::BlackBishop as usize] |= bit;
                            board.pieces[index] = ColorPiece::BlackBishop;
                        }
                        'r' => {
                            board.bbs[ColorPiece::BlackRook as usize] |= bit;
                            board.pieces[index] = ColorPiece::BlackRook;
                        }
                        'q' => {
                            board.bbs[ColorPiece::BlackQueen as usize] |= bit;
                            board.pieces[index] = ColorPiece::BlackQueen;
                        }
                        'k' => {
                            board.bbs[ColorPiece::BlackKing as usize] |= bit;
                            board.pieces[index] = ColorPiece::BlackKing;
                        }

                        'P' => {
                            board.bbs[ColorPiece::WhitePawn as usize] |= bit;
                            board.pieces[index] = ColorPiece::WhitePawn;
                        }
                        'N' => {
                            board.bbs[ColorPiece::WhiteKnight as usize] |= bit;
                            board.pieces[index] = ColorPiece::WhiteKnight;
                        }
                        'B' => {
                            board.bbs[ColorPiece::WhiteBishop as usize] |= bit;
                            board.pieces[index] = ColorPiece::WhiteBishop;
                        }
                        'R' => {
                            board.bbs[ColorPiece::WhiteRook as usize] |= bit;
                            board.pieces[index] = ColorPiece::WhiteRook;
                        }
                        'Q' => {
                            board.bbs[ColorPiece::WhiteQueen as usize] |= bit;
                            board.pieces[index] = ColorPiece::WhiteQueen;
                        }
                        'K' => {
                            board.bbs[ColorPiece::WhiteKing as usize] |= bit;
                            board.pieces[index] = ColorPiece::WhiteKing;
                        }
                        _ => {}
                    }
                    index += 1;
                }
            }
        }

        // All individual bitboards set now calculate the generell bitboards
        board.recalculate_black_white_empty_pieces();

        // Set Active Color Part
        board.current_color = match active_color {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Ungültige aktive Farbe in FEN"),
        };

        // Set Castling bools
        board.white_queen_castle = castling.contains('Q');
        board.white_king_castle = castling.contains('K');
        board.black_king_castle = castling.contains('q');
        board.black_queen_castle = castling.contains('k');

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
