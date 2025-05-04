#[cfg(test)]
mod tests {
    use crate::{
        debug::{r_perft, r_perft_rayon},
        prelude::*,
    };

    #[test]
    fn test_move_encoding_decoding() {
        let moves = [
            "e5f7", "e5d7", "e5g6", "e5c6", "e5g4", "e5c4", "e5d3", "f3f6", "f3h5", "f3f5", "f3g4",
            "f3f4", "f3h3", "f3g3", "f3e3", "f3d3", "f3g2", "c3d5", "c3b5", "c3a4", "c3d1", "c3b1",
            "e2a6", "e2b5", "e2c4", "e2d3", "e2f1", "e2d1", "d2h6", "d2g5", "d2f4", "d2e3", "d2c1",
            "h1g1", "h1f1", "e1d1", "a1d1", "a1c1", "a1b1", "e1c1", "c7c8q", "c7c8r", "c7c8b",
            "c7c8n", "h2h3", "b2b3", "a2a3", "h2h4", "a2a4",
        ];

        let fen = "r3k2r/p1Ppqpb1/bn2pnp1/4N3/1p2P3/2N2Q2/PPPBBPpP/R3K2R w KQkq - 0 1";

        let board = Board::from_fen(fen);

        for mv_ref in moves.iter() {
            let mv = *mv_ref;
            let decoded = DecodedMove::from_coords(mv.to_string(), &board);
            assert_eq!(mv, decoded.to_coords(), "Str -> Decoded -> Str");

            let encoded = decoded.encode();
            let decoded2 = encoded.decode();
            assert_eq!(
                mv,
                decoded2.to_coords(),
                "Str -> Decoded -> Encoded -> Decoded -> Str"
            );
        }
    }

    #[test]
    fn test_legal_move_generation() {
        // adjust to limit the depth
        const MAX_DEPTH: usize = 5;

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
                let depth = depth_idx + 1;
                if depth >= MAX_DEPTH {
                    break;
                }
                let board = Board::from_fen(fen);
                let calculated_node_count = r_perft_rayon(&board, depth);
                assert_eq!(
                    *correct_node_count,
                    calculated_node_count,
                    "Testing node count Fen: {} Depth: {}",
                    fen_idx + 1,
                    depth
                );
            }
        }

        // let perft_results = [[20, 400, 8902, 197281, 4865609], []]
    }
}
