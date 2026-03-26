#!/bin/sh
# requires fastchess to be installed and available through the command 'fastchess'

# ensure we start at root project
# yes this dies if you run it from somewhere else completely but fuck it

ROOT_DIR=$(pwd | sed -E "s#/testing/?##g")

FIRST_VERSION=
BUILD_DIR=$ROOT_DIR/target/release/
PREVIOUS_VERSION=./previous
NEW_VERSION=./current

build() {
    cd $ROOT_DIR
    branch=$(git branch | grep -E "^\* " | cut -c 3-)
    git stash drop && git stash
    git checkout master
    git fetch && git pull
    cargo build --locked --release
    cp -f $BUILD_DIR/thunfisch /tmp/thunfisch-previous
    # done building master
    git checkout $branch
    git stash pop
    cargo build --locked --release
    mv $BUILD_DIR/thunfisch ./testing/current
    mv /tmp/thunfisch-previous ./testing/previous
}

run_sprt() {
    # todo: name the new engine after the branch or sth
    cd $ROOT_DIR/testing
    rm sprt.pgn
    fastchess \
        -engine cmd=$NEW_VERSION name=current -engine cmd=$PREVIOUS_VERSION name=previous \
        -each proto=uci tc=8+0.8 \
        -pgnout file=sprt.pgn \
        -openings file=8moves_v3.pgn format=pgn order=random \
        -concurrency 4 \
        -rounds 5000 \
        -recover \
        -sprt elo0=0 elo1=10 alpha=0.05 beta=0.05
}

build && run_sprt
