mod debug;
mod evaluate;
mod move_generation;
mod move_generator;
mod position_generation;
mod prelude;
mod search;
mod test;
mod types;
mod uci;
fn main() {
    // Starts UCI Communication via std in and out
    uci::handle_uci_communication();
}
