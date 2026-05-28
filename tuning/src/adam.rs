//! Primary Module for tuning
//! Defines an ADAM routine that is used to find an optimum of evaluation parameters
//! through ADAM (a gradient descent variant)
//!
//! Credits to [Fatalii](https://github.com/FitzOReilly/fatalii.git) for implementation inspiration

use std::time::Instant;
use std::{fs, io, path::Path};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use thunfisch::types::board::Board;

use crate::{
    eval::evaluation::evaluate,
    training_data::TrainingSample,
    tunable_params::{TunableParams, WeightVector},
};

use rand::{rng, seq::SliceRandom};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdamParams {
    pub batch_size: usize,
    pub validation_ratio: f64,
    pub learning_rate: f64,
    pub beta_1: f64,
    pub beta_2: f64,
    pub epsilon: f64,
    pub epoch: usize,
    pub t: i32,
    pub m: WeightVector,
    pub v: WeightVector,
    // offset for numerical gradient calculation
    pub delta: f64,
}

/// Serializable checkpoint for resuming ADAM training.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdamCheckpoint {
    pub training_data_path: String,
    pub adam_params: AdamParams,
    pub weights: WeightVector,
    pub k: f64,
    pub final_mse: Option<f64>,
}

impl AdamCheckpoint {
    /// Create a new checkpoint from fresh optimizer state.
    pub fn new(
        training_data_path: impl Into<String>,
        adam_params: AdamParams,
        weights: WeightVector,
        k: f64,
        final_mse: Option<f64>,
    ) -> Self {
        Self {
            training_data_path: training_data_path.into(),
            adam_params,
            weights,
            k,
            final_mse,
        }
    }

    /// Load a checkpoint from a JSON file.
    pub fn read_from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let contents = fs::read_to_string(path)?;
        serde_json::from_str(&contents)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
    }

    /// Write the checkpoint to a JSON file.
    pub fn write_to_file(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        fs::write(path, json)
    }

    /// Split the checkpoint into the state needed by the optimizer.
    pub fn into_parts(self) -> (AdamParams, WeightVector, f64) {
        (self.adam_params, self.weights, self.k)
    }

    /// Build the checkpoint filename for a specific epoch.
    pub fn epoch_path(training_data_path: impl AsRef<Path>, epoch: usize) -> std::path::PathBuf {
        let training_data_path = training_data_path.as_ref();
        let parent = training_data_path.parent().unwrap_or_else(|| Path::new(""));
        let stem = training_data_path
            .file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or("checkpoint");

        parent.join(format!("{stem}.epoch-{epoch}.adam-checkpoint.json"))
    }
}
impl Default for AdamParams {
    /// Default values are taken from <https://www.geeksforgeeks.org/deep-learning/adam-optimizer/>
    fn default() -> Self {
        Self {
            // the prepared zurichess dataset has ~1.4M positions, so this gives us ~100 mini batches per epoch
            batch_size: 14_000,
            validation_ratio: 0.1,
            learning_rate: 0.001,
            beta_1: 0.9,
            beta_2: 0.999,
            epsilon: 1e-8,
            epoch: 0,
            t: 0,
            m: WeightVector::zeros(),
            v: WeightVector::zeros(),
            delta: 1.0,
        }
    }
}

