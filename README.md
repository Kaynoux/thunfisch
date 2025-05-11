## TODO

- if bitwise operations are to slow use `#[inline(always)]` before compare functions to reduce overhead (is maybe done autmaticllay idk)

## Flamegraphs

`samply record ./target/release/rusty-chess-bot`

`uv run autoperft.py stockfish ./target/release/rusty-chess-bot --fen "8/PPPk4/8/8/8/8/4Kppp/8 w - - 0 1" --max_depth 6`

8/PPPk4/8/8/8/8/4Kppp/8 w - - 0 1

perftree ./perft-test.sh

"2B5/PP1k4/8/8/8/8/4Kppp/8 b - - 0 1"

`uv run autoperft.py stockfish ./target/release/rusty-chess-bot --fen "Q7/1PPk4/8/8/8/8/4Kppp/8 b - - 0 1" --max_depth 1`

`uv run autoperft.py stockfish ./target/release/rusty-chess-bot --fen "2B5/PP1k4/8/8/8/8/4Kppp/8 b - - 0 1" --max_depth 1`

`uv run autoperft.py stockfish ./target/release/rusty-chess-bot --fen "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"  --max_depth 4`

" --max_depth 6`

test fens:
sebastian league search test r3k2r/p1ppqpb1/Bn2pnp1/3PN3/1p2P3/2N2Q2/PPPB1PpP/R3K2R b KQ - 0 1

- rust not trait is not const why???

- debug traits and other traits removen if not needed

- es fehlen r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1
  24 normal non capture

On i5 13600kf:

Without Move unmaking:
perft 7 --rayon
Perft: Depth=7 Nodes=3,195,901,860 Time=75.035s Nodes/sec=42,592,214

With move unmaking:
perft 7 --rayon
Perft: Depth=7 Nodes=3,195,901,860 Time=47.432s Nodes/sec=67,379,035

New move generation:
go perft 7 --rayon
Perft: Depth=7 Nodes=3,195,901,860 Time=12.206s Nodes/sec=261,825,410

scc --include-ext rs .

WHY DOES RUST NOT HAVE const operators for custom typesa

naming convetions

- Bitboard
- Bit
- Square

- Piece
- Figure

- https://lichess.org/editor/8/8/8/1K1pP1q1/8/8/8/8_w_-_-_0_1?color=white
- ep pinmask edge case aus der hölle
