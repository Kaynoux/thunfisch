#!/usr/bin/env python3
"""
Convert PGN (Portable Game Notation) or raw moves to UCI (Universal Chess Interface) notation.
Uses the python-chess library.

Usage:
    python pgn_to_uci.py <pgn_string>
    python pgn_to_uci.py -f <pgn_file>
    
Examples:
    python pgn_to_uci.py "1. e4 e5 2. Nf3 Nc6"
    python pgn_to_uci.py -f game.pgn
    cat game.pgn | python pgn_to_uci.py -f -
"""

import sys
import re
import chess
import chess.pgn
from pathlib import Path
from io import StringIO


def extract_moves_from_text(text):
    """
    Extract moves from raw algebraic notation text.
    Handles formats like:
    - "1. e4 e5 2. Nf3" (with move numbers)
    - "e4 e5 Nf3" (without move numbers)
    """
    # Remove move numbers (e.g., "1.", "2.", etc.)
    text_no_numbers = re.sub(r'\d+\.', '', text)
    
    # Split by whitespace
    tokens = text_no_numbers.split()
    
    # Filter tokens that look like moves (not just numbers or dots)
    moves = []
    for token in tokens:
        # Skip if it's just a number or empty
        if not token or token.isdigit():
            continue
        # Keep the token as-is for SAN parsing
        moves.append(token)
    
    return moves


def san_to_uci(san_moves):
    """
    Convert a list of moves in Standard Algebraic Notation (SAN) to UCI notation.
    
    Args:
        san_moves: List of moves in SAN format
        
    Returns:
        List of moves in UCI format
    """
    board = chess.Board()
    uci_moves = []
    
    for san_move in san_moves:
        if not san_move.strip():
            continue
            
        try:
            move = board.parse_san(san_move)
            uci_moves.append(move.uci())
            board.push(move)
        except (ValueError, chess.IllegalMoveError) as e:
            print(f"Warning: Could not parse move '{san_move}': {e}", file=sys.stderr)
            continue
    
    return uci_moves


def pgn_to_uci(pgn_content):
    """
    Convert PGN content to UCI moves.
    Handles both properly formatted PGN and raw move notation.
    
    Args:
        pgn_content: PGN string content
        
    Returns:
        List of tuples (game_number, uci_moves_list)
    """
    results = []
    
    # Try standard PGN parsing first
    games = []
    try:
        pgn_io = StringIO(pgn_content)
        while True:
            game = chess.pgn.read_game(pgn_io)
            if game is None:
                break
            games.append(game)
    except Exception:
        # If standard parsing fails, try to extract raw moves
        games = []
    
    # If we got games from standard PGN parsing, use those
    if games:
        for game_num, game in enumerate(games, 1):
            board = game.board()
            uci_moves = []
            
            for move in game.mainline_moves():
                uci_moves.append(move.uci())
                board.push(move)
            
            results.append((game_num, uci_moves))
    else:
        # Fall back to raw move extraction
        san_moves = extract_moves_from_text(pgn_content)
        if san_moves:
            uci_moves = san_to_uci(san_moves)
            results.append((1, uci_moves))
    
    return results


def main():
    if len(sys.argv) < 2:
        print("Usage: python pgn_to_uci.py <pgn_string>")
        print("       python pgn_to_uci.py -f <pgn_file>")
        print("\nExamples:")
        print('  python pgn_to_uci.py "1. e4 e5 2. Nf3"')
        print("  python pgn_to_uci.py -f game.pgn")
        print("  cat game.pgn | python pgn_to_uci.py -f -")
        sys.exit(1)
    
    pgn_content = None
    
    # Check if using file input
    if sys.argv[1] == "-f":
        if len(sys.argv) < 3:
            print("Error: -f flag requires a file path argument", file=sys.stderr)
            sys.exit(1)
        
        pgn_source = sys.argv[2]
        
        # Read PGN content
        try:
            if pgn_source == "-":
                pgn_content = sys.stdin.read()
            else:
                pgn_path = Path(pgn_source)
                if not pgn_path.exists():
                    print(f"Error: File '{pgn_source}' not found", file=sys.stderr)
                    sys.exit(1)
                with open(pgn_path, 'r') as f:
                    pgn_content = f.read()
        except Exception as e:
            print(f"Error reading input: {e}", file=sys.stderr)
            sys.exit(1)
    else:
        # Use string argument
        pgn_content = sys.argv[1]
    
    # Convert and output
    results = pgn_to_uci(pgn_content)
    
    if not results:
        print("No games found in PGN", file=sys.stderr)
        sys.exit(1)
    
    # Output results
    for game_num, uci_moves in results:
        if len(results) > 1:
            print(f"Game {game_num}:")
        print(" ".join(uci_moves))


if __name__ == "__main__":
    main()