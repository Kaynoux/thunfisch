mod communication;
mod debug;
mod move_generation;
mod prelude;
mod types;
mod utils;
use crate::prelude::*;

fn main() {
    let start_pos = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", // default
        "r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1", // random
        "rnbqkbnr/p1pppppp/8/8/8/1p6/PPPPPPPP/RNBQKBNR", // test pawn
        "rnbq1bnr/pppppppp/8/8/8/3k4/PPPPPPPP/RNBQKBNR", // test king
        "rnbqkbnr/p4ppp/8/2N5/8/8/PPPPPPPP/RNBQKB1R",  // test knight
        "8/8/8/8/3q4/8/8/8",                           // one queen middle
    ];

    let mut board = Board::new(start_pos[5]);
    debug::print_board(
        &board,
        "Test black queen",
        move_generation::get_all_moves_for_one_piece_type_unique(
            &board,
            board.black_queen,
            Color::Black,
            move_generation::get_queen_moves,
        ),
    );
    //print_all_legal_moves(&board);
}
