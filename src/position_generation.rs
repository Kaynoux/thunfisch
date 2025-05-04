use crate::prelude::*;

pub static PAWN_ATTACKS: [[Bitboard; 64]; 2] = {
    let mut table = [[Bitboard(0); 64]; 2];
    let mut square = 0usize;
    while square < 64 {
        let x = square % 8;
        let y = square / 8;

        // White Pawns attack up
        if y < 7 {
            // no left attack on x = 0
            if x > 0 {
                table[0][square].0 |= 1 << ((y + 1) * 8 + (x - 1));
            }
            // no right attack on x = 7
            if x < 7 {
                table[0][square].0 |= 1 << ((y + 1) * 8 + (x + 1));
            }
        }

        // Black Pawns attack down
        if y > 0 {
            // no left attack on x = 0
            if x > 0 {
                table[0][square].0 |= 1 << ((y - 1) * 8 + (x - 1));
            }
            // no right attack on x = 7
            if x < 7 {
                table[0][square].0 |= 1 << ((y - 1) * 8 + (x + 1));
            }
        }
        square += 1;
    }

    table
};

pub static KNIGHT_ATTACKS: [Bitboard; 64] = {
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

    let mut square = 0usize;
    while square < 64 {
        let x = (square % 8) as isize;
        let y = (square / 8) as isize;

        let mut idx = 0;
        while idx < offsets.len() {
            let (dx, dy) = offsets[idx];
            let tx = x + dx;
            let ty = y + dy;
            if tx >= 0 && tx <= 7 && ty >= 0 && ty <= 7 {
                let to = (ty * 8 + tx) as usize;
                table[square].0 |= 1 << to;
            }
            idx += 1;
        }
        square += 1;
    }

    table
};

pub static KING_ATTACKS: [Bitboard; 64] = {
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

    let mut square = 0usize;
    while square < 64 {
        let x = (square % 8) as isize;
        let y = (square / 8) as isize;

        let mut idx = 0;
        while idx < offsets.len() {
            let (dx, dy) = offsets[idx];
            let tx = x + dx;
            let ty = y + dy;
            if tx >= 0 && tx <= 7 && ty >= 0 && ty <= 7 {
                let to = (ty * 8 + tx) as usize;
                table[square].0 |= 1 << to;
            }
            idx += 1;
        }
        square += 1;
    }

    table
};

pub static ROOK_MASK: [Bitboard; 64] = {
    let mut masks = [Bitboard(0); 64];
    let mut square = 0usize;
    while square < 64 {
        let r = square / 8;
        let f = square % 8;
        let mut mask = Bitboard(0);

        // Nach Norden (ohne Reihe 7)
        let mut nr = r + 1;
        while nr < 7 {
            mask.0 |= 1u64 << (nr * 8 + f);
            nr += 1;
        }
        // Nach Süden (ohne Reihe 0)
        let mut sr = r;
        while sr > 0 {
            sr -= 1; // Dekrementieren vor der Prüfung auf 0
            if sr == 0 {
                break;
            } // Reihe 0 nicht maskieren
            mask.0 |= 1u64 << (sr * 8 + f);
        }
        // Nach Osten (ohne Spalte 7)
        let mut ef = f + 1;
        while ef < 7 {
            mask.0 |= 1u64 << (r * 8 + ef);
            ef += 1;
        }
        // Nach Westen (ohne Spalte 0)
        let mut wf = f;
        while wf > 0 {
            wf -= 1; // Dekrementieren vor der Prüfung auf 0
            if wf == 0 {
                break;
            } // Spalte 0 nicht maskieren
            mask.0 |= 1u64 << (r * 8 + wf);
        }
        masks[square] = mask;
        square += 1; // Inkrementieren am Ende der äußeren Schleife
    }
    masks
};

#[inline(always)]
pub fn get_pawn_positions(
    board: &Board,
    pos: Position,
    color: Color,
    only_captures: bool,
) -> Bitboard {
    let mut moves_to_empty = Bitboard(0);
    let mut moves_to_enemy = Bitboard(0);
    let move_direction_y = match color {
        Color::Black => -1,
        Color::White => 1,
    };

    // Add the to possible Strike moves
    moves_to_enemy |= pos.get_offset_pos(-1, move_direction_y);
    moves_to_enemy |= pos.get_offset_pos(1, move_direction_y);

    // Positions need to be enemy to be valid
    moves_to_enemy &= board.get_pieces_by_color(!color);

    if only_captures {
        // early return if only enemy pos are needed
        return moves_to_enemy;
    }

    moves_to_empty |= pos.get_offset_pos(0, move_direction_y);

    // Add possible move by 2 when pawn has not moved in the match and position in front is empty
    match (color, pos.to_index().0 / 8) {
        (Color::Black, 6) => {
            if board
                .get_empty_pieces()
                .is_position_set(pos.get_offset_pos(0, -1))
            {
                moves_to_empty |= pos.get_offset_pos(0, -2)
            }
        }
        (Color::White, 1) => {
            if board
                .get_empty_pieces()
                .is_position_set(pos.get_offset_pos(0, 1))
            {
                moves_to_empty |= pos.get_offset_pos(0, 2)
            }
        }
        (_, _) => {}
    }

    // Positions need to be empty to be valid
    moves_to_empty &= board.get_empty_pieces();

    moves_to_empty | moves_to_enemy
}

