use crate::{prelude::*, search::alpha_beta::MATE_SCORE, settings::settings};
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU8, AtomicU64, Ordering};
#[allow(dead_code)] // because of const settings
#[derive(Clone, Copy, Debug, PartialEq)]

pub enum ReplacementStrategy {
    AlwaysReplace,
    Aging,
}

const MAX_AGE: u8 = 1 << 5; // needs to match TTInfo layout
const AGE_MASK: u8 = MAX_AGE - 1;

// Inspired by Viridithas
/// Holds the age, pv flag, and bound type packed into a single byte.
///
/// Bit layout:
/// - Bits 0-1 (2 bits): Bound (00 = None, 01 = Upper, 10 = Lower, 11 = Exact)
/// - Bit 2    (1 bit) : pv flag (1 = PV node, 0 = Non-PV node)
/// - Bits 3-7 (5 bits): age (0 to 31)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TTInfo {
    data: u8,
}

impl TTInfo {
    const fn encode(age: u8, flag: Bound, is_pv: bool) -> Self {
        Self {
            data: (age << 3) | ((is_pv as u8) << 2) | flag as u8,
        }
    }
    const fn age(self) -> u8 {
        self.data >> 3
    }

    const fn bound(self) -> Bound {
        match self.data & 0b11 {
            0 => Bound::None,
            1 => Bound::Upper,
            2 => Bound::Lower,
            3 => Bound::Exact,
            _ => unreachable!(),
        }
    }

