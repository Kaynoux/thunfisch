use crate::prelude::*;
use crate::search::move_picker::MoveList;
use std::collections::HashMap;

pub fn print_board(board: &Board, moves: Option<&MoveList>) {
    println!(
        "Current Color: {:?}\nHalfmove Clock: {}\nTotal Halfmove Counter: {}\nEn Passant target:{},\nPrevious occurrences: {}\nHash: {}, Previous (latest first): {:?}",
        board.current_color(),
        board.halfmove_clock(),
        board.total_halfmove_counter(),
        board
            .ep_target()
            .map_or_else(|| "-".to_owned(), Bit::to_coords),
        board.count_repetitions(),
        board.hash(),
        board
            .repetition_stack()
            .iter()
            .rev()
            .take(10)
            .map(|&h| h.to_string().chars().take(4).collect())
            .collect::<Vec<String>>()
    );
    println!("FEN: {}", board.generate_fen());
    // println!("Phase: {}", board.get_game_phase());
    if let Some(mv) = moves {
        print_moves(mv);
    }

    let char_board: [char; 64] = get_char_board(board);

    for y in (0..8).rev() {
        print!("{} | ", y + 1);
        for x in 0..8 {
            let idx = y * 8 + x;
            let c = char_board[idx];
            print!("{c:?} ");
        }
        println!();
    }
    println!("    A B C D E F G H");
    println!("-------------------");
}

fn get_char_board(board: &Board) -> [char; 64] {
    let mut char_board = [' '; 64];
    for y in 0usize..=7usize {
        for x in 0usize..=7usize {
            let idx = y * 8 + x;
            let pos = Square(idx);

            let (piece, color) = board.piece_and_color_at_position(pos.to_bit());

            char_board[idx] = Piece::to_unicode_char(piece, color);
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

pub fn print_moves(moves: &MoveList) {
    println!("total moves = {}", moves.list.len());
    let mv_only: Vec<EncodedMove> = moves.list.iter().map(|m| m.mv).collect();
    let moves_by_type = group_moves_by_type(&mv_only);

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

    for move_type_variant in &all_move_types {
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

    for (i, mv) in mv_only.iter().enumerate() {
        print!("{}", mv.decode().to_coords());
        if i < moves.list.len() - 1 {
            print!(", ");
        }
    }
    println!();
}

pub fn format_usize(n: usize) -> String {
    let units = ["", "k", "M", "G", "T", "P", "E"];

    if n < 1000 {
        return format!("{n}");
    }

    let mut value = n;
    let mut exp = 0;

    while value >= 1000 && exp < units.len() - 1 {
        value /= 1000;
        exp += 1;
    }

    #[allow(clippy::cast_possible_truncation)]
    let divisor = 1000usize.pow(exp as u32) / 10;
    let remainder = if divisor > 0 { (n / divisor) % 10 } else { 0 };

    if remainder > 0 {
        format!("{value}.{remainder}{}", units[exp])
    } else {
        format!("{value}{}", units[exp])
    }
}

pub fn format_f64(n: f64) -> String {
    let mut abs_n = n.abs();
    let units = ["", "k", "M", "G", "T", "P", "E"];
    let sign = if n < 0.0 { "-" } else { "" };

    if abs_n < 1000.0 {
        return format!("{sign}{abs_n:.2}");
    }

    let mut exp = 0;
    while abs_n >= 1000.0 && exp < units.len() - 1 {
        abs_n /= 1000.0;
        exp += 1;
    }

    // Always formats to exactly 2 decimal places
    format!("{sign}{abs_n:.2}{}", units[exp])
}
