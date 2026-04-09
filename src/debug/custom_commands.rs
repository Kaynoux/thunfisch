use crate::{
    communication::handle_go,
    debug::{perft, visualize},
    move_generator::{masks, pinmask},
    prelude::*,
    transposition_table::TT,
};

pub fn handle_custom_commands(board: &mut Board, command: &str, args: &[&str]) {
    match command {
        // Custom commands: Mostly used for debugging
        "perft" => {
            perft(board, args);
        }
        "search" => {
            let help = args.contains(&"--help");
            handle_go(board, args, true, help);
        }
        "fen" => println!("{}", board.fen()),
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
    let depth = if args.is_empty() {
        0
    } else {
        args[0].parse().unwrap_or_default()
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
