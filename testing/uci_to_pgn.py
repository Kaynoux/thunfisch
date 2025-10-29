from sys import argv

board = [
    ["R", "N", "B", "K", "Q", "B", "N", "R"],
    ["P", "P", "P", "P", "P", "P", "P", "P"],
    ["", "", "", "", "", "", "", ""],
    ["", "", "", "", "", "", "", ""],
    ["", "", "", "", "", "", "", ""],
    ["", "", "", "", "", "", "", ""],
    ["p", "p", "p", "p", "p", "p", "p", "p"],
    ["r", "n", "b", "k", "q", "b", "n", "r"],
]


def _print_board():
    for row in reversed(board):
        print("| ", end="")
        for cell in reversed(row):
            print(cell if cell != "" else " ", end=" | ")
        print("\n---------------------------------")


def coord_to_arr_index(coord: str) -> (int, int):
    # a = 7
    # h = 0
    return (int(coord[1]) - 1, 7 - (ord(coord[0]) - ord("a")))


def to_potential_castle(mv_from, mv_to) -> str | None:
    from_ind = coord_to_arr_index(mv_from)
    if board[from_ind[0]][from_ind[1]].upper() != "K":
        # sometimes you gotta love yourself some dynamic typing
        return None

    if (mv_from[0], mv_to[0]) == ("e", "g"):
        row = int(mv_from[1]) - 1
        board[row][1] = board[row][3]
        board[row][2] = board[row][0]
        board[row][3] = ""
        board[row][0] = ""
        return "O-O"
    if (mv_from[0], mv_to[0]) == ("e", "c"):
        row = int(mv_from[1]) - 1
        board[row][5] = board[row][3]
        board[row][4] = board[row][7]
        board[row][3] = ""
        board[row][7] = ""
        return "O-O-O"


# okay nice lichess (or pgn) is fucking kulant
# we can ignore checks and everything
# also fuck pure fp
def make_move(uci_move: str) -> str:
    mv_from, mv_to = uci_move[:2], uci_move[2:4]
    if castling := to_potential_castle(mv_from, mv_to):
        return castling
    from_row, from_col = coord_to_arr_index(mv_from)
    to_row, to_col = coord_to_arr_index(uci_move[2:4])
    mv_pgn = (
        board[from_row][from_col].upper()
        + mv_from
        + ("x" if board[to_row][to_col] != "" else "")
        + mv_to
    )

    board[to_row][to_col] = board[from_row][from_col]
    board[from_row][from_col] = ""
    # should only be promotions
    suffix = uci_move[4:]
    if suffix:
        mv_pgn += suffix.upper()
    return mv_pgn


def uci_to_pgn(moves: list[str]) -> str:
    pgn = ""
    for i, move in enumerate(moves):
        if i % 2 == 0:
            pgn += f"{(i // 2) + 1}. "
        pgn += make_move(move) + " "
    return pgn


def main():
    try:
        uci_coords = argv[-1]
        moves = uci_coords.split(" ")
        pgn = uci_to_pgn(moves)
        print(pgn)
        # _print_board()
    except Exception as e:
        print(e)
        print("usage:\npython3 uci_to_pgn.py {pgn}")


if __name__ == "__main__":
    main()
