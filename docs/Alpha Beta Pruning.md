[Chess Wiki](https://www.chessprogramming.org/Alpha-Beta)

> Prune paths of a [[Minimax Algorithmus]] in order to not have to calculate the full tree

- $\alpha$ and $\beta$ are the "memory" from the previously explored tree
-  and  measure the **best possible** scores (of a leaf position) for each player based on the previously explored tree:
	- $\alpha$ is best score for <span style="color:rgb(126, 198, 54)">maximizing</span> player 
	- $\beta$ is best possible score for <span style="color:rgb(245, 154, 35)">minimizing</span> player

> [!hint] Breaking condition is _always_ $\alpha \gt \beta$.
> - $\alpha \gt \beta$ means: "the best path for <span style="color:rgb(126, 198, 54)">maximizing</span> player is better than <span style="color:rgb(245, 154, 35)">minimizing</span>'s best option so far"
>	- if it's <span style="color:rgb(245, 154, 35)">black</span>'s turn -> $\alpha$ came from this branch, so black won't take that
>	- if it's <span style="color:rgb(126, 198, 54)">white</span>'s turn -> means black has a better option on this branch
>		- => the previously explored path is better
## pseudo-implementation
```python
def alpha_beta(pos, alpha, beta, depth, maximizing):
	if pos.game_over() or depth == 0
		return eval(pos)
	if maximizing:
		best_eval = -INFINITY
		for mv in pos.moves():
			leval = alpha_beta(pos + move, alpha, beta, depth - 1, !maximizing)
			best_eval = max(best_eval, leval)
			alpha = max(alpha, best_eval)
			if alpha > beta:
				break
	elif:
		best_eval = INFINITY
		for mv in pos.moves():
			leval = alpha_beta(pos + move, alpha, beta, depth - 1, !maximizing)
			best_eval = min(best_eval, leval)
			alpha = min(alpha, best_eval)
			if alpha > beta:
				break
```

## Optimization
- if better moves are evaluated first:
	- they lead to a higher evaluation => better $\alpha$ value at root
	- it's more likely that later branches will be worse (= lower $\beta$ value) and they will be pruned
## Gekritzel
- annotiert mit $\alpha$/$\beta$  werten
![[Pasted image 20251015153455.png]]
### Beispiel aus https://www.youtube.com/watch?v=l-hh51ncgDI&t=221s
![[Pasted image 20251015153840.png]]

![[Pasted image 20251015132209.png]]
