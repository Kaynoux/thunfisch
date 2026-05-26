use std::time::Instant;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    debug::visualize::{format_f64, format_usize},
    prelude::*,
};

#[derive(Default)]
struct PerftLogs {
    captures: usize,
    normal_promotions: usize,
    capture_promotions: usize,
    queen_castles: usize,
    king_castles: usize,
    ep_captures: usize,
    double_moves: usize,
}
fn r_detailed_perft(board: &mut Board, depth: usize, perft_logs: &mut PerftLogs) -> usize {
    if depth == 0 {
        return 1;
    }
    let mut nodes = 0;
    let moves = board.generate_all_moves();
    for encoded_mv in moves.list {
        board.make_move(encoded_mv.mv);
        nodes += r_detailed_perft(board, depth - 1, perft_logs);
        board.unmake_move();
        let mv = encoded_mv.mv.decode();
        let mv_type = mv.mv_type;

        match mv_type {
            MoveType::Quiet => {} // No specific counter for quiet moves here
            MoveType::DoubleMove => perft_logs.double_moves += 1,
            MoveType::KingCastle => perft_logs.king_castles += 1,
            MoveType::QueenCastle => perft_logs.queen_castles += 1,
            MoveType::Capture => perft_logs.captures += 1,
            MoveType::EpCapture => {
                perft_logs.ep_captures += 1;
            }
            MoveType::KnightPromo
            | MoveType::BishopPromo
            | MoveType::RookPromo
            | MoveType::QueenPromo => {
                perft_logs.normal_promotions += 1;
            }
            MoveType::KnightPromoCapture
            | MoveType::BishopPromoCapture
            | MoveType::RookPromoCapture
            | MoveType::QueenPromoCapture => {
                perft_logs.captures += 1;
                perft_logs.capture_promotions += 1;
            }
        }
    }
    nodes
}

pub fn perft_debug(board: &mut Board, depth: usize) {
    let mut perft_logs = PerftLogs::default();
    if depth == 0 {
        println!("Perft divide depth {}:", 0);
        return;
    }
    let start = Instant::now();
    println!("Perft divide depth {depth}:");
    let mut total_nodes = 0;
    let moves = board.generate_all_moves();
    for encoded_mv in moves.list {
        let mv = encoded_mv.mv;
        board.make_move(mv);
        let nodes_for_move = r_detailed_perft(board, depth - 1, &mut perft_logs);
        board.unmake_move();
        total_nodes += nodes_for_move;

        let mv_type = mv.decode().mv_type;

        match mv_type {
            MoveType::Quiet => {} // No specific counter for quiet moves here
            MoveType::DoubleMove => perft_logs.double_moves += 1,
            MoveType::KingCastle => perft_logs.king_castles += 1,
            MoveType::QueenCastle => perft_logs.queen_castles += 1,
            MoveType::Capture => perft_logs.captures += 1,
            MoveType::EpCapture => {
                perft_logs.ep_captures += 1;
            }
            MoveType::KnightPromo
            | MoveType::BishopPromo
            | MoveType::RookPromo
            | MoveType::QueenPromo => {
                perft_logs.normal_promotions += 1;
            }
            MoveType::KnightPromoCapture
            | MoveType::BishopPromoCapture
            | MoveType::RookPromoCapture
            | MoveType::QueenPromoCapture => {
                perft_logs.captures += 1;
                perft_logs.capture_promotions += 1;
            }
        }

        println!(
            "{}{}: {}",
            mv.decode().from.to_bit().to_coords(),
            mv.decode().to.to_bit().to_coords(),
            nodes_for_move,
        );
    }
    let elapsed = start.elapsed();

    #[allow(clippy::cast_possible_truncation)]
    let nodes_per_seconds = total_nodes / elapsed.as_millis() as usize * 1000;
    #[allow(clippy::cast_precision_loss)]
    let elapsed_str = format_f64(elapsed.as_millis() as f64);
    println!(
        "Perft: Depth={} Nodes={} Time={}ms Nodes/sec={}",
        depth,
        format_usize(total_nodes),
        elapsed_str,
        format_usize(nodes_per_seconds)
    );

    println!("Captures: {0}", perft_logs.captures);
    println!("En Passants: {0}", perft_logs.ep_captures);
    println!(
        "Total Castles: {}  Queen Castles: {}  King Castles: {}",
        perft_logs.queen_castles + perft_logs.king_castles,
        perft_logs.queen_castles,
        perft_logs.king_castles
    );
    println!(
        "Promotions: {}  Normal Promotions: {}  Capture Promotions: {}",
        perft_logs.normal_promotions + perft_logs.capture_promotions,
        perft_logs.normal_promotions,
        perft_logs.capture_promotions
    );
    println!("Double moves: {0}", perft_logs.double_moves);
}

