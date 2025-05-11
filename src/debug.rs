use crate::prelude::*;
use colored;
use colored::Colorize;
use num_format::{Locale, ToFormattedString};
use rayon::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

pub fn print_board(board: &Board, moves: Option<&Vec<EncodedMove>>) {
    println!(
        "Current Color: {:?} Halfmove Clock: {} Fullmove Counter: {}",
        board.current_color, board.halfmove_clock, board.total_halfmove_counter
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

fn get_char_board(board: &Board, moves: Option<&Vec<EncodedMove>>) -> [(char, &'static str); 64] {
    let mut char_board = [(' ', "white"); 64];
    for y in 0usize..=7usize {
        for x in 0usize..=7usize {
            let idx = y * 8 + x;
            let pos = Square(idx);

            let (piece, color) = board.get_piece_and_color_at_position(pos.to_bit());
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

pub fn print_moves(board: &Board, moves: &Vec<EncodedMove>) {
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

pub fn r_perft(board: &mut Board, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }
    let mut nodes = 0;
    let moves = board.get_moves(false);
    for mv in moves {
        board.make_move(&mv.decode());
        nodes += r_perft(board, depth - 1);
        board.unmake_move();
    }
    nodes
}

pub fn r_detailed_perft(
    board: &mut Board,
    depth: usize,
    captures: &mut isize,
    normal_promotions: &mut isize,
    capture_promotions: &mut isize,
    queen_castles: &mut isize,
    king_castles: &mut isize,
    ep_captures: &mut isize,
    double_moves: &mut isize,
) -> usize {
    if depth == 0 {
        return 1;
    }
    let mut nodes = 0;
    let moves = board.get_moves(false);
    for encoded_mv in moves {
        let mut b2 = board.clone();
        b2.make_move(&encoded_mv.decode());
        nodes += r_detailed_perft(
            &mut b2,
            depth - 1,
            captures,
            normal_promotions,
            capture_promotions,
            queen_castles,
            king_castles,
            ep_captures,
            double_moves,
        );

        let mv = encoded_mv.decode();
        let mv_type = mv.mv_type;

        match mv_type {
            MoveType::Quiet => {} // No specific counter for quiet moves here
            MoveType::DoubleMove => *double_moves += 1,
            MoveType::KingCastle => *king_castles += 1,
            MoveType::QueenCastle => *queen_castles += 1,
            MoveType::Capture => *captures += 1,
            MoveType::EpCapture => {
                *ep_captures += 1;
            }
            MoveType::KnightPromo => {
                *normal_promotions += 1;
            }
            MoveType::BishopPromo => {
                *normal_promotions += 1;
            }
            MoveType::RookPromo => {
                *normal_promotions += 1;
            }
            MoveType::QueenPromo => {
                *normal_promotions += 1;
            }
            MoveType::KnightPromoCapture => {
                *captures += 1;
                *capture_promotions += 1;
            }
            MoveType::BishopPromoCapture => {
                *captures += 1;
                *capture_promotions += 1;
            }
            MoveType::RookPromoCapture => {
                *captures += 1;
                *capture_promotions += 1;
            }
            MoveType::QueenPromoCapture => {
                *captures += 1;
                *capture_promotions += 1;
            }
        }
    }
    nodes
}

pub fn debug_perft(board: &mut Board, depth: usize) {
    let mut captures: isize = 0;
    let mut normal_promotions: isize = 0;
    let mut capture_promotions: isize = 0;
    let mut queen_castles: isize = 0;
    let mut king_castles: isize = 0;
    let mut ep_captures: isize = 0;
    let mut double_moves: isize = 0;
    if depth == 0 {
        println!("Perft divide depth {}:", 0);
        return;
    }
    let start = Instant::now();
    println!("Perft divide depth {}:", depth);
    let mut total_nodes = 0;
    let moves = board.get_moves(false);
    for encoded_mv in &moves {
        let mv = encoded_mv.decode();
        let mut b2 = board.clone();
        b2.make_move(&mv);
        let nodes_for_move = r_detailed_perft(
            &mut b2,
            depth - 1,
            &mut captures,
            &mut normal_promotions,
            &mut capture_promotions,
            &mut queen_castles,
            &mut king_castles,
            &mut ep_captures,
            &mut double_moves,
        );
        total_nodes += nodes_for_move;

        let mv_type = mv.mv_type;

        match mv_type {
            MoveType::Quiet => {} // No specific counter for quiet moves here
            MoveType::DoubleMove => double_moves += 1,
            MoveType::KingCastle => king_castles += 1,
            MoveType::QueenCastle => queen_castles += 1,
            MoveType::Capture => captures += 1,
            MoveType::EpCapture => {
                ep_captures += 1;
            }
            MoveType::KnightPromo => {
                normal_promotions += 1;
            }
            MoveType::BishopPromo => {
                normal_promotions += 1;
            }
            MoveType::RookPromo => {
                normal_promotions += 1;
            }
            MoveType::QueenPromo => {
                normal_promotions += 1;
            }
            MoveType::KnightPromoCapture => {
                captures += 1;
                capture_promotions += 1;
            }
            MoveType::BishopPromoCapture => {
                captures += 1;
                capture_promotions += 1;
            }
            MoveType::RookPromoCapture => {
                captures += 1;
                capture_promotions += 1;
            }
            MoveType::QueenPromoCapture => {
                captures += 1;
                capture_promotions += 1;
            }
        }

        println!(
            "{}{}: {}",
            mv.from.to_bit().to_coords(),
            mv.to.to_bit().to_coords(),
            nodes_for_move,
        );
    }
    let elapsed = start.elapsed();
    let nodes_per_seconds = (total_nodes as f64 / elapsed.as_secs_f64()) as usize;
    println!(
        "Perft: Depth={} Nodes={} Time={:.3}s Nodes/sec={}",
        depth,
        total_nodes.to_formatted_string(&Locale::en),
        elapsed.as_secs_f64(),
        nodes_per_seconds.to_formatted_string(&Locale::en)
    );

    println!("Captures: {}", captures);
    println!("En Passants: {}", ep_captures);
    println!(
        "Total Castles: {}  Queen Castles: {}  King Castles: {}",
        queen_castles + king_castles,
        queen_castles,
        king_castles
    );
    println!(
        "Promotions: {}  Normal Promotions: {}  Capture Promotions: {}",
        normal_promotions + capture_promotions,
        normal_promotions,
        capture_promotions
    );
    println!("Double moves: {}", double_moves);
}

pub fn perft(board: &mut Board, depth: usize) {
    if depth == 0 {
        println!("Perft: Depth=0 Nodes=0 Time=0s Nodes/sec=0");
        return;
    }
    let start = Instant::now();
    let total_nodes = r_perft(board, depth);
    let elapsed = start.elapsed();
    let nodes_per_seconds = (total_nodes as f64 / elapsed.as_secs_f64()) as usize;
    println!(
        "Perft: Depth={} Nodes={} Time={:.3}s Nodes/sec={}",
        depth,
        total_nodes.to_formatted_string(&Locale::en),
        elapsed.as_secs_f64(),
        nodes_per_seconds.to_formatted_string(&Locale::en)
    );
}

pub fn perft_rayon(board: &mut Board, depth: usize) {
    if depth == 0 {
        println!("Perft: Depth=0 Nodes=0 Time=0s Nodes/sec=0");
        return;
    }
    let start = Instant::now();
    let total_nodes = r_perft_rayon(board, depth);
    let elapsed = start.elapsed();
    let nodes_per_seconds = (total_nodes as f64 / elapsed.as_secs_f64()) as usize;
    println!(
        "Perft: Depth={} Nodes={} Time={:.3}s Nodes/sec={}",
        depth,
        total_nodes.to_formatted_string(&Locale::en),
        elapsed.as_secs_f64(),
        nodes_per_seconds.to_formatted_string(&Locale::en)
    );
}

pub fn r_perft_rayon(board: &mut Board, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }
    board
        .get_moves(false)
        .par_iter()
        .map(|mv| {
            let mut b2 = board.clone();
            b2.make_move(&mv.decode());
            r_perft(&mut b2, depth - 1) // Rückgabe ohne Semikolon
        })
        .sum::<usize>()
}
