use crate::prelude::*;
use std::collections::VecDeque;

const START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct EngineState {
    pub board: Board,
    applied_moves: Vec<String>,
}

impl EngineState {
    /// Erstellt neuen State ab Startposition
    pub fn new() -> Self {
        EngineState {
            board: Board::from_fen(START_POS),
            applied_moves: Vec::new(),
        }
    }

    /// Handhabt den UCI-`position`-Befehl inkrementell
    pub fn handle_position(&mut self, args: &[&str]) {
        let mut iter = args.iter().peekable();

        // 1) Stellung initialisieren, wenn Startpos oder FEN
        if let Some(&&token) = iter.peek() {
            match token {
                "startpos" => {
                    self.board = Board::from_fen(START_POS);
                    self.applied_moves.clear();
                    iter.next();
                }
                "fen" => {
                    iter.next(); // "fen"
                    if let Some(&fen_str) = iter.next() {
                        self.board = Board::from_fen(fen_str);
                        self.applied_moves.clear();
                    }
                }
                _ => {}
            }
        }

        // 2) Über "moves" zur Zugliste springen
        if let Some(&"moves") = iter.next() {
            // Sammle alle Moves als Strings
            let moves: Vec<&str> = iter.cloned().collect();
            // 3) Ab dem ersten neuen Zug anwenden
            for (idx, &mv_str) in moves.iter().enumerate() {
                if idx < self.applied_moves.len() {
                    continue;
                }
                // Parse und ausführen
                let mv = ChessMove::from_coords(mv_str.to_string(), &self.board);
                self.board.make_move(&mv);
                self.applied_moves.push(mv_str.to_string());
            }
        }
    }
}
