<p align="center">
  <img src="https://github.com/Kaynoux/thunfisch/blob/master/logo.png" alt="logo" width="300"/>
</p>

# thunfisch

Rusty Chess Bot is a UCI-compatible chess engine written from scratch in Rust. It uses magic‐bitboard move generation, iterative deepening with alpha-beta and quiescence search and a transposition table.

## 🚀 Quick Start

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

## Commands
```Basic commands
UCI
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
