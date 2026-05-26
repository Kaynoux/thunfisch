use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize},
};

use crate::{prelude::*, settings::MAX_AB_DEPTH};

/// Contains shared search data in one place, as well as debugging metadata.
/// 'Shared' in this context means that this struct is shared (read and mutated)
/// by multiple nodes at different depths of the search tree
///
/// WARNING: this currently does NOT guarantee thread-safety as we're currently not threading the search.
pub struct SharedSearchData<'sd> {
    pub(crate) board: &'sd mut Board,
    pub(crate) stop: &'sd Arc<AtomicBool>,
    pub(crate) local_seldepth: &'sd mut usize,
    pub(crate) killers: &'sd mut [EncodedMove; MAX_AB_DEPTH + 1],
    pub(crate) ab_ply: usize,

    // From here these are only used for additional info collection
    pub(crate) total_alpha_beta_nodes: AtomicUsize,
    pub(crate) total_qs_nodes: AtomicUsize,
    pub(crate) total_eval_nodes: AtomicUsize,
    pub(crate) total_tt_hits: AtomicUsize,
    pub(crate) total_lmr_researches: AtomicUsize,
    pub(crate) total_pvs_researches: AtomicUsize,
    // stores whether the current search got cancelled due to timeout
    // TODO find out whether this can be eliminated in favor of using only `stop`
    pub(crate) timeout_occurred: AtomicBool,
}

impl<'sd> SharedSearchData<'sd> {
    pub(crate) const fn new(
        board: &'sd mut Board,
        stop: &'sd Arc<AtomicBool>,
        local_seldepth: &'sd mut usize,
        killers: &'sd mut [EncodedMove; MAX_AB_DEPTH + 1],
    ) -> Self {
        Self {
            board,
            stop,
            local_seldepth,
            killers,
            ab_ply: 0,
            timeout_occurred: AtomicBool::new(false),
            total_alpha_beta_nodes: AtomicUsize::new(0),
            total_qs_nodes: AtomicUsize::new(0),
            total_eval_nodes: AtomicUsize::new(0),
            total_tt_hits: AtomicUsize::new(0),
            total_lmr_researches: AtomicUsize::new(0),
            total_pvs_researches: AtomicUsize::new(0),
        }
    }
}
