use crate::{
    move_generator::{
        masks::{self, calculate_attackmask},
        normal_targets::{
            KING_TARGETS, KNIGHT_TARGETS, PAWN_ATTACK_TARGETS, pawn_quiet_double_target,
            pawn_quiet_single_target,
        },
        sliding_targets::{get_bishop_targets, get_rook_targets},
    },
    prelude::*,
};

impl Board {
    /// Checks whether `mv` is legal on `self`.
    /// - [x] null moves
    /// - [x] no piece
    /// - [x] Piece pinned
    /// - [x] Checks
    /// - [x] Target square available
    /// - [x] castling
    /// I somehow need to test this haha
    pub fn is_legal(&mut self, mv: &DecodedMove) -> bool {
        let to = mv.to.to_bit();
        // Catch null moves
        if mv.from == mv.to {
            return false;
        }

        // is there even a piece? If so, is it ours?
        let from_figure = self.figures(mv.from);
        if from_figure == Figure::Empty || from_figure.piece_and_color().1 != self.current_color() {
            return false;
        }

        // Do we capture the king?
        if self.figures(mv.to).piece() == Piece::King {
            return false;
        }

        // Does our move cause there to still be a check?
        // NOTE: doing the entire make move routine here does unnecessary calculationns (repetition stack etc)
        // maybe we should do a stripped-down version that just applies the necessary changes to check for checks to reduce load
        self.toggle_current_color();
        self.make_move(mv);

        let (_, next_pos_check_count) = masks::calc_check_mask(self);
        self.unmake_move();
        self.toggle_current_color();

        if next_pos_check_count > 0 {
            return false;
        }

        // Is the piece even allowed to move there?
        let opponents = self.color_bbs_without_king(!self.current_color());

        if !match from_figure.piece() {
            Pawn => match mv.mv_type {
                MoveType::DoubleMove => {
                    return pawn_quiet_double_target(mv.from.to_bit(), self.current_color()) == to
                        && self.empty().is_position_set(pawn_quiet_single_target(
                            mv.from.to_bit(),
                            self.current_color(),
                        ));
                }
                MoveType::Quiet
                | MoveType::KnightPromo
                | MoveType::BishopPromo
                | MoveType::RookPromo
                | MoveType::QueenPromo => {
                    pawn_quiet_single_target(mv.from.to_bit(), self.current_color()) == to
                }
                MoveType::Capture
                | MoveType::KnightPromoCapture
                | MoveType::BishopPromoCapture
                | MoveType::RookPromoCapture
                | MoveType::QueenPromoCapture => {
                    PAWN_ATTACK_TARGETS[0][mv.from.i()].is_position_set(to)
                        && opponents.is_position_set(to)
                        || PAWN_ATTACK_TARGETS[1][mv.from.i()].is_position_set(to)
                            && opponents.is_position_set(to)
                }
                MoveType::EpCapture => self.ep_target().map_or(false, |target| target == to),
                _ => false,
            },
            Knight => KNIGHT_TARGETS[mv.from.i()].is_position_set(to),
            Bishop => (get_bishop_targets(mv.from, self.occupied()) & (opponents | self.empty()))
                .is_position_set(to),
            Rook => (get_rook_targets(mv.from, self.occupied()) & (opponents | self.empty()))
                .is_position_set(to),
            Queen => ((get_rook_targets(mv.from, self.occupied()) & (opponents | self.empty()))
                | (get_bishop_targets(mv.from, self.occupied() & (opponents | self.empty()))))
            .is_position_set(to),
            King => {
                match mv.mv_type {
                    // Note: omit calculating attacks on the king here because we did that earlier
                    MoveType::Quiet | MoveType::Capture => {
                        KING_TARGETS[mv.from.i()].is_position_set(to)
                    }
                    MoveType::KingCastle | MoveType::QueenCastle => {
                        self.is_castling_legal(self.current_color(), mv.mv_type)
                    }
                    // a king move should never be something else but if it is it's certainly illegal
                    _ => false,
                }
            }
            Empty => false,
        } {
            return false;
        }

        true
    }

