use crate::prelude::*;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicI32, AtomicU8, AtomicU32, AtomicU64, AtomicUsize, Ordering};

#[derive(Debug, PartialEq)]
pub enum ScoreType {
    Exact,
    LowerBound,
    UpperBound,
}

impl ScoreType {
    fn from(enc: u8) -> ScoreType {
        match enc & 3 {
            0 => ScoreType::Exact,
            1 => ScoreType::LowerBound,
            2 => ScoreType::UpperBound,
            // we don't have a use for `0b11`
            _ => ScoreType::UpperBound,
        }
    }

    fn to_u8(&self) -> u8 {
        match self {
            ScoreType::Exact => 0,
            ScoreType::LowerBound => 1,
            ScoreType::UpperBound => 2,
        }
    }
}

/// Encoding:
/// [depth (6 bit) | score type (2 bit)]
struct TTFlags(AtomicU8);

impl TTFlags {
    fn decode(&self) -> (usize, ScoreType) {
        let asu8 = self.0.load(Ordering::Relaxed);
        ((asu8 >> 2) as usize, ScoreType::from(asu8))
    }

    fn store(&self, depth: usize, score_type: ScoreType) {
        let encoded = (((depth & 0x3f) as u8) << 2) | score_type.to_u8();
        self.0.store(encoded, Ordering::Relaxed);
    }
}

/// Single Entry in the transposition table
/// my move has 16 bits but it has currently no way of storing null moves so I use an u32 as a tempory solution
/// this needs to be fixed in the future
/// Also the Score being 4 bytes leads to the struct being padded to 24 bytes (from the 17 it actually needs)
/// should also be fixed
struct TTEntry {
    key: AtomicU64,
    mv: AtomicU32,
    eval: AtomicI32,
    flags: TTFlags,
}

/// https://www.chessprogramming.org/Transposition_Table
pub struct TranspositionTable {
    entries: Vec<TTEntry>,
    mask: usize,
    filled: AtomicUsize,
}

/// Transposition Table shared between all search threads
pub static TT: Lazy<TranspositionTable> = Lazy::new(|| TranspositionTable::new(512));

impl TranspositionTable {
    pub fn new(mb: usize) -> Self {
        let bytes = mb * 1024 * 1024;
        let entry_size = size_of::<TTEntry>();
        let cap = (bytes / entry_size).next_power_of_two();
        let entries = (0..cap)
            .map(|_| TTEntry {
                key: AtomicU64::new(0),
                mv: AtomicU32::new(0),
                eval: AtomicI32::new(0),
                flags: TTFlags(AtomicU8::new(0)),
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

    /// See http://web.archive.org/web/20071031100051/http://www.brucemo.com:80/compchess/programming/hashing.htm
    /// for how the returns work on this
    pub fn probe(
        &self,
        hash: u64,
        alpha: i32,
        beta: i32,
        depth: usize,
    ) -> Option<(i32, EncodedMove)> {
        let e = &self.entries[self.index(hash)];
        if e.key.load(Ordering::Relaxed) != hash {
            return None;
        }
        let (e_depth, e_type) = e.flags.decode();
        let best_mv = e.mv.load(Ordering::Relaxed);
        let eval = e.eval.load(Ordering::Relaxed);

        if e_depth < depth {
            return None;
        }
        if best_mv == 0 {
            return None;
        }

        let best_mv = EncodedMove((best_mv - 1) as u16); // converting here for readable returns below
        match e_type {
            ScoreType::Exact => Some((eval, best_mv)),
            ScoreType::LowerBound => {
                if eval <= alpha {
                    Some((alpha, best_mv))
                } else {
                    None
                }
            }
            ScoreType::UpperBound => {
                if eval >= beta {
                    Some((beta, best_mv))
                } else {
                    None
                }
            }
        }
    }

    /// Currently uses ALWAYS REPLACE scheme for collisions
    pub fn store(
        &self,
        hash: u64,
        mv: Option<EncodedMove>,
        eval: i32,
        depth: usize,
        score_type: ScoreType,
    ) {
        let idx = self.index(hash);
        let entry = &self.entries[idx];
        let prev = entry.key.swap(hash, Ordering::Relaxed);
        if prev == 0 {
            self.filled.fetch_add(1, Ordering::Relaxed);
        }
        // store mv.0+1 as u32
        // if mv is none, represent this as 0
        entry
            .mv
            .store(mv.map_or(0, |mv| mv.0 as u32 + 1), Ordering::Relaxed);
        entry.eval.store(eval, Ordering::Relaxed);
        entry.flags.store(depth, score_type);
    }

    //todo figure out whether there actually is a difference between f and c
    pub fn fill_ratio(&self) -> (usize, usize, f64) {
        let f = self.filled.load(Ordering::Relaxed);
        let c = self.entries.len();
        (f, c, f as f64 * 100.0 / c as f64)
    }
}

#[cfg(test)]
mod test_tt_encodings {
    use super::*;

    // implementing clone makes no sense for the main project but for the tests it does so
    impl Clone for ScoreType {
        fn clone(&self) -> Self {
            match self {
                Self::Exact => Self::Exact,
                Self::LowerBound => Self::LowerBound,
                Self::UpperBound => Self::UpperBound,
            }
        }
    }

    #[test]
    fn test_size_of_struct() {
        println!("{}", size_of::<TTEntry>());
        println!("{}", size_of::<TTFlags>());
        println!("{}", size_of::<AtomicU32>());
        println!("{}", size_of::<AtomicI32>());
        println!("{}", size_of::<AtomicU64>());
    }

    #[test]
    fn test_depth_type_encoding() {
        let depth: usize = 15;
        let score_type = ScoreType::Exact;
        let encoded = TTFlags(AtomicU8::new(0));
        encoded.store(depth, score_type.clone());
        assert_eq!(encoded.0.load(Ordering::Relaxed), 0b00111100);
        assert_eq!(encoded.decode(), (depth, score_type));

        let depth: usize = 4;
        let score_type = ScoreType::UpperBound;
        encoded.store(depth, score_type.clone());
        assert_eq!(encoded.0.load(Ordering::Relaxed), 0b00010010);
        assert_eq!(encoded.decode(), (depth, score_type));

        let encoded = TTFlags(AtomicU8::new(0b00001111));
        assert_eq!(encoded.decode(), (3, ScoreType::UpperBound));
    }
}
