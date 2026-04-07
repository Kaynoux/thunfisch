use arrayvec::ArrayVec;

use crate::{
    move_generator::generator::MAX_MOVES_COUNT, prelude::*, search::move_ordering::mvv_lva,
    settings,
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

#[repr(C)]
pub struct MoveListMove {
    pub score: i32,
    pub mv: EncodedMove,
}

pub struct MoveList {
    pub list: ArrayVec<MoveListMove, MAX_MOVES_COUNT>,
}

impl MoveList {
    pub fn new() -> Self {
        Self {
            list: ArrayVec::<MoveListMove, MAX_MOVES_COUNT>::new(),
        }
    }

    pub fn push(&mut self, mv: EncodedMove) {
        self.list.push(MoveListMove { score: 0, mv });
    }
}

pub struct MovePicker {
    tt_move: Option<EncodedMove>,
    killer_mv: Option<EncodedMove>,
    move_list: MoveList,
    state: GenerationState,
    move_index: usize,
    skip_quiets: bool,
}

impl MovePicker {
    pub fn new(
        tt_move: Option<EncodedMove>,
        killer_mv: Option<EncodedMove>,
        skip_quiets: bool,
    ) -> Self {
        Self {
            tt_move,
            killer_mv,
            move_list: MoveList::new(),
            state: GenerationState::TTMove,
            move_index: 0,
            skip_quiets,
        }
    }

    pub fn next(&mut self, board: &mut Board) -> Option<EncodedMove> {
        match self.state {
            GenerationState::TTMove => {
                self.state = GenerationState::Captures;
                if settings::ORDER_TT_MV_FIRST
                    && let Some(tt_move) = self.tt_move
                    && board.is_legal(&tt_move.decode())
                {
                    self.tt_move
                } else {
                    self.next(board)
                }
            }
            GenerationState::Captures => {
                board.generate_moves::<false>(&mut self.move_list);

                mvv_lva(&mut self.move_list, board);
                self.state = GenerationState::YieldCaptures;
                self.next(board)
            }
            GenerationState::YieldCaptures => {
                if let Some(mv) = self.yield_next_best_move() {
                    Some(mv)
                } else if self.skip_quiets {
                    self.state = GenerationState::Done;
                    None
                } else {
                    self.state = GenerationState::Killer;
                    self.next(board)
                }
            }
            GenerationState::Killer => {
                self.state = GenerationState::Quiets;
                if settings::KILLERS
                    && let Some(killer) = self.killer_mv
                    && self.killer_mv != self.tt_move
                    && board.is_legal(&killer.decode())
                {
                    self.killer_mv
                } else {
                    self.next(board)
                }
            }
            GenerationState::Quiets => {
                board.generate_moves::<true>(&mut self.move_list);
                self.state = GenerationState::YieldQuiets;
                self.next(board)
            }

            GenerationState::YieldQuiets => {
                if let Some(mv) = self.yield_next_best_move() {
                    Some(mv)
                } else {
                    self.state = GenerationState::Done;
                    None
                }
            }

            GenerationState::Done => None,
        }
    }

    pub fn yield_next_best_move(&mut self) -> Option<EncodedMove> {
        loop {
            let remaining = &mut self.move_list.list[self.move_index..];

            if remaining.is_empty() {
                return None;
            }

            // Find highest score in remaining
            let mut best_idx = 0;
            let mut max_score = remaining[0].score;
            for i in 1..remaining.len() {
                if remaining[i].score > max_score {
                    max_score = remaining[i].score;
                    best_idx = i;
                }
            }

            // Swap best move to front
            remaining.swap(0, best_idx);

            // Should we skip this move?
            let best_move = remaining[0].mv;
            self.move_index += 1;
            if (settings::ORDER_TT_MV_FIRST && Some(best_move) == self.tt_move)
                || (settings::KILLERS && Some(best_move) == self.killer_mv)
            {
                continue;
            }

            // Return valid move
            return Some(best_move);
        }
    }
}

#[cfg(test)]
mod perft_test_move_picker {

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
            for (depth_idx, correct_node_count) in perft_results[fen_idx].iter().take(4).enumerate()
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
        let mut mvp = MovePicker::new(None, None, false);
        // let correct_moves = board.generate_moves_legacy::<false>(board);
        // let mut moves: ArrayVec<EncodedMove, 256> = ArrayVec::new();
        // while let Some(next) = mvp.next::<false>(board) {
        //     moves.push(next);
        // }
        // let mut visited: HashSet<EncodedMove> = HashSet::new();
        while let Some(mv) = mvp.next(board) {
            // if !correct_moves.contains(&mv) {
            //     println!("fen: {}", board.generate_fen());
            //     println!("{:?}", mv.decode().to_coords());
            //     assert!(false, "move wrongly generated");
            // }
            // if visited.contains(&mv) {
            //     println!("fen: {}", board.generate_fen());
            //     println!("{:?}", mv.decode().to_coords());
            //     assert!(false, "move visited twice FUCK");
            // }
            board.make_move(mv);
            nodes += move_picker_r_perft(board, depth - 1);
            board.unmake_move();
            // visited.insert(mv);
        }
        nodes
    }
}
