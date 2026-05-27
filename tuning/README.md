# Thunfisch Tuning

This crate is a small companion binary for preparing training data and running
the first step of the Thunfisch chess engine's Texel tuning pipeline.

## Purpose

The tool supports two workflows:

- preparing EPD input by using the engine's quiescence search to find a truly
	quiet position for each entry and writing the resulting positions into a new
	prepared file
- running the current training entrypoint, which loads labeled positions and
	optimizes the sigmoid `k` value used by the Texel loss

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
cargo run -- train <input.epd>
```

- `<input.epd>`: path to the labeled training data file.

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
