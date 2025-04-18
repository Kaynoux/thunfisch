use crate::prelude::*;

pub fn idx_to_position(idx: isize) -> Position {
    Position(1u64 << idx)
}
