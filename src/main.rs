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
mod alpha_beta;
mod communication;
mod debug;
mod evaluation;
mod evaluation_constants;
mod iterative_deepening;
mod move_generator;
mod move_picker;
mod move_scoring;
mod prelude;
mod quiescence_search;
mod settings;
mod time_management;
mod transposition_table;
mod types;
mod utils;
fn main() {
    // Starts UCI Communication via std in and out
    communication::handle_communication();
}
