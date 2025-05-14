use crate::prelude::*;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU32, AtomicU64, AtomicUsize, Ordering};

/// Single Entry in the transposition table
/// my move has 16 bits but it has currently no way of storing null moves so I use an u32 as a tempory solution
/// this needs to be fixed in the future
struct TTEntry {
    key: AtomicU64,
    mv: AtomicU32,
}

/// https://www.chessprogramming.org/Transposition_Table
pub struct TranspositionTable {
    entries: Vec<TTEntry>,
    mask: usize,
    filled: AtomicUsize,
}

pub static TT: Lazy<TranspositionTable> = Lazy::new(|| TranspositionTable::new(128));

impl TranspositionTable {
    pub fn new(mb: usize) -> Self {
        let bytes = mb * 1024 * 1024;
        // 12 because 64bit + 32bit = 12 byte
        let entry_size = 12;
        let cap = (bytes / entry_size).next_power_of_two();
        let entries = (0..cap)
            .map(|_| TTEntry {
                key: AtomicU64::new(0),
                mv: AtomicU32::new(0),
            })
            .collect();
        TranspositionTable {
            entries,
            mask: cap - 1,
            filled: AtomicUsize::new(0),
        }
    }

    #[inline]
    fn index(&self, hash: u64) -> usize {
        (hash as usize) & self.mask
    }

    pub fn probe(&self, hash: u64) -> Option<EncodedMove> {
        let e = &self.entries[self.index(hash)];
        if e.key.load(Ordering::Relaxed) == hash {
            let raw = e.mv.load(Ordering::Relaxed);
            if raw != 0 {
                return Some(EncodedMove((raw - 1) as u16));
            }
        }
        None
    }

    pub fn store(&self, hash: u64, mv: EncodedMove) {
        let idx = self.index(hash);
        let e = &self.entries[idx];
        let prev = e.key.swap(hash, Ordering::Relaxed);
        if prev == 0 {
            self.filled.fetch_add(1, Ordering::Relaxed);
        }
        // store mv.0+1 as u32, 0 == no move
        e.mv.store((mv.0 as u32) + 1, Ordering::Relaxed);
    }

    pub fn fill_ratio(&self) -> (usize, usize, f64) {
        let f = self.filled.load(Ordering::Relaxed);
        let c = self.entries.len();
        (f, c, f as f64 * 100.0 / c as f64)
    }
}