pub fn perft(board: &mut Board, depth: usize) {
    if depth == 0 {
        println!("Perft: Depth=0 Nodes=0 Time=0ms Nodes/sec=0");
        return;
    }
    let start = Instant::now();
    let total_nodes = r_perft(board, depth);
    let elapsed = start.elapsed();

    #[allow(clippy::cast_possible_truncation)]
    let nodes_per_seconds = total_nodes / elapsed.as_millis() as usize * 1000;
    #[allow(clippy::cast_precision_loss)]
    let elapsed_str = format_f64(elapsed.as_millis() as f64);
    println!(
        "Perft: Depth={} Nodes={} Time={}ms Nodes/sec={}",
        depth,
        format_usize(total_nodes),
        elapsed_str,
        format_usize(nodes_per_seconds)
    );
}

pub fn perft_rayon(board: &mut Board, depth: usize) {
    if depth == 0 {
        println!("Perft: Depth=0 Nodes=0 Time=0ms Nodes/sec=0");
        return;
    }
    let start = Instant::now();
    let total_nodes = r_perft_rayon(board, depth);
    let elapsed = start.elapsed();

    #[allow(clippy::cast_possible_truncation)]
    let nodes_per_seconds = total_nodes / elapsed.as_millis() as usize * 1000;
    #[allow(clippy::cast_precision_loss)]
    let elapsed_str = format_f64(elapsed.as_millis() as f64);
    println!(
        "Perft: Depth={} Nodes={} Time={}ms Nodes/sec={}",
        depth,
        format_usize(total_nodes),
        elapsed_str,
        format_usize(nodes_per_seconds)
    );
}

pub fn r_perft_rayon(board: &mut Board, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }
    board
        .generate_all_moves()
        .list
        .par_iter()
        .map(|mv| {
            let mut b2 = board.clone();
            b2.make_move(mv.mv);
            r_perft(&mut b2, depth - 1)
        })
        .sum::<usize>()
}

#[cfg(test)]
pub fn hash_test_perft(board: &mut Board, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }
    board
        .generate_all_moves()
        .list
        .par_iter()
        .map(|mv| {
            let mut b2 = board.clone();
            b2.make_move(mv.mv);
            // Compare hash from scratch with the incremental hash to test if the hashing works
            assert_eq!(b2.hash(), b2.generate_hash());
            r_perft(&mut b2, depth - 1)
        })
        .sum::<usize>()
}

pub fn perft_perftree_format(board: &mut Board, depth: usize) {
    let mut total_nodes = 0;
    let moves = board.generate_all_moves();
    for encoded_mv in &moves.list {
        let mv = encoded_mv.mv;
        board.make_move(mv);
        let nodes_for_move = r_perft(board, depth - 1);
        board.unmake_move();
        total_nodes += nodes_for_move;

        println!("{} {} ", mv.decode().to_coords(), nodes_for_move,);
    }
    println!();
    println!("{total_nodes}");
}
pub fn r_perft(board: &mut Board, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }
    let mut nodes = 0;
    let moves = board.generate_all_moves();

    if depth == 1 {
        return moves.list.len();
    }

    for mv in moves.list {
        board.make_move(mv.mv);
        nodes += r_perft(board, depth - 1);
        board.unmake_move();
    }
    nodes
}
