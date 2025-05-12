use std::sync::{Arc, atomic::AtomicBool};

pub struct SearchInfo {
    pub total_alpha_beta_nodes: usize,
    pub total_qs_nodes: usize,
    pub stop_signal: Arc<AtomicBool>,
    pub timeout_occurred: bool,
}
