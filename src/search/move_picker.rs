use crate::{prelude::*, settings::settings};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum GenerationState {
    TTMove,
    Captures,
    Killers,
    Quiets,
}

pub struct MovePicker<'b> {
    board: &'b Board,
    tt_move: Option<EncodedMove>,
    killer_mv: Option<EncodedMove>,
    state: GenerationState
}

impl<'mp> MovePicker<'mp> {
    pub fn new<'b>(
        board: &'b Board,
        tt_move: Option<EncodedMove>,
        killer_mv: Option<EncodedMove>,
    ) -> MovePicker<'b> {
        MovePicker {
            board,
            tt_move,
            killer_mv,
            state: GenerationState::TTMove
        }
    }

    pub fn next(&mut self) -> Option<EncodedMove> {
        match self.state {
            GenerationState::TTMove => {
                if !settings::ORDER_TT_MV_FIRST {
                    self.state = GenerationState::Captures;
                }
                if let Some(tt_move) = self.tt_move {
                    if self.board.is_legal(&tt_move.decode()) {
                        return self.tt_move;
                    }
                }
            },
            GenerationState::Captures => return None,
            GenerationState::Killers => todo!(),
            GenerationState::Quiets => todo!(),
        }
        None
    }
}
