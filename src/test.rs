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
        const MAX_DEPTH: usize = 6;

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
                let mut board = Board::from_fen(fen);
                let calculated_node_count = r_perft_rayon(&mut board, depth);
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

    // #[test]
    // fn test_fen_parsing_specifics() {
    //     // FEN with specific castling rights, en passant target, and side to move
    //     let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq e3 0 2";
    //     let board = Board::from_fen(fen);

    //     assert_eq!(
    //         board.current_color,
    //         Color::Black,
    //         "FEN Parsing: Current color should be Black"
    //     );
    //     assert!(
    //         board.white_king_castle,
    //         "FEN Parsing: White kingside castling should be possible"
    //     );
    //     assert!(
    //         board.white_queen_castle,
    //         "FEN Parsing: White queenside castling should be possible"
    //     );
    //     assert!(
    //         board.black_king_castle,
    //         "FEN Parsing: Black kingside castling should be possible"
    //     );
    //     assert!(
    //         board.black_queen_castle,
    //         "FEN Parsing: Black queenside castling should be possible"
    //     );
    //     assert_eq!(
    //         board.ep_target,
    //         Some(Square::from_coords("e3").unwrap().to_bit()),
    //         "FEN Parsing: En passant target should be e3"
    //     );
    //     assert_eq!(
    //         board.halfmove_clock, 0,
    //         "FEN Parsing: Halfmove clock should be 0"
    //     );
    //     assert_eq!(
    //         board.total_halfmove_counter, 4,
    //         "FEN Parsing: Fullmove number should be 2 (means 4 halfmoves if starting from 0)"
    //     ); // Assuming total_halfmove_counter is ply
    //     assert_eq!(
    //         board.pieces[Square::from_coords("e4").unwrap().0],
    //         Figure::WhitePawn,
    //         "FEN Parsing: Piece at e4 should be White Pawn"
    //     );
    //     assert_eq!(
    //         board.pieces[Square::from_coords("c5").unwrap().0],
    //         Figure::BlackPawn,
    //         "FEN Parsing: Piece at c5 should be Black Pawn"
    //     );
    //     assert_eq!(
    //         board.pieces[Square::from_coords("f3").unwrap().0],
    //         Figure::WhiteKnight,
    //         "FEN Parsing: Piece at f3 should be White Knight"
    //     );
    // }

    #[test]
    fn test_checkmate_detection() {
        // Fool's mate
        let mut board =
            Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/6P1/5P2/PPPPP2P/RNBQKBNR b KQkq - 0 2");
        // Black moves Qh4#
        let mv = DecodedMove::from_coords("d8h4".to_string(), &board);
        board.make_move(&mv);

        assert!(
            board.is_in_check(),
            "Checkmate Test: White should be in check"
        );
        let white_moves = board.generate_moves(false);
        assert!(
            white_moves.is_empty(),
            "Checkmate Test: White should have no legal moves"
        );
        // Add a specific is_checkmate() if available, otherwise the two asserts above are a good proxy
    }

    // #[test]
    // fn test_stalemate_detection() {
    //     // King in the corner, queen prevents all moves
    //     let mut board = Board::from_fen("k7/8/8/8/8/8/5Q2/K7 b - - 0 1");
    //     // It's Black's turn, Black is not in check, but has no legal moves.
    //     assert!(
    //         !board.is_in_check(),
    //         "Stalemate Test: Black should not be in check"
    //     );
    //     let black_moves = board.generate_moves(false);
    //     assert!(
    //         black_moves.is_empty(),
    //         "Stalemate Test: Black should have no legal moves, resulting in stalemate"
    //     );
    //     // Add a specific is_stalemate() if available
    // }

    #[test]
    fn test_en_passant_execution() {
        let mut board =
            Board::from_fen("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2");
        // White pawn at e5, Black pawn just moved d7-d5, ep target is d6.
        // White captures en passant e5xd6
        let ep_move = DecodedMove {
            from: Square::from_coords("e5").unwrap(),
            to: Square::from_coords("d6").unwrap(),
            mv_type: MoveType::EpCapture,
        };
        board.make_move(&ep_move);

        assert_eq!(
            board.pieces[Square::from_coords("d6").unwrap().0],
            Figure::WhitePawn,
            "EP Test: White pawn should be on d6"
        );
        assert_eq!(
            board.pieces[Square::from_coords("d5").unwrap().0],
            Figure::Empty,
            "EP Test: Black captured pawn on d5 should be empty"
        );
        assert_eq!(
            board.pieces[Square::from_coords("e5").unwrap().0],
            Figure::Empty,
            "EP Test: White pawn's original square e5 should be empty"
        );
        assert_eq!(
            board.current_color,
            Color::Black,
            "EP Test: Color to move should be Black"
        );
        assert_eq!(
            board.ep_target, None,
            "EP Test: En passant target should be cleared"
        );
    }
}
