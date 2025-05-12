use thunfisch::prelude::*;

#[test]
fn test_checkmate() {
    let mut board =
        Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/6P1/5P2/PPPPP2P/RNBQKBNR b KQkq - 0 2");

    let mv = DecodedMove::from_coords("d8h4".to_string(), &board);
    board.make_move(&mv);

    assert!(board.is_in_check(), "White should be in check");
    let white_moves = board.generate_moves(false);
    assert!(white_moves.is_empty(), "White should have no legal moves");
}