/// This throws all idiomatic efficiency-focussed rust out of the fucking window because it has a complexity of
/// O(w * e * p), where w is the number of weights, e is the number of epochs and p is the number of test positions
/// All because we're doing numeric calculations of gradients this was a horrible design choice
pub fn adam(
    training_data: &mut [TrainingSample],
    weights: &mut WeightVector,
    adam_params: &mut AdamParams,
    k: f64,
    epochs: usize,
    training_data_path: impl Into<String>,
) -> io::Result<()> {
    let training_data_path = training_data_path.into();
    let mut m = adam_params.m;
    let mut v = adam_params.v;
    let mut t = adam_params.t;
    let beta_1 = adam_params.beta_1;
    let beta_2 = adam_params.beta_2;
    let delta = adam_params.delta;
    let epoch_start_index = adam_params.epoch;
    const LOG_EVERY: usize = 30;

    // core adam loop
    for epoch in epoch_start_index..(epoch_start_index + epochs) {
        let epoch_start = Instant::now();
        // randomly creating mini_batches
        training_data.shuffle(&mut rng());
        let mini_batches = training_data.chunks(adam_params.batch_size);
        // reserve batches for validation
        let training_batch_count =
            ((1.0 - adam_params.validation_ratio) * mini_batches.len() as f64) as usize;

        assert!(training_batch_count < mini_batches.len());

        for (i, batch) in mini_batches.clone().take(training_batch_count).enumerate() {
            let batch_start = Instant::now();
            t += 1;

            let mut gradient = WeightVector::zeros();
            calculate_weight_gradients(&mut gradient, weights, batch, delta, k);

            // update momentum and RMSProp
            m = (beta_1 * m) + (1.0 - beta_1) * gradient;
            v = (beta_2 * v) + (1.0 - beta_2) * gradient.component_mul(&gradient);

            // bias correction
            let m_corrected = m / (1.0 - beta_1.powi(t));
            let v_corrected = v / (1.0 - beta_2.powi(t));

            // upate weights
            *weights = weights.zip_zip_map(&v_corrected, &m_corrected, |w, v, m| {
                w - adam_params.learning_rate / (v.sqrt() + adam_params.epsilon) * m
            });

            if i.is_multiple_of(LOG_EVERY) {
                println!(
                    "Batch {i} of {training_batch_count} ({:?}, total {:?})",
                    batch_start.elapsed(),
                    epoch_start.elapsed()
                )
            }
        }

        let params: TunableParams = weights.into();

        // validation
        let mut scored_samples: Vec<(f64, f64)> = vec![];
        for remaining_batch in mini_batches.skip(training_batch_count) {
            scored_samples.extend(
                remaining_batch
                    .par_iter()
                    .map(|TrainingSample { fen, result }| {
                        (result, evaluate(&Board::new(fen), &params))
                    })
                    .map(|(&label, pred)| (label.into(), sigmoid(pred as f64, k)))
                    .collect::<Vec<(f64, f64)>>(),
            );
        }
        assert!(!scored_samples.is_empty());
        let validation_error = mse(&scored_samples);
        println!(
            "Epoch {epoch} of {epochs}: validation error: {validation_error}, took: {:?}",
            epoch_start.elapsed()
        );

        adam_params.m = m;
        adam_params.v = v;
        adam_params.t = t;
        adam_params.epoch = epoch + 1;

        let checkpoint = AdamCheckpoint::new(
            training_data_path.clone(),
            adam_params.clone(),
            *weights,
            k,
            Some(validation_error),
        );
        let checkpoint_path = AdamCheckpoint::epoch_path(&training_data_path, epoch + 1);
        checkpoint.write_to_file(&checkpoint_path)?;
    }

    adam_params.m = m;
    adam_params.v = v;
    adam_params.t = t;
    adam_params.epoch = epoch_start_index + epochs;

    Ok(())
}

/// Numerically calculate the gradient of the MSE loss function over the current weights
/// This has so fucking horrible performance it hurts my soul
/// but I don't have the time nor the fucks to give to refactor the eval to be linear in respect to its weights
/// (especially since for king safety this is NOT the case and that is the thing I feel is in the most desperate need of tuning)
///
/// So yes throw more hardware at the problem 200 cores werden schon irgendwie regeln hahahaha
///
/// this function is the this is fine meme turned code
fn calculate_weight_gradients(
    gradient: &mut WeightVector,
    weights: &mut WeightVector,
    batch: &[TrainingSample],
    delta: f64,
    k: f64,
) {
    for w in 0..weights.len() {
        weights[w] -= 1.0;
        let params_lo: TunableParams = weights.into();
        weights[w] += 2.0;
        let params_hi: TunableParams = weights.into();
        // reset weights to where they were before
        weights[w] -= 1.0;

        // score every sample in the batch with the params
        let scored_samples: [Vec<(f64, f64)>; 2] = [
            batch
                .par_iter()
                .map(|TrainingSample { fen, result }| {
                    (result, evaluate(&Board::new(fen), &params_lo))
                })
                .map(|(&label, pred)| (label.into(), sigmoid(pred as f64, k)))
                .collect(),
            batch
                .par_iter()
                .map(|TrainingSample { fen, result }| {
                    (result, evaluate(&Board::new(fen), &params_hi))
                })
                .map(|(&label, pred)| (label.into(), sigmoid(pred as f64, k)))
                .collect(),
        ];
        let mse = [mse(&scored_samples[0]), mse(&scored_samples[1])];
        gradient[w] = (mse[1] - mse[0]) / (2.0 * delta);
    }
}

