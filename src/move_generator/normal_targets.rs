use crate::prelude::*;

pub static PAWN_ATTACK_TARGETS: [[Bitboard; 64]; 2] = {
    let mut table = [[Bitboard(0); 64]; 2];
    let mut pos = Square(0);
    while pos.0 < 64 {
        let x = pos.0 % 8;
        let y = pos.0 / 8;

        // White Pawns attack up
        if y < 7 {
            // no left attack on x = 0
            if x > 0 {
                table[0][pos.0].0 |= 1 << ((y + 1) * 8 + (x - 1));
            }
            // no right attack on x = 7
            if x < 7 {
                table[0][pos.0].0 |= 1 << ((y + 1) * 8 + (x + 1));
            }
        }

        // Black Pawns attack down
        if y > 0 {
            // no left attack on x = 0
            if x > 0 {
                table[0][pos.0].0 |= 1 << ((y - 1) * 8 + (x - 1));
            }
            // no right attack on x = 7
            if x < 7 {
                table[0][pos.0].0 |= 1 << ((y - 1) * 8 + (x + 1));
            }
        }
        pos.0 += 1;
    }

    table
};

pub static KNIGHT_TARGETS: [Bitboard; 64] = {
    let mut table = [Bitboard(0); 64];
    let offsets = [
        (-2, -1),
        (-2, 1),
        (-1, -2),
        (-1, 2),
        (1, -2),
        (1, 2),
        (2, -1),
        (2, 1),
    ];

    let mut pos = Square(0);
    while pos.0 < 64 {
        let x = (pos.0 % 8) as isize;
        let y = (pos.0 / 8) as isize;

        let mut idx = 0;
        while idx < offsets.len() {
            let (dx, dy) = offsets[idx];
            let tx = x + dx;
            let ty = y + dy;
            if tx >= 0 && tx <= 7 && ty >= 0 && ty <= 7 {
                let to = (ty * 8 + tx) as usize;
                table[pos.0].0 |= 1 << to;
            }
            idx += 1;
        }
        pos.0 += 1;
    }

    table
};

pub static KING_TARGETS: [Bitboard; 64] = {
    let mut table = [Bitboard(0); 64];
    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut pos = Square(0);
    while pos.0 < 64 {
        let x = (pos.0 % 8) as isize;
        let y = (pos.0 / 8) as isize;

        let mut idx = 0;
        while idx < offsets.len() {
            let (dx, dy) = offsets[idx];
            let tx = x + dx;
            let ty = y + dy;
            if tx >= 0 && tx <= 7 && ty >= 0 && ty <= 7 {
                let to = (ty * 8 + tx) as usize;
                table[pos.0].0 |= 1 << to;
            }
            idx += 1;
        }
        pos.0 += 1;
    }

    table
};

/// Retruns a valid single target pos
pub fn pawn_quiet_single_target(from: Bit, color: Color) -> Bit {
    match color {
        Color::White => from << 8,
        Color::Black => from >> 8,
    }
}
