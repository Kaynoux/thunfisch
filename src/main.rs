use thunfisch::communication;

fn main() {
    // Starts UCI Communication via std in and out
    communication::uci::handle_uci_communication();
}
