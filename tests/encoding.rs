use thunfisch::prelude::*;

#[test]
fn test_move_encoding_decoding() {
    let moves = [
        "e5f7", "e5d7", "e5g6", "e5c6", "e5g4", "e5c4", "e5d3", "f3f6", "f3h5", "f3f5", "f3g4",
        "f3f4", "f3h3", "f3g3", "f3e3", "f3d3", "f3g2", "c3d5", "c3b5", "c3a4", "c3d1", "c3b1",
        "e2a6", "e2b5", "e2c4", "e2d3", "e2f1", "e2d1", "d2h6", "d2g5", "d2f4", "d2e3", "d2c1",
        "h1g1", "h1f1", "e1d1", "a1d1", "a1c1", "a1b1", "e1c1", "c7c8q", "c7c8r", "c7c8b", "c7c8n",
        "h2h3", "b2b3", "a2a3", "h2h4", "a2a4",
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
