import io

import chess

# ------ Konfiguration ------
fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
board = chess.Board(fen)

# ------ Analyse ------
results = {}
initial_white_moves = list(board.legal_moves)  # Liste der Züge von Weiß

print(f"Analysiere FEN: {fen}")
print(f"Anzahl initialer weißer Züge: {len(initial_white_moves)}")
print("-" * 40)
print("Format: [Weißer Zug]: [Legale Züge Schwarz] Promotions: [Promotionen Schwarz]")
print("-" * 40)


for white_move in initial_white_moves:
    # Mache den Zug von Weiß auf dem Brett
    board.push(white_move)

    # Jetzt ist Schwarz am Zug. Zähle die legalen Züge und Promotions von Schwarz.
    black_legal_moves = list(board.legal_moves)
    black_total_legal_moves = len(black_legal_moves)
    black_promotion_count = 0
    for black_move in black_legal_moves:
        if black_move.promotion:
            black_promotion_count += 1

    # Speichere die Ergebnisse für den ursprünglichen weißen Zug
    results[white_move.uci()] = (black_total_legal_moves, black_promotion_count)

    # Mache den Zug von Weiß rückgängig, um das Brett für die nächste Iteration zurückzusetzen
    board.pop()

# ------ Ergebnisse ausgeben (sortiert nach Zug-UCI) ------
for move_uci in sorted(results.keys()):
    total_moves, promo_count = results[move_uci]
    print(f"{move_uci}: {total_moves} Promotions: {promo_count}")

print("-" * 40)
print(f"Analyse für {len(results)} Züge abgeschlossen.")
