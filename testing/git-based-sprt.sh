#!/bin/sh
# requires fastchess to be installed and available through the command 'fastchess'

# ensure we start at root project
# yes this dies if you run it from somewhere else completely but fuck it

ROOT_DIR=$(pwd | sed -E "s#/testing/?##g")

FIRST_VERSION=
BUILD_DIR=$ROOT_DIR/target/release/
PREVIOUS_VERSION=./master
NEW_VERSION=./dev

build() {
    cd $ROOT_DIR
    branch=$(git branch | grep -E "^\* " | cu2t -c 3-)
    git stash drop && git stash
    git checkout master
    git fetch && git pull
    cargo build --locked --release
    cp -f $BUILD_DIR/thunfisch /tmp/thunfisch-master
    # done building master
    git checkout $branch
    git stash pop
    cargo build --locked --release
    mv $BUILD_DIR/thunfisch ./testing/engines/dev
    mv /tmp/thunfisch-previous ./testing/engines/master
}
source sprt.sh


build && run_sprt engines/$NEW_VERSION engines/$PREVIOUS_VERSION
