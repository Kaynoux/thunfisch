#!/usr/bin/env python3
import re
from collections import OrderedDict
from pathlib import Path

import matplotlib.pyplot as plt

LOG_DIR = Path("./plot_logs")
OUT_PNG = Path("./plot_depth_time.png")

# Beispiele:
# info  depth 5 ... time 9652 ... timeout true total_time 10010
# info  depth 4 ... time 315 ... timeout false total_time 358
INFO_RE = re.compile(
    r"\bdepth\s+(\d+)\b.*?\btimeout\s+(true|false)\b.*?\btotal_time\s+(\d+)\b",
    re.IGNORECASE,
)


def parse_log(path: Path):
    # Liefert OrderedDict{depth -> total_time_ms} (letzte gültige info je depth gewinnt)
    depth_time = OrderedDict()
    with path.open("r", encoding="utf-8", errors="replace") as f:
        for line in f:
            m = INFO_RE.search(line)
            if not m:
                continue
            depth = int(m.group(1))
            timeout_flag = m.group(2).lower() == "true"
            total_time_ms = int(m.group(3))
            if timeout_flag:
                # Zeilen mit timeout true ignorieren
                continue
            depth_time[depth] = total_time_ms
    return depth_time


def main():
    logs = sorted(LOG_DIR.glob("*.log"))
    if not logs:
        print(f"Keine .log Dateien in {LOG_DIR} gefunden.")
        return

    series = {}  # name -> {depth: total_time_ms}
    for log in logs:
        data = parse_log(log)
        if not data:
            continue
        name = log.stem  # Dateiname ohne .log
        series[name] = data

    if not series:
        print("Keine verwertbaren info-Zeilen gefunden.")
        return

    plt.figure(figsize=(8, 5), dpi=120)
    for name, dt in sorted(series.items()):
        depths = sorted(dt.keys())
        times = [dt[d] for d in depths]
        plt.plot(depths, times, marker="o", linewidth=1.5, label=name)

    plt.xlabel("Depth")
    plt.ylabel("Total time (ms)")
    plt.title("Depth vs. Total time per Binary (timeout=true ignoriert)")
    plt.grid(True, linestyle="--", alpha=0.3)
    plt.legend(fontsize=8, loc="best")
    OUT_PNG.parent.mkdir(parents=True, exist_ok=True)
    plt.tight_layout()
    plt.savefig(OUT_PNG)
    print(f"Plot gespeichert: {OUT_PNG}")
    try:
        plt.show()
    except Exception:
        pass


if __name__ == "__main__":
    main()
