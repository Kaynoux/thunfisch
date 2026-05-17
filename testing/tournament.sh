#!/bin/bash
# Tournament script for comparing multiple engine versions with fastchess

# Get the tournament directory from argument or use default
TOURNAMENT_DIR="${1:-./ tournament}"

# Validate directory exists
if [ ! -d "$TOURNAMENT_DIR" ]; then
    echo "Error: Tournament directory not found: $TOURNAMENT_DIR"
    exit 1
fi

# Base command
CMD="fastchess"

# Add 'master' first if it exists
if [ -x "$TOURNAMENT_DIR/master" ]; then
    CMD="$CMD \\
    -engine cmd=$TOURNAMENT_DIR/master name=master"
fi

# Add other engines from the tournament folder (excluding master)
for engine_path in "$TOURNAMENT_DIR"/*; do
    if [ -x "$engine_path" ] && [ -f "$engine_path" ]; then
        engine_name=$(basename "$engine_path")
        if [ "$engine_name" != "master" ]; then
            CMD="$CMD \\
    -engine cmd=$engine_path name=$engine_name"
        fi
    fi
done

# Verify we have at least 2 engines
engine_count=$(echo "$CMD" | grep -o '\-engine' | wc -l)
if [ "$engine_count" -lt 2 ]; then
    echo "Error: Found only $engine_count engine(s). Need at least 2 engines."
    exit 1
fi

# Add the rest of the arguments
CMD="$CMD \\
    -each proto=uci tc=8+0.08 \\
    -concurrency 10 \\
    -rounds 1000 \\
    -repeat \\
    -recover \\
    -openings file=8moves_v3.pgn format=pgn order=random \\
    -pgnout file=results.pgn \\
    -sprt elo0=0 elo1=10 alpha=0.05 beta=0.05 \\
    | tee benchmark_summary_\$(date +%Y-%m-%d_%H-%M).txt"

# Print and execute
echo "Tournament Directory: $TOURNAMENT_DIR"
echo "Found $engine_count engine(s)"
echo "================================"
echo "Executing: $CMD"
echo "================================"
eval "$CMD"
