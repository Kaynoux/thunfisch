use crate::types::{Board, Color, Piece};
use log::error;
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

/// Converts a FEN to a Board
/// FEN describes the position of all pieces on the board
/// lowercase = black and uppercase = white
pub fn parse_fen(fen: &str, board: &mut Board) {
    let mut index: usize = 0;

    for c in fen.chars() {
        // Shift 1 as u64 by index amount of bits to the left
        let current_bit = 1u64 << index;
        // Sets the current_bit in the bitmap of the corresponding field
        match c {
            '/' => index = (index + 7) / 8 * 8,
            '1'..'9' => index += c.to_digit(10).unwrap_or(0) as usize,
            'p' => {
                board.black_pawns |= current_bit;
                index += 1
            }
            'n' => {
                board.black_knights |= current_bit;
                index += 1
            }
            'b' => {
                board.black_bishops |= current_bit;
                index += 1
            }
            'r' => {
                board.black_rooks |= current_bit;
                index += 1
            }
            'q' => {
                board.black_queen |= current_bit;
                index += 1
            }
            'k' => {
                board.black_king |= current_bit;
                index += 1
            }
            'P' => {
                board.white_pawns |= current_bit;
                index += 1
            }
            'N' => {
                board.white_knights |= current_bit;
                index += 1
            }
            'B' => {
                board.white_bishops |= current_bit;
                index += 1
            }
            'R' => {
                board.white_rooks |= current_bit;
                index += 1
            }
            'Q' => {
                board.white_queen |= current_bit;
                index += 1
            }
            'K' => {
                board.white_king |= current_bit;
                index += 1
            }

            _ => {
                error!("Invalid Character")
            }
        }
    }

    board.white_pieces = board.white_pawns
        | board.white_knights
        | board.white_bishops
        | board.white_rooks
        | board.white_queen
        | board.white_king;

    board.black_pieces = board.black_pawns
        | board.black_knights
        | board.black_bishops
        | board.black_rooks
        | board.black_queen
        | board.black_king;

    board.empty_pieces = !(board.white_pieces | board.black_pieces);
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
