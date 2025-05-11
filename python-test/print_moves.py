import sys

import chess


def get_legal_moves_in_custom_format(fen):
    """
    Generiert legale Züge für eine FEN-Position im Format 'a1b2' oder 'a7a8q'.

    Args:
        fen: Die FEN-Zeichenkette der Brettstellung.

    Returns:
        Eine Liste von Zügen im benutzerdefinierten String-Format.
    """
    try:
        board = chess.Board(fen)
    except ValueError:
        print(f"Error: Ungültige FEN-Zeichenkette: {fen}", file=sys.stderr)
        return None

    custom_format_moves = []
    for move in board.legal_moves:
        # Konvertiere Start- und Zielfeld-Indizes (0-63) in algebraische Notation
        from_sq = chess.square_name(move.from_square)
        to_sq = chess.square_name(move.to_square)

        promo_char = ""
        if move.promotion:
            # Füge das kleingeschriebene Symbol der Promotionsfigur hinzu
            # chess.piece_symbol gibt Großbuchstaben zurück (Q, R, B, N)
            promo_char = chess.piece_symbol(move.promotion).lower()

        custom_format_moves.append(f"{from_sq}{to_sq}{promo_char}")

    return custom_format_moves


# --- Beispielverwendung ---
# fen_string = "rnbqkbnr/pppp2pp/8/4pP2/8/7P/PPPP2P1/RNBQKBNR w KQkq e6 0 2"
fen_string = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"

legal_moves_list = get_legal_moves_in_custom_format(fen_string)

if legal_moves_list:
    # Gib die Liste aus (nützlich für weitere Verarbeitung in Python)
    # print(legal_moves_list)

    # Gib die Züge als eine einzelne, durch Leerzeichen getrennte Zeichenkette aus
    # (oft praktisch zum Kopieren als Testdaten)
    print(", ".join(legal_moves_list))
