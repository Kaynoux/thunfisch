import chess

# board = chess.Board("rnbqkbnr/pppp2pp/8/4pP2/8/7P/PPPP2P1/RNBQKBNR w KQkq e6 0 2")
board = chess.Board(
    "r3k2r/p1Ppqpb1/bn2pnp1/4N3/1p2P3/2N2Q2/PPPBBPpP/R3K2R w KQkq - 0 1"
)
print(board.legal_moves)
