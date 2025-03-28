use log::{error, info, warn};
use std::{
    future::poll_fn,
    io::{self, BufRead},
    ptr::copy,
};

#[derive(Clone, Copy, Debug)]
enum Piece {
    Empty,
    Pawn,
    Knight,
    Rook,
    Bishop,
    Queen,
    King,
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let field_symbol = match (self.piece, self.color) {
            (Piece::Empty, Color::None) => " ",
            (Piece::Pawn, Color::Black) => "p",
            (Piece::Knight, Color::Black) => "n",
            (Piece::Rook, Color::Black) => "r",
            (Piece::Bishop, Color::Black) => "b",
            (Piece::Queen, Color::Black) => "q",
            (Piece::King, Color::Black) => "k",
            (Piece::Pawn, Color::White) => "P",
            (Piece::Knight, Color::White) => "N",
            (Piece::Rook, Color::White) => "R",
            (Piece::Bishop, Color::White) => "B",
            (Piece::Queen, Color::White) => "Q",
            (Piece::King, Color::White) => "K",
            (_, _) => "E",
        };
        write!(f, "{}", field_symbol)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Color {
    Black,
    White,
    None,
}

impl Color {
    fn opposite(self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
            Color::None => Color::None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Field {
    piece: Piece,
    color: Color,
    position: Position,
}

impl Position {
    fn is_within_bounds(&self) -> bool {
        self.x >= 0 && self.x <= 7 && self.y >= 0 && self.y <= 7
    }
}

#[derive(Copy, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Debug)]
struct Move {
    x: i32,
    y: i32,
}

impl std::ops::Add<Move> for Position {
    type Output = Position;
    fn add(self, offset: Move) -> Self {
        Self {
            x: self.x + offset.x,
            y: self.y + offset.y,
        }
    }
}

fn main() {
    let start_pos = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
        "r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1",
        "rnbqkbnr/p1pppppp/8/8/8/1p6/PPPPPPPP/RNBQKBNR",
    ];
    let mut board: [[Field; 8]; 8] = parse_fen(start_pos[2]);
    print_board(board);
    println!("{:?}", Position { x: 8, y: 0 }.is_within_bounds());
    println!("{:?}", get_field(board, Position { x: 1, y: 2 }));
    print_all_legal_moves(board);
}

/// Converts a FEN to a Board
/// FEN describes the position of all pieces on the board
/// lowercase = black and uppercase = white
fn parse_fen(fen: &str) -> [[Field; 8]; 8] {
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

/// Prints the current board formatted
/// lowercase letters = black and uppercase letters = white    
fn print_board(board: [[Field; 8]; 8]) {
    let mut column_idx: i32 = 0;
    let mut row_idx: i32 = 7;
    println!("    0   1   2   3   4   5   6   7");
    while row_idx >= 0 {
        println!("  ---------------------------------");
        print!("{} |", row_idx);
        while column_idx <= 7 {
            print!(
                " {} |",
                board[column_idx as usize][row_idx as usize].to_string()
            );
            column_idx += 1;
        }
        column_idx = 0;
        row_idx -= 1;

        println!();
    }
}

fn generate_pseudo_legal_pawn_moves(board: [[Field; 8]; 8], field: Field) -> Vec<Position> {
    let mut pseudo_legal_positions: Vec<Position> = Vec::new();
    let mut potential_positions_1: Vec<Position> = Vec::new();
    let mut potential_positions_2: Vec<Position> = Vec::new();
    let move_direction = match field.color {
        Color::Black => -1,
        Color::White => 1,
        Color::None => panic!("Pawn provided is not valid"),
    };

    //Adds potential positions
    match (field.position, field.color) {
        (Position { x: _, y: 1 }, Color::White) => {
            potential_positions_1.push(Position {
                x: field.position.x,
                y: field.position.y + move_direction,
            });
            potential_positions_2.push(Position {
                x: field.position.x,
                y: field.position.y + (move_direction * 2),
            })
        }
        (Position { x: _, y: 6 }, Color::Black) => {
            potential_positions_1.push(Position {
                x: field.position.x,
                y: field.position.y + move_direction,
            });
            potential_positions_2.push(Position {
                x: field.position.x,
                y: field.position.y + (move_direction * 2),
            })
        }

        (_, _) => potential_positions_1.push(Position {
            x: field.position.x,
            y: field.position.y + move_direction,
        }),
    };

    // Adds psedo leglal position when moved by one field
    for potential_position in potential_positions_1.iter() {
        // Skip if out of bounds
        if !potential_position.is_within_bounds() {
            continue;
        }

        // Skip if target field is not empty
        let forward_field: Field = get_field(board, *potential_position);
        if forward_field.color != Color::None {
            continue;
        }

        pseudo_legal_positions.push(*potential_position);
    }

    // Adds psedo leglal position when moved by two field
    for potential_position in potential_positions_2.iter() {
        // Skip if out of bounds
        if !potential_position.is_within_bounds() {
            continue;
        }

        // Skip if target field is not empty
        let forward_field: Field = get_field(board, *potential_position);
        if forward_field.color != Color::None {
            continue;
        }

        // Skip if field in between is not free
        if get_field(
            board,
            Position {
                x: potential_position.x,
                y: potential_position.y - move_direction,
            },
        )
        .color
            != Color::None
        {
            continue;
        }

        pseudo_legal_positions.push(*potential_position);
    }

    // Adds possible strikes
    let potential_strike_moves: [Move; 2] = [Move { x: 1, y: 1 }, Move { x: -1, y: 1 }];
    for potential_strike_move in potential_strike_moves.iter() {
        let target_pos = field.position
            + Move {
                x: potential_strike_move.x,
                y: potential_strike_move.y * move_direction,
            };

        if !target_pos.is_within_bounds() {
            continue;
        }

        let target_color = get_field(board, target_pos).color;
        if target_color != field.color.opposite() {
            continue;
        }

        pseudo_legal_positions.push(target_pos);
    }

    pseudo_legal_positions
}

fn print_moves(field: Field, moves: Vec<Position>) {
    println!("Field: {:?}", field);
    for move_pos in moves.iter() {
        println!("  Move: {:?}", move_pos);
    }
}

fn handle_uci_communication() {
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

fn print_all_legal_moves(board: [[Field; 8]; 8]) {
    for y in 0..8 {
        for x in 0..8 {
            match board[x][y].piece {
                Piece::Pawn => print_moves(
                    board[x][y],
                    generate_pseudo_legal_pawn_moves(board, board[x][y]),
                ),
                _ => {}
            }
        }
    }
}

fn get_field(board: [[Field; 8]; 8], position: Position) -> Field {
    board[position.x as usize][position.y as usize]
}
