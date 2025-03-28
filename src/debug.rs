use crate::move_generation::generate_pseudo_legal_knight_moves;
use crate::types::{Field, Piece, Position};
use crate::utils::get_field;

pub fn print_all_legal_moves(board: [[Field; 8]; 8]) {
    for y in 0..8 {
        for x in 0..8 {
            let field = get_field(board, Position { x: x, y: y });
            match field.piece {
                // Piece::Pawn => {
                //     let moves = generate_pseudo_legal_pawn_moves(board, field);
                //     print_moves(field, moves.clone());
                //     print_moves_as_board(board, field, moves);
                // }
                // Piece::King => {
                //     let moves = generate_pseudo_legal_king_moves(board, field);
                //     print_moves(field, moves.clone());
                //     print_moves_as_board(board, field, moves);
                // }
                Piece::Knight => {
                    let moves = generate_pseudo_legal_knight_moves(board, field);
                    //print_moves(field, moves.clone());
                    print_moves_as_board(board, field, moves);
                }
                _ => {}
            }
        }
    }
}

pub fn print_moves(field: Field, moves: Vec<Position>) {
    println!();
    println!("Potential Moves for: {}", field);
    for move_pos in moves.iter() {
        println!("  {} {}", move_pos.x, move_pos.y);
    }
}

/// Prints the current board formatted
/// lowercase letters = black and uppercase letters = white    
pub fn print_board_as_board(board: [[Field; 8]; 8]) {
    let mut column_idx: i32 = 0;
    let mut row_idx: i32 = 7;
    println!();
    println!("Current Board");
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

/// Prints moves on board formatted
pub fn print_moves_as_board(board: [[Field; 8]; 8], field: Field, moves: Vec<Position>) {
    let mut y: i32 = 7;
    let mut x: i32 = 0;
    println!();
    println!("Potential Moves for {}", field);
    println!("    0   1   2   3   4   5   6   7");
    while y >= 0 {
        println!("  ---------------------------------");
        print!("{} |", y);
        while x <= 7 {
            let text = if moves.contains(&Position { x: x, y: y }) {
                "X".to_string()
            } else {
                board[x as usize][y as usize].to_string()
            };
            print!(" {} |", text);
            x += 1;
        }
        x = 0;
        y -= 1;

        println!();
    }
}
