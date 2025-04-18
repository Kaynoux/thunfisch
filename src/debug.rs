use crate::prelude::*;
use crate::utils;
use colored;
use colored::Colorize;

pub fn print_board(board: &Board, title: &str, moves: Vec<ChessMove>) {
    let char_board: [(char, &str); 64] = get_char_board(board, moves.clone());
    let mut y: i32 = 7;
    let mut x: i32 = 0;

    println!("{}", title);
    println!("Amount of moves: {}", moves.len());

    while y >= 0 {
        print!("{} | ", y);
        while x <= 7 {
            let idx = (y * 8 + x) as usize;
            let colored_str = char_board[idx].0.to_string().color(char_board[idx].1);
            print!("{} ", colored_str);

            x += 1;
        }
        x = 0;
        y -= 1;
        println!();
    }
    println!("    0 1 2 3 4 5 6 7");
    println!("-------------------");
}

fn get_char_board(board: &Board, moves: Vec<ChessMove>) -> [(char, &str); 64] {
    let mut char_board = [(' ', "white"); 64];
    for y in 0usize..=7usize {
        for x in 0usize..=7usize {
            let idx = y * 8 + x;
            let pos = utils::idx_to_position(idx as isize);

            let (piece, color) = board.get_piece_and_color_at_position(pos);
            let mut text_color = "white";
            if moves.iter().any(|chess_move| chess_move.0 == pos) {
                text_color = "green";
            }
            if moves.iter().any(|chess_move| chess_move.1 == pos) {
                text_color = "red";
            }
            char_board[idx] = (Piece::get_unicode_symbol(piece, color), text_color);
        }
    }
    char_board
}

pub fn print_moves(moves: Vec<ChessMove>) {
    println!("Potential Moves:");
    for chess_move in moves.iter() {
        let (x1, y1) = chess_move.0.to_xy();
        let (x2, y2) = chess_move.1.to_xy();
        println!("{}{} {}{}", x1, y1, x2, y2);
    }
}
