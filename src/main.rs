mod communication;
mod debug;
mod position_generation;
mod prelude;
mod pseudo_legal_move_generation;
mod types;
use crate::prelude::*;

fn main() {
    let start_pos = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", // 0. defautl
        "rnbqkbnr/pppp2pp/8/3PpP2/8/7P/PPP3P1/RNBQKBNR w KQkq e6 0 2", // 1. test en-passant
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -", // 2. test perft
        "rnbqkbnr/ppp1p1pp/8/8/3pPp2/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1", // 3. en passant black
        "8/2p5/8/8/8/2P5/8/8 b - - 0 1",                            // 4. test double move
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -", // 5. perft pos 2
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

    let moves = board.generate_legal_moves();
    debug::print_moves(&board, &moves);
    debug::print_board(&board, Some(&moves));

    debug::perft_divide(&board, 1);
    debug::perft_divide(&board, 2);
    debug::perft_divide(&board, 3);
    debug::perft_divide(&board, 4);
    debug::perft_divide(&board, 5);

    // println!("Perft Depth 0 : {:?} Nodes", debug::perft(&board, 0));
    // println!("Perft Depth 1 : {:?} Nodes", debug::perft(&board, 1));
    // println!("Perft Depth 2 : {:?} Nodes", debug::perft(&board, 2));
    // println!("Perft Depth 3 : {:?} Nodes", debug::perft(&board, 3));
    // println!("Perft Depth 4 : {:?} Nodes", debug::perft(&board, 4));
    // println!("Perft Depth 5 : {:?} Nodes", debug::perft(&board, 5));
    //println!("Perft Depth 6 : {:?} Nodes", debug::perft(&board, 6));
    //println!("Perft Depth 7 : {:?} Nodes", debug::perft(&board, 6));

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
