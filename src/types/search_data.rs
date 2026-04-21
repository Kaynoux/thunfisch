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
    pub board: &'sd mut Board,
    pub stop: &'sd Arc<AtomicBool>,
    pub local_seldepth: &'sd mut usize,
    pub killers: &'sd mut [EncodedMove; MAX_AB_DEPTH],
    pub ab_ply: usize,

    // From here these are only used for additional info collection
    pub total_alpha_beta_nodes: AtomicUsize,
    pub total_qs_nodes: AtomicUsize,
    pub total_eval_nodes: AtomicUsize,
    pub total_tt_hits: AtomicUsize,
    // stores whether the current search got cancelled due to timeout
    // TODO find out whether this can be eliminated in favor of using only `stop`
    pub timeout_occurred: AtomicBool,
}

impl<'sd> SharedSearchData<'sd> {
    pub const fn new(
        board: &'sd mut Board,
        stop: &'sd Arc<AtomicBool>,
        local_seldepth: &'sd mut usize,
        killers: &'sd mut [EncodedMove; MAX_AB_DEPTH],
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
        }
    }
}
