use std::time::Instant;

use num_format::{Locale, ToFormattedString};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::prelude::*;

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
    let moves = board.generate_moves(false);
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

pub fn perft_debug(board: &mut Board, depth: usize) {
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
    let moves = board.generate_moves(false);
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
        .generate_moves(false)
        .par_iter()
        .map(|mv| {
            let mut b2 = board.clone();
            b2.make_move(&mv.decode());
            r_perft(&mut b2, depth - 1) // Rückgabe ohne Semikolon
        })
        .sum::<usize>()
}

pub fn perft_test(board: &mut Board, depth: usize) {
    let mut total_nodes = 0;
    let moves = board.generate_moves(false);
    for encoded_mv in &moves {
        let mv = encoded_mv.decode();
        let mut b2 = board.clone();
        b2.make_move(&mv);
        let nodes_for_move = r_perft(&mut b2, depth - 1);
        total_nodes += nodes_for_move;

        println!("{} {}", mv.to_coords(), nodes_for_move,);
    }
    println!();
    println!("Nodes searched: {}", total_nodes);
}

pub fn perft_perftree(board: &mut Board, depth: usize) {
    let mut total_nodes = 0;
    let moves = board.generate_moves(false);
    for encoded_mv in &moves {
        let mv = encoded_mv.decode();
        let mut b2 = board.clone();
        b2.make_move(&mv);
        let nodes_for_move = r_perft(&mut b2, depth - 1);
        total_nodes += nodes_for_move;

        println!("{} {} ", mv.to_coords(), nodes_for_move,);
    }
    println!();
    println!("{}", total_nodes);
}
pub fn r_perft(board: &mut Board, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }
    let mut nodes = 0;
    let moves = board.generate_moves(false);
    for mv in moves {
        board.make_move(&mv.decode());
        nodes += r_perft(board, depth - 1);
        board.unmake_move();
    }
    nodes
}
