use crate::{
    move_generator::{
        normal_targets::{
            KING_TARGETS, KNIGHT_TARGETS, PAWN_ATTACK_TARGETS, pawn_quiet_double_target,
            pawn_quiet_single_target,
        },
        sliding_targets::{get_bishop_targets, get_rook_targets},
    },
    prelude::*,
};

#[derive(PartialEq, Eq, Debug)]
pub enum MoveDirection {
    HV,
    Diag,
    Knight,
    Teleport,
}

impl DecodedMove {
    /// Identifies the `MoveDirection` of the move based on where the squares lie relative to
    /// each other on the chess board.
    pub const fn move_direction(&self) -> MoveDirection {
        match (
            self.to.x().abs_diff(self.from.x()),
            self.to.y().abs_diff(self.from.y()),
        ) {
            // null move
            (0, 1..=7) | (1..=7, 0) => MoveDirection::HV,
            (1, 1) | (2, 2) | (3, 3) | (4, 4) | (5, 5) | (6, 6) | (7, 7) => MoveDirection::Diag,
            (1, 2) | (2, 1) => MoveDirection::Knight,
            _ => MoveDirection::Teleport, // Everything else (including (0,0) is considered teleport)
        }
    }
}

impl Board {
    /// Assumption: Our move generation generates only legal moves but these moves can be now illegal
    /// Does not handle castles at all atm, they are completly handled in `is_legal`
    #[inline]
    pub fn is_pseudo_legal(&self, mv: &DecodedMove) -> bool {
        let mv_direction = mv.move_direction();
        let current_color = self.current_color();
        let mv_type = mv.mv_type;

        // we don't do that here
        // Null moves are classified as teleports and thus filtered out here
        if mv_direction == MoveDirection::Teleport {
            return false;
        }

        // is there even a piece? If so, is it ours?
        let from_figure = self.figures(mv.from);
        let from_piece = from_figure.piece();
        if from_figure == Figure::Empty || from_figure.piece_and_color().1 != current_color {
            return false;
        }

        // Do we capture our own piece
        let (to_piece, to_color) = self.figures(mv.to).piece_and_color();
        let to_is_empty = to_piece == Piece::Empty;

        if to_piece != Piece::Empty && to_color == current_color {
            return false;
        }

        // See if the move type is now wrong
        // Capture move but not an capture
        let is_capture = MoveType::is_capture(mv_type);
        if is_capture && to_is_empty && mv_type != MoveType::EpCapture {
            return false;
        }

        // Not capture move but somehow capturing
        if !is_capture && !to_is_empty && mv_type != MoveType::EpCapture {
            return false;
        }

        // Ep or promotion but not by a pawn
        if (mv_type == MoveType::EpCapture || MoveType::is_promotion(mv_type))
            && from_piece != Piece::Pawn
        {
            return false;
        }

        // Do we capture a king?
        if to_piece == Piece::King {
            return false;
        }

        true
    }
    /// Checks whether `mv` is legal on `self`.
    #[allow(clippy::too_many_lines)]
    pub fn is_legal(&mut self, mv: &DecodedMove) -> bool {
        if !self.is_pseudo_legal(mv) {
            return false;
        }

        let to_bit = mv.to.to_bit();
        let from_bit = mv.from.to_bit();
        let from_figure = self.figures(mv.from);

        //calculating here to avoid recalculation costs
        let rook_targets = get_rook_targets(mv.from, self.occupied());
        let bishop_targets = get_bishop_targets(mv.from, self.occupied());

        // are we pinned? if so, do we move off the pin?
        let (hv_pinmask, diag_pinmask) = self.get_pinmasks();
        let move_direction = mv.move_direction();

        if diag_pinmask.is_position_set(from_bit) {
            if move_direction != MoveDirection::Diag {
                return false;
            }
            // if we're moving on the wrong diagonal
            if !(bishop_targets & diag_pinmask).is_position_set(to_bit) {
                return false;
            }
        }

        if hv_pinmask.is_position_set(from_bit) {
            if move_direction != MoveDirection::HV {
                return false;
            }
            if !(rook_targets & hv_pinmask).is_position_set(to_bit) {
                return false;
            }
        }

        let check_count = self.get_check_counter();
        let checkmask = self.get_checkmask();

        // does the king need to move because of double check?
        if check_count >= 2 && from_figure.piece() != Piece::King {
            return false;
        }

        // Is the piece even allowed to move there?
        let opponents = self.color_bbs_without_king(!self.current_color());
        let attackmask = if from_figure.piece() == Piece::King {
            let occupied_without_king = self.occupied() & !from_bit;
            crate::move_generator::masks::calculate_attackmask(
                self,
                occupied_without_king,
                !self.current_color(),
                None,
            )
        } else {
            self.get_attackmask()
        };
        let empty_or_opponent = opponents | self.empty();

        if !match from_figure.piece() {
            Pawn => match mv.mv_type {
                MoveType::DoubleMove => {
                    return pawn_quiet_double_target(from_bit, self.current_color()) == to_bit
                        && (checkmask & self.empty()).is_position_set(to_bit)
                        && self.empty().is_position_set(pawn_quiet_single_target(
                            from_bit,
                            self.current_color(),
                        ));
                }
                MoveType::Quiet
                | MoveType::KnightPromo
                | MoveType::BishopPromo
                | MoveType::RookPromo
                | MoveType::QueenPromo => {
                    return (pawn_quiet_single_target(from_bit, self.current_color())
                        & self.empty()
                        & checkmask)
                        == to_bit;
                }
                MoveType::Capture
                | MoveType::KnightPromoCapture
                | MoveType::BishopPromoCapture
                | MoveType::RookPromoCapture
                | MoveType::QueenPromoCapture => {
                    (PAWN_ATTACK_TARGETS[0][mv.from.i()] & opponents & checkmask)
                        .is_position_set(to_bit)
                        || (PAWN_ATTACK_TARGETS[1][mv.from.i()] & opponents & checkmask)
                            .is_position_set(to_bit)
                }
                MoveType::EpCapture => self.ep_target().is_some_and(|target| {
                    let captured_pawn = pawn_quiet_single_target(target, !self.current_color());
                    target == to_bit
                        && (checkmask.is_position_set(to_bit)
                            || checkmask.is_position_set(captured_pawn))
                        && !diag_pinmask.is_position_set(captured_pawn)
                }),
                _ => false,
            },
            Knight => (KNIGHT_TARGETS[mv.from.i()] & empty_or_opponent & checkmask)
                .is_position_set(to_bit),
            Bishop => (bishop_targets & empty_or_opponent & checkmask).is_position_set(to_bit),
            Rook => (rook_targets & empty_or_opponent & checkmask).is_position_set(to_bit),
            Queen => ((rook_targets & empty_or_opponent & checkmask)
                | (bishop_targets & empty_or_opponent) & checkmask)
                .is_position_set(to_bit),
            King => {
                match mv.mv_type {
                    MoveType::Quiet | MoveType::Capture => (KING_TARGETS[mv.from.i()]
                        & (empty_or_opponent & !attackmask))
                        .is_position_set(to_bit),
                    MoveType::KingCastle | MoveType::QueenCastle => {
                        check_count == 0 && self.is_castling_legal(self.current_color(), mv.mv_type)
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
    /// If `MoveType` is not a castling move this returns false.
    fn is_castling_legal(&mut self, friendly: Color, side: MoveType) -> bool {
        let occupied = self.occupied();
        let attackmask = self.get_attackmask();
        match friendly {
            White => {
                const NEED_TO_BE_EMPTY_QUEEN: Bitboard = Bitboard::from_idx([1, 2, 3]);
                const NEED_TO_BE_NOT_ATTACKED_QUEEN: Bitboard = Bitboard::from_idx([2, 3]);

                const NEED_TO_BE_EMPTY_KING: Bitboard = Bitboard::from_idx([5, 6]);
                const NEED_TO_BE_NOT_ATTACKED_KING: Bitboard = Bitboard::from_idx([5, 6]);

                if self.white_queen_castle()
                    && side == MoveType::QueenCastle
                    && NEED_TO_BE_EMPTY_QUEEN & occupied == Bitboard::EMPTY
                    && attackmask & NEED_TO_BE_NOT_ATTACKED_QUEEN == Bitboard::EMPTY
                {
                    return true;
                }

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

                const NEED_TO_BE_EMPTY_KING: Bitboard = Bitboard::from_idx([61, 62]);
                const NEED_TO_BE_NOT_ATTACKED_KING: Bitboard = Bitboard::from_idx([61, 62]);

                if self.black_queen_castle()
                    && side == MoveType::QueenCastle
                    && NEED_TO_BE_EMPTY_QUEEN & occupied == Bitboard::EMPTY
                    && attackmask & NEED_TO_BE_NOT_ATTACKED_QUEEN == Bitboard::EMPTY
                {
                    return true;
                }

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
mod tests_perft {
    use crate::search::move_picker::MoveList;

    use super::*;

    #[test]
    fn test_knight_move_direction() {
        // FEN: White king on a1, white knight on e4, black king on h8
        let fen = "7k/8/8/8/4N3/8/8/K7 w - - 0 1";
        let mut board = Board::from_fen(fen);
        let moves = board.generate_all_moves();

        for mv in moves.list {
            let decoded = mv.mv.decode();
            let direction = decoded.move_direction();
            if board.piece_at_position(decoded.from) != Knight {
                continue;
            }
            assert_eq!(
                direction,
                MoveDirection::Knight,
                "Knight move {} should have Knight direction",
                decoded.to_coords()
            );
        }
    }

    #[test]
    fn test_bishop_move_direction() {
        // Test multiple bishop positions: e4, d2 (dark diagonal), a1 (dark corner), h1 (light corner)
        let fens = [
            ("7k/8/8/8/4B3/8/8/K7 w - - 0 1", "e4"),
            ("7k/8/8/8/8/8/3B4/K7 w - - 0 1", "d2"),
            ("7k/8/8/8/8/8/8/B6K w - - 0 1", "a1"),
            ("7k/8/8/8/8/8/8/K6B w - - 0 1", "h1"),
        ];

        for (fen, position) in fens {
            let mut board = Board::from_fen(fen);
            let moves = board.generate_all_moves();

            for mv in moves.list {
                let decoded = mv.mv.decode();
                let direction = decoded.move_direction();
                if board.piece_at_position(decoded.from) != Bishop {
                    continue;
                }
                assert_eq!(
                    direction,
                    MoveDirection::Diag,
                    "Bishop at {} move {} should have Diagonal direction",
                    position,
                    decoded.to_coords()
                );
            }
        }
    }

    #[test]
    fn test_rook_move_direction() {
        // Test multiple rook positions: e4 and a1 (corner)
        let fens = [
            ("7k/8/8/8/4R3/8/8/K7 w - - 0 1", "e4"),
            ("7k/8/8/8/8/8/6K1/R7 w - - 0 1", "a1"),
        ];

        for (fen, position) in fens {
            let mut board = Board::from_fen(fen);
            let moves = board.generate_all_moves();

            for mv in moves.list {
                let decoded = mv.mv.decode();
                let direction = decoded.move_direction();
                if board.piece_at_position(decoded.from) != Rook {
                    continue;
                }
                assert_eq!(
                    direction,
                    MoveDirection::HV,
                    "Rook at {} move {} should have HV direction",
                    position,
                    decoded.to_coords()
                );
            }
        }
    }

    #[test]
    fn test_pawn_quiet_move_direction() {
        // FEN: White king on a1, white pawn on e4 (position allows quiet moves), black king on h8
        let fen = "7k/8/8/8/4P3/8/8/K7 w - - 0 1";
        let mut board = Board::from_fen(fen);
        let moves = board.generate_all_moves();

        for mv in moves.list {
            let decoded = mv.mv.decode();
            // Filter for quiet moves only
            if board.piece_at_position(decoded.from) != Pawn {
                continue;
            }
            if decoded.is_quiet() {
                let direction = decoded.move_direction();
                assert_eq!(
                    direction,
                    MoveDirection::HV,
                    "Quiet pawn move {} should have HV direction",
                    decoded.to_coords()
                );
            }
        }
    }

    #[test]
    fn test_pawn_capture_move_direction() {
        // FEN: White king on a1, white pawn on e4, black pawns on d5 and f5 to test captures
        let fen = "7k/8/8/3p1p2/4P3/8/8/K7 w - - 0 1";
        let mut board = Board::from_fen(fen);
        let moves = board.generate_all_moves();

        for mv in moves.list {
            let decoded = mv.mv.decode();
            if board.piece_at_position(decoded.from) != Pawn {
                continue;
            }
            // Filter for capture moves only
            if !decoded.is_quiet() {
                let direction = decoded.move_direction();
                assert_eq!(
                    direction,
                    MoveDirection::Diag,
                    "Capturing pawn move {} should have Diagonal direction",
                    decoded.to_coords()
                );
            }
        }
    }

    #[test]
    fn test_king_move_direction() {
        // FEN: Black king on a8 (far away), white king on e4 (empty board otherwise)
        let fen = "k7/8/8/8/4K3/8/8/8 w - - 0 1";
        let mut board = Board::from_fen(fen);
        let moves = board.generate_all_moves();

        for mv in moves.list {
            let decoded = mv.mv.decode();
            let direction = decoded.move_direction();
            // King can move diagonally or horizontally/vertically
            assert!(
                direction == MoveDirection::Diag || direction == MoveDirection::HV,
                "King move {} should have either Diagonal or HV direction, got {:?}",
                decoded.to_coords(),
                direction
            );
        }
    }

    #[test]
    fn test_teleportation_move_direction() {
        // Test various moves that should result in Teleportation classification
        // These are manually constructed invalid/impossible moves with unusual distances
        let test_cases = vec![
            (Square(0), Square(11)),
            (Square(0), Square(13)),
            (Square(0), Square(14)),
            (Square(0), Square(25)),
            (Square(0), Square(20)),
            (Square(0), Square(30)),
            (Square(7), Square(8)),
            (Square(32), Square(5)),
            (Square(10), Square(32)),
            (Square(5), Square(50)),
        ];

        for (from, to) in test_cases {
            let mv = DecodedMove {
                from,
                to,
                mv_type: MoveType::Quiet,
            };
            let direction = mv.move_direction();
            assert_eq!(
                direction,
                MoveDirection::Teleport,
                "Move from {} to {} (distance {}) should be Teleport",
                from.to_bit().to_coords(),
                to.to_bit().to_coords(),
                (to.0.cast_signed() - from.0.cast_signed()).abs()
            );
        }
    }

    /// Only used for testing
    /// Important to pass board as value (so copying it) because we change the masks
    fn pseudo_legal_generate(mut board: Board) -> MoveList {
        board.set_pinmasks(Bitboard::EMPTY, Bitboard::EMPTY);
        board.set_checkmask(Bitboard::FULL, 0);

        board.generate_all_moves()
    }

    #[test]
    /// Tests the move generation by checking if it finds the correct amount of moves
    /// Also tests if the hashing works by checking if the incremental hash is the same as a newly calculated one from scratch
    fn test_islegal_perft() {
        // Source: https://www.chessprogramming.org/Perft_Results
        // Position 2 at depth 4 has all types of moves covered
        let fens = [
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ", // Pos 2
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",          // Initial Pos
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1 ",                        // Pos 3
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",  // Pos 4
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8 ",        // Pos 5
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ", // Pos 6
        ];
        let perft_results: [[usize; 5]; 6] = [
            [48, 2_039, 97_862, 4_085_603, 193_690_690], // Pos 2
            [20, 400, 8_902, 197_281, 4_865_609],        // Initial Pos
            [14, 191, 2_812, 43_238, 674_624],           // Pos 3
            [6, 264, 9_467, 422_333, 15_833_292],        // Pos 4
            [44, 1_486, 62_379, 2_103_487, 89_941_194],  // Pos 5
            [46, 2_079, 89_890, 3_894_594, 164_075_551], // Pos 6
        ];

        // TODO detailed test option with take 6
        for (fen_idx, fen) in fens.iter().enumerate() {
            for (depth_idx, correct_node_count) in perft_results[fen_idx].iter().take(4).enumerate()
            {
                let mut board = Board::from_fen(fen);
                let calculated_node_count = is_legal_r_perft(&mut board, depth_idx + 1);
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

    // comments are debug prints
    // if this ever fails uncomment those and you'll get the fen and move(s) that are incorrect printed out
    fn is_legal_r_perft(board: &mut Board, depth: usize) -> usize {
        if depth == 0 {
            return 1;
        }
        let mut nodes = 0;
        let moves = pseudo_legal_generate(board.clone());
        let correct_moves = board.generate_all_moves();

        assert!(moves.list.len() >= correct_moves.list.len());

        for mv in moves.list {
            if board.is_legal(&mv.mv.decode()) {
                // if !correct_moves.contains(&mv) {
                //     println!("{}", board.generate_fen());
                //     println!("{:?}", mv.decode().to_coords());
                //     assert!(false, "wrongly classified as legal");
                // }
                let mut b2 = board.clone();
                b2.make_move(mv.mv);
                nodes += is_legal_r_perft(&mut b2, depth - 1);
            } else {
                // if correct_moves.contains(&mv) {
                //     println!("{}", board.generate_fen());
                //     println!("{:?}", mv.decode().to_coords());
                //     assert!(false, "wrongly classified as illegal");
                // }
            }
        }
        nodes
    }
}
