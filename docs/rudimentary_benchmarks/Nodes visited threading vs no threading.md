![[Pasted image 20251022110135.png]]
## Threading
(the traditional thunfisch approach of [[Iterative Deepening]])

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

| Depth | Nodes      |
| ----- | ---------- |
| 1     | 21         |
| 2     | 146        |
| 3     | 3 028      |
| 4     | 21 993     |
| 5     | 871 367    |
| 6     | 1 877 450  |
| 7     | 21 852 777 |



# Pos 1
![[Pasted image 20251022112332.png]]

### pos 1 - non threaded

| Depth | Nodes      |
| ----- | ---------- |
| 1     | 245        |
| 2     | 3 251      |
| 3     | 5 267      |
| 4     | 214 274    |
| 5     | 274 646    |
| 6     | 3 375 970  |
| 7     | 6 425 725  |
| 8     | 52 907 319 |

### pos 2 - threaded

| Depth | Nodes      |
| ----- | ---------- |
| 1     | 4 171      |
| 2     | 25 190     |
| 3     | 32 742     |
| 4     | 148 362    |
| 5     | 421 265    |
| 6     | 3 529 536  |
| 7     | 9 838 290  |
| 8     | 68 067 695 |
