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
            (Piece::Pawn, Color::White) => "p",
            (Piece::Knight, Color::White) => "n",
            (Piece::Rook, Color::White) => "r",
            (Piece::Bishop, Color::White) => "b",
            (Piece::Queen, Color::White) => "q",
            (Piece::King, Color::White) => "k",
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
}
fn main() {
    let mut board: [[Field; 8]; 8] = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
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

fn parse_fen(fen: &str) -> [[Field; 8]; 8] {
    let mut board: [[Field; 8]; 8] = [[Field {
        piece: Piece::Empty,
        color: Color::None,
    }; 8]; 8];
    let mut current_row: usize = 7;
    let mut current_col = 0;

    for c in fen.chars() {
        match c {
            '/' => {
                current_col = 0;
                current_row -= 1;
            }
            '1'..='8' => current_col += c.to_digit(10).unwrap_or(0) as usize,
            'p' => {
                board[current_col][current_row] = Field {
                    piece: Piece::Pawn,
                    color: Color::Black,
                };
                current_col += 1;
            }
            'n' => {
                board[current_col][current_row] = Field {
                    piece: Piece::Knight,
                    color: Color::Black,
                };
                current_col += 1;
            }
            'r' => {
                board[current_col][current_row] = Field {
                    piece: Piece::Rook,
                    color: Color::Black,
                };
                current_col += 1;
            }
            'b' => {
                board[current_col][current_row] = Field {
                    piece: Piece::Bishop,
                    color: Color::Black,
                };
                current_col += 1;
            }
            'q' => {
                board[current_col][current_row] = Field {
                    piece: Piece::Queen,
                    color: Color::Black,
                };
                current_col += 1;
            }
            'k' => {
                board[current_col][current_row] = Field {
                    piece: Piece::King,
                    color: Color::Black,
                };
                current_col += 1;
            }
            'P' => {
                board[current_col][current_row] = Field {
                    piece: Piece::Pawn,
                    color: Color::White,
                };
                current_col += 1;
            }
            'N' => {
                board[current_col][current_row] = Field {
                    piece: Piece::Knight,
                    color: Color::White,
                };
                current_col += 1;
            }
            'R' => {
                board[current_col][current_row] = Field {
                    piece: Piece::Rook,
                    color: Color::White,
                };
                current_col += 1;
            }
            'B' => {
                board[current_col][current_row] = Field {
                    piece: Piece::Bishop,
                    color: Color::White,
                };
                current_col += 1;
            }
            'Q' => {
                board[current_col][current_row] = Field {
                    piece: Piece::Queen,
                    color: Color::White,
                };
                current_col += 1;
            }
            'K' => {
                board[current_col][current_row] = Field {
                    piece: Piece::King,
                    color: Color::White,
                };
                current_col += 1;
            }
            _ => {
                println!("Invalid Character")
            }
        }
    }
    board
}

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
