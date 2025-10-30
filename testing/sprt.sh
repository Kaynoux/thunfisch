#!/bin/sh
# requires fastchess to be installed and available through the command 'fastchess'

FIRST_VERSION=
PREVIOUS_VERSION=./previous
NEW_VERSION=./current

run_sprt() {
    # todo: name the new engine after the branch or sth
    fastchess \
        -engine cmd=$NEW_VERSION name=current -engine cmd=$PREVIOUS_VERSION name=previous \
        -each proto=uci tc=5+0.5 \
        -pgnout file=sprt.pgn \
        -openings file=8moves_v3.pgn format=pgn order=random \
        -concurrency 4 \
        -rounds 5000 \
        -recover \
        -sprt elo0=0 elo1=100 alpha=0.005 beta=0.005
}
run_sprt
