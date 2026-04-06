use crate::search::transposition_table::TT;
mod communication;
mod debug;
mod move_generator;
mod prelude;
mod search;
mod settings;
mod types;
mod utils;

fn main() {
    // trigger lazy initialization before we do anything to avoid
    // paying that cost during a game
    TT.clear();

    // Starts UCI Communication via std in and out
    communication::uci::handle_uci_communication();
}
