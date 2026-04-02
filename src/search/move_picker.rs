use std::collections::VecDeque;

use crate::{
    move_generator::generator::ARRAY_LENGTH,
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
    board: Board,
    tt_move: Option<EncodedMove>,
    killer_mv: Option<EncodedMove>,
    next_moves: VecDeque<EncodedMove>,
    state: GenerationState,
}

impl MovePicker {
    pub fn new(
        board: Board,
        tt_move: Option<EncodedMove>,
        killer_mv: Option<EncodedMove>,
    ) -> MovePicker {
        MovePicker {
            board,
            tt_move,
            killer_mv,
            next_moves: VecDeque::with_capacity(ARRAY_LENGTH),
            state: GenerationState::TTMove,
        }
    }

    pub fn next(&mut self) -> Option<EncodedMove> {
        match self.state {
            GenerationState::TTMove => {
                if settings::ORDER_TT_MV_FIRST {
                    if let Some(tt_move) = self.tt_move {
                        if self.board.is_legal(&tt_move.decode()) {
                            return self.tt_move;
                        }
                    }
                }
                self.state = GenerationState::Captures;
                return self.next();
            }
            GenerationState::Captures => {
                let mut capture_moves = self.board.generate_moves::<true>();
                let tt_move_occurred = mvv_lva(&mut capture_moves, &self.board, self.tt_move);
                self.next_moves.extend(capture_moves);
                // yeet the tt move if it was part of the captured move, we don't want to try it twice
                if tt_move_occurred {
                    _ = self.next_moves.pop_front();
                }
                self.state = GenerationState::YieldCaptures;
                return self.next();
            }
            GenerationState::YieldCaptures => self.next_moves.pop_front().or_else(|| {
                self.state = GenerationState::Killer;
                self.next()
            }),
            GenerationState::Killer => {
                None
                // if let Some(killer) = self.killer_mv {

                // }
            }
            GenerationState::Quiets => todo!(),
            GenerationState::YieldQuiets => todo!(),
            GenerationState::Done => None,
        }
    }
}
