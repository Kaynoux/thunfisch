> Gesamter State des Games

- enthält z.B.
	- verschiedene [[Bitboard]]s
	- etc etc

> [!hint] Basically einfach nur ein [[FEN]]  string

(see code for documentation)
## Code
```rust
pub struct Board {  
    color_bbs: [Bitboard; 2],  
    occupied: Bitboard,  
    figure_bbs: [Bitboard; 13],  
    figures: [Figure; 64],  
    black_king_castle: bool,  
    black_queen_castle: bool,  
    white_queen_castle: bool,  
    white_king_castle: bool,  
    ep_target: Option<Bit>,  
    current_color: Color,  
    halfmove_clock: usize,  
    total_halfmove_counter: usize,  
    unmake_info_stack: Vec<UnmakeInfo>,  
    hash: u64,  
}
```



### Unmake info stack
