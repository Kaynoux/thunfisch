use crate::prelude::*;
const MATE_SCORE: i32 = -100000;
impl Board {
    /// Modified Mini Max algorithm which always picks the best move for each side until a given depth is reached and the evaluates the outcomes to pick the best move at the first layer
    pub fn negamax(&self, depth: usize, mut alpha: i32, mut beta: i32) -> (Option<ChessMove>, i32) {
        if depth == 0 {
            return (None, self.evaluate());
        }

        // set the best evaluation very low to begin with
        let mut best_eval = MATE_SCORE * 2;
        let mut best_move: Option<ChessMove> = None;

        let moves = self.get_legal_moves().1;

        // returns the mate score (very low) when in check but adds the depth to give a later check a better eval
        if moves.is_empty() {
            return (None, MATE_SCORE + (depth as i32));
        }

        //println!("{}", moves.len());
        for mv in moves {
            let mut bc = self.clone();
            bc.make_move(&mv);
            let eval = bc.negamax(depth - 1, -beta, -alpha).1;
            let eval = -eval;
            if eval > best_eval {
                best_eval = eval;
                best_move = Some(mv);
                if eval > alpha {
                    alpha = eval;
                }
            }
            if eval >= beta {
                return (best_move, best_eval);
            }
        }

        (best_move, best_eval)
    }
}
