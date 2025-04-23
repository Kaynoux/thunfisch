use crate::prelude::*;

const START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct EngineState {
    pub board: Board,
    applied_moves: Vec<String>,
}

impl EngineState {
    pub fn new() -> Self {
        EngineState {
            board: Board::from_fen(START_POS),
            applied_moves: Vec::new(),
        }
    }

    pub fn handle_position(&mut self, args: &[&str]) {
        let mut iter = args.iter().peekable();

        if let Some(&&token) = iter.peek() {
            match token {
                "fen" => {
                    iter.next(); // Skips fen keyword

                    //collects the parts which belong to the fen
                    let mut fen_parts = Vec::new();
                    while let Some(&&s) = iter.peek() {
                        if s == "moves" {
                            break;
                        }
                        fen_parts.push(*iter.next().unwrap());
                    }
                    //joins them back together and creates board with them
                    let fen = fen_parts.join(" ");
                    self.board = Board::from_fen(&fen);
                    self.applied_moves.clear();
                }
                _ => {}
            }
        }

        // if keyword moves appear then we will execute the following moves on the board
        if let Some(&"moves") = iter.next() {
            let moves: Vec<&str> = iter.cloned().collect();

            // makes every move in order the perfectly recreate the input
            for (idx, &mv_str) in moves.iter().enumerate() {
                if idx < self.applied_moves.len() {
                    continue;
                }
                let mv = ChessMove::from_coords(mv_str.to_string(), &self.board);
                self.board.make_move(&mv);
                self.applied_moves.push(mv_str.to_string());
            }
        }
    }
}
