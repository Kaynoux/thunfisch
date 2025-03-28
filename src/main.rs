#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
enum Color {
    Black,
    White,
    None,
}

#[derive(Clone, Copy)]
struct Field {
    piece: Piece,
    color: Color,
    position: Position,
    is_occupied: bool,
}

impl Position {
    fn is_within_bounds(&self) -> bool {
        self.x < 8 && self.y < 8
    }
}

#[derive(Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

fn main() {
    let start_pos = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
        "r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1",
    ];
    let mut board: [[Field; 8]; 8] = parse_fen(start_pos[1]);
    print_board(board);

    // let stdin = io::stdin();
    // for line in stdin.lock().lines() {
    //     let input = line.unwrap();
    //     match input.as_str() {
    //         "uci" => {
    //             println!("id name rusty-chess-bot");
    //             println!("id author Lukas");
    //             println!("uciok");
    //         }
    //         "isready" => {
    //             println!("readyok");
    //         }
    //         "quit" => {
    //             break;
    //         }
    //         _ => {
    //             println!("Error")
    //         }
    //     }
    //}
}

/// Converts a FEN to a Board
/// FEN describes the position of all pieces on the board
/// lowercase = black and uppercase = white
fn parse_fen(fen: &str) -> [[Field; 8]; 8] {
    let mut board: [[Field; 8]; 8] = [[Field {
        piece: Piece::Empty,
        color: Color::None,
        position: Position { x: 0, y: 0 },
        is_occupied: false,
    }; 8]; 8];
    let mut current_x = 0;
    let mut current_y: usize = 7;

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
                        x: current_x,
                        y: current_y,
                    },
                    is_occupied: true,
                };
                current_x += 1;
            }
            'n' => {
                board[current_x][current_y] = Field {
                    piece: Piece::Knight,
                    color: Color::Black,
                    position: Position {
                        x: current_x,
                        y: current_y,
                    },
                    is_occupied: true,
                };
                current_x += 1;
            }
            'r' => {
                board[current_x][current_y] = Field {
                    piece: Piece::Rook,
                    color: Color::Black,
                    position: Position {
                        x: current_x,
                        y: current_y,
                    },
                    is_occupied: true,
                };
                current_x += 1;
            }
            'b' => {
                board[current_x][current_y] = Field {
                    piece: Piece::Bishop,
                    color: Color::Black,
                    position: Position {
                        x: current_x,
                        y: current_y,
                    },
                    is_occupied: true,
                };
                current_x += 1;
            }
            'q' => {
                board[current_x][current_y] = Field {
                    piece: Piece::Queen,
                    color: Color::Black,
                    position: Position {
                        x: current_x,
                        y: current_y,
                    },
                    is_occupied: true,
                };
                current_x += 1;
            }
            'k' => {
                board[current_x][current_y] = Field {
                    piece: Piece::King,
                    color: Color::Black,
                    position: Position {
                        x: current_x,
                        y: current_y,
                    },
                    is_occupied: true,
                };
                current_x += 1;
            }
            'P' => {
                board[current_x][current_y] = Field {
                    piece: Piece::Pawn,
                    color: Color::White,
                    position: Position {
                        x: current_x,
                        y: current_y,
                    },
                    is_occupied: true,
                };
                current_x += 1;
            }
            'N' => {
                board[current_x][current_y] = Field {
                    piece: Piece::Knight,
                    color: Color::White,
                    position: Position {
                        x: current_x,
                        y: current_y,
                    },
                    is_occupied: true,
                };
                current_x += 1;
            }
            'R' => {
                board[current_x][current_y] = Field {
                    piece: Piece::Rook,
                    color: Color::White,
                    position: Position {
                        x: current_x,
                        y: current_y,
                    },
                    is_occupied: true,
                };
                current_x += 1;
            }
            'B' => {
                board[current_x][current_y] = Field {
                    piece: Piece::Bishop,
                    color: Color::White,
                    position: Position {
                        x: current_x,
                        y: current_y,
                    },
                    is_occupied: true,
                };
                current_x += 1;
            }
            'Q' => {
                board[current_x][current_y] = Field {
                    piece: Piece::Queen,
                    color: Color::White,
                    position: Position {
                        x: current_x,
                        y: current_y,
                    },
                    is_occupied: true,
                };
                current_x += 1;
            }
            'K' => {
                board[current_x][current_y] = Field {
                    piece: Piece::King,
                    color: Color::White,
                    position: Position {
                        x: current_x,
                        y: current_y,
                    },
                    is_occupied: true,
                };
                current_x += 1;
            }
            _ => {
                println!("Invalid Character")
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

fn generate_legal_pawn_moves(board: [[Field; 8]; 8], field: Field) {
    let mut legal_moves: Vec<Position> = Vec::new();

    let forward_field: Field = board[field.position.x][field.position.y];
    if !forward_field.is_occupied {
        legal_moves.push(forward_field.position);
    }
}
