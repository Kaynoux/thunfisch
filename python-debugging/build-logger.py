#!/usr/bin/env python3
import os
import subprocess
import sys
import time
from pathlib import Path

BIN_DIR = Path("./bin")
LOG_DIR = Path("./logs")
FIXTIME_MS = 10_000  # 10s
DEPTH = None  # nicht genutzt, wir verwenden fixtime


def run_bin(bin_path: Path):
    LOG_DIR.mkdir(parents=True, exist_ok=True)
    log_path = LOG_DIR / (bin_path.name + ".log")

    with open(log_path, "w", encoding="utf-8") as log:
        log.write(f"# RUN {bin_path.name} go fixtime {FIXTIME_MS}\n")
        log.flush()

        proc = subprocess.Popen(
            [str(bin_path)],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            bufsize=1,
        )

        def send(cmd: str):
            if proc.stdin:
                proc.stdin.write(cmd + "\n")
                proc.stdin.flush()
                log.write(f">>> {cmd}\n")
                log.flush()

        # Start: Stellung setzen und fixtime-Go senden
        send("position index 1")
        send(f"go fixtime {FIXTIME_MS}")

        bestmove_seen = False

        try:
            while True:
                line = proc.stdout.readline() if proc.stdout else ""
                if not line:
                    if proc.poll() is not None:
                        break
                    time.sleep(0.005)
                    continue

                line = line.rstrip("\n")
                log.write(line + "\n")
                log.flush()

                if "bestmove" in line.lower():
                    bestmove_seen = True
                    # Optional: weiter lesen, falls du mehr Output willst.
                    # Hier brechen wir direkt ab.
                    break
        finally:
            # stdin schließen
            try:
                if proc.stdin:
                    proc.stdin.close()
            except Exception:
                pass

            # Prozess normal auslaufen lassen
            try:
                proc.wait(timeout=2.0)
            except Exception:
                pass

        log.write("OK: bestmove received\n" if bestmove_seen else "NOTE: no bestmove\n")
        log.flush()

    print(f"Done: {bin_path.name} -> {log_path}")


def main():
    if not BIN_DIR.exists():
        print(f"Bin-Ordner nicht gefunden: {BIN_DIR}", file=sys.stderr)
        sys.exit(1)

    bins = sorted(
        p
        for p in BIN_DIR.iterdir()
        if p.is_file() and os.access(p, os.X_OK) and not p.name.endswith(".log")
    )
    if not bins:
        print(f"Keine Binaries in {BIN_DIR} gefunden.", file=sys.stderr)
        sys.exit(1)

    for b in bins:
        run_bin(b)


if __name__ == "__main__":
    main()
