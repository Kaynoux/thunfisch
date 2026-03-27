#!/bin/sh
# this is j
ROOT_DIR=$(pwd | sed -E "s#/testing/?##g")

run_sprt() {
    # todo: name the new engine after the branch or sth
    cd $ROOT_DIR/testing
    rm sprt.pgn
    echo "dev: $1"
    echo "base: $2"
    fastchess \
        -engine cmd=$1 name=dev -engine cmd=$2 name=base \
        -each proto=uci tc=8+0.8 \
        -pgnout file=sprt.pgn \
        -openings file=8moves_v3.pgn format=pgn order=random \
        -concurrency 4 \
        -rounds 5000 \
        -recover \
        -sprt elo0=0 elo1=10 alpha=0.05 beta=0.05
}
run_sprt $@
