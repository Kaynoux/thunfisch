#!/usr/bin/env python3
import os
import shutil
import subprocess
import argparse
from pathlib import Path

FAST = shutil.which("fastchess")
if not FAST:
    print("fastchess nicht gefunden. Abbruch.")
    raise SystemExit(1)

p = argparse.ArgumentParser(description="compact fastchess runner")
p.add_argument("--new",    default=os.environ.get("NEW_VERSION", "../target/release/thunfisch"))
p.add_argument("--prev",   default=os.environ.get("PREVIOUS_VERSION", "/tmp/thunfisch"))
p.add_argument("--tc",     default=os.environ.get("TC", "5+0.5"))
p.add_argument("--rounds", type=int, default=int(os.environ.get("ROUNDS", "20")))
p.add_argument("--open",   default=os.environ.get("OPENINGS", "8moves_v3.pgn"))
p.add_argument("--concur", type=int, default=int(os.environ.get("CONCURRENCY", "4")))
p.add_argument("--mode", choices=("dummy","sprt","both"), default="both")
args = p.parse_args()

def run(cmd):
    print("RUN:", " ".join(cmd))
    subprocess.run(cmd, check=False)

if args.mode in ("dummy","both"):
    run([FAST,
         "-engine", f"cmd={args.new}", "name=new-1",
         "-engine", f"cmd={args.new}", "name=new-2",
         "-each", f"proto=uci", f"tc={args.tc}",
         "-rounds", "2",
         "-openings", f"file={args.open}", "format=pgn", "order=random",
         "-concurrency", str(args.concur),
    ])

if args.mode in ("sprt","both"):
    run([FAST,
         "-engine", f"cmd={args.new}", "name=new",
         "-engine", f"cmd={args.prev}", "name=old",
         "-each", "proto=uci", f"tc={args.tc}",
         "-rounds", str(args.rounds),
         "-repeat",
         "-openings", f"file={args.open}", "format=pgn", "order=random",
         "-sprt", "elo0=0", "elo1=5", "alpha=0.05", "beta=0.05",
    ])
`