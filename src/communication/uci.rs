use super::bestmove;
use crate::communication::generate_board;
use crate::debug::visualize;
use crate::move_generator::masks;
use crate::move_generator::pinmask;
use crate::prelude::*;
use crate::search::iterative_deepening;
use crate::search::transposition_table::TT;
use std::env;
use std::io::{self, BufRead, Write};
use std::process::exit;
use std::time::Duration;

#[allow(clippy::too_many_lines)]
pub fn handle_uci_communication() {
    const START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut board = Board::from_fen(START_POS);

    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg.contains("flamegraph")) {
        let best_move = iterative_deepening::iterative_deepening(
            &mut board,
            100,
            Duration::from_millis(1000),
            false,
            false,
        );
        if let Some(mv) = best_move {
            println!("info pv {}", mv.decode().to_coords());
            println!("bestmove {}", mv.decode().to_coords());
        }
        exit(0);
    }

    for line_res in stdin.lock().lines() {
        let Ok(line) = line_res else { break };

        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("help") => {
                println!("{HELP_TEXT}");
            }
            Some("uci") => {
                println!("id name Thunfisch");
                println!("id author Lukas Piorek (Kaynoux), Emil Schläger (heofthetea)");
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
                bestmove::bestmove(&args, &mut board);
            }
            Some("fen") => println!("Current Fen: {}", board.generate_fen()),
            Some("draw") => visualize::print_board(&board, None),
            Some("moves") => {
                let moves = board.generate_all_moves();
                visualize::print_board(&board, Some(&moves));
            }
            Some("eval") => println!("Depth 0 Board Evaluation: {}", board.evaluate()),
            Some("do") => {
                let args: Vec<&str> = parts.collect();
                let mv_str: &str = args[0];
                let mv = DecodedMove::from_coords(mv_str, &board);
                board.make_move(mv.encode());
            }
            Some("islegal") => {
                let args: Vec<&str> = parts.collect();
                let mv_str: &str = args[0];
                let mv = DecodedMove::from_coords(mv_str, &board);
                if board.is_legal(&mv) {
                    println!("yes");
                } else {
                    println!("no");
                }
            }
            Some("pinmask") => {
                let (hv_pinmask, diag_pinmask) = pinmask::generate_pin_masks(&board);
                println!("{:?}", hv_pinmask | diag_pinmask);
            }
            Some("checkmask") => {
                let (check_mask, check_counter) = masks::calc_check_mask(&board);

                println!("Check Counter: {check_counter}");
                println!("{check_mask:?}");
            }
            Some("attackmask") => {
                let attackmask = masks::calculate_attackmask(
                    &board,
                    board.occupied(),
                    !board.current_color(),
                    None,
                );
                println!("{attackmask:?}");
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
            Some("piece") => {
                if let Some(&args) = parts.take(1).collect::<Vec<&str>>().first() {
                    let mut parse_error_occurred = false;

                    let bitboard = args
                        .chars()
                        .map(|c| c.to_ascii_lowercase())
                        .map_while(|c| {
                            Piece::from_lowercase_char(c)
                                .inspect_err(|()| {
                                    println!("invalid character: {c}");
                                    parse_error_occurred = true;
                                })
                                .ok()
                        })
                        .map(|piece| {
                            board.figure_bb(Color::White, piece)
                                ^ board.figure_bb(Color::Black, piece)
                        })
                        .reduce(|a, b| a | b);

                    if !parse_error_occurred && let Some(piece_bb) = bitboard {
                        println!("{piece_bb:?}");
                    }
                }
            }
            Some("hash") => {
                println!("{:?}", board.hash());
            }
            Some("hashtest") => {
                println!("Incremental Hash: {:#x} {:?}", board.hash(), board.hash());
                println!(
                    "Hash from Scratch: {:#x} {:?}",
                    board.generate_hash(),
                    board.generate_hash()
                );
            }

            Some("tt") => {
                let args: Vec<&str> = parts.collect();
                match TT.handle_debug(&args, board.hash()) {
                    Err(e) => eprintln!("{e}"),
                    Ok(v) => println!("{v}"),
                }
            }
            Some("quit") => break,
            Some(cmd) => {
                eprintln!("Unknown command: {cmd}");
            }
            None => {}
        }

        stdout.flush().unwrap();
    }
}

const HELP_TEXT: &str = r"Commands:
  uci                - Identify engine and author
  isready            - Engine readiness check
  ucinewgame         - Start new game (resets engine state)
  position [options] - Set up position (see below)
  go [parameters]    - Start search (see below)
  quit               - Exit engine
  fen                - Print current FEN

position options
  startpos           - Set up the standard chess starting position
  fen <FEN>          - Set up a position from a FEN string
  moves <m1> <m2>    - Play moves from the given position

go parameters:
  depth <n>          - Search to fixed depth n (plies)
  wtime <ms>         - White time left (ms)
  btime <ms>         - Black time left (ms)
  winc <ms>          - White increment per move (ms)
  binc <ms>          - Black increment per move (ms)
  movestogo <n>      - Moves to next time control
  movetime <ms>      - Search exactly this many ms
  fixtime <ms>.      - Harcoded searchtime

perft commands:
  go perft <depth> [--debug|--perftree|--rayon]
    --debug     - Print debug info for perft
    --perftree  - Print perft formatted for perftree
    --rayon     - Use rayon for parallel perft

Examples:
  position startpos moves e2e4 e7e5
  go depth 6
  go wtime 60000 btime 60000 winc 0 binc 0
  go perft 7 --rayon

Debugging:
  draw               - Print board
  moves              - Print legal moves
  eval               - Prints current Evaluation with Depth of 0
  do <move>          - Play move (e.g. do e2e4)
  islegal <move>     - Check whether move is legal (e.g. islegal e2e4)
  pinmask            - Show pin masks
  checkmask          - Show check mask
  attackmask         - Show attack mask
  empty              - Empty Squares Bitboard
  white              - White Squares Bitboard
  black              - Black Squares Bitboard
  occupied           - Occupied Squares Bitboard
  piece [pbnrqk]     - All squares occupied by the specified pieces
  hash               - Current board hash
  tt <parameters>    - Perform actions with the Transposition table";
