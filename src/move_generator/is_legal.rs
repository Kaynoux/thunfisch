use crate::{
    move_generator::{
        masks::{self, calculate_attackmask},
        normal_targets::{
            KING_TARGETS, KNIGHT_TARGETS, PAWN_ATTACK_TARGETS, pawn_quiet_double_target,
            pawn_quiet_single_target,
        },
        pinmask::generate_pin_masks,
        sliding_targets::{get_bishop_targets, get_rook_targets},
    },
    prelude::*,
};

#[derive(PartialEq, Debug)]
pub enum MoveDirection {
    HV,
    Diag,
    Knight,
    Teleport,
}
impl DecodedMove {
    /// Identifies the `MoveDirection` of the move based on where the squares lie relative to
    /// each other on the chess board.
    pub fn move_direction(&self) -> MoveDirection {
        match (self.to - self.from).abs() {
            // vertical; divisible by 8
            8 | 16 | 24 | 32 | 40 | 48 | 56 => MoveDirection::HV,
            // bottom left -> top right; divisible by 9
            9 | 18 | 27 | 36 | 45 | 54 | 63 => MoveDirection::Diag,
            // top left -> bottom right; divisible by 7
            7 | 14 | 21 | 28 | 35 | 42 | 49 => MoveDirection::Diag,
            // knight
            10 | 15 | 17 | 22 => MoveDirection::Knight,
            6 => match self.to.x().abs_diff(self.from.x()) {
                0 => MoveDirection::HV,
                1 => MoveDirection::Diag,
                2 => MoveDirection::Knight,
                _ => MoveDirection::Teleport,
            },
            // horizontal
            0..=7 => match self.to.y() == self.from.y() {
                true => MoveDirection::HV,
                false => MoveDirection::Teleport
            },
            _ => MoveDirection::Teleport,
        }
    }
}

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
                    // print!(
                    //     "{:?}",
                    //     pawn_quiet_single_target(mv.from.to_bit(), self.current_color())
                    //         .to_coords()
                    // );
                    return pawn_quiet_single_target(mv.from.to_bit(), self.current_color()) == to;
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
            Knight => {
                (KNIGHT_TARGETS[mv.from.i()] & (opponents | self.empty())).is_position_set(to)
            }
            Bishop => (get_bishop_targets(mv.from, self.occupied()) & (opponents | self.empty()))
                .is_position_set(to),
            Rook => (get_rook_targets(mv.from, self.occupied()) & (opponents | self.empty()))
                .is_position_set(to),
            Queen => ((get_rook_targets(mv.from, self.occupied()) & (opponents | self.empty()))
                | (get_bishop_targets(mv.from, self.occupied()) & (opponents | self.empty())))
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

        let (hv_pinmask, diag_pinmask) = generate_pin_masks(self);

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

    use super::*;

    #[test]
    fn test_legal_moves_are_legal() {
        let fen1 = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1"; // position index 4
        let mut board = Board::from_fen(fen1);
        let legal_moves = board.generate_moves::<false>();
        for mv in legal_moves {
            // println!("{}", mv.decode().to_coords());
            assert!(board.is_legal(&mv.decode()))
        }
        // Move the king out of check and check the rest of the clusterfuck for legality
        board.make_move(&DecodedMove::from_coords("g1h1".to_owned(), &board));
        let legal_moves = board.generate_moves::<false>();
        for mv in legal_moves {
            // println!("{}", mv.decode().to_coords());
            assert!(board.is_legal(&mv.decode()))
        }
    }

    #[test]
    fn try_bruteforce_testing_hehe() {
        let fen = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1R1K b kq - 0 1";
        let mut board = Board::from_fen(fen);
        let legal_moves: ArrayVec<EncodedMove, 256> = board.generate_moves::<false>();
        let legal_moves_as_coords: Vec<String> = legal_moves
            .iter()
            .map(|mv| mv.decode().to_coords())
            .collect();
        let mut illegal_moves: Vec<EncodedMove> = vec![];
        for from_row in (97..=104).map(|n| (n as u8 as char).to_string()) {
            for from_col in 1..=8 {
                for to_row in (97..=104).map(|n| (n as u8 as char).to_string()) {
                    for to_col in 1..=8 {
                        let mv_coords = format!("{}{}{}{}", from_row, from_col, to_row, to_col);
                        if legal_moves_as_coords.contains(&mv_coords) {
                            continue;
                        }
                        let mv = DecodedMove::from_coords(mv_coords, &board).encode();
                        illegal_moves.push(mv);
                    }
                }
            }
        }
        for mv in illegal_moves {
            // println!("{}", mv.decode().to_coords());
            // work-around to only print the information we need for debugging
            if board.is_legal(&mv.decode()) {
                println!("{}", mv.decode().to_coords());
                assert!(false);
            }
            assert!(true)
        }
    }
}

#[cfg(test)]
mod test_lukas {
    use crate::move_generator::{generator::ARRAY_LENGTH, moves};
    use crate::prelude::*;
    use arrayvec::ArrayVec;

