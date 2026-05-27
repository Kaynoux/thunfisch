# Thunfisch Tuning
somehow most of this is NOT vibe coded, and still the fact that the tuning has cubic performance is because of my dumb ass believing claude that "trust me bro numeric derivation is gonna be fine" (and the fact that I'm too dumb to understand both the Ethereal paper on analytical derivation and whatever fatalii does with feature evaluation)

## Usage

### Obtain training data
just grab the epd file from zurichess: https://bitbucket.org/zurichess/tuner/downloads/quiet-labeled.v7.epd.gz

### 1. Prepare training data

```bash
cargo run -- prepare <input.epd> [output.epd]
```

- `<input.epd>`: path to the original training data file.
- `[output.epd]`: optional path for the prepared file.

If the output path is omitted, the program writes to
`<input>.prepared.epd`.

### 2. Run training

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

## EPD File format

The parser supports lines containing a FEN prefix followed by a quoted game
result, for example:

```text
r2qkr2/p1pp1ppp/1pn1pn2/2P5/3Pb3/2N1P3/PP3PPP/R1B1KB1R b KQq - c9 "0-1";
```

The prepared output file will contain the same result label, but the FEN will be
replaced with the final quiet position found by the quiescence search.
