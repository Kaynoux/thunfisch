//! Public library exports for thunfisch
//! Used so that the tuning can reference modules from the main project
#![allow(missing_docs, clippy::new_without_default, clippy::result_unit_err)]

mod alpha_beta;
mod communication;
mod debug;
mod iterative_deepening;
mod time_management;

pub mod evaluation;
pub mod evaluation_constants;
pub mod move_generator;
pub mod move_picker;
pub mod move_scoring;
pub mod prelude;
pub mod quiescence_search;
pub mod settings;
pub mod transposition_table;
pub mod types;
pub mod utils;

/// Initialize engine state (clears TT and history)
pub fn init() {
    // no-op here; binary may keep its own initialization
}
