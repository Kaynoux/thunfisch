use std::io::{self, BufRead};

pub fn handle_uci_communication() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input = line.unwrap();
        match input.as_str() {
            "uci" => {
                println!("id name rusty-chess-bot");
                println!("id author Lukas");
                println!("uciok");
            }
            "isready" => {
                println!("readyok");
            }
            "quit" => {
                break;
            }
            _ => {
                println!("Error")
            }
        }
    }
}
