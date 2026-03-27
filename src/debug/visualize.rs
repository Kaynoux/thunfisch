use crate::move_generator::generator::ARRAY_LENGTH;
use crate::prelude::*;
use arrayvec::ArrayVec;
use colored;
use colored::Colorize;
use std::collections::HashMap;

pub fn print_board(board: &Board, moves: Option<&ArrayVec<EncodedMove, ARRAY_LENGTH>>) {
    println!(
        "Current Color: {:?}\nHalfmove Clock: {}\nTotal Halfmove Counter: {}\nPrevious occurrences: {}\nHash: {}, Previous: {:?}",
        board.current_color(),
        board.halfmove_clock(),
        board.total_halfmove_counter(),
        board.count_repetitions(),
        board.hash(),
        board
            .repetition_stack()
            .iter()
            .rev()
            .take(6)
            .map(|&h| h.to_string().chars().take(4).collect())
            .collect::<Vec<String>>()
    );
    println!("FEN: {}", board.generate_fen());
    // println!("Phase: {}", board.get_game_phase());
    if let Some(m) = moves {
        print_moves(m);
    }

    let char_board: [(char, &str); 64] = get_char_board(board, moves);
    let mut y: i32 = 7;
    let mut x: i32 = 0;

    while y >= 0 {
        print!("{} | ", y + 1);
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
    println!("    A B C D E F G H");
    println!("-------------------");
}

fn get_char_board(
    board: &Board,
    moves: Option<&ArrayVec<EncodedMove, ARRAY_LENGTH>>,
) -> [(char, &'static str); 64] {
    let mut char_board = [(' ', "white"); 64];
    for y in 0usize..=7usize {
        for x in 0usize..=7usize {
            let idx = y * 8 + x;
            let pos = Square(idx);

            let (piece, color) = board.piece_and_color_at_position(pos.to_bit());
            let mut text_color = "white";
            if let Some(m) = moves {
                if m.iter().any(|chess_move| chess_move.decode().from == pos) {
                    text_color = "green";
                }
            }

            if let Some(m) = moves {
                if m.iter().any(|chess_move| chess_move.decode().to == pos) {
                    text_color = "red";
                }
            }
            char_board[idx] = (Piece::to_unicode_char(piece, color), text_color);
        }
    }
    char_board
}

pub fn group_moves_by_type(moves: &[EncodedMove]) -> HashMap<MoveType, Vec<EncodedMove>> {
    let mut map = HashMap::new();
    for &enc in moves {
        let mv = enc.decode();
        map.entry(mv.mv_type).or_insert_with(Vec::new).push(enc);
    }
    map
}

pub fn print_moves(moves: &ArrayVec<EncodedMove, ARRAY_LENGTH>) {
    println!("total moves = {}", moves.len());
    let moves_by_type = group_moves_by_type(moves);

    let all_move_types = [
        MoveType::Quiet,
        MoveType::DoubleMove,
        MoveType::KingCastle,
        MoveType::QueenCastle,
        MoveType::Capture,
        MoveType::EpCapture,
        MoveType::KnightPromo,
        MoveType::BishopPromo,
        MoveType::RookPromo,
        MoveType::QueenPromo,
        MoveType::KnightPromoCapture,
        MoveType::BishopPromoCapture,
        MoveType::RookPromoCapture,
        MoveType::QueenPromoCapture,
    ];

    for move_type_variant in all_move_types.iter() {
        let current_moves: &[EncodedMove] = match moves_by_type.get(move_type_variant) {
            Some(mvs) => mvs,
            None => &[],
        };

        if !current_moves.is_empty() {
            print!("{:?} Moves = {}: ", move_type_variant, current_moves.len());
            for (i, mv) in current_moves.iter().enumerate() {
                print!("{}", mv.decode().to_coords());
                if i < current_moves.len() - 1 {
                    print!(", ");
                }
            }
            println!();
        }
    }

    for (i, mv) in moves.iter().enumerate() {
        print!("{}", mv.decode().to_coords());
        if i < moves.len() - 1 {
            print!(", ");
        }
    }
    println!();
}

pub fn format_usize(n: usize) -> String {
    let n_f = n as f64;
    let units = ["", "k", "M", "G", "T", "P", "E"];

    if n < 1000 {
        return format!("{}", n);
    }

    // Calculates the exponent (0 for <1000, 1 for k, 2 for M, etc.)
    let exp = (n_f.ln() / 1000.0_f64.ln()).floor() as usize;

    // Ensure we don't go out of bounds of the array
    let exp = exp.min(units.len() - 1);

    let value = n_f / 1000.0_f64.powi(exp as i32);

    // Formats to a maximum of 1 decimal place, removes .0 if it's an integer
    format!("{:.1}{}", value, units[exp]).replace(".0", "")
}

pub fn format_f64(n: f64) -> String {
    let abs_n = n.abs();
    let units = ["", "k", "M", "G", "T", "P", "E"];
    let sign = if n < 0.0 { "-" } else { "" };

    if abs_n < 1000.0 {
        return format!("{}{:.2}", sign, abs_n);
    }

    // Calculates the exponent (0 for <1000, 1 for k, 2 for M, etc.)
    let exp = (abs_n.ln() / 1000.0_f64.ln()).floor() as usize;

    // Ensure we don't go out of bounds of the array
    let exp = exp.min(units.len() - 1);

    let value = abs_n / 1000.0_f64.powi(exp as i32);

    // Always formats to exactly 2 decimal places
    format!("{}{:.2}{}", sign, value, units[exp])
}
