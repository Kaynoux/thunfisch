use crate::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

/// Typ eines Eintrags im Transposition Table: exakter Wert oder Schranke
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TTFlag {
    Exact,
    LowerBound, // Beta node
    UpperBound, // Alpha node
}

/// Ein Eintrag im Transposition Table
#[derive(Clone, Copy)]
pub struct TTEntry {
    /// Zobrist-Schlüssel der Stellung
    pub key: u64,
    /// Suchtiefe, in der dieser Wert erzeugt wurde
    pub depth: u8,
    /// Bewertungswert
    pub eval: i32,
    /// Node-Typ
    pub flag: TTFlag,
    /// Optional bester Zug aus dieser Stellung
    pub best_move: Option<EncodedMove>,
}

impl Default for TTEntry {
    fn default() -> Self {
        TTEntry {
            key: 0,
            depth: 0,
            eval: 0,
            flag: TTFlag::Exact,
            best_move: None,
        }
    }
}

/// Simple, nicht-threadsafe Transposition Table mit Power-of-Two-Größe
pub struct TranspositionTable {
    buckets: Vec<AtomicU64>, // speichert die 64-Bit word-pair key|info
    entries: Vec<TTEntry>,   // paralleles Array der Einträge
    mask: usize,             // index mask = size-1
}

impl TranspositionTable {
    /// Erzeugt eine TT mit `size` Einträgen (muss Power of two sein)
    pub fn new(size: usize) -> Self {
        assert!(size.is_power_of_two(), "TT size must be a power of two");
        TranspositionTable {
            buckets: (0..size).map(|_| AtomicU64::new(0)).collect(),
            entries: vec![TTEntry::default(); size],
            mask: size - 1,
        }
    }

    /// Speichert oder ersetzt einen Eintrag
    pub fn store(
        &mut self,
        key: u64,
        depth: u8,
        eval: i32,
        flag: TTFlag,
        best_move: Option<EncodedMove>,
    ) {
        let idx = (key as usize) & self.mask;
        // einfache Ersetzungsstrategie: neuer Eintrag überschreibt alten, wenn deeper oder same key
        let old = &mut self.entries[idx];
        if old.key == key || depth >= old.depth {
            old.key = key;
            old.depth = depth;
            old.eval = eval;
            old.flag = flag;
            old.best_move = best_move;
            // speichere low-Bits-Kopie in buckets für atomare Vergleiche
            let info = key;
            self.buckets[idx].store(info, Ordering::Relaxed);
        }
    }

    /// Probe: liefert Some(&TTEntry) wenn Schlüssel passt und Tiefe ausreichend, sonst None
    pub fn probe(&self, key: u64, depth: u8) -> Option<&TTEntry> {
        let idx = (key as usize) & self.mask;
        if self.buckets[idx].load(Ordering::Relaxed) == key {
            let entry = &self.entries[idx];
            if entry.depth >= depth {
                return Some(entry);
            }
        }
        None
    }

    /// Volle Tabelle löschen
    pub fn clear(&mut self) {
        for bucket in &self.buckets {
            bucket.store(0, Ordering::Relaxed);
        }
        for entry in &mut self.entries {
            *entry = TTEntry::default();
        }
    }

    pub fn fill_amount(&self) -> f64 {
        let filled = self.entries.iter().filter(|e| e.key != 0).count();
        filled as f64 / self.entries.len() as f64
    }
}