    /// Calculate whether castling to `side` is legal. for the given color.
    /// If MoveType is not a castling move this returns false.
    fn is_castling_legal(&self, friendly: Color, side: MoveType) -> bool {
        let occupied = self.occupied();
        let attackmask = calculate_attackmask(self, occupied, !friendly, None);
        match friendly {
            White => {
                const NEED_TO_BE_EMPTY_QUEEN: Bitboard = Bitboard::from_idx([1, 2, 3]);
                const NEED_TO_BE_NOT_ATTACKED_QUEEN: Bitboard = Bitboard::from_idx([2, 3]);

                if self.white_queen_castle()
                    && side == MoveType::QueenCastle
                    && NEED_TO_BE_EMPTY_QUEEN & occupied == Bitboard::EMPTY
                    && attackmask & NEED_TO_BE_NOT_ATTACKED_QUEEN == Bitboard::EMPTY
                {
                    return true;
                }
                const NEED_TO_BE_EMPTY_KING: Bitboard = Bitboard::from_idx([5, 6]);
                const NEED_TO_BE_NOT_ATTACKED_KING: Bitboard = Bitboard::from_idx([5, 6]);

                if self.white_king_castle()
                    && side == MoveType::KingCastle
                    && NEED_TO_BE_EMPTY_KING & occupied == Bitboard::EMPTY
                    && attackmask & NEED_TO_BE_NOT_ATTACKED_KING == Bitboard::EMPTY
                {
                    return true;
                }
                false
            }
            Black => {
                const NEED_TO_BE_EMPTY_QUEEN: Bitboard = Bitboard::from_idx([57, 58, 59]);
                const NEED_TO_BE_NOT_ATTACKED_QUEEN: Bitboard = Bitboard::from_idx([58, 59]);

                if self.black_queen_castle()
                    && side == MoveType::QueenCastle
                    && NEED_TO_BE_EMPTY_QUEEN & occupied == Bitboard::EMPTY
                    && attackmask & NEED_TO_BE_NOT_ATTACKED_QUEEN == Bitboard::EMPTY
                {
                    return true;
                }
                const NEED_TO_BE_EMPTY_KING: Bitboard = Bitboard::from_idx([61, 62]);
                const NEED_TO_BE_NOT_ATTACKED_KING: Bitboard = Bitboard::from_idx([61, 62]);

                if self.black_king_castle()
                    && side == MoveType::KingCastle
                    && NEED_TO_BE_EMPTY_KING & occupied == Bitboard::EMPTY
                    && attackmask & NEED_TO_BE_NOT_ATTACKED_KING == Bitboard::EMPTY
                {
                    return true;
                }
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use arrayvec::ArrayVec;

    use crate::move_generator::is_legal;

    use super::*;

    #[test]
    fn test_legal_moves_are_legal() {
        let fen1 = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1"; // position index 4
        let mut board = Board::from_fen(fen1);
        let legal_moves = board.generate_moves::<false>();
        for mv in legal_moves {
            if !board.is_legal(&mv.decode()) {
                println!("{}", mv.decode().to_coords());
                assert!(false);
            }
            assert!(true)
        }
        // Move the king out of check and check the rest of the clusterfuck for legality
        board.make_move(&DecodedMove::from_coords("g1h1".to_owned(), &board));
        let legal_moves = board.generate_moves::<false>();
        for mv in legal_moves {
            if !board.is_legal(&mv.decode()) {
                println!("{}", mv.decode().to_coords());
                assert!(false);
            }
            assert!(true)
        }
    }

    #[test]
    fn try_bruteforce_testing_hehe() {
        let fen = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1R1K b kq - 0 1";
        let mut board = Board::from_fen(fen);
        let legal_moves: ArrayVec<EncodedMove, 256> = board.generate_moves::<false>();
        let legal_moves_as_coords: Vec<String> = legal_moves.iter().map(|mv| mv.decode().to_coords()).collect();
        let mut illegal_moves: Vec<EncodedMove> = vec![];
        for from_row in (97..=104).map(|n| (n as u8 as char).to_string()) {
            for from_col in 1..=8 {
                for to_row in (97..=104).map(|n| (n as u8 as char).to_string()) {
                    for to_col in 1..=8 {
                        let mv_coords =  format!("{}{}{}{}", from_row, from_col, to_row, to_col);
                        if legal_moves_as_coords.contains(&mv_coords) {
                            continue;
                        }
                        let mv = DecodedMove::from_coords(
                            mv_coords,
                            &board,
                        )
                        .encode();
                        illegal_moves.push(mv);
                    }
                }
            }
        }
        for mv in illegal_moves {
                println!("{}", mv.decode().to_coords());
            // work-around to only print the information we need for debugging
            if board.is_legal(&mv.decode()) {
                println!("{}", mv.decode().to_coords());
                assert!(false);
            }
            assert!(true)
        }
    }
}
