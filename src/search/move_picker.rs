use std::collections::VecDeque;

use crate::{
    move_generator::generator::MAX_MOVES_COUNT,
    prelude::*,
    search::{move_ordering::mvv_lva, mvv_lva},
    settings::settings,
};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum GenerationState {
    TTMove,
    Captures,
    YieldCaptures,
    Killer,
    Quiets,
    YieldQuiets,
    Done,
}

pub struct MovePicker {
    tt_move: Option<EncodedMove>,
    killer_mv: Option<EncodedMove>,
    next_moves: VecDeque<EncodedMove>,
    state: GenerationState,
}

impl MovePicker {
    pub fn new(tt_move: Option<EncodedMove>, killer_mv: Option<EncodedMove>) -> MovePicker {
        MovePicker {
            tt_move,
            killer_mv,
            next_moves: VecDeque::with_capacity(MAX_MOVES_COUNT),
            state: GenerationState::TTMove,
        }
    }

    // TODO deal with double checks maybe?
    // I mean they should be handled correctly
    pub fn next<const SPECIAL_MOVES_ONLY: bool>(&mut self, board: &Board) -> Option<EncodedMove> {
        match self.state {
            GenerationState::TTMove => {
                if settings::ORDER_TT_MV_FIRST {
                    if let Some(tt_move) = self.tt_move {
                        if board.is_legal(&tt_move.decode()) {
                            self.state = GenerationState::Captures;
                            return self.tt_move;
                        }
                    }
                }
                self.state = GenerationState::Captures;
                return self.next::<SPECIAL_MOVES_ONLY>(board);
            }
            GenerationState::Captures => {
                let mut capture_moves = board.generate_moves::<true>();
                let tt_move_occurred = mvv_lva(&mut capture_moves, &board, self.tt_move);

                // let mut seen = std::collections::HashSet::new();
                // for mv in &capture_moves {
                //     if !seen.insert(mv) {
                //         panic!(
                //             "Duplicate move in capture_moves: {:?}",
                //             mv.decode().to_coords()
                //         );
                //     }
                // }

                self.next_moves.extend(capture_moves);
                // yeet the tt move if it was part of the captured move, we don't want to try it twice
                if tt_move_occurred {
                    _ = self.next_moves.pop_front();
                }
                self.state = GenerationState::YieldCaptures;
                return self.next::<SPECIAL_MOVES_ONLY>(board);
            }
            GenerationState::YieldCaptures => self.next_moves.pop_front().or_else(|| {
                self.state = GenerationState::Killer;
                self.next::<SPECIAL_MOVES_ONLY>(board)
            }),
            GenerationState::Killer => {
                if settings::KILLERS {
                    if let Some(killer) = self.killer_mv {
                        if board.is_legal(&killer.decode()) {
                            self.state = GenerationState::Quiets;
                            return self.killer_mv;
                        }
                    }
                }
                self.state = GenerationState::Quiets;
                self.next::<SPECIAL_MOVES_ONLY>(board)
            }
            GenerationState::Quiets => {
                let quiet_moves = board.generate_moves::<false>();

                // // Verify that quiet_moves does not contain any capturing pawn moves
                // for mv in &quiet_moves {
                //     let decoded = mv.decode();
                //     if board.piece_at_position(decoded.from) == Piece::Pawn {
                //         if let crate::move_generator::is_legal::MoveDirection::Diag = decoded.move_direction() {
                //             panic!(
                //                 "Pawn capturing move found in quiet_moves: {:?}",
                //                 decoded.to_coords()
                //             );
                //         }
                //     }
                // }

                self.next_moves.extend(quiet_moves);
                self.state = GenerationState::YieldQuiets;
                return self.next::<SPECIAL_MOVES_ONLY>(board);
            }
            GenerationState::YieldQuiets => self
                .next_moves
                .pop_front()
                .and_then(|mv| {
                    if Some(mv) == self.killer_mv {
                        self.next::<SPECIAL_MOVES_ONLY>(board)
                    } else {
                        Some(mv)
                    }
                })
                .or_else(|| {
                    self.state = GenerationState::Done;
                    None
                }),
            GenerationState::Done => None,
        }
    }
}

#[cfg(test)]
mod perft_test_move_picker {
    use std::collections::HashSet;

    use super::*;

    #[test]
    /// Tests the move generation by checking if it finds the correct amount of moves
    /// Also tests if the hashing works by checking if the incremental hash is the same as a newly calculated one from scratch

    fn test_move_picker_perft() {
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
                let calculated_node_count = move_picker_r_perft(&mut board, depth_idx + 1);
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
    fn move_picker_r_perft(board: &mut Board, depth: usize) -> usize {
        if depth == 0 {
            return 1;
        }
        let mut nodes = 0;
        let mut mvp = MovePicker::new(None, None);
        // let correct_moves = board.generate_moves_legacy::<false>(board);
        // let mut moves: ArrayVec<EncodedMove, 256> = ArrayVec::new();
        // while let Some(next) = mvp.next::<false>(board) {
        //     moves.push(next);
        // }
        let mut visited: HashSet<EncodedMove> = HashSet::new();
        while let Some(mv) = mvp.next::<false>(board) {
            // if !correct_moves.contains(&mv) {
            //     println!("fen: {}", board.generate_fen());
            //     println!("{:?}", mv.decode().to_coords());
            //     assert!(false, "move wrongly generated");
            // }
            if visited.contains(&mv) {
                println!("fen: {}", board.generate_fen());
                println!("{:?}", mv.decode().to_coords());
                assert!(false, "move visited twice FUCK");
            }
            board.make_move(&mv.decode());
            nodes += move_picker_r_perft(board, depth - 1);
            board.unmake_move();
            visited.insert(mv);
        }
        nodes
    }
}
