# Thunfisch Tuning
somehow most of this is NOT vibe coded, and still the fact that the tuning has cubic performance is because of my dumb ass believing claude that "trust me bro numeric derivation is gonna be fine" (and the fact that I'm too dumb to understand both the Ethereal paper on analytical derivation and whatever fatalii does with feature evaluation)

## Purpose

The tool supports two workflows:

- preparing EPD input by using the engine's quiescence search to find a truly
	quiet position for each entry and writing the resulting positions into a new
	prepared file
- running the current training entrypoint, which loads labeled positions and
	runs the ADAM optimizer with per-epoch checkpointing

## Usage

From the `tuning` directory:

### Prepare training data

```bash
cargo run -- prepare <input.epd> [output.epd]
```

- `<input.epd>`: path to the original training data file.
- `[output.epd]`: optional path for the prepared file.

If the output path is omitted, the program writes to
`<input>.prepared.epd`.

### Run training

```bash
cargo run -- train <input.epd> [epochs] [restore-checkpoint.json]
```

- `<input.epd>`: path to the labeled training data file.
- `[epochs]`: number of epochs to train in this run, defaults to `1`.
- `[restore-checkpoint.json]`: optional explicit checkpoint file to resume
  from. If omitted, training starts from fresh defaults.

Per-epoch checkpoints are written next to the input file with an epoch suffix,
for example `<input>.epoch-3.adam-checkpoint.json`.

Note that the training data is assumed to be prepared by `prepare`.

## Example

```bash
cargo run -- prepare zurichess_quiet-labeled.v7.epd zurichess_quiet-labeled.prepared.epd
```

## File format

The parser supports lines containing a FEN prefix followed by a quoted game
result, for example:

```text
r2qkr2/p1pp1ppp/1pn1pn2/2P5/3Pb3/2N1P3/PP3PPP/R1B1KB1R b KQq - c9 "0-1";
```

The prepared output file will contain the same result label, but the FEN will be
replaced with the final quiet position found by the quiescence search.

## Estimation of runtime
Locally on a 10-core mac, evaluating one batch of the training data takes ~250ms. (release mode).
We'll assume that the time scales anti-proportionally with the cores available; meaning a single core would take $250\text{ms} \cdot 10 = 2.5\text{s}$.
We have approximately 2000 weights in our eval at the time of writing.
Since we calculate derivatives numerically, we'll need to evaluate positions twice ($\vec{w_{t}} - \vec{\delta}$, $\vec{w_{t}} + \vec{\delta}$); meaning calculating the gradient of the Loss for one epoch costs us $0.5$ seconds.
**NOTE** that this assumes full batch-sizes; mini batching will reduce this significantly.

The formula for estimating how long tuning will take thus is somewhere along the following:

$$\frac{2.5}{c} * \frac{e}{b}\ \text{s}$$

where:
- $c$ is the number of available CPU cores
- $e$ is the number of total epochs
- $b$ is the amount of mini-batches per epoch

So assuming full batch sizes (so no mini-batching), training with $4000$ Epochs on my local mac should take $1000\text{s} \approx 16\  \text{minutes}$.
