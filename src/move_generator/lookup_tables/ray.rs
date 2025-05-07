use crate::move_generator::move_generator;
use crate::move_generator::move_generator::get_bishop_targets;
use crate::move_generator::move_generator::get_rook_targets;
use crate::prelude::*;

// U=Up D=Down L=Left R=Right
const U: usize = 0;
const UR: usize = 1;
const R: usize = 2;
const DR: usize = 3;
const D: usize = 4;
const DL: usize = 5;
const L: usize = 6;
const UL: usize = 7;

const DIRECTION_OFFSETS: [(isize, isize); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

const fn get_ray_positions_between(from: Square, to: Square) -> Bitboard {
    let from_bb = from.to_bitboard();
    let to_bb = to.to_bitboard();
    let target_pos_1 =
        Bitboard(get_rook_targets(from, to_bb).0 | get_bishop_targets(from, to_bb).0);
    let target_pos_2 =
        Bitboard(get_rook_targets(to, from_bb).0 | get_bishop_targets(to, from_bb).0);

    Bitboard(target_pos_1.0 & target_pos_2.0)
}

pub const RAY_BETWEEN: [[Bitboard; 64]; 64] = {
    let mut out = [[Bitboard(0); 64]; 64];
    let mut from = Square(0);
    while from.0 < 64 {
        let mut to = Square(0);
        while to.0 < 64 {
            out[from.0][to.0] = get_ray_positions_between(from, to);
            to.next();
        }
        from.next()
    }
    out
};

pub const RAY_DIRECTION: [[Bitboard; 8]; 64] = {
    let mut out = [[Bitboard(0); 8]; 64];
    let mut from = Square(0);
    while from.0 < 64 {
        let mut dir_idx = 0;
        while dir_idx < 8 {
            let (dx, dy) = DIRECTION_OFFSETS[dir_idx as usize];
            let to = get_edge_square(from, dx, dy);
            out[from.i()][dir_idx] =
                Bitboard(RAY_BETWEEN[from.i()][to.i()].0 | (to.to_bitboard()).0); // also add the edge position
            dir_idx += 1;
        }
        from.next()
    }
    out
};

const fn get_edge_square(sq: Square, dx: isize, dy: isize) -> Square {
    let mut current_x = (sq.0 % 8) as isize;
    let mut current_y = (sq.0 / 8) as isize;
    let mut last_valid_x = current_x;
    let mut last_valid_y = current_y;

    let next_x = current_x + dx;
    let next_y = current_y + dy;
    if next_x < 0 || next_x >= 8 || next_y < 0 || next_y >= 8 {
        return sq;
    }
    current_x = next_x;
    current_y = next_y;
    last_valid_x = current_x;
    last_valid_y = current_y;

    loop {
        current_x += dx;
        current_y += dy;
        if current_x < 0 || current_x >= 8 || current_y < 0 || current_y >= 8 {
            return Square::from_xy(last_valid_x as usize, last_valid_y as usize);
        }
        last_valid_x = current_x;
        last_valid_y = current_y;
    }
}
