Helpful Python Scripts to validate the engine

fastchess \
  -engine cmd=./bin-base/thunfisch \
  name=old \
  -engine cmd=./bin-features/thunfisch \
  name=new \
  -each proto=uci \
  -each tc=0/0:30+1 \
  -rounds 5000 \
  -concurrency 6 \
  -openings file=opening_book.pgn format=pgn order=random \
  -sprt elo0=0 elo1=10 alpha=0.05 beta=0.05