//! Training data preparation entrypoint for the Thunfisch texel-tuning workflow.
//!
//! This binary reads an EPD-style position file, runs a quiescence search from
//! each position, and writes a new file where every FEN corresponds to the
//! best quiet position found by the engine.

use std::time::Instant;

use crate::{
    adam::optimize_k,
    preparation::handle_prepare,
    training_data::{GameResult, TrainingSample},
    tunable_params::TunableParams,
};

mod eval;
mod adam;
mod preparation;
mod training_data;
mod tunable_params;


/// Program entrypoint.
/// Matches over the first command to identify what to do.
/// Accepts a subcommand and its positional arguments.
///
/// Supported subcommands:
/// - `prepare <input.epd> [output.epd]`
/// - `train <input.epd>`
///
/// If the output path is omitted, the prepared file is written as
/// `<input>.prepared.epd`.
fn main() -> std::io::Result<()> {
    let mut args: Vec<_> = std::env::args().collect();
    args.remove(0);
    println!("{args:?}");
    match &args.first().map(|s| s.as_str()) {
        Some("prepare") => handle_prepare(&args[1..])?,
        Some("train") => train(&args[1..])?,
        None | Some(_) => {
            eprintln!("Usage: tuning [prepare|train]");
            std::process::exit(1);
        }
    }
    Ok(())
}

/// Run the current training workflow.
///
/// For now this only loads the input positions, constructs the default
/// evaluator parameters, and optimizes `k` for the sigmoid loss.
fn train(args: &[String]) -> std::io::Result<()> {
    let [input] = args else {
        eprintln!("Usage: tuning train <input.epd>");
        std::process::exit(1);
    };

    let training_data = TrainingSample::read_epd_file(input)?;
    println!("Size of Training data: {}", training_data.len());
    // assert_eq!(training_data.iter().find(|sample| sample.fen.contains("6r1/5q1k/p7/8/P3K3/1n6/7q/3q4")).unwrap().result, GameResult::BlackWin);

    println!("\nOptimizing k");
    println!("-----------------");
    let params = TunableParams::default();
    let optimization_start = Instant::now();
    let best_k = optimize_k(&training_data, &params);

    println!("Best k: {best_k}\nTook: {:?}", optimization_start.elapsed());
    Ok(())
}
