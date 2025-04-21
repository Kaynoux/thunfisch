mod communication;
mod debug;
mod position_generation;
mod prelude;
mod pseudo_legal_move_generation;
mod types;
use debug::perft;

use crate::prelude::*;

fn main() {
    let start_pos = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", // 0. defautl
        "rnbqkbnr/pppp2pp/8/3PpP2/8/7P/PPP3P1/RNBQKBNR w KQkq e6 0 2", // 1. test en-passant
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -", // 2. test perft
        "rnbqkbnr/ppp1p1pp/8/8/3pPp2/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1", // 3. en passant black
        "8/2p5/8/8/8/2P5/8/8 b - - 0 1",                            // 4. test double move
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -", // 5. perft pos 2
        "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",                // 6: perft pos 3
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", // 7. perft pos 4
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", // 8. perft pos 5
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", // 9. perft 6
    ];

    let mut board = Board::from_fen(start_pos[0]);

    // let mut idx = 0;
    // loop {
    //     idx += 1;
    //     if idx % 1000 == 0 {
    //         println!("{:?}", idx);
    //     }
    //     legal_move_generation::generate_legal_moves(
    //         &board.clone(),
    //         Color::Black,
    //         &mut moves.clone(),
    //     );
    // }
    // for mv in &moves {
    //     let mut bc = board.clone();
    //     bc.make_move(&mv);
    //     debug::print_board(&bc, "Test", None);
    // }

    let moves = board.generate_legal_moves().1;
    debug::print_moves(&board, &moves);
    debug::print_board(&board, Some(&moves));

    // debug::perft_divide(&board, 1);
    // debug::perft_divide(&board, 2);
    // debug::perft_divide(&board, 3);
    // debug::perft_divide(&board, 4);
    // debug::perft_divide(&board, 5);

    for i in 0..10 {
        perft(&board, i);
    }

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
