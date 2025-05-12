use crate::prelude::*;

pub struct EngineState {
    pub board: Board,
}

impl EngineState {
    pub fn new() -> Self {
        EngineState {
            board: Board::from_fen(START_POS),
        }
    }
}
