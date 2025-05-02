use crate::prelude::*;

const START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct EngineState {
    pub board: Board,
}

impl EngineState {
    pub fn new() -> Self {
        EngineState {
            board: Board::from_fen(START_POS),
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
                }
                "startpos" => {
                    iter.next();
                    self.board = Board::from_fen(START_POS);
                }
                "index" => {
                    let fens =
                        ["r3k2r/p1ppqpb1/Bn2pnp1/3PN3/1p2P3/2N2Q2/PPPB1PpP/R3K2R b KQ - 0 1"]; // 0. sebastian lague alpa beta test
                    iter.next();
                    if let Some(&idx_str) = iter.next() {
                        match idx_str.parse::<usize>() {
                            Ok(index_val) => {
                                if index_val >= fens.len() {
                                    eprintln!("Index to large, FEN not found");
                                } else {
                                    self.board = Board::from_fen(fens[index_val]);
                                }
                            }
                            Err(err) => eprintln!("Could not parse index `{}`: {}", idx_str, err),
                        }
                    }
                }
                _ => {}
            }
        }

        // if keyword moves appear then we will execute the following moves on the board
        if let Some(&"moves") = iter.next() {
            let moves: Vec<&str> = iter.cloned().collect();

            // makes every move in order the perfectly recreate the input
            for &mv_str in moves.iter() {
                let mv = ChessMove::from_coords(mv_str.to_string(), &self.board);
                self.board.make_move(&mv);
            }
        }
    }
}