    const fn pv(self) -> bool {
        self.data & 0b100 != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Bound {
    None = 0,
    Upper = 1,
    Lower = 2,
    Exact = 3,
}

impl Bound {
    pub fn is_lower(self) -> bool {
        self as u8 & 0b10 != 0
    }

    pub fn is_upper(self) -> bool {
        self as u8 & 0b01 != 0
    }

    pub fn invert(self) -> Self {
        match self {
            Self::Upper => Self::Lower,
            Self::Lower => Self::Upper,
            x => x,
        }
    }
}

#[derive(Default)]
pub struct EncodedHashEntry {
    data: AtomicU64,
}

impl Clone for EncodedHashEntry {
    fn clone(&self) -> Self {
        Self {
            data: AtomicU64::new(self.data.load(Ordering::Relaxed)),
        }
    }
}

/// Single Entry in the transposition table
/// my move has 16 bits but it has currently no way of storing null moves so I use an u32 as a tempory solution
/// this needs to be fixed in the future
/// Also the Score being 4 bytes leads to the struct being padded to 24 bytes (from the 17 it actually needs)
/// should also be fixed

#[repr(C)]
pub struct DecodedTTEntry {
    key: u16,
    best_move: EncodedMove,
    score: i16,
    depth: i8,
    info: TTInfo,
}
impl DecodedTTEntry {
    pub fn depth(&self) -> i32 {
        i32::from(self.depth)
    }

    pub fn bound(&self) -> Bound {
        self.info.bound()
    }

    pub fn score(&self) -> i32 {
        i32::from(self.score)
    }

    pub fn best_move(&self) -> Option<EncodedMove> {
        if self.best_move.0 == 0 {
            return None;
        }

        Some(self.best_move)
    }

    pub fn from_internal(atom: EncodedHashEntry) -> Self {
        unsafe { std::mem::transmute(atom.data.load(Ordering::Relaxed)) } // should probably measure how much we actually benefit from unsafe
    }

    pub fn to_u64(self) -> u64 {
        unsafe { std::mem::transmute(self) }
    }
}

// impl fmt::Display for DecodedTTEntry {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let (depth, score_type) = self.flags.decode();
//         write!(
//             f,
//             "key: {}\neval: {}\nmv: {}\ndepth: {}\nscore_type: {:?}",
//             self.key.load(Ordering::Relaxed),
//             self.eval.load(Ordering::Relaxed),
//             EncodedMove((self.mv.load(Ordering::Relaxed) - 1) as u16)
//                 .decode()
//                 .to_coords(),
//             depth,
//             score_type
//         )
//     }
// }

/// https://www.chessprogramming.org/Transposition_Table
pub struct TranspositionTable {
    entries: Vec<EncodedHashEntry>,
    age: AtomicU8,
}

/// Transposition Table shared between all search threads
pub static TT: Lazy<TranspositionTable> = Lazy::new(|| TranspositionTable::new(512));

impl TranspositionTable {
    pub fn new(mb: usize) -> Self {
        let bytes = mb * 1024 * 1024;
        let entry_size = size_of::<EncodedHashEntry>();

        // Calculate max entries to the next lower power of 2
        let max_entries = bytes / entry_size;
        let cap = if max_entries > 0 {
            1_usize << max_entries.ilog2() // ilog2 gets rounded down next log2 
        } else {
            1
        };

        let entries = (0..cap)
            .map(|_| EncodedHashEntry {
                data: AtomicU64::new(0),
            })
            .collect();

        TranspositionTable {
            entries,
            age: AtomicU8::new(0),
        }
    }

    // Adds one but limits age to 63
    pub fn increase_age(&self) {
        self.age
            .store((self.get_age() + 1) & AGE_MASK, Ordering::Relaxed);
    }

    pub fn get_age(&self) -> u8 {
        self.age.load(Ordering::Relaxed)
    }

    /// Currently uses ALWAYS REPLACE scheme for collisions
    pub fn store(
        &self,
        hash: u64,
        mv: Option<EncodedMove>,
        mut score: i32,
        depth: i8,
        ply: i32,
        bound: Bound,
        is_pv: bool,
    ) {
        let key = (hash >> 48) as u16; // highest 16 bits of zobrist board hash as key
        let idx = (hash as usize) & (self.entries.len() - 1); // hash index for new entry, works because table size is power of 2
        let previous_entry = DecodedTTEntry::from_internal(self.entries[idx].clone());

        if settings::REPLACEMENT_STRATEGY == ReplacementStrategy::Aging {
            // Replacement strategy:
            // we dont store new entry if:
            // - we are not at the root
            // - we are at the same position
            // - the new entry depth is lower than old depth + a penalty for its age
            let diff = (self.get_age() + MAX_AGE - previous_entry.info.age()) & AGE_MASK;
            if ply > 0
                && key == previous_entry.key
                && depth as u8 + settings::DEPTH_PENALTY_PER_AGE * diff < previous_entry.depth as u8
            {
                return;
            }
        }

        // if score is a mate we add ply to make the score independant of position (we added ply previously)
        // not 100% tested tbh
        score += if score.abs() > (MATE_SCORE - 256) {
            score.signum() * ply
        } else {
            0
        };

        let best_move = mv.unwrap_or(EncodedMove(0));

        debug_assert!(
            score >= i16::MIN as i32 && score <= i16::MAX as i32,
            "Score must fit into i16"
        );

        let new_entry = DecodedTTEntry {
            key,
            best_move: best_move,
            score: score as i16,
            depth,
            info: TTInfo::encode(self.get_age(), bound, is_pv),
        }
        .to_u64();

        self.entries[idx].data.store(new_entry, Ordering::Relaxed);
    }

    pub fn probe(&self, hash: u64, ply: i32) -> Option<DecodedTTEntry> {
        let idx = (hash as usize) & (self.entries.len() - 1);
        let mut entry = DecodedTTEntry::from_internal(self.entries[idx].clone());

        if entry.key != (hash >> 48) as u16 {
            return None;
        }

        entry.score -= if entry.score.abs() > (MATE_SCORE - 256) as i16 {
            entry.score.signum() * ply as i16
        } else {
            0
        };

        Some(entry)
    }

    pub fn info(&self) -> (usize, usize, f64, usize) {
        // Sample up to 1000 entries to estimate fill percentage (standard UCI behavior)
        let sample_size = self.entries.len().min(1000);
        let mut filled_sample = 0;

        for i in 0..sample_size {
            if self.entries[i].data.load(Ordering::Relaxed) != 0 {
                filled_sample += 1;
            }
        }

        let fill_rate = if sample_size > 0 {
            filled_sample as f64 / sample_size as f64
        } else {
            0.0
        };

        let total_entries = self.entries.len();
        let filled_entries = (total_entries as f64 * fill_rate) as usize;
        let size_in_bytes = total_entries * std::mem::size_of::<EncodedHashEntry>();

        (
            filled_entries,
            total_entries,
            fill_rate * 100.0,
            size_in_bytes,
        )
    }

    /// Clears the transposition table by resetting all entries and the age to 0.
    pub fn clear(&self) {
        for entry in &self.entries {
            entry.data.store(0, Ordering::Relaxed);
        }
        self.age.store(0, Ordering::Relaxed);
    }

    // pub fn handle_debug(&self, args: &[&str], hash: u64) -> Result<String, String> {
    //     match args.get(0) {
    //         Some(&"help") => Ok("usage: tt [fill | clear | probe]".to_owned()),
    //         Some(&"clear") => {
    //             self.clear();
    //             Ok(format!("{:?}", self.info()))
    //         }
    //         Some(&"fill") => Ok(format!("{:?}", self.info())),
    //         Some(&"probe") => {
    //             let entry = &self.entries[self.index(hash)];
    //             if entry.key.load(Ordering::Relaxed) != hash {
    //                 return Ok("No Entry".to_owned());
    //             }
    //             Ok(format!("{}", entry))
    //         }
    //         Some(cmd) => Err(format!("Unknown command: tt {}", cmd)),
    //         None => Err("Argument Required".to_owned()),
    //     }
    // }
}

#[cfg(test)]
mod test_tt_encodings {
    use super::*;

    #[test]
    fn test_ttinfo_encoding() {
        let info = TTInfo::encode(13, Bound::Exact, true);
        assert_eq!(info.age(), 13);
        assert_eq!(info.bound(), Bound::Exact);
        assert!(info.pv());

        let info2 = TTInfo::encode(31, Bound::Upper, false);
        assert_eq!(info2.age(), 31);
        assert_eq!(info2.bound(), Bound::Upper);
        assert!(!info2.pv());
    }

    #[test]
    fn test_tt_entry_packing() {
        let info = TTInfo::encode(7, Bound::Lower, true);
        let entry = DecodedTTEntry {
            key: 0xABCD,
            best_move: EncodedMove(0x1234),
            score: -150, // Test with negative score to ensure sign bit extension doesn't ruin upper bits
            depth: 8,
            info,
        };

        let packed = entry.to_u64();
        let unpacked = DecodedTTEntry::from_internal(EncodedHashEntry {
            data: AtomicU64::new(packed),
        });

        assert_eq!(unpacked.key, 0xABCD);
        assert_eq!(unpacked.best_move.0, 0x1234);
        assert_eq!(unpacked.score, -150);
        assert_eq!(unpacked.depth, 8);
        assert_eq!(unpacked.info.age(), 7);
        assert_eq!(unpacked.info.bound(), Bound::Lower);
        assert!(unpacked.info.pv());
    }

    #[test]
    fn test_tt_store_and_probe() {
        // Temporarily enable TT if it'settings behind a static setting flag in tests
        // (Assuming settings::TRANSPOSITION_TABLE is true during tests or mocked)

        let tt = TranspositionTable::new(1); // 1 MB
        let hash = 0x1234567890ABCDEF;
        let mv = EncodedMove(42);

        // Store exact score
        tt.store(hash, Some(mv), 100, 5, 0, Bound::Exact, true);

        // Probe with correct hash
        let probed = tt.probe(hash, 0).expect("Entry should be present");
        assert_eq!(probed.score(), 100);
        assert_eq!(probed.depth(), 5);
        assert_eq!(probed.bound(), Bound::Exact);
        assert!(probed.info.pv());
        assert_eq!(probed.best_move().unwrap(), mv);

        // Probe with incorrect hash (colliding index, different upper bits)
        assert!(tt.probe(hash ^ (1 << 50), 0).is_none());
    }

    #[test]
    fn test_mate_score_adjustment_comprehensive() {
        let tt = TranspositionTable::new(1);
        let hash_win = 0xAAABBBCCCDDDEEEF;
        let hash_loss = 0x1112223334445556;
        let mv = EncodedMove(111);

        // --- Positive Mate Score (we are winning) ---
        // A score of MATE_SCORE - 5 (Mate in 5 half-moves) found at ply 2.
        let mate_in_5 = MATE_SCORE - 5;
        tt.store(hash_win, Some(mv), mate_in_5, 10, 2, Bound::Exact, false);

        // When probed at the same depth (ply 2), the score must remain exactly the same.
        let probed_win_same_ply = tt.probe(hash_win, 2).unwrap();
        assert_eq!(probed_win_same_ply.score(), mate_in_5);

        // If the same TT position is found at ply 4, the mate is closer relative to the new node (Mate in 3).
        let probed_win_deeper = tt.probe(hash_win, 4).unwrap();
        assert_eq!(probed_win_deeper.score(), MATE_SCORE - 7);

        // --- Negative Mate Score (we are being mated) ---
        // A score of -MATE_SCORE + 5 (we are mated in 5 half-moves) at ply 2.
        let mated_in_5 = -MATE_SCORE + 5;
        tt.store(hash_loss, Some(mv), mated_in_5, 10, 2, Bound::Exact, false);

        // Probed at the same depth:
        let probed_loss_same_ply = tt.probe(hash_loss, 2).unwrap();
        assert_eq!(probed_loss_same_ply.score(), mated_in_5);

        // Probed at ply 4 (only 3 half-moves left until we lose):
        let probed_loss_deeper = tt.probe(hash_loss, 4).unwrap();
        assert_eq!(probed_loss_deeper.score(), -MATE_SCORE + 7);
    }

    #[test]
    fn test_aging_replacement_strategy() {
        let tt = TranspositionTable::new(1);
        let hash = 0x5555666677778888;
        let mv_deep = EncodedMove(10);
        let mv_shallow = EncodedMove(20);

        // 1. Initial storage with high depth
        tt.store(hash, Some(mv_deep), 100, 10, 1, Bound::Exact, false);

        // 2. Attempt to overwrite with lower depth (should fail because age diff = 0)
        tt.store(hash, Some(mv_shallow), 200, 2, 1, Bound::Exact, false);

        let probed = tt.probe(hash, 1).unwrap();
        assert_eq!(probed.depth(), 10); // The old entry should still be present
        assert_eq!(probed.score(), 100);

        // 3. Simulate several search iterations (engine ages the TT)
        // How often we need to age depends on settings::DEPTH_PENALTY_PER_AGE
        // Assuming Penalty is 2, we need diff = 5 (2 + 2*5 = 12 > 10)
        for _ in 0..10 {
            tt.increase_age();
        }

        // 4. Store again. Since the old entry is now "old", the penalty should apply
        tt.store(hash, Some(mv_shallow), 200, 2, 1, Bound::Exact, false);

        let probed_new = tt.probe(hash, 1).unwrap();

        // Verification: If the aging strategy is active, the entry should now be overwritten
        if settings::REPLACEMENT_STRATEGY == ReplacementStrategy::Aging {
            assert_eq!(probed_new.depth(), 2);
            assert_eq!(probed_new.score(), 200);
            assert_eq!(probed_new.best_move().unwrap(), mv_shallow);
        }
    }

    #[test]
    fn test_replacement_scheme() {
        let tt = TranspositionTable::new(1);
        let hash = 0x1111222233334444;

        // 1. Initial store at deep depth
        tt.store(hash, Some(EncodedMove(1)), 50, 6, 0, Bound::Exact, false);

        // 2. Try overwriting with lower depth at non-root (ply > 0)
        tt.store(hash, Some(EncodedMove(2)), 100, 2, 1, Bound::Upper, false);

        // The deep depth entry should NOT be overwritten (if age penalty didn't kick in)
        let probed = tt.probe(hash, 0).unwrap();
        assert_eq!(probed.depth(), 6);
        assert_eq!(probed.score(), 50);
        assert_eq!(probed.best_move().unwrap(), EncodedMove(1));

        // 3. Try overwriting at root (ply = 0) even with lower depth.
        // Always Replace overrides everything at ply = 0
        tt.store(hash, Some(EncodedMove(3)), 150, 2, 0, Bound::Lower, true);

        // The entry should now be overwritten
        let probed_root = tt.probe(hash, 0).unwrap();
        assert_eq!(probed_root.score(), 150);
        assert_eq!(probed_root.depth(), 2);
        assert_eq!(probed_root.best_move().unwrap(), EncodedMove(3));
        assert_eq!(probed_root.bound(), Bound::Lower);
    }
}
