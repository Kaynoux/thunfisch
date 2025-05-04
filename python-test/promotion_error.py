from collections import (
    Counter,
)  # Counter wird hier nicht mehr direkt benötigt, aber schadet nicht

import chess

# Die ursprüngliche Funktion wird für diese spezielle Aufgabe nicht benötigt.
# def collect_promotions(board: chess.Board, depth: int) -> Counter:
#     """
#     Geht alle legalen Züge bis 'depth' durch und zählt,
#     wie oft jeder Promotion-Zug (in UCI-Notation) vorkommt.
#     """
#     cnt = Counter()
#     def dfs(b: chess.Board, d: int):
#         if d == 0:
#             return
#         for mv in b.legal_moves:
#             b.push(mv)
#             if mv.promotion:
#                 cnt[mv.uci()] += 1
#             dfs(b, d - 1)
#             b.pop()
#     dfs(board, depth)
#     return cnt

# ------ Konfiguration ------
fen = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1"
board = chess.Board(fen)

# ------ Analyse ------
# Wir wollen wissen, wie viele Promotions Schwarz nach jedem weißen Zug hat.
promotions_after_white_move = {}
initial_white_moves = list(board.legal_moves)  # Liste der Züge von Weiß

print(f"Analysiere {len(initial_white_moves)} legale Züge für Weiß aus FEN: {fen}")
print("-" * 30)

for white_move in initial_white_moves:
    # Mache den Zug von Weiß auf dem Brett
    board.push(white_move)

    # Jetzt ist Schwarz am Zug. Zähle die legalen Promotionszüge von Schwarz.
    black_promotion_count = 0
    for black_move in board.legal_moves:
        if black_move.promotion:
            # In dieser spezifischen FEN kann Schwarz nur auf b1 umwandeln (b2xb1=X)
            # Wir zählen einfach jede Promotion, die Schwarz jetzt machen kann.
            black_promotion_count += 1

    # Speichere die Anzahl der schwarzen Promotions für den ursprünglichen weißen Zug
    promotions_after_white_move[white_move.uci()] = black_promotion_count

    # Mache den Zug von Weiß rückgängig, um das Brett für die nächste Iteration zurückzusetzen
    board.pop()

# ------ Ergebnisse ausgeben ------
print("\nAnzahl der schwarzen Promotionsmöglichkeiten direkt nach jedem weißen Zug:")
total_moves_checked = 0
moves_with_0_promotions = 0
moves_with_4_promotions = 0

# Sortieren nach Zug für bessere Lesbarkeit
for move_uci, count in sorted(promotions_after_white_move.items()):
    print(f"{move_uci}: {count}")
    total_moves_checked += 1
    if count == 0:
        moves_with_0_promotions += 1
    elif count == 4:  # In dieser Stellung kann Schwarz nur 0 oder 4 Promotions haben
        moves_with_4_promotions += 1

print("-" * 30)
print(f"Gesamtzahl analysierter weißer Züge: {total_moves_checked}")
print(f"Züge, nach denen Schwarz 0 Promotions hat: {moves_with_0_promotions}")
print(f"Züge, nach denen Schwarz 4 Promotions hat: {moves_with_4_promotions}")

# ------ Deine ursprünglichen Prints am Ende (bezogen auf den *initialen* Zustand) ------
# Das Brett sollte nach der Schleife wieder im Ausgangszustand sein.
print("\n--- Info zum initialen Brettzustand (Weiß am Zug) ---")
initial_legal_moves_list = list(board.legal_moves)
print(
    "Anzahl legaler Züge für Weiß in der Ausgangsposition:",
    len(initial_legal_moves_list),
)
# print("Legale Züge für Weiß in der Ausgangsposition:", board.legal_moves) # Kann sehr lang sein
