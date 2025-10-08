## 1. main::main()
- call `handle_uci_communication()`
## 2. communication::uci::handle_uci_communication()
- I/O Interface using stateless [[UCI]] Protocoll
- `"go [...]"` starts search for bestmove from current board
## 3. communication::bestmove::bestmove()
- Calculates time budget using go arguments (wtime, btime, ...)
- calculate bestmove with [[Iterative Deepening]]
### cases
- depth exists -> keine time control, sondern er "weiß wie weit"
- wtime (i.e. no fixed depth) -> [[Iterative Deepening]]
## 4. iterative_deepening::iterative_deepening()
- generates the first set of moves (depth=1) from current position
- generate the deeper moves by iterating over the root moves with `par_iter` 
- this uses [[Alpha Beta Pruning]]
## 5. search::alpha_beta::alpha_beta()
- generates array of all moves using `generate_moves()`
## 6.  move_generator::generator::generate_moves()
- generates the needed array with all legal moves for a given board
- calculates pinmask
- calculates check_mask
- can be configured to only search for special moves (attacks or promotions)
- calculation is seperated into
	- king moves only if there are 2 checks because the it can only be resolved by moving king
	- pawn moves 
	- knight moves
	- bishop moves
	- rook moves
	- queen moves
	- king moves
	- castle moves
	- en passant moves
## 7. search::alpha_beta::alpha_beta()
- sort moves to evalute using `order_moves`
- this brings faster cut offs in the alpha beta search
## 8. search::move_ordering::order_moves()
- scores all moves using [[MVV-LVA Move Ordering]]
- and then sorts them into an array regarding the score
- basically it gives each piece a score based on its position and gives bonusses for captures and promotions
- test if the board has already been hashed, if so it retrieves the last best position from the hash to sort it first
- goes through all sorted moves and does every move, scores it and unmakes it 
- alpha beta search is done recursivly until depth is 0 
- starts the [[Quiescene Search]]
- stores the best move in the hash table
- return best move
## 9.  search::quiescence_search::quiescence_search()
-  is just another [[Alpha Beta Pruning]] search but only searches special moves or all moves if board is in check
- only if the recursive quiescence search reaches depth 0 is the board evaluated, calls `evaluate`
## 10. search::evaluate::evaluate()
- gives each piece a score based on its position
- Uses [[Piece Square Table Evaluation]]  specifcally [PeSTO](https://www.chessprogramming.org/PeSTO%27s_Evaluation_Function)
- this eval uses game phase to score position differently based on how many pieces are left on the board
- then it goes up back the recursive tree and just prints the best move it found
