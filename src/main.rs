mod debug;
mod evaluate;
mod position_generation;
mod prelude;
mod pseudo_legal_move_generation;
mod search;
mod test;
mod types;
mod uci;
fn main() {
    // Starts UCI Communication via std in and out
    uci::handle_uci_communication();
}