pub fn optimize_k(training_data: &[TrainingSample], params: &TunableParams) -> f64 {
    // augment the training data with evaluations from the default weights
    let eval_start = Instant::now();
    let outcomes_with_evals: Vec<(f64, f64)> = training_data
        .par_iter()
        .map(|TrainingSample { fen, result }| {
            ((*result).into(), evaluate(&Board::new(fen), params) as f64)
        })
        .collect();
    println!("evaluation of one batch took: {:?}", eval_start.elapsed());
    let fens_with_evals: Vec<(&String, (f64, i32))> = training_data
        .par_iter()
        .map(|TrainingSample { fen, result }| {
            (fen, ((*result).into(), evaluate(&Board::new(fen), params)))
        })
        .collect();
    println!(
        "Sanity check: Eval Range [{:?}, {:?}]",
        fens_with_evals
            .iter()
            .min_by_key(|(_, (_, eval))| *eval)
            .unwrap(),
        fens_with_evals
            .iter()
            .max_by_key(|(_, (_, eval))| *eval)
            .unwrap()
    );

    // grid search for the best K value
    const K_PRECISION: usize = 10;
    let mut start = 0.0;
    let mut end = 20.0;
    let step_count = 20;
    let mut min_err = 1.0;
    let mut best_k = 0.0;
    for _ in 0..K_PRECISION {
        let step_size = (end - start) / step_count as f64;
        for k in (0..=step_count).map(|step| start + step_size * step as f64) {
            let evals_scaled_with_k: Vec<(f64, f64)> = outcomes_with_evals
                .iter()
                .map(|(label, eval)| (*label, sigmoid(*eval, k)))
                .collect();
            let error = mse(evals_scaled_with_k.as_slice());
            if error < min_err {
                min_err = error;
                best_k = k;
            }
        }
        start = best_k - step_size;
        end = best_k + step_size;
    }
    best_k
}

/// Calculate the mean squared error over a zipped vector of labels and predictions.
/// Format for one sample: `(label, prediction)`
/// Assumes label and prediction have been scaled beforehand
pub fn mse(samples: &[(f64, f64)]) -> f64 {
    samples
        .iter()
        .map(|(label, pred)| (label - pred).powi(2))
        .sum::<f64>()
        / samples.len() as f64
}

/// I've tried using the simpler base e sigmoid proposed by Ethereal, however that
/// (notably due to the lack of the /400) drives k very close to zero. This should yield a more reasonable k.
pub fn sigmoid(eval: f64, k: f64) -> f64 {
    1.0 / (1.0 + f64::powf(10.0, -k * eval / 400.0))
}

#[cfg(test)]
mod tests {
    use super::{AdamCheckpoint, AdamParams, WeightVector};

    #[test]
    fn checkpoint_round_trip_json() {
        let checkpoint = AdamCheckpoint::new(
            "training.epd",
            AdamParams::default(),
            WeightVector::zeros(),
            1.23,
            Some(0.42),
        );

        let json = serde_json::to_string(&checkpoint).expect("checkpoint should serialize");
        let reloaded: AdamCheckpoint =
            serde_json::from_str(&json).expect("checkpoint should deserialize");

        assert_eq!(checkpoint, reloaded);
    }
}
