import chess

board = chess.Board("rnbqkbnr/pppp2pp/8/4pP2/8/7P/PPPP2P1/RNBQKBNR w KQkq e6 0 2")
print(len(list(board.legal_moves)))
