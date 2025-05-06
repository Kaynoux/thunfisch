use crate::prelude::*;
// Arrray that returns all positions which block a rook movement in this direction
// the corner rows and collumns always block the rook so they are irrelevant
//. . . . 1 . . .
//. . . . 1 . . .
//. . . . 1 . . .
//. 1 1 1 X 1 1 .
//. . . . 1 . . .
//. . . . 1 . . .
//. . . . 1 . . .
//. . . . . . . .
pub static ROOK_BLOCKERS: [Bitboard; 64] = {
    let mut blockers = [Bitboard(0); 64];
    let mut pos = IndexPosition(0);
    while pos.0 < 64 {
        let x = pos.0 % 8;
        let y = pos.0 / 8;
        let mut blocker = Bitboard(0);

        let mut up = y + 1;
        while up < 7 {
            blocker.0 |= 1u64 << (up * 8 + x);
            up += 1;
        }

        let mut down = y;
        while down > 0 {
            down -= 1;
            if down == 0 {
                break;
            }
            blocker.0 |= 1u64 << (down * 8 + x);
        }

        let mut right = x + 1;
        while right < 7 {
            blocker.0 |= 1u64 << (y * 8 + right);
            right += 1;
        }

        let mut left = x;
        while left > 0 {
            left -= 1;
            if left == 0 {
                break;
            }
            blocker.0 |= 1u64 << (y * 8 + left);
        }
        blockers[pos.0] = blocker;
        pos.0 += 1;
    }
    blockers
};

// Array that returns all positions which block a bishop movement in this direction
// the corner rows and columns always block the bishop so they are irrelevant
// . . . . . . . .
// . 1 . . . 1 . .
// . . 1 . 1 . . .
// . . . X . . . .
// . . 1 . 1 . . .
// . 1 . . . 1 . .
// . . . . . . 1 .
// . . . . . . . .
pub const BISHOP_BLOCKERS: [Bitboard; 64] = {
    let mut blockers = [Bitboard(0); 64];
    let mut pos = IndexPosition(0);
    while pos.0 < 64 {
        let x = pos.0 % 8;
        let y = pos.0 / 8;
        let mut blocker = Bitboard(0);

        let mut top_right_x = x + 1;
        let mut top_right_y = y + 1;
        while top_right_x < 7 && top_right_y < 7 {
            blocker.0 |= 1u64 << (top_right_y * 8 + top_right_x);
            top_right_x += 1;
            top_right_y += 1;
        }

        let mut down_right_x = x + 1;
        let mut down_right_y = y;
        while down_right_x < 7 && down_right_y > 0 {
            down_right_y -= 1;

            blocker.0 |= 1u64 << (down_right_y * 8 + down_right_x);
            down_right_x += 1;
        }

        let mut down_left_x = x;
        let mut down_left_y = y;
        while down_left_x > 0 && down_left_y > 0 {
            down_left_x -= 1;
            down_left_y -= 1;
            blocker.0 |= 1u64 << (down_left_y * 8 + down_left_x);
        }

        let mut up_left_x = x;
        let mut up_left_y = y + 1;
        while up_left_x > 0 && up_left_y < 7 {
            up_left_x -= 1;

            blocker.0 |= 1u64 << (up_left_y * 8 + up_left_x);
            up_left_y += 1;
        }

        blockers[pos.0] = blocker;
        pos.0 += 1;
    }
    blockers
};

// Queen Mask is just bitwise or of rook and bishops
pub const QUEEN_BLOCKERS: [Bitboard; 64] = {
    let mut blockers = [Bitboard(0); 64];
    let mut pos = IndexPosition(0); // Index Position
    while pos.0 < 64 {
        let mut blocker = Bitboard(0);
        blocker.0 |= ROOK_BLOCKERS[pos.0].0;
        blocker.0 |= BISHOP_BLOCKERS[pos.0].0;
        blockers[pos.0] = blocker;
        pos.0 += 1;
    }
    blockers
};
