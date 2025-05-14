use crate::prelude::*;

/// Return all Pos between from and to both exclusive
pub const IN_BETWEEN: [[Bitboard; 64]; 64] = {
    let mut arr = [[Bitboard::EMPTY; 64]; 64];
    let mut from = 0;
    while from < 64 {
        let mut to = 0;
        while to < 64 {
            arr[from][to] = init_in_between(Square(from), Square(to));
            to += 1;
        }
        from += 1;
    }
    arr
};

/// Returns all pos on a line between from (incluseive) and to (inclusive)
#[allow(unused)]
pub const LINE_THROUGH: [[Bitboard; 64]; 64] = {
    let mut arr = [[Bitboard::EMPTY; 64]; 64];
    let mut from = 0;
    while from < 64 {
        let mut to = 0;
        while to < 64 {
            arr[from][to] = init_line_through(Square(from), Square(to));
            to += 1;
        }
        from += 1;
    }
    arr
};

const fn init_in_between(from: Square, to: Square) -> Bitboard {
    if from.0 == to.0 {
        return Bitboard::EMPTY;
    }

    let from_x = from.x();
    let from_y = from.y();
    let to_x = to.x();
    let to_y = to.y();

    let mut result_bb = 0u64;

    // same y
    if from_y == to_y {
        let step: isize = if from_x < to_x { 1 } else { -1 };
        let mut current_x = from_x as isize + step;
        while current_x != to_x as isize {
            if current_x < 0 || current_x > 7 {
                break; // Should not happen if sq1 and sq2 are on the same line
            }
            result_bb |= 1u64 << (from_y * 8 + current_x as usize);
            current_x += step;
        }
    }
    // same x
    else if from_x == to_x {
        let step: isize = if from_y < to_y { 1 } else { -1 };
        let mut current_y = from_y as isize + step;
        while current_y != to_y as isize {
            if current_y < 0 || current_y > 7 {
                break;
            }
            result_bb |= 1u64 << (current_y as usize * 8 + from_x);
            current_y += step;
        }
    }
    // Check for diagonal  a1h8
    else if (from_y as isize - from_x as isize) == (to_y as isize - to_x as isize) {
        let step_r: isize = if from_y < to_y { 1 } else { -1 };
        let step_f: isize = if from_x < to_x { 1 } else { -1 };
        if step_r == step_f {
            let mut current_y = from_y as isize + step_r;
            let mut current_x = from_x as isize + step_f;
            while current_y != to_y as isize && current_x != to_x as isize {
                if current_y < 0 || current_y > 7 || current_x < 0 || current_x > 7 {
                    break;
                }
                result_bb |= 1u64 << (current_y as usize * 8 + current_x as usize);
                current_y += step_r;
                current_x += step_f;
            }
        }
    }
    // other diagonal
    else if (from_y + from_x) == (to_y + to_x) {
        let step_r: isize = if from_y < to_y { 1 } else { -1 };
        let step_f: isize = if from_x < to_x { 1 } else { -1 };

        if step_r == -step_f {
            let mut current_y = from_y as isize + step_r;
            let mut current_x = from_x as isize + step_f;
            while current_y != to_y as isize && current_x != to_x as isize {
                if current_y < 0 || current_y > 7 || current_x < 0 || current_x > 7 {
                    break;
                }
                result_bb |= 1u64 << (current_y as usize * 8 + current_x as usize);
                current_y += step_r;
                current_x += step_f;
            }
        }
    }

    Bitboard(result_bb)
}

const fn init_line_through(from: Square, to: Square) -> Bitboard {
    let from_x = from.x();
    let from_y = from.y();
    let to_y = to.y();
    let to_x = to.x();

    let mut result_bb = 0u64;

    // Same y iterate through x
    if from_x == to_x {
        let mut x_iter = 0;
        while x_iter < 8 {
            result_bb |= 1u64 << (x_iter * 8 + from_x);
            x_iter += 1;
        }
        return Bitboard(result_bb);
    }

    // Same x iterate through y
    if from_y == to_y {
        let mut y_iter = 0;
        while y_iter < 8 {
            result_bb |= 1u64 << (from_y * 8 + y_iter);
            y_iter += 1;
        }
        return Bitboard(result_bb);
    }

    // a1h8 diagonal
    if (from_y as isize - from_x as isize) == (to_y as isize - to_x as isize) {
        let diag_const = from_y as isize - from_x as isize;
        let mut x_iter = 0;
        while x_iter < 8 {
            let y_calc = x_iter as isize - diag_const;
            if y_calc >= 0 && y_calc <= 7 {
                result_bb |= 1u64 << (x_iter * 8 + y_calc as usize);
            }
            x_iter += 1;
        }
        return Bitboard(result_bb);
    }

    // h1a8 diagonal
    if (from_y + from_x) == (to_y + to_x) {
        let anti_diag_const = from_y + from_x;
        let mut x_iter = 0;
        while x_iter < 8 {
            let y_calc = anti_diag_const as isize - x_iter as isize;
            if y_calc >= 0 && y_calc <= 7 {
                result_bb |= 1u64 << (x_iter * 8 + y_calc as usize);
            }
            x_iter += 1;
        }
        return Bitboard(result_bb);
    }

    Bitboard::EMPTY // if to is not on any straight line from from square
}
