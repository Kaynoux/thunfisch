use crate::{
    debug::custom_commands::handle_custom_commands, iterative_deepening::iterative_deepening,
    prelude::*, time_management::calc_search_time, transposition_table::TT,
    types::board::START_POS,
};
use std::{
    io::{self, BufRead, Write},
    process::exit,
};

pub fn handle_communication(board: &mut Board) {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line_res in stdin.lock().lines() {
        let Ok(line) = line_res else { break };
        let mut parts = line.split_whitespace();

        if let Some(command) = parts.next() {
            let args: Vec<&str> = parts.collect();

            // Try to handle UCI commands first
            if !handle_uci_commands(board, command, &args) {
                // If no uci command matched, then try matching a custom command
                handle_custom_commands(board, command, &args);
            }
        }

        stdout.flush().unwrap();
    }
}

// UCI Commands: needed to comply with UCI protocoll
fn handle_uci_commands(board: &mut Board, command: &str, args: &[&str]) -> bool {
    #[allow(clippy::match_same_arms)]
    match command {
        "uci" => {
            println!("id name Thunfisch");
            println!("id author Lukas Piorek (Kaynoux), Emil Schläger (heofthetea)");
            println!("uciok");
        }
        "debug" => {
            println!("info currently no dynamic debug mode, use compiletime features instead");
        }
        "isready" => {
            // We are always ready
            println!("readyok");
        }
        "setoption" => {
            // TODO: we need to be able to change TT size atleast here
            println!("info currently no options, use compiletime features instead");
        }
        "register" => {
            // This is probably a legacy UCI feature
            println!("info register not implemented");
        }
        "ucinewgame" => {
            *board = Board::new(START_POS);
        }
        "position" => {
            set_position(board, args);
        }
        "go" => {
            handle_go(board, args, false, false);
        }
        "stop" => {
            // This will not work in search atm because it will not read this line because it will be stuck in search
        }
        "ponderhit" => {
            // Pondering is not implemented at all atm
        }
        "quit" => exit(0),
        _ => return false,
    }
    true
}

fn set_position(board: &mut Board, args: &[&str]) {
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
                *board = Board::new(&fen);
            }
            "startpos" => {
                iter.next();
                *board = Board::new(START_POS);
            }
            "index" => {
                // usesd for debugging only positions from here https://www.codeproject.com/Articles/5313417/Worlds-fastest-Bitboard-Chess-Movegenerator
                let fens = [
                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", // 0. Initial Pos
                    "r3k2r/p1ppqpb1/Bn2pnp1/3PN3/1p2P3/2N2Q2/PPPB1PpP/R3K2R b KQ - 0 1", // 1. sebastian lague alpa beta test
                    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ", // Pos 2
                    "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1 ",                        // Pos 3
                    "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", // Pos 4 - weird ahh clusterfuck with a check in progress
                    "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8 ",       // Pos 5
                    "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ", // Pos 6
                    "6Q1/pp6/8/8/1kp2N2/1n2R1P1/3r4/1K6 b - - 0 1", // 7 - Threefold Repetition,  Kasparov v. Deep Blue (somewhere on https://en.wikipedia.org/wiki/Threefold_repetition)
                    "1kr5/6pp/R1P2p2/1N6/8/1P6/P2q1PPP/6K1 w - - 0 1", // 8 - Forced perpetual check (3-fold repetition after 8 ply) to avoid checkmate
                    "r1b1k2r/1pqp1ppp/p2bpnn1/6Q1/2BNP3/2N1B3/PPP2PPP/2KR3R w kq - 8 12", // 9 - Position where including the TT with the QS blunders its queen
                    "8/k7/3p4/p2P1p2/P2P1P2/8/8/K7 w - - 0 1", // 10 - Endgame Position taken from Sebatian Lague forcing a Triangulation (through a1b1 instead of a1b2) to win
                    "1r5k/8/1Q6/5Pp1/1KB4r/8/8/8 w - g6 2 3",  // 11 - Test islegal with double pin
                ];

                iter.next();
                if let Some(&idx_str) = iter.next() {
                    match idx_str.parse::<usize>() {
                        Ok(index_val) => {
                            if index_val >= fens.len() {
                                eprintln!("Index to large, FEN not found");
                            } else {
                                *board = Board::new(fens[index_val]);
                            }
                        }
                        Err(err) => eprintln!("Could not parse index `{idx_str}`: {err}"),
                    }
                }
            }
            _ => {}
        }
    }

    // if keyword moves appear then we will execute the following moves on the board
    if iter.next() == Some(&"moves") {
        let moves: Vec<&str> = iter.copied().collect();

        // makes every move in order the perfectly recreate the input
        for &mv_str in &moves {
            let mv = DecodedMove::from_coords(mv_str, board);
            board.make_move(mv.encode());
        }
    }
}

pub fn handle_go(board: &mut Board, args: &[&str], debug: bool, help: bool) {
    let (max_depth, time_limit) = calc_search_time(args, board);
    let best_move = iterative_deepening(board, max_depth, time_limit, debug, help);

    if let Some(mv) = best_move {
        println!("info pv {}", mv.decode().to_coords());
        println!("bestmove {}", mv.decode().to_coords());
    } else if board.is_in_check() {
        println!("Game over: Checkmate!");
    } else {
        println!("Game over: Stalemate!");
    }

    TT.increase_age();
}