    fn is_legal_generate(board: &mut Board) -> ArrayVec<EncodedMove, ARRAY_LENGTH> {
        const SPECIAL_MOVES_ONLY: bool = false;
        let friendly = board.current_color();
        let mut moves = ArrayVec::<EncodedMove, ARRAY_LENGTH>::new();

        let (hv_pinmask, diag_pinmask) = (Bitboard::EMPTY, Bitboard::EMPTY);
        let pinmask = Bitboard::EMPTY;

        let (check_mask, check_counter) = (Bitboard::FULL, 0);

        moves::generate_pawn_moves::<false>(
            &mut moves,
            board,
            friendly,
            hv_pinmask,
            diag_pinmask,
            check_mask,
        );
        moves::generate_knight_moves::<SPECIAL_MOVES_ONLY>(
            &mut moves, pinmask, friendly, board, check_mask,
        );
        moves::generate_bishop_moves::<SPECIAL_MOVES_ONLY>(
            &mut moves,
            hv_pinmask,
            diag_pinmask,
            friendly,
            board,
            check_mask,
        );

        moves::generate_rook_moves::<SPECIAL_MOVES_ONLY>(
            &mut moves,
            hv_pinmask,
            diag_pinmask,
            friendly,
            board,
            check_mask,
        );

        moves::generate_queen_moves::<SPECIAL_MOVES_ONLY>(
            &mut moves,
            hv_pinmask,
            diag_pinmask,
            friendly,
            board,
            check_mask,
        );

        moves::generate_king_move::<SPECIAL_MOVES_ONLY>(&mut moves, friendly, board);

        if !SPECIAL_MOVES_ONLY {
            moves::generate_castle_moves(&mut moves, check_counter, friendly, board);
        }

        moves::generate_ep_moves(board, &mut moves, friendly, hv_pinmask, diag_pinmask);

        moves
    }

    #[test]
    /// Tests the move generation by checking if it finds the correct amount of moves
    /// Also tests if the hashing works by checking if the incremental hash is the same as a newly calculated one from scratch
    fn test_is_legal() {
        // Source: https://www.chessprogramming.org/Perft_Results
        // Position 2 at depth 4 has all types of moves covered
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
            for (depth_idx, correct_node_count) in perft_results[fen_idx].iter().take(6).enumerate()
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

    fn is_legal_r_perft(board: &mut Board, depth: usize) -> usize {
        if depth == 0 {
            return 1;
        }
        let mut nodes = 0;
        let moves = is_legal_generate(board);
        assert!(moves.len() > 0);

        for mv in moves {
            if board.is_legal(&mv.decode()) {
                let mut b2 = board.clone();
                b2.make_move(&mv.decode());
                nodes += is_legal_r_perft(&mut b2, depth - 1);
            }
        }
        nodes
    }
}

#[cfg(test)]
mod test_move_direction {
    use super::*;

    #[test]
    fn test_knight_move_direction() {
        // FEN: White king on a1, white knight on e4, black king on h8
        let fen = "7k/8/8/8/4N3/8/8/K7 w - - 0 1";
        let mut board = Board::from_fen(fen);
        let moves = board.generate_moves::<false>();

        for mv in moves {
            let decoded = mv.decode();
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
        // FEN: White king on a1, white bishop on e4, black king on h8
        let fen = "7k/8/8/8/4B3/8/8/K7 w - - 0 1";
        let mut board = Board::from_fen(fen);
        let moves = board.generate_moves::<false>();

        for mv in moves {
            let decoded = mv.decode();
            let direction = decoded.move_direction();
            if board.piece_at_position(decoded.from) != Bishop {
                continue;
            }
            assert_eq!(
                direction,
                MoveDirection::Diag,
                "Bishop move {} should have Diagonal direction",
                decoded.to_coords()
            );
        }
    }

    #[test]
    fn test_rook_move_direction() {
        // FEN: White king on a1, white rook on e4, black king on h8
        let fen = "7k/8/8/8/4R3/8/8/K7 w - - 0 1";
        let mut board = Board::from_fen(fen);
        let moves = board.generate_moves::<false>();

        for mv in moves {
            let decoded = mv.decode();
            let direction = decoded.move_direction();
            if board.piece_at_position(decoded.from) != Rook {
                continue;
            }
            assert_eq!(
                direction,
                MoveDirection::HV,
                "Rook move {} should have HV direction",
                decoded.to_coords()
            );
        }
    }

    #[test]
    fn test_pawn_quiet_move_direction() {
        // FEN: White king on a1, white pawn on e4 (position allows quiet moves), black king on h8
        let fen = "7k/8/8/8/4P3/8/8/K7 w - - 0 1";
        let mut board = Board::from_fen(fen);
        let moves = board.generate_moves::<false>();

        for mv in moves {
            let decoded = mv.decode();
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
        let moves = board.generate_moves::<false>();

        for mv in moves {
            let decoded = mv.decode();
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
        let moves = board.generate_moves::<false>();

        for mv in moves {
            let decoded = mv.decode();
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
}
