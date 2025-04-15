use crate::types::color::Color;
use crate::types::piece::Piece;

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

/// Returns the correct FIN Symbol by Piece and Color
pub fn get_fin_symbol(piece: Piece, color: Color) -> char {
    let char = match (piece, color) {
        (Piece::Empty, Color::None) => ' ',
        (Piece::Pawn, Color::Black) => 'p',
        (Piece::Knight, Color::Black) => 'n',
        (Piece::Rook, Color::Black) => 'r',
        (Piece::Bishop, Color::Black) => 'b',
        (Piece::Queen, Color::Black) => 'q',
        (Piece::King, Color::Black) => 'k',
        (Piece::Pawn, Color::White) => 'P',
        (Piece::Knight, Color::White) => 'N',
        (Piece::Rook, Color::White) => 'R',
        (Piece::Bishop, Color::White) => 'B',
        (Piece::Queen, Color::White) => 'Q',
        (Piece::King, Color::White) => 'K',
        (_, _) => 'E',
    };
    char
}
