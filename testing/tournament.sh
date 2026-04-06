#!/bin/sh
# Tournament script for comparing multiple engine versions with pentanomial distribution

ROOT_DIR=$(pwd | sed -E "s#/testing/?##g")

run_tournament() {
    if [ $# -ne 4 ]; then
        echo "usage: ./tournament.sh engine1 engine2 engine3 engine4"
        echo ""
        echo "example: ./tournament.sh ./engine-all ./engine-no-nmp ./engine-no-rfp ./engine-baseline"
        exit 1
    fi

    ENGINE1=$1
    ENGINE2=$2
    ENGINE3=$3
    ENGINE4=$4

    # Validate that all engines exist
    for engine in "$ENGINE1" "$ENGINE2" "$ENGINE3" "$ENGINE4"; do
        if [ ! -f "$engine" ] && [ ! -x "$engine" ]; then
            echo "Error: Engine not found or not executable: $engine"
            exit 1
        fi
    done

    cd $ROOT_DIR/testing
    rm -f tournament.pgn

    echo "=== Tournament Configuration ==="
    echo "Engine 1: $ENGINE1"
    echo "Engine 2: $ENGINE2"
    echo "Engine 3: $ENGINE3"
    echo "Engine 4: $ENGINE4"
    echo "Time Control: 8+0.8"
    echo "Distribution: Pentanomial"
    echo "================================"

    fastchess \
        -engine cmd=$ENGINE1 name=$ENGINE1  \
        -engine cmd=$ENGINE2 name=$ENGINE2 \
        -engine cmd=$ENGINE3 name=$ENGINE3 \
        -engine cmd=$ENGINE4 name=$ENGINE4 \
        -each proto=uci tc=8+0.8 \
        -pgnout file=tournament.pgn \
        -openings file=8moves_v3.pgn format=pgn order=random \
        -concurrency 4 \
        -rounds 1000 \
        -recover
}

run_tournament $@
