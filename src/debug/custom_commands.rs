use crate::{
    communication::handle_go,
    debug::{perft, visualize},
    evaluation::{GAMEPHASE_INC, MOBILITY_COEFFICIENTS},
    move_generator::{
        masks::{self, king_safety_mask},
        pinmask,
    },
    move_picker::MoveList,
    move_scoring::{mvv_lva, score_quiets},
    prelude::*,
    settings,
    transposition_table::TT,
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
            #[cfg(debug_assertions)]
            print_debug_eval_info(board);
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
            if args.len() >= 2 {
                // Parse color and piece from arguments
                let color_str = args[0].to_lowercase();
                let piece_char = args[1].to_lowercase().chars().next().unwrap_or(' ');

                let color = match color_str.as_str() {
                    "white" | "w" => Color::White,
                    "black" | "b" => Color::Black,
                    _ => {
                        println!("info invalid color: {}", args[0]);
                        return;
                    }
                };

                let Some(piece) = Piece::from_char(piece_char) else {
                    println!("info invalid piece: {}", args[1]);
                    return;
                };

                let figure_idx = piece as usize * 2 + color as usize;
                let attackmask = masks::calculate_attackmask_by_figure(
                    board,
                    board.occupied(),
                    figure_idx,
                    None,
                );
                println!("{attackmask:?}");
            } else {
                // Default behavior: use all attacking pieces of the opponent
                let attackmask = masks::calculate_attackmask(
                    board,
                    board.occupied(),
                    !board.current_color(),
                    None,
                );
                println!("{attackmask:?}");
            }
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
        "kingmask" => {
            let color_str = args[0];
            let color = match color_str {
                "white" | "w" => Color::White,
                "black" | "b" => Color::Black,
                _ => panic!("illegal color"),
            };
            println!(
                "{:?}",
                king_safety_mask(board, color) ^ board.figure_bb(color, Piece::King)
            );
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

/// Yes this is ugly af
/// I'm inlining a lot of stuff in the eval for caching optimizations and better readability there
/// so we just accept that this is a lot of duplicated code and kinda ugly, this is only part of the debug utils anyway after all'
/// WARNING: THIS IS EXCLUSIVELY VIBE-CODED SLOP, FOR YOUR OWN SANITY DO NOT ATTEMPT TO WORK ON THIS WITHOUT AI
#[allow(
    clippy::needless_range_loop,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::similar_names
)]
fn print_debug_eval_info(board: &Board) {
    if settings::DOUBLED_PAWNS {
        // Doubled pawns
        let doubled_pawns = board.doubled_pawn_penalties();
        println!(
            "Doubled Pawn offset: {} - {} = {}",
            doubled_pawns[0],
            doubled_pawns[1],
            doubled_pawns[0] - doubled_pawns[1]
        );
    }

    // Pawn structure
    let (mg_pawn, eg_pawn) = board.pawn_structure();

    // Calculate game phase
    let mut phase = 32;

    for i in 0..=11 {
        let mut bb = board.figure_bb_by_index(i);
        let count = bb.iter_mut().count() as i32;
        phase -= count * GAMEPHASE_INC[i];
    }
    let gamephase = (phase * 256 + 16) / 32;

    let white_blended =
        (i32::from(mg_pawn[0]) * (256 - gamephase) + i32::from(eg_pawn[0]) * gamephase) >> 8;
    let black_blended =
        (i32::from(mg_pawn[1]) * (256 - gamephase) + i32::from(eg_pawn[1]) * gamephase) >> 8;

    if settings::PASSED_PAWNS || settings::ISOLATED_PAWNS {
        println!(
            "Pawn Structure (passed, isolated):\n  White: MG {}, EG {}\n  Black: MG {}, EG {}",
            mg_pawn[0], eg_pawn[0], mg_pawn[1], eg_pawn[1]
        );
        println!("Blended Passed Pawn eval:\n  White: {white_blended}\n  Black: {black_blended}",);
    }

    // Mobility evaluation
    let mut mg_mobility = [0i32; 2];
    let mut eg_mobility = [0i32; 2];
    let mut figure_movements = [Bitboard::EMPTY; 12];
    for i in 0..=11 {
        let mobility = board.calculate_piece_mobility(i, &mut figure_movements);
        // println!("mobility {:?}, {mobility}", Figure::from_idx(i));
        mg_mobility[i & 1] += MOBILITY_COEFFICIENTS[0][i >> 1] * mobility;
        eg_mobility[i & 1] += MOBILITY_COEFFICIENTS[1][i >> 1] * mobility;
    }

    if settings::MOBILITY {
        println!(
            "Mobility scores:\n  White: MG {}, EG {}\n  Black: MG {}, EG {}",
            mg_mobility[0], eg_mobility[0], mg_mobility[1], eg_mobility[1]
        );

        let mg_mobility_diff = mg_mobility[0] - mg_mobility[1];
        let eg_mobility_diff = eg_mobility[0] - eg_mobility[1];
        let blended_mobility =
            (mg_mobility_diff * (256 - gamephase) + eg_mobility_diff * gamephase) >> 8;

        println!(
            "Mobility blended (White - Black): MG {mg_mobility_diff}, EG {eg_mobility_diff} -> Blended: {blended_mobility}",
        );
    }

    if settings::KING_SAFETY {
        // King safety evaluation
        for i in 0..=11 {
            if figure_movements[i].is_empty() {
                // Ensure movement masks are populated if they were not filled by mobility evaluation.
                figure_movements[i] =
                    masks::calculate_attackmask_by_figure(board, board.occupied(), i, None);
            }
            figure_movements[i] |= board.figure_bb_by_index(i);
        }

        let (mg_king_safety, eg_king_safety) = board.king_safety(&figure_movements);
        println!(
            "King Safety table scores:\n  White: MG {}, EG {}\n  Black: MG {}, EG {}",
            mg_king_safety[0], eg_king_safety[0], mg_king_safety[1], eg_king_safety[1]
        );

        let mg_king_safety_diff = mg_king_safety[0] - mg_king_safety[1];
        let eg_king_safety_diff = eg_king_safety[0] - eg_king_safety[1];
        let blended_king_safety = (i32::from(mg_king_safety_diff) * (256 - gamephase)
            + i32::from(eg_king_safety_diff) * gamephase)
            >> 8;

        println!(
            "King Safety blended (White - Black): MG {mg_king_safety_diff}, EG {eg_king_safety_diff} -> Blended: {blended_king_safety}",
        );
    }
}
