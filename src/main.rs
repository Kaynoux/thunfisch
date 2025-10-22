mod communication;
mod debug;
mod move_generator;
mod prelude;
mod search;
mod settings;
mod types;
mod utils;

fn main() {
    // Starts UCI Communication via std in and out
    communication::uci::handle_uci_communication();
}
