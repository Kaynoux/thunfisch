use thunfisch::prelude::*;
mod evaluation;
mod quiescence_search;

fn main() {
    // tuning entrypoint — use thunfisch crate types directly
    println!("tuning: thunfisch available: Board start fen = {}", thunfisch::types::board::START_POS);
}
