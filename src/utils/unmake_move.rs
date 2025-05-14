use crate::prelude::*;

impl Board {
    /// Unmakes the last move by using unmake info from the stack
    /// https://www.chessprogramming.org/Unmake_Move
    pub fn unmake_move(&mut self) {
        let prev = match self.pop_unmake_info_stack() {
            Some(info) => info,
            None => {
                println!("info: Could not undo move because there was no previous move");
                return;
            }
        };
        let mv = prev.mv.decode();
        let mv_type = mv.mv_type;
        let color_that_moved = !self.current_color();
        let from = mv.from;
        let to = mv.to;

        let captured_figure = prev.capture;
        let figure_on_to = self.figures(to);

        self.toggle_current_color();
        self.set_total_halfmove_counter(self.total_halfmove_counter() - 1);
        self.set_halfmove_clock(prev.halfmove_clock);
        self.set_ep_target(prev.ep_target);

        self.set_castling_rights(
            prev.white_queen_castle,
            prev.white_king_castle,
            prev.black_queen_castle,
            prev.black_king_castle,
        );

        let original_figure_from = if mv_type.is_promotion() {
            match color_that_moved {
                White => Figure::WhitePawn,
                Black => Figure::BlackPawn,
            }
        } else {
            figure_on_to
        };

        match mv_type {
            MoveType::Quiet | MoveType::DoubleMove => {
                self.toggle(color_that_moved, original_figure_from, to);
                self.toggle(color_that_moved, original_figure_from, from);
            }
            MoveType::Capture => {
                self.toggle(color_that_moved, original_figure_from, to);
                self.toggle(color_that_moved, original_figure_from, from);
                self.toggle(!color_that_moved, captured_figure, to);
            }
            MoveType::KnightPromo
            | MoveType::BishopPromo
            | MoveType::RookPromo
            | MoveType::QueenPromo => {
                self.toggle(color_that_moved, figure_on_to, to);

                self.toggle(color_that_moved, original_figure_from, from);
            }
            MoveType::KnightPromoCapture
            | MoveType::BishopPromoCapture
            | MoveType::RookPromoCapture
            | MoveType::QueenPromoCapture => {
                self.toggle(color_that_moved, figure_on_to, to);
                self.toggle(color_that_moved, original_figure_from, from);
                self.toggle(!color_that_moved, captured_figure, to);
            }
            MoveType::EpCapture => {
                self.toggle(color_that_moved, original_figure_from, to);
                self.toggle(color_that_moved, original_figure_from, from);
                let ep_captured_square = match color_that_moved {
                    Color::White => Square(to.0 - 8),
                    Color::Black => Square(to.0 + 8),
                };
                self.toggle(
                    !color_that_moved,
                    Piece::Pawn.to_color_piece(!color_that_moved),
                    ep_captured_square,
                );
            }
            MoveType::QueenCastle => {
                let (rook_original_sq, rook_moved_to_sq) = match color_that_moved {
                    Color::White => (Square(0), Square(3)),
                    Color::Black => (Square(56), Square(59)),
                };
                self.toggle(color_that_moved, original_figure_from, to);
                self.toggle(color_that_moved, original_figure_from, from);

                self.toggle(
                    color_that_moved,
                    Piece::Rook.to_color_piece(color_that_moved),
                    rook_moved_to_sq,
                );
                self.toggle(
                    color_that_moved,
                    Piece::Rook.to_color_piece(color_that_moved),
                    rook_original_sq,
                );
            }
            MoveType::KingCastle => {
                let (rook_original_sq, rook_moved_to_sq) = match color_that_moved {
                    Color::White => (Square(7), Square(5)),
                    Color::Black => (Square(63), Square(61)),
                };

                self.toggle(color_that_moved, original_figure_from, to);
                self.toggle(color_that_moved, original_figure_from, from);

                self.toggle(
                    color_that_moved,
                    Piece::Rook.to_color_piece(color_that_moved),
                    rook_moved_to_sq,
                );
                self.toggle(
                    color_that_moved,
                    Piece::Rook.to_color_piece(color_that_moved),
                    rook_original_sq,
                );
            }
        }

        self.set_hash(prev.hash);
    }
}
