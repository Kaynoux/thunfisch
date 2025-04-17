use crate::prelude::*;
use crate::utils;

pub fn print_board(board: &Board, title: &str, moves: Vec<ChessMove>) {
    let char_board = get_char_board(board, moves);
    let mut y: i32 = 7;
    let mut x: i32 = 0;

    println!();
    println!("{}", title);
    println!("    0   1   2   3   4   5   6   7");

    while y >= 0 {
        println!("  ---------------------------------");
        print!("{} |", y);
        while x <= 7 {
            print!(" {} |", char_board[(y * 8 + x) as usize]);

            x += 1;
        }
        x = 0;
        y -= 1;

        println!();
    }
}

fn get_char_board(board: &Board, moves: Vec<ChessMove>) -> [char; 64] {
    let mut char_board = [' '; 64];
    for i in 0usize..=63usize {
        let pos = utils::idx_to_position(i as isize);
        if moves.iter().any(|chess_move| chess_move.1 == pos) {
            char_board[i] = 'X';
        } else {
            let (piece, color) = board.get_piece_and_color_at_position(pos);
            char_board[i] = Piece::get_fin_symbol(piece, color);
        }
    }
    char_board
}
