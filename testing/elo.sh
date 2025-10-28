#!/bin/sh
# requires fastchess to be installed and available through the command 'fastchess'

FIRST_VERSION=
PREVIOUS_VERSION=/tmp/thunfisch
NEW_VERSION=../target/release/thunfisch

run_sprt() {
    # todo: name the new engine after the branch or sth
    fastchess \
        -engine cmd=../target/release/thunfisch name=new -engine cmd=$PREVIOUS_VERSION name=old \
        -each proto=uci tc=5+0.5 \
        -rounds 20 \
        -repeat \
        -openings file=8moves_v3.pgn format=pgn order=random \
        -sprt elo0=0 elo1=5 alpha=0.05 beta=0.05
}

# dummy game for testing setup
fastchess \
  -engine cmd=$NEW_VERSION name=new-1 -engine cmd=$NEW_VERSION name=new-2 \
  -each proto=uci tc=5+0.5 \
  -rounds 2 \
  -openings file=8moves_v3.pgn format=pgn order=random \
  -concurrency 4
