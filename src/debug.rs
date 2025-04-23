use crate::prelude::*;
use colored;
use colored::Colorize;
use std::time::Instant;

pub fn print_board(board: &Board, moves: Option<&[ChessMove]>) {
    let moves_slice = moves.unwrap_or(&[]);
    let char_board: [(char, &str); 64] = get_char_board(board, moves_slice);
    let mut y: i32 = 7;
    let mut x: i32 = 0;

    println!(
        "Current Color: {:?} Halfmove Clock: {} Fullmove Counter: {}",
        board.current_color, board.halfmove_clock, board.fullmove_counter
    );
    println!("{}", board.generate_fen());
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
            char_board[idx] = (Piece::to_unicode_char(piece, color), text_color);
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

pub fn r_perft(board: &Board, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }
    let mut nodes = 0;
    let moves = board.generate_legal_moves().1;
    for mv in moves {
        let mut b2 = board.clone();
        b2.make_move(&mv);
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
    let moves = board.generate_legal_moves().1;
    for mv in moves {
        let mut b2 = board.clone();
        b2.make_move(&mv);
        nodes += r_detailed_perft(
            &b2,
            depth - 1,
            captures,
            promotions,
            castles,
            en_passants,
            double_moves,
        );

        if mv.is_capture {
            *captures += 1;
        }
        if mv.is_promotion {
            *promotions += 1;
        }
        if mv.is_castle {
            *castles += 1;
        }
        if mv.is_en_passant {
            *en_passants += 1;
        }
        if mv.is_double_move {
            *double_moves += 1;
        }
    }
    nodes
}

pub fn perft_divide(board: &Board, depth: usize) {
    let mut captures: isize = 0;
    let mut promotions: isize = 0;
    let mut castles: isize = 0;
    let mut en_passants: isize = 0;
    let mut double_moves: isize = 0;
    if depth == 0 {
        println!("Perft divide depth {}:", 0);
        return;
    }
    let start = Instant::now();
    println!("Perft divide depth {}:", depth);
    let mut total_nodes = 0;
    let moves = board.generate_legal_moves().1;
    for mv in &moves {
        let mut b2 = board.clone();
        b2.make_move(&mv);
        let nodes_for_move = r_detailed_perft(
            &b2,
            depth - 1,
            &mut captures,
            &mut promotions,
            &mut castles,
            &mut en_passants,
            &mut double_moves,
        );
        total_nodes += nodes_for_move;
        println!(
            "{}{}: {}",
            mv.from.to_coords(),
            mv.to.to_coords(),
            nodes_for_move
        );
    }
    let elapsed = start.elapsed();
    println!("Total: {}", total_nodes);
    println!(
        "Time: {:.3}s, Nodes/sec: {:.0}",
        elapsed.as_secs_f64(),
        total_nodes as f64 / elapsed.as_secs_f64()
    );

    for m in moves {
        if m.is_capture {
            captures += 1;
        }
        if m.is_promotion {
            promotions += 1;
        }
        if m.is_castle {
            castles += 1;
        }
        if m.is_en_passant {
            en_passants += 1;
        }
        if m.is_double_move {
            double_moves += 1;
        }
    }
    println!("Captures: {}", captures);
    println!("En Passants: {}", en_passants);
    println!("Castles: {}", castles);
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
    println!(
        "Perft: Depth={} Nodes={} Time={:.3}s Nodes/sec={:.0}",
        depth,
        total_nodes,
        elapsed.as_secs_f64(),
        total_nodes as f64 / elapsed.as_secs_f64()
    );
}
