use std::sync::atomic::{AtomicBool, AtomicUsize};

pub struct SearchInfo {
    pub total_alpha_beta_nodes: AtomicUsize,
    pub total_qs_nodes: AtomicUsize,
    pub total_eval_nodes: AtomicUsize,
    pub timeout_occurred: AtomicBool,
}

impl SearchInfo {
    pub fn new() -> Self {
        SearchInfo {
            total_alpha_beta_nodes: AtomicUsize::new(0),
            total_qs_nodes: AtomicUsize::new(0),
            total_eval_nodes: AtomicUsize::new(0),
            timeout_occurred: AtomicBool::new(false),
        }
    }
}
