use crate::communication::get_fin_symbol;
use crate::move_generation::{
    get_pseudo_legal_bishop_moves, get_pseudo_legal_king_moves, get_pseudo_legal_knight_moves,
    get_pseudo_legal_queen_moves, get_pseudo_legal_rook_moves,
};
use crate::types::bitboard::Bitboard
use crate::types::position::Position
use crate::types::board::Board
use crate::types::bitboard::Bitboard

use crate::utils::{bitboard_to_xy_list, idx_to_bit, pos_to_color_and_piece};

pub fn print_all_legal_moves(board: &Board) {
    for pos_idx in 0..64 {
        let pos_bit: Bit = idx_to_bit(pos_idx);

        let (color, piece) = pos_to_color_and_piece(board, pos_bit);
        match piece {
            // Piece::Pawn => {
            //     let moves = get_pseudo_legal_pawn_moves(board, pos_idx, color);
            //     print_moves_as_row(color, piece, moves);
            //     print_moves_as_board(board, color, piece, moves);
            // }
            // Piece::Knight => {
            //     let moves = get_pseudo_legal_knight_moves(board, pos_idx, color);
            //     print_moves_as_row(color, piece, moves);
            //     print_moves_as_board(board, color, piece, moves);
            // }
            // Piece::Bishop => {
            //     let moves = get_pseudo_legal_bishop_moves(board, pos_idx, color);
            //     print_moves_as_row(color, piece, moves);
            //     print_moves_as_board(board, color, piece, moves);
            // }
            // Piece::Rook => {
            //     let moves = get_pseudo_legal_rook_moves(board, pos_idx, color);
            //     print_moves_as_row(color, piece, moves);
            //     print_moves_as_board(board, color, piece, moves);
            // }
            Piece::Queen => {
                let moves = get_pseudo_legal_queen_moves(board, pos_idx, color);
                print_moves_as_row(color, piece, moves);
                print_moves_as_board(board, color, piece, moves);
            }
            // Piece::King => {
            //     let moves = get_pseudo_legal_king_moves(board, pos_idx, color);
            //     print_moves_as_row(color, piece, moves);
            //     print_moves_as_board(board, color, piece, moves);
            // }
            _ => {}
        }
    }
}

/// Prints the current board formatted
/// lowercase letters = black and uppercase letters = white    
pub fn print_board_as_board(board: &Board) {
    let mut char_board: [char; 64] = [' '; 64];
    for idx in 0..64 {
        let curr_pos: Bit = idx_to_bit(idx);
        let (color, piece) = pos_to_color_and_piece(board, curr_pos);
        char_board[idx] = get_fin_symbol(piece, color);
    }
}

/// Prints moves on board formatted
pub fn print_moves_as_board(board: &Board, color: Color, piece: Piece, moves: Bitboard) {
    let mut char_board: [char; 64] = [' '; 64];
    for idx in 0..64 {
        let curr_pos = idx_to_bit(idx);
        if moves & curr_pos != 0 {
            char_board[idx] = 'X';
        } else {
            let (color, piece) = pos_to_color_and_piece(board, curr_pos);
            char_board[idx] = get_fin_symbol(piece, color);
        };
    }

    print_board(
        &format!(
            "Potential Moves from: {}  Amount: {} ",
            get_fin_symbol(piece, color),
            moves.count_ones()
        ),
        char_board,
    );
}

fn print_board(title: &str, char_board: [char; 64]) {
    let mut y: i32 = 7;
    let mut x: i32 = 0;

    println!();
    println!("{}", title);
    println!("    0   1   2   3   4   5   6   7");

    while y >= 0 {
        println!("  ---------------------------------");
        print!("{} |", y);
        while x <= 7 {
            print!(" {} |", char_board[(y * 8 + x) as usize]);

            x += 1;
        }
        x = 0;
        y -= 1;

        println!();
    }
}

pub fn print_moves_as_row(color: Color, piece: Piece, moves: Bitboard) {
    println!(
        "Potential Moves from: {}  Amount: {} ",
        get_fin_symbol(piece, color),
        moves.count_ones()
    );

    let moves_list = bitboard_to_xy_list(moves);
    println!("{:?}", moves_list);
}