#[inline(always)]
pub fn get_pawn_attack_positions(board: &Board, pos: Position, color: Color) -> Bitboard {
    let mut moves_to_enemy = Bitboard(0);
    let non_friendly_pieces = board.get_non_friendly_pieces(color);
    let move_direction_y = match color {
        Color::Black => -1,
        Color::White => 1,
    };
    // Add the to possible Strike moves
    moves_to_enemy |= pos.get_offset_pos(-1, move_direction_y);
    moves_to_enemy |= pos.get_offset_pos(1, move_direction_y);
    moves_to_enemy &= non_friendly_pieces;

    moves_to_enemy
}

#[inline(always)]
pub fn get_king_positions(board: &Board, pos: Position, color: Color) -> Bitboard {
    let mut moves = Bitboard(0);
    let non_friendly_pieces = board.get_non_friendly_pieces(color);
    moves |= pos.get_offset_pos(-1, 1);
    moves |= pos.get_offset_pos(0, 1);
    moves |= pos.get_offset_pos(1, 1);
    moves |= pos.get_offset_pos(-1, 0);
    moves |= pos.get_offset_pos(1, 0);
    moves |= pos.get_offset_pos(-1, -1);
    moves |= pos.get_offset_pos(0, -1);
    moves |= pos.get_offset_pos(1, -1);
    moves & non_friendly_pieces
}

#[inline(always)]
pub fn get_knight_positions(board: &Board, pos: Position, color: Color) -> Bitboard {
    let mut moves = Bitboard(0);
    let non_friendly_pieces = board.get_non_friendly_pieces(color);
    moves |= pos.get_offset_pos(-2, 1);
    moves |= pos.get_offset_pos(-1, 2);
    moves |= pos.get_offset_pos(1, 2);
    moves |= pos.get_offset_pos(2, 1);
    moves |= pos.get_offset_pos(-2, -1);
    moves |= pos.get_offset_pos(-1, -2);
    moves |= pos.get_offset_pos(1, -2);
    moves |= pos.get_offset_pos(2, -1);
    moves & non_friendly_pieces
}

#[inline(always)]
pub fn get_sliding_positions(
    board: &Board,
    pos: Position,
    color: Color,
    dx: isize,
    dy: isize,
) -> Bitboard {
    let mut moves = Bitboard(0);
    let non_friendly_pieces = board.get_non_friendly_pieces(color);
    let mut current_dx = 0isize;
    let mut current_dy = 0isize;
    loop {
        current_dx += dx;
        current_dy += dy;
        let current_pos = pos.get_offset_pos(current_dx, current_dy);
        if current_pos == Position(0) {
            break;
        }

        if current_pos.is_friendly(board, color) {
            break;
        }

        moves |= current_pos;

        if current_pos.is_enemy(board, color) {
            break;
        }
    }
    moves & non_friendly_pieces
}

#[inline(always)]
pub fn get_queen_positions(board: &Board, pos: Position, color: Color) -> Bitboard {
    let mut moves = Bitboard(0);
    moves |= get_sliding_positions(board, pos, color, 1, -1);
    moves |= get_sliding_positions(board, pos, color, 1, 0);
    moves |= get_sliding_positions(board, pos, color, 1, 1);
    moves |= get_sliding_positions(board, pos, color, 0, -1);
    moves |= get_sliding_positions(board, pos, color, 0, 1);
    moves |= get_sliding_positions(board, pos, color, -1, -1);
    moves |= get_sliding_positions(board, pos, color, -1, 0);
    moves |= get_sliding_positions(board, pos, color, -1, 1);
    moves
}

#[inline(always)]
pub fn get_bishop_positions(board: &Board, pos: Position, color: Color) -> Bitboard {
    let mut moves = Bitboard(0);
    moves |= get_sliding_positions(board, pos, color, 1, -1);
    moves |= get_sliding_positions(board, pos, color, 1, 1);
    moves |= get_sliding_positions(board, pos, color, -1, -1);
    moves |= get_sliding_positions(board, pos, color, -1, 1);
    moves
}

#[inline(always)]
pub fn get_rook_positions(board: &Board, pos: Position, color: Color) -> Bitboard {
    let mut moves = Bitboard(0);
    moves |= get_sliding_positions(board, pos, color, 1, 0);
    moves |= get_sliding_positions(board, pos, color, 0, -1);
    moves |= get_sliding_positions(board, pos, color, 0, 1);
    moves |= get_sliding_positions(board, pos, color, -1, 0);
    moves
}
