use crate::{prelude::*, types::unmake_info::UnmakeInfo};

impl Board {
    pub fn make_move(&mut self, mv: &DecodedMove) {
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
