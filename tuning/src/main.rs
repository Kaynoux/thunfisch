//! Training data preparation entrypoint for the Thunfisch texel-tuning workflow.
//!
//! This binary reads an EPD-style position file, runs a quiescence search from
//! each position, and writes a new file where every FEN corresponds to the
//! best quiet position found by the engine.

use std::{io, path::{Path, PathBuf}, time::Instant};

use crate::{
    adam::{adam, mse, optimize_k, sigmoid, AdamCheckpoint, AdamParams},
    eval::evaluation::evaluate,
    preparation::handle_prepare,
    training_data::TrainingSample,
    tunable_params::{TunableParams, WeightVector},
};

mod adam;
mod eval;
mod preparation;
mod output_paths;
mod training_data;
mod tunable_params;

/// Program entrypoint.
/// Matches over the first command to identify what to do.
/// Accepts a subcommand and its positional arguments.
///
/// Supported subcommands:
/// - `prepare <input.epd> [output.epd]`
/// - `train <input.epd> [epochs] [restore-checkpoint.json]`
/// - `export <checkpoint.json>`
///
/// If the output path is omitted, the prepared file is written as
/// `<input>.prepared.epd`.
fn main() -> std::io::Result<()> {
    let mut args: Vec<_> = std::env::args().collect();
    args.remove(0);
    match &args.first().map(|s| s.as_str()) {
        Some("prepare") => handle_prepare(&args[1..])?,
        Some("train") => train(&args[1..])?,
        Some("export") => export(&args[1..])?,
        None | Some(_) => {
            eprintln!("Usage: tuning [prepare|train|export]");
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
    let [input, rest @ ..] = args else {
        eprintln!("Usage: tuning train <input.epd> [epochs] [checkpoint.json]");
        std::process::exit(1);
    };

    let epochs = rest
        .first()
        .map(|value| value.parse::<usize>())
        .transpose()
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?
        .unwrap_or(1);

    let restore_checkpoint_path = rest.get(1).map(std::path::PathBuf::from);

    let mut training_data = TrainingSample::read_epd_file(input)?;
    println!("Size of Training data: {}", training_data.len());
    // assert_eq!(training_data.iter().find(|sample| sample.fen.contains("6r1/5q1k/p7/8/P3K3/1n6/7q/3q4")).unwrap().result, GameResult::BlackWin);

    println!("\nRunning ADAM training");
    println!("---------------------");

    let (mut adam_params, mut weights, k) = if let Some(restore_checkpoint_path) = restore_checkpoint_path {
        let checkpoint = AdamCheckpoint::read_from_file(&restore_checkpoint_path)?;
        if checkpoint.training_data_path != input.as_str() {
            eprintln!(
                "warning: checkpoint was created for '{}' but the current input is '{}'",
                checkpoint.training_data_path, input
            );
        }
        checkpoint.into_parts()
    } else {
        let params = TunableParams::default();
        let weights: WeightVector = (&params).into();
        println!("\nOptimizing k");
        println!("-----------------");
        let optimization_start = Instant::now();
        let k = optimize_k(&training_data, &params);
        println!("Best k: {k}\nTook: {:?}", optimization_start.elapsed());

        let initial_mse = training_data
            .iter()
            .map(|TrainingSample { fen, result }| {
                let eval = evaluate(&thunfisch::types::board::Board::new(fen), &params) as f64;
                ((*result).into(), sigmoid(eval, k))
            })
            .collect::<Vec<(f64, f64)>>();
        println!("Initial MSE at optimized k: {}", mse(&initial_mse));

        (AdamParams::default(), weights, k)
    };

    let optimization_start = Instant::now();
    adam(
        &mut training_data,
        &mut weights,
        &mut adam_params,
        k,
        epochs,
        input,
    )?;

    println!("Training took: {:?}", optimization_start.elapsed());
    Ok(())
}

/// Export a checkpoint as a Rust constants file.
fn export(args: &[String]) -> std::io::Result<()> {
    let [checkpoint_path] = args else {
        eprintln!("Usage: tuning export <checkpoint.json>");
        std::process::exit(1);
    };

    let checkpoint_path = PathBuf::from(checkpoint_path);
    let checkpoint = AdamCheckpoint::read_from_file(&checkpoint_path)?;
    let params: TunableParams = checkpoint.weights.into();
    let export_path = export_path_for_checkpoint(&checkpoint_path)?;

    params.write_constants_file(&export_path)?;
    println!("Exported tunable constants to {}", export_path.display());
    Ok(())
}

fn export_path_for_checkpoint(checkpoint_path: impl AsRef<Path>) -> std::io::Result<PathBuf> {
    let checkpoint_path = checkpoint_path.as_ref();
    let file_name = checkpoint_path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("checkpoint.json");

    output_paths::in_tuning_data(format!("export-{file_name}.rs"))
}
