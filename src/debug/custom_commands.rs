use crate::debug::perft;
use crate::debug::visualize;
use crate::move_generator::masks;
use crate::move_generator::pinmask;
use crate::prelude::*;
use crate::transposition_table::TT;

pub fn handle_custom_commands(board: &mut Board, command: &str, args: &[&str]) {
    match command {
        // Custom commands: Mostly used for debugging
        "perft" => {
            perft(board, args);
        }
        "help" => {
            println!("{HELP_TEXT}");
        }
        "fen" => println!("Current Fen: {}", board.fen()),
        "draw" => visualize::print_board(board, None),
        "moves" => {
            let moves = board.generate_all_moves();
            visualize::print_board(board, Some(&moves));
        }
        "eval" => println!("Depth 0 Board Evaluation: {}", board.evaluate()),
        "do" => {
            let mv_str: &str = args[0];
            let mv = DecodedMove::from_coords(mv_str, board);
            board.make_move(mv.encode());
        }
        "islegal" => {
            let mv_str: &str = args[0];
            let mv = DecodedMove::from_coords(mv_str, board);
            if board.is_legal(&mv) {
                println!("yes");
            } else {
                println!("no");
            }
        }
        "pinmask" => {
            let (hv_pinmask, diag_pinmask) = pinmask::generate_pin_masks(board);
            println!("{:?}", hv_pinmask | diag_pinmask);
        }
        "checkmask" => {
            let (check_mask, check_counter) = masks::calc_check_mask(board);

            println!("Check Counter: {check_counter}");
            println!("{check_mask:?}");
        }
        "attackmask" => {
            let attackmask =
                masks::calculate_attackmask(board, board.occupied(), !board.current_color(), None);
            println!("{attackmask:?}");
        }
        "empty" => {
            println!("{:?}", board.empty());
        }
        "white" => {
            println!("{:?}", board.color_bbs(White));
        }
        "black" => {
            println!("{:?}", board.color_bbs(Black));
        }
        "occupied" => {
            println!("{:?}", board.occupied());
        }
        "hash" => {
            println!("{:?}", board.hash());
        }
        "hashtest" => {
            println!("Incremental Hash: {:#x} {:?}", board.hash(), board.hash());
            println!(
                "Hash from Scratch: {:#x} {:?}",
                board.generate_hash(),
                board.generate_hash()
            );
        }

        "tt" => match TT.handle_debug(args, board.hash()) {
            Err(e) => eprintln!("{e}"),
            Ok(v) => println!("{v}"),
        },
        cmd => println!("info unknown command {cmd}"),
    }
}

fn perft(board: &mut Board, args: &[&str]) {
    let perftree = args.contains(&"--perftree");
    let rayon = args.contains(&"--rayon");
    let debug = args.contains(&"--debug");

    // Fixed Depth
    let depth = if args.len() >= 2 {
        args[1].parse().unwrap_or_default()
    } else {
        0
    };

    if debug {
        perft::perft_debug(board, depth);
    } else if rayon {
        perft::perft_rayon(board, depth);
    } else if perftree {
        perft::perft_perftree_format(board, depth);
    } else {
        perft::perft(board, depth);
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
