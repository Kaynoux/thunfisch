use crate::prelude::*;
use colored;
use colored::Colorize;
use num_format::{Locale, ToFormattedString};
use rayon::prelude::*;
use std::time::Instant;

pub fn print_board(board: &Board, moves: Option<&[EncodedMove]>) {
    let moves_slice = moves.unwrap_or(&[]);
    let char_board: [(char, &str); 64] = get_char_board(board, moves_slice);
    let mut y: i32 = 7;
    let mut x: i32 = 0;

    println!(
        "Current Color: {:?} Halfmove Clock: {} Fullmove Counter: {}",
        board.current_color, board.halfmove_clock, board.fullmove_counter
    );
    println!("FEN: {}", board.generate_fen());
    // println!("Phase: {}", board.get_game_phase());
    if moves.is_some() {
        println!("Moves Possible: {}", moves_slice.len());
    }

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

fn get_char_board(board: &Board, moves: &[EncodedMove]) -> [(char, &'static str); 64] {
    let mut char_board = [(' ', "white"); 64];
    for y in 0usize..=7usize {
        for x in 0usize..=7usize {
            let idx = y * 8 + x;
            let pos = IndexPosition(idx);

            let (piece, color) = board.get_piece_and_color_at_position(pos.to_position());
            let mut text_color = "white";
            if moves
                .iter()
                .any(|chess_move| chess_move.decode().from == pos)
            {
                text_color = "green";
            }
            if moves.iter().any(|chess_move| chess_move.decode().to == pos) {
                text_color = "red";
            }
            char_board[idx] = (Piece::to_unicode_char(piece, color), text_color);
        }
    }
    char_board
}

pub fn print_moves(board: &Board, moves: &Vec<EncodedMove>) {
    println!("Potential Moves:");
    let (
        mut prev_pos,
        mut prev_is_queen_castle,
        mut prev_is_king_castle,
        mut prev_is_promotion,
        mut prev_is_ep_capture,
    ) = (Position(0), false, false, false, false);
    for encoded_mv in moves {
        let mv = encoded_mv.decode();
        let (
            current_pos,
            current_is_queen_castle,
            current_is_king_castle,
            current_is_promotion,
            current_is_ep_capture,
        ) = (
            mv.from.to_position(),
            mv.is_queen_castle,
            mv.is_king_castle,
            mv.promotion.is_some(),
            mv.is_ep_capture,
        );
        let (current_color, current_piece) = board.get_piece_and_color_at_position(current_pos);
        if current_pos != prev_pos
            || current_is_queen_castle != prev_is_queen_castle
            || current_is_king_castle != prev_is_king_castle
            || current_is_promotion != prev_is_promotion
            || current_is_ep_capture != prev_is_ep_capture
        {
            println!();
            if current_is_queen_castle {
                print!("Queen Castle: ")
            }
            if current_is_king_castle {
                print!("King Castle: ")
            }
            if current_is_promotion {
                print!("Promotion: ")
            }
            if current_is_ep_capture {
                print!("En-Passant: ")
            }
            print!(
                "{:?} {:?} {:?} -> ",
                current_color,
                current_piece,
                mv.from.to_position()
            );
            (
                prev_pos,
                prev_is_queen_castle,
                prev_is_king_castle,
                prev_is_promotion,
                prev_is_ep_capture,
            ) = (
                current_pos,
                current_is_queen_castle,
                current_is_king_castle,
                current_is_promotion,
                current_is_ep_capture,
            );
        }
        print!(" {:?},", mv.to.to_position());
    }
    println!()
}

pub fn r_perft(board: &Board, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }
    let mut nodes = 0;
    let moves = board.get_legal_moves(false);
    for mv in moves {
        let mut b2 = board.clone();
        b2.make_move(&mv.decode());
        nodes += r_perft(&b2, depth - 1);
    }
    nodes
}

pub fn r_detailed_perft(
    board: &Board,
    depth: usize,
    captures: &mut isize,
    promotions: &mut isize,
    castles: &mut isize,
    en_passants: &mut isize,
    double_moves: &mut isize,
) -> usize {
    if depth == 0 {
        return 1;
    }
    let mut nodes = 0;
    let moves = board.get_legal_moves(false);
    for encoded_mv in moves {
        let mut b2 = board.clone();
        b2.make_move(&encoded_mv.decode());
        nodes += r_detailed_perft(
            &b2,
            depth - 1,
            captures,
            promotions,
            castles,
            en_passants,
            double_moves,
        );

        let mv = encoded_mv.decode();

        if mv.is_capture {
            *captures += 1;
        }
        if mv.promotion.is_some() {
            *promotions += 1;
        }
        if mv.is_king_castle || mv.is_queen_castle {
            *castles += 1;
        }
        if mv.is_ep_capture {
            *en_passants += 1;
        }
        if mv.is_double_move {
            *double_moves += 1;
        }
    }
    nodes
}

pub fn debug_perft(board: &Board, depth: usize) {
    let mut captures: isize = 0;
    let mut promotions: isize = 0;
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
    let moves = board.get_legal_moves(false);
    for encoded_mv in &moves {
        let mv = encoded_mv.decode();
        let mut b2 = board.clone();
        b2.make_move(&mv);
        let nodes_for_move = r_detailed_perft(
            &b2,
            depth - 1,
            &mut captures,
            &mut promotions,
            &mut queen_castles,
            &mut ep_captures,
            &mut double_moves,
        );
        total_nodes += nodes_for_move;
        println!(
            "{}{}: {}",
            mv.from.to_position().to_coords(),
            mv.to.to_position().to_coords(),
            nodes_for_move
        );

        if mv.is_capture {
            captures += 1;
        }
        if mv.promotion.is_some() {
            promotions += 1;
        }
        if mv.is_queen_castle {
            queen_castles += 1;
        }
        if mv.is_king_castle {
            king_castles += 1;
        }
        if mv.is_ep_capture {
            ep_captures += 1;
        }
        if mv.is_double_move {
            double_moves += 1;
        }
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

    for m in moves {}
    println!("Captures: {}", captures);
    println!("En Passants: {}", ep_captures);
    println!("Castles: {}", queen_castles);
    println!("Promotions: {}", promotions);
    println!("Double moves: {}", double_moves);
}

pub fn perft(board: &Board, depth: usize) {
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

pub fn perft_rayon(board: &Board, depth: usize) {
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

pub fn r_perft_rayon(board: &Board, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }
    board
        .get_legal_moves(false)
        .par_iter()
        .map(|mv| {
            let mut b2 = board.clone();
            b2.make_move(&mv.decode());
            r_perft_rayon(&b2, depth - 1) // Rückgabe ohne Semikolon
        })
        .sum::<usize>()
}
