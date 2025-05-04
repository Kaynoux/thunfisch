## TODO

- if bitwise operations are to slow use `#[inline(always)]` before compare functions to reduce overhead (is maybe done autmaticllay idk)

## Flamegraphs

`samply record ./target/release/rusty-chess-bot`

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
