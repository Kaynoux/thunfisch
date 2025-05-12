use crate::communication::generate_board;
use crate::debug;
use crate::debug::perft;
use crate::debug::visualize;
use crate::move_generator::masks;
use crate::move_generator::pinmask;
use crate::prelude::*;
use crate::search;
use std::io::{self, BufRead, Write};
use std::time::Duration;

pub fn handle_uci_communication() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    const START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_fen(START_POS);

    for line_res in stdin.lock().lines() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => break,
        };

        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("uci") => {
                println!("id name Thunfisch");
                println!("id author Lukas Piorek");
                println!("uciok");
            }
            Some("isready") => {
                println!("readyok");
            }
            Some("ucinewgame") => {
                board = Board::from_fen(START_POS);
            }
            Some("position") => {
                let args: Vec<&str> = parts.collect();
                generate_board::handle_input(&mut board, &args);
            }
            Some("go") => {
                let args: Vec<&str> = parts.collect();

                if args.len() >= 2 && args[0] == "perft" {
                    let depth: usize = match args[1].parse() {
                        Ok(d) => d,
                        Err(_) => 0,
                    };

                    let debug = args.iter().any(|&flag| flag == "--debug");
                    let perftree = args.iter().any(|&flag| flag == "--perftree");
                    let rayon = args.iter().any(|&flag| flag == "--rayon");

                    if debug == true {
                        perft::perft_debug(&mut board, depth);
                    } else if rayon == true {
                        perft::perft_rayon(&mut board, depth);
                    } else if perftree == true {
                        perft::perft_perftree(&mut board, depth);
                    } else {
                        perft::perft_test(&mut board, depth);
                    }
                } else if args.len() >= 2 && args[0] == "depth" {
                    let depth: usize = match args[1].parse() {
                        Ok(d) => d,
                        Err(_) => 0,
                    };

                    let best_move = search::iterative_deepening::iterative_deepening(
                        &mut board,
                        depth,
                        Duration::new(3600, 0),
                    );

                    if let Some(best_move) = best_move {
                        println!("bestmove {}", best_move.decode().to_coords());
                    } else {
                        println!("bestmove (none)");
                    }

                    continue;
                } else {
                    let best_move = search::iterative_deepening::iterative_deepening(
                        &mut board,
                        5,
                        Duration::new(1, 0),
                    );

                    if let Some(best_move) = best_move {
                        println!("bestmove {}", best_move.decode().to_coords());
                    } else {
                        if board.is_in_check() {
                            println!("Game over: Checkmate!");
                        } else {
                            println!("Game over: Stalemate!");
                        }
                    }
                }
            }

            Some("fen") => println!("Current Fen: {}", board.generate_fen()),
            Some("draw") => visualize::print_board(&board, None),
            Some("moves") => {
                let moves = board.generate_moves(false);
                visualize::print_board(&board, Some(&moves));
            }
            Some("eval") => loop {},
            Some("do") => {
                let args: Vec<&str> = parts.collect();
                let mv_str: &str = args[0];
                let mv = DecodedMove::from_coords(mv_str.to_string(), &board);
                board.make_move(&mv);
            }
            Some("test") => {
                let moves = board.generate_moves(false);
                visualize::print_board(&board, Some(&moves));
                visualize::print_moves(&board, &moves);
            }
            Some("pinmask") => {
                let (hv_pinmask, diag_pinmask) = pinmask::generate_pin_masks(&board);
                println!("Pin Mask");
                println!("{:?}", hv_pinmask | diag_pinmask);
            }
            Some("checkmask") => {
                let (check_mask, check_counter) = masks::calc_check_mask(&board);

                println!("Check Mask: {}", check_counter);
                println!("{:?}", check_mask);
            }
            Some("attackmask") => {
                let attackmask =
                    masks::calculate_attackmask(&board, board.occupied, !board.current_color, None);
                println!("Attack Mask:");
                println!("{:?}", attackmask);
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
