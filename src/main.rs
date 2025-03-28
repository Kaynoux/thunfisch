mod communication;
mod debug;
mod move_generation;
mod types;
mod utils;
use crate::communication::parse_fen;
use crate::debug::{print_all_legal_moves, print_board_as_board};
use crate::types::Field;

fn main() {
    let start_pos = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", // default
        "r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1", // random
        "rnbqkbnr/p1pppppp/8/8/8/1p6/PPPPPPPP/RNBQKBNR", // test pawn
        "rnbq1bnr/pppppppp/8/8/8/3k4/PPPPPPPP/RNBQKBNR", // test king
        "rnbqkbnr/p4ppp/8/2N5/8/8/PPPPPPPP/RNBQKB1R",  // test knight
    ];
    let mut board: [[Field; 8]; 8] = parse_fen(start_pos[4]);
    print_board_as_board(board);
    print_all_legal_moves(board);
}
