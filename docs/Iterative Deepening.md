[Chess Wiki](https://www.chessprogramming.org/Iterative_Deepening)



## Ablauf
1. Züge mit Tiefe 1 generieren (um rekursives multithreading zu vermeiden)
2. Für jeden der Züge aus Tiefe 1 (hier beginnt threading):
	1. Zug wird ausgeführt
	2. _danach_ wird [[Alpha Beta Pruning|Alpha Beta Search]] gestartet
	3. generiert: Tupel `(bester move, eval after ausführen,` [[Seldepth]])
3. linear search durch alle züge nach der besten evaluation
	1. "zwei-stufige Sortierung": Größere Tiefe immer besser, danach bester move
4. Change Promotion to always do a queen