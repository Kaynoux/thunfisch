#![warn(clippy::trivially_copy_pass_by_ref)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::undocumented_unsafe_blocks
)]
#![warn(missing_docs)]

//! Thunfisch is a UCI chess engine
use crate::search::transposition_table::TT;
mod communication;
mod debug;
mod move_generator;
mod prelude;
mod search;
mod settings;
mod types;
mod utils;

#[cfg(not(target_pointer_width = "64"))]
compile_error!("Dieses Projekt unterstützt nur 64-Bit-Systeme.");

fn main() {
    // trigger lazy initialization before we do anything to avoid
    // paying that cost during a game
    TT.clear();

    // Starts UCI Communication via std in and out
    communication::uci::handle_uci_communication();
}
