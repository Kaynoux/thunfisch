use thunfisch::prelude::*;

#[test]
fn test_en_passant_execution() {
    let mut board = Board::from_fen("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2");

    let ep_move = DecodedMove {
        from: Square::from_coords("e5").unwrap(),
        to: Square::from_coords("d6").unwrap(),
        mv_type: MoveType::EpCapture,
    };
    board.make_move(&ep_move);

    assert_eq!(
        board.figures(Square::from_coords("d6").unwrap()),
        Figure::WhitePawn,
        "EP Test: White pawn should be on d6"
    );
    assert_eq!(
        board.figures(Square::from_coords("d5").unwrap()),
        Figure::Empty,
        "EP Test: Black captured pawn on d5 should be empty"
    );
    assert_eq!(
        board.figures(Square::from_coords("e5").unwrap()),
        Figure::Empty,
        "EP Test: White pawn's original square e5 should be empty"
    );
    assert_eq!(
        board.current_color(),
        Black,
        "EP Test: Color to move should be Black"
    );
    assert_eq!(
        board.ep_target(),
        None,
        "EP Test: En passant target should be cleared"
    );
}
