//! Training data preparation entrypoint for the Thunfisch texel-tuning workflow.
//!
//! This binary reads an EPD-style position file, runs a quiescence search from
//! each position, and writes a new file where every FEN corresponds to the
//! best quiet position found by the engine.

use crate::preparation::handle_prepare;

mod eval;
mod training_data;
mod preparation;


/// Program entrypoint.
/// Matches over the first command to identify what to do.
/// Accepts one or two positional arguments:
/// 1. input EPD file path
/// 2. optional output EPD file path
///
/// If the output path is omitted, the prepared file is written as
/// `<input>.prepared.epd`.
fn main() -> std::io::Result<()> {
    let mut args: Vec<_> = std::env::args().collect();
    args.remove(0);
    println!("{args:?}");
    match &args.first().map(|s| s.as_str()) {
        Some("prepare") => handle_prepare(&args[1..])?,
        None | Some(_) => {
            eprintln!("Usage: tuning [prepare]");
            std::process::exit(1);
        }
    }
    Ok(())
}
