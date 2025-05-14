import chess

# Prints legal moves given a fen
board = chess.Board("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b - - 0 1")
print(board.legal_moves)
