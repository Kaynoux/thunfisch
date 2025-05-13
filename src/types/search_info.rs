use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};

pub struct SearchInfo {
    pub total_alpha_beta_nodes: AtomicUsize,
    pub total_qs_nodes: AtomicUsize,
    pub total_eval_nodes: AtomicUsize,
    pub stop_signal: Arc<AtomicBool>,
    pub timeout_occurred: AtomicBool,
}

impl SearchInfo {
    pub fn new(stop_signal: Arc<AtomicBool>) -> Self {
        SearchInfo {
            total_alpha_beta_nodes: AtomicUsize::new(0),
            total_qs_nodes: AtomicUsize::new(0),
            total_eval_nodes: AtomicUsize::new(0),
            stop_signal,
            timeout_occurred: AtomicBool::new(false),
        }
    }
}
