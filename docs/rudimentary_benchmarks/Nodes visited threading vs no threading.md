


## Threading
(the traditional thunfisch approach of [[Iterative Deepening]])

```
go depth 7
info  depth 1 seldepth 1  score cp 144 nodes 20 nps 3609 time 5 tt 0 pv g1f3
info  depth 2 seldepth 2  score cp 0 nodes 428 nps 1000350 time 0 tt 0 pv g1f3 g8f6
info  depth 3 seldepth 5  score cp 126 nodes 3028 nps 5214881 time 0 tt 0 pv g1f3 g8f6 b1c3
info  depth 4 seldepth 6  score cp 0 nodes 59109 nps 9647849 time 6 tt 0 pv g1f3 g8f6 b1c3 b8c6
info  depth 5 seldepth 9  score cp 10 nodes 376210 nps 12369219 time 30 tt 0 pv g1f3 g8f6 b1c3 b8c6 a1b1
info  depth 6 seldepth 16  score cp 0 nodes 10563766 nps 15187538 time 695 tt 0 pv g1f3 g8f6 b1c3 b8c6 a1b1 a8b8
info  depth 7 seldepth 19  score cp 69 nodes 50415959 nps 12340882 time 4085 tt 1 pv g1f3 g8f6 b1c3 b8c6 e2e3 a8b8 f1d3
```


| Depth | Nodes      |
| ----- | ---------- |
| 1     | 20         |
| 2     | 428        |
| 3     | 3 028      |
| 4     | 59 109     |
| 5     | 376 210    |
| 6     | 10 563 766 |
| 7     | 50 415 959 |

## No Threading
-> [[Alpha Beta Pruning]] at root level


