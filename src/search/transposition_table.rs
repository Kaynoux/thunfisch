use crate::{prelude::*, search::alpha_beta::MATE_SCORE};
use std::sync::atomic::{AtomicU8, AtomicU64, Ordering};

const MAX_AGE: i32 = 1 << 5; // needs to match TTInfo layout
const AGE_MASK: i32 = MAX_AGE - 1;

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

    pub const fn bound(&self) -> Bound {
        self.info.bound()
    }

    pub fn score(&self) -> i32 {
        i32::from(self.score)
    }

    pub const fn best_move(&self) -> Option<EncodedMove> {
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

/// <https://www.chessprogramming.org/Transposition_Table>
pub struct TranspositionTable {
    entries: Vec<EncodedHashEntry>,
    age: AtomicU8,
}

/// Transposition Table shared between all search threads
pub static TT: std::sync::LazyLock<TranspositionTable> =
    std::sync::LazyLock::new(|| TranspositionTable::new(512));

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

        Self {
            entries,
            age: AtomicU8::new(0),
        }
    }

    // Adds one but limits age to 63
    pub fn increase_age(&self) {
        self.age
            .store((self.get_age() + 1) & (AGE_MASK as u8), Ordering::Relaxed);
    }

    pub fn get_age(&self) -> u8 {
        self.age.load(Ordering::Relaxed)
    }

    /// For the most part taken from Viridithas
    pub fn store(
        &self,
        hash: u64,
        mut best_move: Option<EncodedMove>,
        score: i32,
        depth: i8,
        ply: i32,
        bound: Bound,
        is_pv: bool,
    ) {
        let key = (hash >> 48) as u16;
        let idx = (hash as usize) & (self.entries.len() - 1);
        let previous = DecodedTTEntry::from_internal(self.entries[idx].clone());
        let tt_age = i32::from(self.get_age());

        // if we don't have a best move, and the entry is for the same position,
        // then we should retain the best move from the previous entry.
        if best_move.is_none() && previous.key == key {
            best_move = previous.best_move();
        }

        // give entries a bonus for type:
        // exact = 3, lower = 2, upper = 1
        let insert_flag_bonus = bound as i32;
        let record_flag_bonus = previous.info.bound() as i32;

        // preferentially overwrite entries that are from searches on previous positions in the game.
        let age_differential = (MAX_AGE + tt_age - i32::from(previous.info.age())) & AGE_MASK;

        // we use quadratic scaling of the age to allow entries that aren't too old to be kept,
        // but to ensure that really old entries are overwritten even if they are of high depth.
        let insert_priority = i32::from(depth)
            + insert_flag_bonus
            + (age_differential * age_differential) / 4
            + i32::from(is_pv);
        let record_priority = i32::from(previous.depth) + record_flag_bonus;

        // replace the entry if:
        // 1. the entry is for a different position
        // 2. it's an exact entry and the old entry is not exact
        // 3. the new entry is of higher priority than the old entry
        if previous.key != key
            || bound == Bound::Exact && previous.info.bound() != Bound::Exact
            || insert_priority * 3 >= record_priority * 2
        {
            // normalise mate  scores:
            let normalised_score = if score.abs() > (MATE_SCORE - 256) {
                score + score.signum() * ply
            } else {
                score
            };

            debug_assert!(
                i16::try_from(normalised_score).is_ok(),
                "Score must fit into i16"
            );

            let new_entry = DecodedTTEntry {
                key,
                best_move: best_move.unwrap_or(EncodedMove(0)),
                score: normalised_score as i16,
                depth,
                info: TTInfo::encode(self.get_age(), bound, is_pv),
            }
            .to_u64();

            self.entries[idx].data.store(new_entry, Ordering::Relaxed);
        }
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
            f64::from(filled_sample) / sample_size as f64
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

    pub fn handle_debug(&self, args: &[&str], hash: u64) -> Result<String, String> {
        match args.first() {
            Some(&"help") => Ok("usage: tt [fill | clear | probe]".to_owned()),
            Some(&"clear") => {
                self.clear();
                Ok(format!("{:?}", self.info()))
            }
            Some(&"fill") => Ok(format!("{:?}", self.info())),
            Some(&"probe") => {
                if let Some(entry) = self.probe(hash, 0) {
                    let move_info = match entry.best_move() {
                        Some(mv) => {
                            let decoded = mv.decode();
                            format!("Encoded({}), Coords: {}", mv.0, decoded.to_coords())
                        }
                        None => "None".to_string(),
                    };

                    Ok(format!(
                        "Hash Key (16-bit): {:X}\nScore: {}\nDepth: {}\nBound: {:?}\nPV Node: {}\nAge: {}\nMove: [{}]",
                        entry.key,
                        entry.score(),
                        entry.depth(),
                        entry.bound(),
                        entry.info.pv(),
                        entry.info.age(),
                        move_info
                    ))
                } else {
                    Ok("No Entry".to_owned())
                }
            }
            Some(cmd) => Err(format!("Unknown command: tt {cmd}")),
            None => Err("Argument Required".to_owned()),
        }
    }
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
    fn test_priority_replacement_same_age() {
        let tt = TranspositionTable::new(1);
        let hash = 0x5555666677778888;
        let mv_deep = EncodedMove(10);
        let mv_shallow = EncodedMove(20);

        // 1. Initial storage with high depth (Exact bound = flag bonus 3)
        // record_priority = 10 + 3 = 13
        tt.store(hash, Some(mv_deep), 100, 10, 1, Bound::Exact, false);

        // 2. Attempt to overwrite with much lower depth (Exact bound = flag bonus 3)
        // insert_priority = 2 + 3 + 0 + 0 = 5, record_priority = 13
        // 5 * 3 = 15 < 13 * 2 = 26 => NOT replaced
        tt.store(hash, Some(mv_shallow), 200, 2, 1, Bound::Exact, false);

        let probed = tt.probe(hash, 1).unwrap();
        assert_eq!(probed.depth(), 10); // The old entry should still be present
        assert_eq!(probed.score(), 100);
    }

    #[test]
    fn test_priority_replacement_with_aging() {
        let tt = TranspositionTable::new(1);
        let hash = 0x5555666677778888;
        let mv_deep = EncodedMove(10);
        let mv_shallow = EncodedMove(20);

        // 1. Initial storage with high depth
        tt.store(hash, Some(mv_deep), 100, 10, 1, Bound::Exact, false);

        // 2. Age the TT significantly so old entries become stale
        for _ in 0..10 {
            tt.increase_age();
        }

        // 3. Store again with lower depth. The quadratic age bonus should override depth.
        // age_diff = 10, quadratic bonus = 100/4 = 25
        // insert_priority = 2 + 3 + 25 + 0 = 30, record_priority = 10 + 3 = 13
        // 30 * 3 = 90 >= 13 * 2 = 26 => REPLACED
        tt.store(hash, Some(mv_shallow), 200, 2, 1, Bound::Exact, false);

        let probed = tt.probe(hash, 1).unwrap();
        assert_eq!(probed.depth(), 2);
        assert_eq!(probed.score(), 200);
        assert_eq!(probed.best_move().unwrap(), mv_shallow);
    }

    #[test]
    fn test_exact_bound_overrides_non_exact() {
        let tt = TranspositionTable::new(1);
        let hash = 0x1111222233334444;

        // 1. Store an Upper bound entry at depth 6
        tt.store(hash, Some(EncodedMove(1)), 50, 6, 0, Bound::Upper, false);

        // 2. Store an Exact entry at lower depth — should always replace a non-Exact entry
        tt.store(hash, Some(EncodedMove(2)), 100, 2, 0, Bound::Exact, false);

        let probed = tt.probe(hash, 0).unwrap();
        assert_eq!(probed.depth(), 2);
        assert_eq!(probed.score(), 100);
        assert_eq!(probed.bound(), Bound::Exact);
        assert_eq!(probed.best_move().unwrap(), EncodedMove(2));
    }

    #[test]
    fn test_different_position_always_replaced() {
        let tt = TranspositionTable::new(1);
        // Two hashes that map to the same index but have different keys
        let hash1 = 0x1234567890ABCDEF_u64;
        let idx = (hash1 as usize) & (tt.entries.len() - 1);
        // Construct hash2 with same lower bits (same index) but different upper 16 bits (different key)
        let hash2 = (hash1 & !(0xFFFF << 48)) | (0xAAAA_u64 << 48);
        assert_eq!((hash2 as usize) & (tt.entries.len() - 1), idx);
        assert_ne!((hash1 >> 48) as u16, (hash2 >> 48) as u16);

        // Store deep entry for position 1
        tt.store(hash1, Some(EncodedMove(1)), 500, 20, 0, Bound::Exact, true);

        // Store shallow entry for different position — should always replace
        tt.store(hash2, Some(EncodedMove(2)), 100, 1, 0, Bound::Upper, false);

        let probed = tt.probe(hash2, 0).unwrap();
        assert_eq!(probed.depth(), 1);
        assert_eq!(probed.score(), 100);
    }

    #[test]
    fn test_best_move_retained_from_previous_entry() {
        let tt = TranspositionTable::new(1);
        let hash = 0xAAAABBBBCCCCDDDD;
        let mv = EncodedMove(42);

        // Store with a best move
        tt.store(hash, Some(mv), 100, 5, 0, Bound::Exact, false);

        // Store again for the same position without a best move but high enough priority to replace.
        // The best move from the previous entry should be retained.
        // insert_priority = 10 + 3 + 0 + 0 = 13, record_priority = 5 + 3 = 8
        // 13 * 3 = 39 >= 8 * 2 = 16 => REPLACED
        tt.store(hash, None, 200, 10, 0, Bound::Exact, false);

        let probed = tt.probe(hash, 0).unwrap();
        assert_eq!(probed.depth(), 10);
        assert_eq!(probed.score(), 200);
        assert_eq!(probed.best_move().unwrap(), mv); // best move retained!
    }
}
