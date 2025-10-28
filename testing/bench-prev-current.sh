fastchess \
-engine cmd=./current \
name=current \
-engine cmd=./previous \
name=previous \
-each proto=uci \
-each tc=0/1:00+2 \
-rounds 1 \
-concurrency 4 \
-openings file=8moves_v3.pgn format=pgn order=random