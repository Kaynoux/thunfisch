import sys

import chess


def get_legal_moves_in_different_format(fen):
    try:
        board = chess.Board(fen)
    except ValueError:
        print(f"Error: Bad FEN: {fen}", file=sys.stderr)
        return None

    custom_format_moves = []
    for move in board.legal_moves:
        # Convert to FEN Char
        from_sq = chess.square_name(move.from_square)
        to_sq = chess.square_name(move.to_square)

        promo_char = ""
        if move.promotion:
            # Add promotion char if promotion
            promo_char = chess.piece_symbol(move.promotion).lower()

        custom_format_moves.append(f"{from_sq}{to_sq}{promo_char}")

    return custom_format_moves


fen_string = "8/8/8/1K1pP1qk/8/8/8/8 w - - 0 1"
legal_moves_list = get_legal_moves_in_different_format(fen_string)

# Prints all legal moves in a list
if legal_moves_list:
    print(len(legal_moves_list))
    print(", ".join(legal_moves_list))
