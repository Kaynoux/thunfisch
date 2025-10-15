[Chess Wiki](https://www.chessprogramming.org/Minimax)
> Durchrechnen eines Zugbasierten Spiels durch Simulation der bestmöglichen Züge

> [!hint] Thunfisch benutzt [[Negamax]]
## Tree
- MInimax builds up a Tree:
	- Nodes are positions in the game/ game States
	- Edges are moves done by the player, transposing the current position (Node)) into its child

## Algorithm
1. build up Minimax tree until specified depth
2. Evaluate all final positions
3. Evaluate all positions upwards:
	1. Each node is the **minimum** (if player to move is minimizing player) or **max** (if player to move is maximizing) of its children
4. Make first move in root position depending on whether player to move plans to do maximum or minimum