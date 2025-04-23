use crate::prelude::*;
use std::io::{self, BufRead, Write};

pub fn handle_uci_communication() {
    const START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

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
                // Neue Partie: State komplett zurücksetzen
                state = EngineState::new();
            }
            Some("position") => {
                // Alle restlichen Tokens als Slice weiterreichen
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
            Some(cmd) => {
                eprintln!("Unknown command: {}", cmd);
            }
            None => {}
        }

        stdout.flush().unwrap();
    }
}
