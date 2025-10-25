use crate::prelude::*;

impl Board {
    /// Executes a given move on the board
    /// https://www.chessprogramming.org/Make_Move
    pub fn make_move(&mut self, mv: &DecodedMove) {
        self.push_repetition_stack();
        let mv_type = mv.mv_type;
        let friendly = self.current_color();
        let from = mv.from;
        let to = mv.to;

        let from_figure = self.figures(from);
        let to_figure = self.figures(to);

        // Unmake Info if move needs to be undone
        self.push_unmake_info_stack(mv.encode(), to_figure);

        self.update_ep(friendly, mv);
        self.update_castling(friendly, from_figure.piece(), mv, to_figure.piece());
        self.toggle_current_color();
        match mv_type {
            MoveType::Quiet | MoveType::DoubleMove => {
                self.toggle(friendly, from_figure, from);
                self.toggle(friendly, from_figure, to);
            }
            MoveType::Capture => {
                self.toggle(!friendly, to_figure, to);
                self.toggle(friendly, from_figure, from);
                self.toggle(friendly, from_figure, to);
            }
            MoveType::KnightPromo => {
                self.toggle(friendly, from_figure, from);
                self.toggle(friendly, Knight.to_color_piece(friendly), to);
            }
            MoveType::BishopPromo => {
                self.toggle(friendly, from_figure, from);
                self.toggle(friendly, Bishop.to_color_piece(friendly), to);
            }
            MoveType::RookPromo => {
                self.toggle(friendly, from_figure, from);
                self.toggle(friendly, Rook.to_color_piece(friendly), to);
            }
            MoveType::QueenPromo => {
                self.toggle(friendly, from_figure, from);
                self.toggle(friendly, Queen.to_color_piece(friendly), to);
            }
            MoveType::KnightPromoCapture => {
                self.toggle(!friendly, to_figure, to);
                self.toggle(friendly, from_figure, from);
                self.toggle(friendly, Knight.to_color_piece(friendly), to);
            }
            MoveType::BishopPromoCapture => {
                self.toggle(!friendly, to_figure, to);
                self.toggle(friendly, from_figure, from);
                self.toggle(friendly, Bishop.to_color_piece(friendly), to);
            }
            MoveType::RookPromoCapture => {
                self.toggle(!friendly, to_figure, to);
                self.toggle(friendly, from_figure, from);
                self.toggle(friendly, Rook.to_color_piece(friendly), to);
            }
            MoveType::QueenPromoCapture => {
                self.toggle(!friendly, to_figure, to);
                self.toggle(friendly, from_figure, from);
                self.toggle(friendly, Queen.to_color_piece(friendly), to);
            }
            MoveType::EpCapture => {
                let ep_to = match friendly {
                    White => Square(to.0 - 8),
                    Black => Square(to.0 + 8),
                };
                self.toggle(!friendly, Piece::Pawn.to_color_piece(!friendly), ep_to);
                self.toggle(friendly, from_figure, from);
                self.toggle(friendly, from_figure, to);
            }
            MoveType::QueenCastle => {
                let (from_rook, to_rook) = match friendly {
                    White => (Square(0), Square(3)),
                    Black => (Square(56), Square(59)),
                };

                self.toggle(friendly, Rook.to_color_piece(friendly), from_rook);
                self.toggle(friendly, Rook.to_color_piece(friendly), to_rook);
                self.toggle(friendly, from_figure, from);
                self.toggle(friendly, from_figure, to);
            }
            MoveType::KingCastle => {
                let (from_rook, to_rook) = match friendly {
                    White => (Square(7), Square(5)),
                    Black => (Square(63), Square(61)),
                };

                self.toggle(friendly, Rook.to_color_piece(friendly), from_rook);
                self.toggle(friendly, Rook.to_color_piece(friendly), to_rook);
                self.toggle(friendly, from_figure, from);
                self.toggle(friendly, from_figure, to);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_checkmate() {
        let mut board =
            Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/6P1/5P2/PPPPP2P/RNBQKBNR b KQkq - 0 2");

        let mv = DecodedMove::from_coords("d8h4".to_string(), &board);
        board.make_move(&mv);

        assert!(board.is_in_check(), "White should be in check");
        let white_moves = board.generate_moves::<false>();
        assert!(white_moves.is_empty(), "White should have no legal moves");
    }

    #[test]
    fn test_en_passant_execution() {
        let mut board =
            Board::from_fen("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2");

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
}
