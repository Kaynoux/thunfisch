import chess

# board = chess.Board("rnbqkbnr/pppp2pp/8/4pP2/8/7P/PPPP2P1/RNBQKBNR w KQkq e6 0 2")
board = chess.Board("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b - - 0 1")
print(board.legal_moves)
