use crate::prelude::*;

pub const fn get_rook_positions(pos: IndexPosition, occ: Bitboard) -> Bitboard {
    let mut attacks = Bitboard(0);
    let x = pos.0 % 8;
    let y = pos.0 / 8;

    let mut top_y = y + 1; // Start one row above
    while top_y < 8 {
        // Loop until row 8 exclusive
        let current_sq_idx = top_y * 8 + x;
        attacks.0 |= 1u64 << current_sq_idx; // Add this pos.0 to attacks
        // Check if the current pos.0 is occupied
        if (occ.0 >> current_sq_idx) & 1 != 0 {
            break; // Stop if blocked
        }
        top_y += 1; // Move to the next row up
    }

    let mut down_y = y;
    while down_y > 0 {
        down_y -= 1;
        let current_sq_idx = down_y * 8 + x;
        attacks.0 |= 1u64 << current_sq_idx;
        if (occ.0 >> current_sq_idx) & 1 != 0 {
            break;
        }
    }

    let mut right_x = x + 1;
    while right_x < 8 {
        let current_sq_idx = y * 8 + right_x;
        attacks.0 |= 1u64 << current_sq_idx;

        if (occ.0 >> current_sq_idx) & 1 != 0 {
            break;
        }
        right_x += 1;
    }

    let mut left_x = x;
    while left_x > 0 {
        left_x -= 1;
        let current_sq_idx = y * 8 + left_x;
        attacks.0 |= 1u64 << current_sq_idx;

        if (occ.0 >> current_sq_idx) & 1 != 0 {
            break;
        }
    }

    attacks
}

pub const fn get_bishop_positions(pos: IndexPosition, occ: Bitboard) -> Bitboard {
    let mut positions = Bitboard(0);
    let x = pos.0 % 8;
    let y = pos.0 / 8;

    let mut top_right_y = y + 1; // Start one row above
    let mut top_right_x = x + 1; // Start one column right
    // Loop until edge is reached
    while top_right_y < 8 && top_right_x < 8 {
        let current_sq_idx = top_right_y * 8 + top_right_x;
        positions.0 |= 1u64 << current_sq_idx; // Add this pos  to attacks
        // Check if the current pos.0 is occupied
        if (occ.0 >> current_sq_idx) & 1 != 0 {
            break; // Stop if blocked
        }
        // Move diagonally up-right
        top_right_y += 1;
        top_right_x += 1;
    }

    let mut down_right_y = y;
    let mut down_right_x = x + 1;
    while down_right_y > 0 && down_right_x < 8 {
        down_right_y -= 1;
        let current_sq_idx = down_right_y * 8 + down_right_x;
        positions.0 |= 1u64 << current_sq_idx;

        if (occ.0 >> current_sq_idx) & 1 != 0 {
            break;
        }
        down_right_x += 1;
    }

    let mut down_left_y = y;
    let mut down_left_x = x;
    while down_left_y > 0 && down_left_x > 0 {
        down_left_y -= 1;
        down_left_x -= 1;
        let current_sq_idx = down_left_y * 8 + down_left_x;
        positions.0 |= 1u64 << current_sq_idx;

        if (occ.0 >> current_sq_idx) & 1 != 0 {
            break;
        }
    }

    let mut top_left_y = y + 1;
    let mut top_left_x = x;
    while top_left_y < 8 && top_left_x > 0 {
        top_left_x -= 1;
        let current_sq_idx = top_left_y * 8 + top_left_x;
        positions.0 |= 1u64 << current_sq_idx;

        if (occ.0 >> current_sq_idx) & 1 != 0 {
            break;
        }
        top_left_y += 1;
    }

    positions
}
