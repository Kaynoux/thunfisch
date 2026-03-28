use crate::prelude::*;

/// Exclusive - a depth == MAX_DEPTH is out of bounds.
pub const MAX_DEPTH: usize = 128;

pub struct PVTable {
    table: [EncodedMove; MAX_DEPTH * (MAX_DEPTH - 1) / 2 + 1],
    current_length: usize,
}

const TRIANGULAR_INDEX: [usize; MAX_DEPTH] = init_index_table();

const fn init_index_table() -> [usize; MAX_DEPTH] {
    let mut table = [0; MAX_DEPTH];
    let mut i = 0;
    let mut current_idx = 0;
    while i < MAX_DEPTH {
        table[i] = current_idx;
        current_idx += MAX_DEPTH - 1 - i;
        i += 1;
    }
    table
}

impl PVTable {
    pub fn new() -> PVTable {
        PVTable {
            table: [EncodedMove(0); MAX_DEPTH * (MAX_DEPTH - 1) / 2 + 1],
            current_length: 0,
        }
    }

    /// Push a move on to the PV table.
    /// Currently does unchecked access on the array; so if ply > MAX_DEPTH everything goes up in flames.
    pub fn push_move(&mut self, ply: usize, mv: EncodedMove) {
        assert!(ply < MAX_DEPTH, "Attempted to store a PV past MAX_DEPTH");

        self.table[TRIANGULAR_INDEX[ply]] = mv;

        if ply >= self.current_length {
            self.current_length = ply + 1;
            // no need to copy up moves since we reached a new maximum depth
            return;
        }

        // copy PV from higher depth to this depth.
        for i in 1..(MAX_DEPTH - ply) {
            self.table[TRIANGULAR_INDEX[ply] + i] = self.table[TRIANGULAR_INDEX[ply + 1] + i - 1];
        }
    }

    /// Get the entire PV stored in the PV table starting from `ply`.
    pub fn get_pv(&self, ply: usize) -> &[EncodedMove] {
        &self
            .table
            .get(TRIANGULAR_INDEX[ply]..(TRIANGULAR_INDEX[ply] + self.current_length))
            .expect("Attempted to read a PV past MAX_DEPTH")
    }

    /// Get the Best move from the PV at `ply_from_root`.
    pub fn get_bestmove(&self, ply_from_root: usize) -> Option<EncodedMove> {
        return match self.table[TRIANGULAR_INDEX[ply_from_root]] {
            EncodedMove(0) => None,
            mv => Some(mv),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangular_index_table() {
        let pvt = PVTable::new();
        // print!("{:?}", TRIANGULAR_INDEX);
        assert_eq!(TRIANGULAR_INDEX[0], 0);
        assert_eq!(TRIANGULAR_INDEX[1], 127);
        assert_eq!(TRIANGULAR_INDEX[2], 253);
        assert_eq!(TRIANGULAR_INDEX[MAX_DEPTH - 1], pvt.table.len() - 1);
    }
}
