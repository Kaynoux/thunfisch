use crate::prelude::*;
use colored;
use colored::Colorize;

pub fn print_board(board: &Board, moves: Option<&[ChessMove]>) {
    let moves_slice = moves.unwrap_or(&[]);
    let char_board: [(char, &str); 64] = get_char_board(board, moves_slice);
    let mut y: i32 = 7;
    let mut x: i32 = 0;

    println!(
        "Current Color: {:?} Halfmove Clock: {} Fullmove Counter: {}",
        board.current_color, board.halfmove_clock, board.fullmove_counter
    );
    println!("Possible amount of moves: {}", moves_slice.len());

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

fn get_char_board(board: &Board, moves: &[ChessMove]) -> [(char, &'static str); 64] {
    let mut char_board = [(' ', "white"); 64];
    for y in 0usize..=7usize {
        for x in 0usize..=7usize {
            let idx = y * 8 + x;
            let pos = Position::from_idx(idx as isize);

            let (piece, color) = board.get_piece_and_color_at_position(pos);
            let mut text_color = "white";
            if moves.iter().any(|chess_move| chess_move.from == pos) {
                text_color = "green";
            }
            if moves.iter().any(|chess_move| chess_move.to == pos) {
                text_color = "red";
            }
            char_board[idx] = (Piece::get_unicode_symbol(piece, color), text_color);
        }
    }
    char_board
}

pub fn print_moves(board: &Board, moves: &Vec<ChessMove>) {
    println!("Potential Moves:");
    let (mut prev_pos, mut prev_is_castle, mut prev_is_promotion, mut prev_is_en_passant) =
        (Position(0), false, false, false);
    for mv in moves {
        let (current_pos, current_is_castle, current_is_promotion, current_is_en_passant) =
            (mv.from, mv.is_castle, mv.is_promotion, mv.is_en_passant);
        let (current_color, current_piece) = board.get_piece_and_color_at_position(current_pos);
        if current_pos != prev_pos
            || current_is_castle != prev_is_castle
            || current_is_promotion != prev_is_promotion
            || current_is_en_passant != prev_is_en_passant
        {
            println!();
            if current_is_castle {
                print!("Castle: ")
            }
            if current_is_promotion {
                print!("Promotion: ")
            }
            if current_is_en_passant {
                print!("En-Passant: ")
            }
            print!("{:?} {:?} {:?} -> ", current_color, current_piece, mv.from);
            (
                prev_pos,
                prev_is_castle,
                prev_is_promotion,
                prev_is_en_passant,
            ) = (
                current_pos,
                current_is_castle,
                current_is_promotion,
                current_is_en_passant,
            );
        }
        print!(" {:?},", mv.to);
    }
    println!()
}

pub fn perft(board: &Board, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }
    let mut nodes = 0;
    let moves = board.generate_legal_moves();
    for mv in moves {
        let mut b2 = board.clone();
        b2.make_move(&mv);
        nodes += perft(&b2, depth - 1);
    }
    nodes
}

pub fn perft_divide(board: &Board, depth: usize) {
    if depth == 0 {
        println!("Perft divide depth {}:", 0);
        return;
    }
    println!("Perft divide depth {}:", depth);
    let mut total = 0;
    let moves = board.generate_legal_moves();
    for mv in moves {
        let mut b2 = board.clone();
        b2.make_move(&mv);
        let cnt = perft(&b2, depth - 1);
        total += cnt;
        println!("{}{}: {}", mv.from.to_coords(), mv.to.to_coords(), cnt);
    }
    println!("Total: {}", total);
}
