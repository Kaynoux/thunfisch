use crate::prelude::*;

impl Board {
    /// Evaluates the board
    /// positive -> advantage for white
    /// black -> advantage for black
    /// Unit = Centipawns, 100 Centipawns => 1 Pawn
    pub fn evaluate(&self) -> i32 {
        let mut valuation = self.evaluate_material();
    }

    pub fn evaluate_material(&self) {
        const PAWN: i32 = 100;
        const KNIGHT: i32 = 320;
        const BISHOP: i32 = 330;
        const ROOK: i32 = 500;
        const QUEEN: i32 = 900;
    }
}
