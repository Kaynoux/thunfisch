use crate::{
    communication::handle_go, debug::{perft, visualize}, evaluation::GAMEPHASE_INC, move_generator::{masks, pinmask}, move_picker::MoveList, move_scoring::{mvv_lva, score_quiets}, prelude::*, settings, transposition_table::TT
};

#[allow(clippy::too_many_lines)]
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
        "settings" => {
            println!("{}", settings::repr());
        }
        "fen" => println!("{}", board.fen()),
        "draw" => visualize::print_board(board, None),
        "moves" => {
            let moves = board.generate_all_moves();
            visualize::print_board(board, Some(&moves));
        }
        "score" => {
            let mut quiets = MoveList::new();
            board.generate_moves::<true>(&mut quiets);
            score_quiets(&mut quiets, board);

            let mut captures = MoveList::new();
            board.generate_moves::<false>(&mut captures);
            mvv_lva(&mut captures, board);
            println!("Quiets: {quiets:?}");
            println!("Captures: {captures:?}");
        }
        "eval" => {
            println!("Depth 0 Board Evaluation: {}\n", board.evaluate());
            let doubled_pawns = board.doubled_pawn_penalties();
            println!(
                "Doubled Pawn offset: {} - {} = {}",
                doubled_pawns[0],
                doubled_pawns[1],
                doubled_pawns[0] - doubled_pawns[1]
            );

            let (mg_pawn, eg_pawn) = board.pawn_structure();

            let mut phase = 32;
            #[allow(
                clippy::needless_range_loop,
                clippy::cast_possible_truncation,
                clippy::cast_possible_wrap
            )]
            for i in 0..=11 {
                let mut bb = board.figure_bb_by_index(i);
                let count = bb.iter_mut().count() as i32;
                phase -= count * GAMEPHASE_INC[i];
            }
            let gamephase = (phase * 256 + 16) / 32;

            let white_blended = (i32::from(mg_pawn[0]) * (256 - gamephase)
                + i32::from(eg_pawn[0]) * gamephase)
                >> 8;
            let black_blended = (i32::from(mg_pawn[1]) * (256 - gamephase)
                + i32::from(eg_pawn[1]) * gamephase)
                >> 8;

            println!(
                "Passed Pawn eval:\n  White: MG {}, EG {} -> Blended: {}\n  Black: MG {}, EG {} -> Blended: {}",
                mg_pawn[0], eg_pawn[0], white_blended, mg_pawn[1], eg_pawn[1], black_blended
            );
        }
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
        "openfiles" => {
            println!("{:?}", board.open_files());
        }
        "passedmask" => {
            let square_str: &str = args[0];
            let color_str: &str = args[1];
            let bit = Bit::from_coords(square_str).unwrap();
            let color = match color_str {
                "white" | "w" => Color::White,
                "black" | "b" => Color::Black,
                _ => panic!("illegal color"),
            };
            let passed_pawn_mask = Bitboard::passed_pawn_mask(bit, color);
            println!("{passed_pawn_mask:?}");
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
