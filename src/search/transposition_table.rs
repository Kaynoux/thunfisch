use crate::{prelude::*, settings::settings};
use once_cell::sync::Lazy;
use std::{
    fmt,
    sync::atomic::{AtomicI32, AtomicU8, AtomicU32, AtomicU64, AtomicUsize, Ordering},
};

/// Encoding:
/// [depth (6 bit) | score type (2 bit)] (could be risky if we go over a depth of 63)
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

impl fmt::Display for TTEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (depth, score_type) = self.flags.decode();
        write!(
            f,
            "key: {}\neval: {}\nmv: {}\ndepth: {}\nscore_type: {:?}",
            self.key.load(Ordering::Relaxed),
            self.eval.load(Ordering::Relaxed),
            EncodedMove((self.mv.load(Ordering::Relaxed) - 1) as u16)
                .decode()
                .to_coords(),
            depth,
            score_type
        )
    }
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

        // Calculate max entries to the next lower power of 2
        let max_entries = bytes / entry_size;
        let cap = if max_entries > 0 {
            1_usize << max_entries.ilog2() // ilog2 gets rounded down next log2 
        } else {
            1
        };

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
        if !settings::TRANSPOSITION_TABLE {
            return None;
        }
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
                if eval >= beta {
                    Some((beta, best_mv))
                } else {
                    None
                }
            }
            ScoreType::UpperBound => {
                if eval <= alpha {
                    Some((alpha, best_mv))
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
        if !settings::TRANSPOSITION_TABLE {
            return;
        }
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

    pub fn info(&self) -> (usize, usize, f64, usize) {
        let filled_entries = self.filled.load(Ordering::Relaxed);
        let total_entries = self.entries.len();
        let size_in_bytes = total_entries * size_of::<TTEntry>();
        (
            filled_entries,
            total_entries,
            filled_entries as f64 * 100.0 / total_entries as f64,
            size_in_bytes,
        )
    }

    pub fn clear(&self) {
        self.entries.iter().for_each(|f| {
            f.key.store(0, Ordering::Relaxed);
            f.eval.store(0, Ordering::Relaxed);
            f.flags.0.store(0, Ordering::Relaxed);
            f.mv.store(0, Ordering::Relaxed);
        });
        self.filled.store(0, Ordering::Relaxed);
    }

    pub fn handle_debug(&self, args: &[&str], hash: u64) -> Result<String, String> {
        match args.get(0) {
            Some(&"help") => Ok("usage: tt [fill | clear | probe]".to_owned()),
            Some(&"clear") => {
                self.clear();
                Ok(format!("{:?}", self.info()))
            }
            Some(&"fill") => Ok(format!("{:?}", self.info())),
            Some(&"probe") => {
                let entry = &self.entries[self.index(hash)];
                if entry.key.load(Ordering::Relaxed) != hash {
                    return Ok("No Entry".to_owned());
                }
                Ok(format!("{}", entry))
            }
            Some(cmd) => Err(format!("Unknown command: tt {}", cmd)),
            None => Err("Argument Required".to_owned()),
        }
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

    #[test]
    fn test_tt_store_and_probe() {
        let tt = TranspositionTable::new(1); // 1 MB should be plenty for tests
        let hash = 0x1234567890ABCDEF; // random hash for testing
        let mv = EncodedMove::encode(Square(12), Square(13), MoveType::Quiet);

        // 1. Test Exact Score
        tt.store(hash, Some(mv), 100, 5, ScoreType::Exact);

        // Probe with same depth, should succeed
        assert_eq!(tt.probe(hash, -10000, 10000, 5), Some((100, mv)));
        // Probe with lesser depth, should succeed
        assert_eq!(tt.probe(hash, -10000, 10000, 3), Some((100, mv)));
        // Probe with greater depth, should fail (return None)
        assert_eq!(tt.probe(hash, -10000, 10000, 6), None);
        // Probe with wrong hash, should fail
        assert_eq!(tt.probe(hash + 1, -10000, 10000, 5), None);

        // 2. Test LowerBound (Fail High)
        // TT contains LowerBound of 200 at depth 5
        tt.store(hash, Some(mv), 200, 5, ScoreType::LowerBound);

        // Beta is 150. Eval (200) >= Beta (150) -> Cutoff! Returns Beta
        assert_eq!(tt.probe(hash, -10000, 150, 5), Some((150, mv)));
        // Beta is 250. Eval (200) < Beta (250) -> No cutoff! Returns None
        assert_eq!(tt.probe(hash, -10000, 250, 5), None);

        // 3. Test UpperBound (Fail Low)
        // TT contains UpperBound of -50 at depth 5
        tt.store(hash, Some(mv), -50, 5, ScoreType::UpperBound);

        // Alpha is -20. Eval (-50) <= Alpha (-20) -> Cutoff! Returns Alpha
        assert_eq!(tt.probe(hash, -20, 10000, 5), Some((-20, mv)));
        // Alpha is -100. Eval (-50) > Alpha (-100) -> No cutoff! Returns None
        assert_eq!(tt.probe(hash, -100, 10000, 5), None);

        // 4. Test Replacement Scheme
        let new_hash = 0xAAAABBBBCCCCDDDD;
        let new_mv = EncodedMove(99);
        tt.store(new_hash, Some(new_mv), 300, 6, ScoreType::Exact);
        assert_eq!(tt.probe(new_hash, -10000, 10000, 6), Some((300, new_mv)));

        // 5. Test Clear
        tt.clear();
        assert_eq!(tt.probe(new_hash, -10000, 10000, 6), None);
        assert_eq!(tt.probe(hash, -10000, 10000, 5), None);
    }
}
