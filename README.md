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
