#!/bin/bash
# Tournament script for comparing multiple engine versions with fastchess

# Parse arguments
TOURNAMENT_DIR="./tournament"
RUN_IN_BACKGROUND=false

# Parse flags and directory
while [ $# -gt 0 ]; do
    case "$1" in
        --background)
            RUN_IN_BACKGROUND=true
            shift
            ;;
        --)
            shift
            break
            ;;
        -*)
            echo "Error: Unknown flag: $1"
            exit 1
            ;;
        *)
            TOURNAMENT_DIR="$1"
            shift
            ;;
    esac
done

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

# Print configuration
echo "Tournament Directory: $TOURNAMENT_DIR"
echo "Found $engine_count engine(s)"
echo "Background Mode: $RUN_IN_BACKGROUND"
echo "================================"
echo "Executing: $CMD"
echo "================================"

# Execute with or without background
if [ "$RUN_IN_BACKGROUND" = true ]; then
    eval "$CMD &"
    BG_PID=$!
    echo ""
    echo "Tournament started in background (PID: $BG_PID)"
    echo "To monitor progress: tail -f benchmark_summary_*.txt"
    echo "To stop: kill $BG_PID"
else
    eval "$CMD"
fi
