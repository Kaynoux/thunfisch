use crate::debug;
use crate::prelude::*;
use std::io::{self, BufRead, Write};

pub fn handle_uci_communication() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut state = EngineState::new(); // State enthält jetzt das Board

    for line_res in stdin.lock().lines() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => break,
        };

        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("uci") => {
                println!("id name MeinRustBot");
                println!("id author DeinName");
                println!("uciok");
            }
            Some("isready") => {
                println!("readyok");
            }
            Some("ucinewgame") => {
                state = EngineState::new();
            }
            Some("position") => {
                let args: Vec<&str> = parts.collect();
                state.handle_position(&args);
            }
            Some("go") => {
                let moves = state.board.generate_legal_moves().1;
                if let Some(best_move) = moves.first() {
                    state.board.make_move(best_move);
                    println!("bestmove {}", best_move.to_coords());
                } else {
                    println!("bestmove (none)");
                }
            }
            Some("quit") => break,
            Some("perft") => {
                let args: Vec<&str> = parts.collect();
                let depth: usize = match args[0].parse() {
                    Ok(d) => d,
                    Err(_) => 0,
                };

                let debug = args.iter().any(|&flag| flag == "--debug");
                let rayon = args.iter().any(|&flag| flag == "--rayon");

                if debug == true {
                    debug::debug_perft(&state.board, depth);
                } else if rayon == true {
                    debug::perft_rayon(&state.board, depth);
                } else {
                    debug::perft(&state.board, depth);
                }
            }
            Some("fen") => println!("Current Fen: {}", state.board.generate_fen()),
            Some("draw") => debug::print_board(&state.board, None),
            Some(cmd) => {
                eprintln!("Unknown command: {}", cmd);
            }
            None => {}
        }

        stdout.flush().unwrap();
    }
}
