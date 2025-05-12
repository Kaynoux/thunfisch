use thunfisch::{debug::perft::r_perft_rayon, prelude::*};

#[test]
fn test_move_generation() {
    let fens = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", // Initial Pos
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ", // Pos 2
        "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1 ",               // Pos 3
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", // Pos 4
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8 ", // Pos 5
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ", // Pos 6
    ];
    let perft_results: [[usize; 5]; 6] = [
        [20, 400, 8_902, 197_281, 4_865_609],        // Initial Pos
        [48, 2_039, 97_862, 4_085_603, 193_690_690], // Pos 2
        [14, 191, 2_812, 43_238, 674_624],           // Pos 3
        [6, 264, 9_467, 422_333, 15_833_292],        // Pos 4
        [44, 1_486, 62_379, 2_103_487, 89_941_194],  // Pos 5
        [46, 2_079, 89_890, 3_894_594, 164_075_551], // Pos 6
    ];

    for (fen_idx, fen) in fens.iter().enumerate() {
        for (depth_idx, correct_node_count) in perft_results[fen_idx].iter().enumerate() {
            let mut board = Board::from_fen(fen);
            let calculated_node_count = r_perft_rayon(&mut board, depth_idx + 1);
            assert_eq!(
                *correct_node_count,
                calculated_node_count,
                "Testing node count Fen: {} Depth: {}",
                fen_idx + 1,
                depth_idx + 1
            );
        }
    }
}
