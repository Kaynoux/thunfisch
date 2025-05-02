use std::sync::{Arc, atomic::AtomicBool};

pub struct SearchInfo {
    pub total_nodes: usize,
    pub stop_signal: Arc<AtomicBool>,
    pub timeout_occurred: bool,
}
