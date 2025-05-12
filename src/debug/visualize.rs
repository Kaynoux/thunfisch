use crate::move_generator::generator::ARRAY_LENGTH;
use crate::prelude::*;
use arrayvec::ArrayVec;
use colored;
use colored::Colorize;
use num_format::{Locale, ToFormattedString};
use rayon::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

pub fn print_board(board: &Board, moves: Option<&ArrayVec<EncodedMove, ARRAY_LENGTH>>) {
    println!(
        "Current Color: {:?} Halfmove Clock: {} Fullmove Counter: {}",
        board.current_color(),
        board.halfmove_clock(),
        board.total_halfmove_counter()
    );
    println!("FEN: {}", board.generate_fen());
    // println!("Phase: {}", board.get_game_phase());
    if let Some(m) = moves {
        print_moves(board, m);
    }

    let char_board: [(char, &str); 64] = get_char_board(board, moves);
    let mut y: i32 = 7;
    let mut x: i32 = 0;

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

pub fn print_moves(board: &Board, moves: &ArrayVec<EncodedMove, ARRAY_LENGTH>) {
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
            None => &[], // Leerer Slice, falls dieser Typ nicht vorkommt
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
