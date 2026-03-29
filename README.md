<p align="center">
  <img src="./media/logo.png" alt="logo" width="300"/>
</p>

# thunfisch [![Thunfisch](https://github.com/Kaynoux/thunfisch/actions/workflows/thunfisch.yml/badge.svg)](https://github.com/Kaynoux/thunfisch/actions/workflows/thunfisch.yml)

Thunfisch is a UCI-compatible chess engine written from scratch in Rust. It uses magic‐bitboard move generation, iterative deepening with alpha-beta and quiescence search and a transposition table. For evaluation, Piece-Square Tables are used.

It is a listed Bot-Account on Lichess. If it is online you can challenge it [here](https://lichess.org/@/thunfisch-bot).

## Build Features
For easy isolation of individual engine features, these have been added as compiler features to the project. See [Cargo.toml](Cargo.toml) for what features are available, and [settings.rs](src/settings.rs) for how they relate to the code.
### Building with all features
``` bash
cargo build
# or: cargo build --features all
````

### Building with only specific features
For example, say you want *only* Quiescence Search, Alpha Beta and MVV-LVA move ordering enabled. Then run this command:
```bash
cargo build --no-default-features --features "ab,qs,mvv-lva"
```
## How to Play Locally
Thunfisch is a command-line application that implements the Universal Chess Interface (UCI). To play against it comfortably, you should load the compiled binary into a chess GUI. We recommend [Cutechess](https://github.com/cutechess/cutechess). There are instruction on how to add the bot to the gui [here](https://lczero.org/play/gui/cutechess/).

**General Setup Steps:**
1. Build the engine using `cargo build --release`.
2. Locate the compiled executable at `target/release/thunfisch` (or `thunfisch.exe` on Windows).
3. Open your preferred chess GUI and look for an "Add Engine" or "Manage Engines" option in the settings.
4. Point the GUI to the executable file. You can then start a new game and select Thunfisch as your opponent.

## Codebase Overview
- `src/types/`: Core data structures representing the chess board, pieces, bitboards, and moves.
- `src/move_generator/`: Logic for generating legal chess moves efficiently using magic bitboards and pre-calculated masks.
- `src/search/`: The primary chess intelligence containing alpha-beta pruning, iterative deepening, quiescence search, move ordering, and the static evaluation function.
- `src/utils/`: Helper functions including make/unmake move mechanics and Zobrist hashing for the transposition table.
- `src/communication/`: UCI protocol parser and FEN string handling.
- `src/debug/`: Performance testing (`perft`) algorithms and visualization tools for debugging the board state.
- `python-debugging/`: Python scripts for advanced visualization, depth/time plotting, and log analysis.
- `testing/`: Scripts for engine match testing (SPRT).

## Commands
```
Basic commands
  uci                - Identify engine and author
  isready            - Engine readiness check
  ucinewgame         - Start new game (resets engine state)
  position [options] - Set up position (see below)
  go [parameters]    - Start search (see below)
  quit               - Exit engine
  fen                - Print current FEN

position options
  startpos           - Set up the standard chess starting position
  fen <FEN>          - Set up a position from a FEN string
  moves <m1> <m2>    - Play moves from the given position

go parameters:
  depth <n>          - Search to fixed depth n (plies)
  wtime <ms>         - White time left (ms)
  btime <ms>         - Black time left (ms)
  winc <ms>          - White increment per move (ms)
  binc <ms>          - Black increment per move (ms)
  movestogo <n>      - Moves to next time control
  movetime <ms>      - Search exactly this many ms

perft commands:
  go perft <depth> [--debug|--perftree|--rayon]
    --debug     - Print debug info for perft
    --perftree  - Print perft formatted for perftree
    --rayon     - Use rayon for parallel perft

Examples:
  position startpos moves e2e4 e7e5
  go depth 6
  go wtime 60000 btime 60000 winc 0 binc 0
  go perft 7 --rayon

Debugging:
  draw               - Print board
  moves              - Print legal moves
  eval               - Prints current Evaluation with Depth of 0
  do <move>          - Play move (e.g. do e2e4)
  pinmask            - Show pin masks
  checkmask          - Show check mask
  attackmask         - Show attack mask
  empty              - Empty Squares Bitboard
  white              - White Squares Bitboard
  black              - Black Squares Bitboard
  occupied           - Occupied Squares Bitboard
  hash               - Current board hash
```

## Development Quickstart
1. Clone the repository

   ```bash
   git clone https://github.com/Kaynoux/thunfisch.git
   cd thunfisch
   ```

2. Run the tests

   ```bash
   cargo test --release
   ```

3. Run the engine

   ```bash
   cargo run --release
   ```


## License
This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.
