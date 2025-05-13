import sys

import chess
import chess.polyglot


def main():
    if len(sys.argv) != 2:
        print(f'Usage: {sys.argv[0]} "<FEN>"')
        sys.exit(1)

    fen = sys.argv[1]
    board = chess.Board(fen)
    h = chess.polyglot.zobrist_hash(board)

    print("Decimal: ", h)
    print("Hex:     ", hex(h))


if __name__ == "__main__":
    main()
