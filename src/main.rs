mod communication;
mod debug;
mod move_generation;
mod types;
mod utils;
use std::ops::Shl;
use types::position;

use crate::communication::parse_fen;
use crate::debug::{print_all_legal_moves, print_board_as_board};

fn main() {
    let start_pos = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", // default
        "r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1", // random
        "rnbqkbnr/p1pppppp/8/8/8/1p6/PPPPPPPP/RNBQKBNR", // test pawn
        "rnbq1bnr/pppppppp/8/8/8/3k4/PPPPPPPP/RNBQKBNR", // test king
        "rnbqkbnr/p4ppp/8/2N5/8/8/PPPPPPPP/RNBQKB1R",  // test knight
        "8/8/8/8/3q4/8/8/8",                           // one queen middle
    ];
    let mut board: Board = Board {
        white_pieces: 0,
        black_pieces: 0,
        empty_pieces: 0,
        white_pawns: 0,
        white_knights: 0,
        white_rooks: 0,
        white_bishops: 0,
        white_queen: 0,
        white_king: 0,
        black_pawns: 0,
        black_knights: 0,
        black_rooks: 0,
        black_bishops: 0,
        black_queen: 0,
        black_king: 0,
    };
    parse_fen(start_pos[5], &mut board);
    print_board_as_board(&board);
    //print_all_legal_moves(&board);
}
