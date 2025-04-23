mod debug;
mod evaluate;
mod position_generation;
mod prelude;
mod pseudo_legal_move_generation;
mod types;
mod uci;
fn main() {
    uci::handle_uci_communication();
}
