use super::bestmove;
use crate::communication::generate_board;
use crate::debug::visualize;
use crate::move_generator::masks;
use crate::move_generator::pinmask;
use crate::prelude::*;
use std::io::{self, BufRead, Write};

pub fn handle_uci<R: BufRead, W: Write>(reader: R, mut writer: W) -> io::Result<()> {
    const START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board = Board::from_fen(START_POS);

    for line_res in reader.lines() {
        let line = line_res?;

        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("help") => {
                writeln!(writer, "Commands:")?;
                writeln!(writer, "  uci                - Identify engine and author")?;
                writeln!(writer, "  isready            - Engine readiness check")?;
                writeln!(
                    writer,
                    "  ucinewgame         - Start new game (resets engine state)"
                )?;
                writeln!(writer, "  position [options] - Set up position (see below")?;
                writeln!(writer, "  go [parameters]    - Start search (see below")?;
                writeln!(writer, "  quit               - Exit engine")?;
                writeln!(writer, "  fen                - Print current FEN")?;
                writeln!(writer,)?;

                writeln!(writer, "position options")?;
                writeln!(
                    writer,
                    "  startpos           - Set up the standard chess starting position"
                )?;
                writeln!(
                    writer,
                    "  fen <FEN>          - Set up a position from a FEN string"
                )?;
                writeln!(
                    writer,
                    "  moves <m1> <m2>    - Play moves from the given position"
                )?;
                writeln!(writer,)?;

                writeln!(writer, "go parameters:")?;
                writeln!(
                    writer,
                    "  depth <n>          - Search to fixed depth n (plies)"
                )?;
                writeln!(writer, "  wtime <ms>         - White time left (ms")?;
                writeln!(writer, "  btime <ms>         - Black time left (ms")?;
                writeln!(
                    writer,
                    "  winc <ms>          - White increment per move (ms)"
                )?;
                writeln!(
                    writer,
                    "  binc <ms>          - Black increment per move (ms)"
                )?;
                writeln!(writer, "  movestogo <n>      - Moves to next time control")?;
                writeln!(writer, "  movetime <ms>      - Search exactly this many ms")?;
                writeln!(writer,)?;

                writeln!(writer, "perft commands:")?;
                writeln!(writer, "  go perft <depth> [--debug|--perftree|--rayon]")?;
                writeln!(writer, "    --debug     - Print debug info for perft")?;
                writeln!(
                    writer,
                    "    --perftree  - Print perft formatted for perftree"
                )?;
                writeln!(writer, "    --rayon     - Use rayon for parallel perft")?;
                writeln!(writer,)?;

                writeln!(writer, "Examples:")?;
                writeln!(writer, "  position startpos moves e2e4 e7e5")?;
                writeln!(writer, "  go depth 6")?;
                writeln!(writer, "  go wtime 60000 btime 60000 winc 0 binc 0")?;
                writeln!(writer, "  go perft 7 --rayon")?;
                writeln!(writer,)?;

                writeln!(writer, "Debugging:")?;
                writeln!(writer, "  draw               - Print board")?;
                writeln!(writer, "  moves              - Print legal moves")?;
                writeln!(
                    writer,
                    "  eval               - Prints current Evaluation with Depth of 0"
                )?;
                writeln!(writer, "  do <move>          - Play move (e.g. do e2e4")?;
                writeln!(writer, "  pinmask            - Show pin masks")?;
                writeln!(writer, "  checkmask          - Show check mask")?;
                writeln!(writer, "  attackmask         - Show attack mask")?;
                writeln!(writer, "  empty              - Empty Squares Bitboard")?;
                writeln!(writer, "  white              - White Squares Bitboard")?;
                writeln!(writer, "  black              - Black Squares Bitboard")?;
                writeln!(writer, "  occupied           - Occupied Squares Bitboard")?;
                writeln!(writer, "  hash               - Current board hash")?;
            }
            Some("uci") => {
                writeln!(writer, "id name Thunfisch")?;
                writeln!(writer, "id author Lukas Piorek")?;
                writeln!(writer, "uciok")?;
            }
            Some("isready") => {
                writeln!(writer, "readyok")?;
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
            Some("fen") => {
                writeln!(writer, "Current Fen: {}", board.generate_fen())?;
            }
            Some("draw") => visualize::print_board(&board, None),
            Some("moves") => {
                let moves = board.generate_moves::<false>();
                visualize::print_board(&board, Some(&moves));
            }
            Some("eval") => writeln!(writer, "Depth 0 Board Evaluation: {}", board.evaluate())?,
            Some("do") => {
                let args: Vec<&str> = parts.collect();
                let mv_str: &str = args[0];
                let mv = DecodedMove::from_coords(mv_str.to_string(), &board);
                board.make_move(&mv);
            }
            Some("pinmask") => {
                let (hv_pinmask, diag_pinmask) = pinmask::generate_pin_masks(&board);
                writeln!(writer, "{:?}", hv_pinmask | diag_pinmask)?;
            }
            Some("checkmask") => {
                let (check_mask, check_counter) = masks::calc_check_mask(&board);

                writeln!(writer, "Check Counter: {}", check_counter)?;
                writeln!(writer, "{:?}", check_mask)?;
            }
            Some("attackmask") => {
                let attackmask = masks::calculate_attackmask(
                    &board,
                    board.occupied(),
                    !board.current_color(),
                    None,
                );
                writeln!(writer, "{:?}", attackmask)?;
            }
            Some("empty") => {
                writeln!(writer, "{:?}", board.empty())?;
            }
            Some("white") => {
                writeln!(writer, "{:?}", board.color_bbs(White))?;
            }
            Some("black") => {
                writeln!(writer, "{:?}", board.color_bbs(Black))?;
            }
            Some("occupied") => {
                writeln!(writer, "{:?}", board.occupied())?;
            }
            Some("hash") => {
                writeln!(writer, "{:?}", board.hash())?;
            }
            Some("quit") => break,
            Some(cmd) => {
                writeln!(writer, "Unknown command: {}", cmd)?;
            }
            None => {}
        }

        writer.flush()?;
    }
    Ok(())
}
