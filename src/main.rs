mod communication;
mod debug;
mod move_generator;
mod prelude;
mod search;
mod types;
mod utils;

use std::io::{self, BufReader, BufWriter};

fn main() {
    // Starts UCI Communication via std in and out

    let stdin = io::stdin();
    let stdout = io::stdout();
    let reader = BufReader::new(stdin.lock());
    let writer = BufWriter::new(stdout.lock());
    if let Err(e) = communication::uci::handle_uci(reader, writer) {
        eprintln!("Error handling UCI: {}", e);
    }
}
