use super::bestmove;
use crate::communication::generate_board;
use crate::debug::visualize;
use crate::move_generator::masks;
use crate::move_generator::pinmask;
use crate::prelude::*;
use std::io::{self, BufRead, Write};

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
            Some("help") => {
                println!("Commands:");
                println!("  uci                - Identify engine and author");
                println!("  isready            - Engine readiness check");
                println!("  ucinewgame         - Start new game (resets engine state)");
                println!("  position [options] - Set up position (see below)");
                println!("  go [parameters]    - Start search (see below)");
                println!("  quit               - Exit engine");
                println!("  fen                - Print current FEN");
                println!();

                println!("position options");
                println!("  startpos           - Set up the standard chess starting position");
                println!("  fen <FEN>          - Set up a position from a FEN string");
                println!("  moves <m1> <m2>    - Play moves from the given position");
                println!();

                println!("go parameters:");
                println!("  depth <n>          - Search to fixed depth n (plies)");
                println!("  wtime <ms>         - White time left (ms)");
                println!("  btime <ms>         - Black time left (ms)");
                println!("  winc <ms>          - White increment per move (ms)");
                println!("  binc <ms>          - Black increment per move (ms)");
                println!("  movestogo <n>      - Moves to next time control");
                println!("  movetime <ms>      - Search exactly this many ms");
                println!("  fixtime <ms>.      - Harcoded searchtime");
                println!();

                println!("perft commands:");
                println!("  go perft <depth> [--debug|--perftree|--rayon]");
                println!("    --debug     - Print debug info for perft");
                println!("    --perftree  - Print perft formatted for perftree");
                println!("    --rayon     - Use rayon for parallel perft");
                println!();

                println!("Examples:");
                println!("  position startpos moves e2e4 e7e5");
                println!("  go depth 6");
                println!("  go wtime 60000 btime 60000 winc 0 binc 0");
                println!("  go perft 7 --rayon");
                println!();

                println!("Debugging:");
                println!("  draw               - Print board");
                println!("  moves              - Print legal moves");
                println!("  eval               - Prints current Evaluation with Depth of 0");
                println!("  do <move>          - Play move (e.g. do e2e4)");
                println!("  pinmask            - Show pin masks");
                println!("  checkmask          - Show check mask");
                println!("  attackmask         - Show attack mask");
                println!("  empty              - Empty Squares Bitboard");
                println!("  white              - White Squares Bitboard");
                println!("  black              - Black Squares Bitboard");
                println!("  occupied           - Occupied Squares Bitboard");
                println!("  hash               - Current board hash");
            }
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
                bestmove::bestmove(args, &mut board);
            }
            Some("fen") => println!("Current Fen: {}", board.generate_fen()),
            Some("draw") => visualize::print_board(&board, None),
            Some("moves") => {
                let moves = board.generate_moves::<false>();
                visualize::print_board(&board, Some(&moves));
            }
            Some("eval") => println!("Depth 0 Board Evaluation: {}", board.evaluate()),
            Some("do") => {
                let args: Vec<&str> = parts.collect();
                let mv_str: &str = args[0];
                let mv = DecodedMove::from_coords(mv_str.to_string(), &board);
                board.make_move(&mv);
            }
            Some("pinmask") => {
                let (hv_pinmask, diag_pinmask) = pinmask::generate_pin_masks(&board);
                println!("{:?}", hv_pinmask | diag_pinmask);
            }
            Some("checkmask") => {
                let (check_mask, check_counter) = masks::calc_check_mask(&board);

                println!("Check Counter: {}", check_counter);
                println!("{:?}", check_mask);
            }
            Some("attackmask") => {
                let attackmask = masks::calculate_attackmask(
                    &board,
                    board.occupied(),
                    !board.current_color(),
                    None,
                );
                println!("{:?}", attackmask);
            }
            Some("empty") => {
                println!("{:?}", board.empty());
            }
            Some("white") => {
                println!("{:?}", board.color_bbs(White));
            }
            Some("black") => {
                println!("{:?}", board.color_bbs(Black));
            }
            Some("occupied") => {
                println!("{:?}", board.occupied());
            }
            Some("hash") => {
                println!("{:?}", board.hash());
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
