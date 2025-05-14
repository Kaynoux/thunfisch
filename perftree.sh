# This script uses perftree
# Explained here and how to use: 
# https://github.com/agausmann/perftree.git

depth="$1"
fen="$2"
moves="$3"
engine="./target/release/rusty-chess-bot"

if [[ -z "$depth" || -z "$fen" ]]; then
  echo "Usage: $0 <depth> \"<fen>\" \"[moves]\""
  exit 1
fi

{
  if [[ -n "$moves" ]]; then
    echo "position fen $fen moves $moves"
  else
    echo "position fen $fen"
  fi
  echo "go perft $depth --perftree"
  echo "quit"
} | "$engine"