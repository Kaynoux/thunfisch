use crate::prelude::*;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU32, AtomicU64, AtomicUsize, Ordering};

/// Ein Eintrag im TT: (Zobrist-Key, EncodedMove as u32, 0=leer)
struct TTEntry {
    key: AtomicU64,
    mv: AtomicU32,
}

pub struct TranspositionTable {
    entries: Vec<TTEntry>,
    mask: usize,
    filled: AtomicUsize,
}

pub static TT: Lazy<TranspositionTable> = Lazy::new(|| TranspositionTable::new(128));

impl TranspositionTable {
    pub fn new(mb: usize) -> Self {
        let bytes = mb * 1024 * 1024;
        // Eintrag-Größe: 8 (u64) + 4 (u32) = 12 Bytes
        let entry_size = 12;
        let mut cap = (bytes / entry_size).next_power_of_two();
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

// public API...
