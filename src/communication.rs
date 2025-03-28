use crate::types::{Color, Field, Piece, Position};
use log::error;
use std::io::{self, BufRead};

pub fn handle_uci_communication() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input = line.unwrap();
        match input.as_str() {
            "uci" => {
                println!("id name rusty-chess-bot");
                println!("id author Lukas");
                println!("uciok");
            }
            "isready" => {
                println!("readyok");
            }
            "quit" => {
                break;
            }
            _ => {
                println!("Error")
            }
        }
    }
}

/// Converts a FEN to a Board
/// FEN describes the position of all pieces on the board
/// lowercase = black and uppercase = white
pub fn parse_fen(fen: &str) -> [[Field; 8]; 8] {
    let mut board: [[Field; 8]; 8] = [[Field {
        piece: Piece::Empty,
        color: Color::None,
        position: Position { x: 0, y: 0 },
    }; 8]; 8];
    let mut current_x = 0;
    let mut current_y: usize = 7;

    for y in 0..8 {
        for x in 0..8 {
            board[x][y].position = Position {
                x: x as i32,
                y: y as i32,
            };
        }
    }

    for c in fen.chars() {
        match c {
            '/' => {
                current_x = 0;
                current_y -= 1;
            }
            '1'..='8' => current_x += c.to_digit(10).unwrap_or(0) as usize,
            'p' => {
                board[current_x][current_y] = Field {
                    piece: Piece::Pawn,
                    color: Color::Black,
                    position: Position {
                        x: current_x as i32,
                        y: current_y as i32,
                    },
                };
                current_x += 1;
            }
            'n' => {
                board[current_x][current_y] = Field {
                    piece: Piece::Knight,
                    color: Color::Black,
                    position: Position {
                        x: current_x as i32,
                        y: current_y as i32,
                    },
                };
                current_x += 1;
            }
            'r' => {
                board[current_x][current_y] = Field {
                    piece: Piece::Rook,
                    color: Color::Black,
                    position: Position {
                        x: current_x as i32,
                        y: current_y as i32,
                    },
                };
                current_x += 1;
            }
            'b' => {
                board[current_x][current_y] = Field {
                    piece: Piece::Bishop,
                    color: Color::Black,
                    position: Position {
                        x: current_x as i32,
                        y: current_y as i32,
                    },
                };
                current_x += 1;
            }
            'q' => {
                board[current_x][current_y] = Field {
                    piece: Piece::Queen,
                    color: Color::Black,
                    position: Position {
                        x: current_x as i32,
                        y: current_y as i32,
                    },
                };
                current_x += 1;
            }
            'k' => {
                board[current_x][current_y] = Field {
                    piece: Piece::King,
                    color: Color::Black,
                    position: Position {
                        x: current_x as i32,
                        y: current_y as i32,
                    },
                };
                current_x += 1;
            }
            'P' => {
                board[current_x][current_y] = Field {
                    piece: Piece::Pawn,
                    color: Color::White,
                    position: Position {
                        x: current_x as i32,
                        y: current_y as i32,
                    },
                };
                current_x += 1;
            }
            'N' => {
                board[current_x][current_y] = Field {
                    piece: Piece::Knight,
                    color: Color::White,
                    position: Position {
                        x: current_x as i32,
                        y: current_y as i32,
                    },
                };
                current_x += 1;
            }
            'R' => {
                board[current_x][current_y] = Field {
                    piece: Piece::Rook,
                    color: Color::White,
                    position: Position {
                        x: current_x as i32,
                        y: current_y as i32,
                    },
                };
                current_x += 1;
            }
            'B' => {
                board[current_x][current_y] = Field {
                    piece: Piece::Bishop,
                    color: Color::White,
                    position: Position {
                        x: current_x as i32,
                        y: current_y as i32,
                    },
                };
                current_x += 1;
            }
            'Q' => {
                board[current_x][current_y] = Field {
                    piece: Piece::Queen,
                    color: Color::White,
                    position: Position {
                        x: current_x as i32,
                        y: current_y as i32,
                    },
                };
                current_x += 1;
            }
            'K' => {
                board[current_x][current_y] = Field {
                    piece: Piece::King,
                    color: Color::White,
                    position: Position {
                        x: current_x as i32,
                        y: current_y as i32,
                    },
                };
                current_x += 1;
            }
            _ => {
                error!("Invalid Character")
            }
        }
    }
    board
}
