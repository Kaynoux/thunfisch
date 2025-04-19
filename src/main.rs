mod communication;
mod debug;
mod legal_move_generation;
mod prelude;
mod pseudo_legal_move_generation;
mod types;
use crate::prelude::*;

fn main() {
    let start_pos = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", // default
        "r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1", // random
        "rnbqkbnr/p1pppppp/8/8/8/1p6/PPPPPPPP/RNBQKBNR", // test pawn
        "rnbq1bnr/pppppppp/8/8/8/3k4/PPPPPPPP/RNBQKBNR", // test king
        "rnbqkbnr/p4ppp/8/2N5/8/8/PPPPPPPP/RNBQKB1R",  // test knight
        "8/8/8/8/3q4/8/8/8",                           // one queen middle
        "8/8/8/4p3/8/3N4/8/8",
        "R6R/3Q4/1Q4Q1/4Q3/2Q4Q/Q4Q2/pp1Q4/kBNN1KB1", // 218 moves
        "r3k2r/pppppppp/5bn1/1nbq4/2BQ1BN1/1N6/PPPPPPPP/R3K2R", // test castling all
    ];

    let mut board = Board::new(start_pos[8]);

    let mut moves: Vec<ChessMove> = Vec::new();
    legal_move_generation::generate_legal_moves(&board, Color::Black, &mut moves);
    // for mv in &moves {
    //     let mut bc = board.clone();
    //     bc.make_move(&mv);
    //     debug::print_board(&bc, "Test", None);
    // }
    debug::print_moves(&board, &moves);
    debug::print_board(&board, "Test", None);
    debug::print_board(&board, "Test", Some(&moves[..]));

    // debug::print_board(
    //     &board,
    //     "Test black queen",
    //     move_generation::get_all_moves_for_one_piece_type_unique(
    //         &board,
    //         board.black_queen,
    //         Color::Black,
    //         move_generation::get_queen_moves,
    //     ),
    // );
}
