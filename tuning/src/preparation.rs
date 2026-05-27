//! Module for preparation of EPD data
//! Runs Quiescence Search on every position to ensure that the end positions are truly quiet

use std::{path::PathBuf, sync::Arc};

use std::sync::atomic::AtomicBool;

use thunfisch::settings;
use thunfisch::types::board::Board;
use thunfisch::types::encoded_move::EncodedMove;
use thunfisch::types::search_data::SharedSearchData;

use crate::eval::quiescence_search::quiescence_search;
use thunfisch::evaluation::MATE_SCORE;

use crate::training_data::TrainingSample;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

/// Evaluation cutoff ove which we discard positions because they could poison the data set
/// intuition: Assuming the evaluation is even somewhat in the right ballpark, an advantage of 20
/// means the game is completely won, being almost semantically equivalent to a mated position.
///
/// Note that _technically_ this is an optimizable hyperparameter but I think 20 cp is a good ballpark
const EVALUATION_CUTOFF: i32 = 2000;

/// Prepares the training data
/// First loads the file into internal structs
/// Then uses rayon to run QS on the positions in parallel
pub fn handle_prepare(args: &[String]) -> std::io::Result<()> {
    let (input_path, output_path) = match args {
        [input, output] => (PathBuf::from(input), PathBuf::from(output)),
        [input] => {
            let output = format!("{}.prepared.epd", input);
            (PathBuf::from(input), PathBuf::from(output))
        }
        _ => {
            eprintln!("Usage: tuning prepare <input.epd> [output.epd]");
            std::process::exit(1);
        }
    };

    let positions = TrainingSample::read_epd_file(&input_path)?;

    let prepared: Vec<TrainingSample> = positions
        .par_iter()
        .map(|position| prepare_quiet_training_position(position.clone()))
        .filter(|result| result.is_some())
        .map(|result| result.unwrap())
        .collect();

    TrainingSample::write_epd_file(&output_path, &prepared)?;
    println!(
        "Prepared {} positions into {}",
        prepared.len(),
        output_path.display()
    );
    Ok(())
}

/// Run the quiescence search from the root position and return a quieted entry.
///
/// The returned `TrainingData` keeps the original game result label, but uses the
/// best-line FEN from the quiescence search so that the training set only
/// contains quiet positions.
pub fn prepare_quiet_training_position(position: TrainingSample) -> Option<TrainingSample> {
    let mut board = Board::new(&position.fen);
    let stop = Arc::new(AtomicBool::new(false));
    let mut local_seldepth = 0usize;
    let mut killers = [EncodedMove(0); settings::MAX_AB_DEPTH + 1];

    let mut search_data =
        SharedSearchData::new(&mut board, &stop, &mut local_seldepth, &mut killers);
    let search_result = quiescence_search(
        settings::MAX_QS_DEPTH,
        -MATE_SCORE,
        MATE_SCORE,
        &mut search_data,
        0,
    );
    // filter out positions which we can't use (e.g. mated positions)
    if search_result.score.abs() > EVALUATION_CUTOFF {
        return None;
    }

    Some(TrainingSample {
        fen: search_result.best_line_fen,
        result: position.result,
    })
}
