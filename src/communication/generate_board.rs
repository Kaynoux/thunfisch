use crate::prelude::*;

pub fn handle_input(board: &mut Board, args: &[&str]) {
    const START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
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
                *board = Board::from_fen(&fen);
            }
            "startpos" => {
                iter.next();
                *board = Board::from_fen(START_POS);
            }
            "index" => {
                // usesd for debugging only positions from here https://www.codeproject.com/Articles/5313417/Worlds-fastest-Bitboard-Chess-Movegenerator
                let fens = [
                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", // 0. Initial Pos
                    "r3k2r/p1ppqpb1/Bn2pnp1/3PN3/1p2P3/2N2Q2/PPPB1PpP/R3K2R b KQ - 0 1", // 1. sebastian lague alpa beta test
                    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ", // Pos 2
                    "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1 ",                        // Pos 3
                    "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",  // Pos 4
                    "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8 ",        // Pos 5
                    "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ", // Pos 6
                ];

                iter.next();
                if let Some(&idx_str) = iter.next() {
                    match idx_str.parse::<usize>() {
                        Ok(index_val) => {
                            if index_val >= fens.len() {
                                eprintln!("Index to large, FEN not found");
                            } else {
                                *board = Board::from_fen(fens[index_val]);
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
            let mv = DecodedMove::from_coords(mv_str.to_string(), &board);
            board.make_move(&mv);
        }
    }
}
