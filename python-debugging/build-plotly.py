#!/usr/bin/env python3
import re
from collections import OrderedDict
from pathlib import Path

import plotly.graph_objects as go
from plotly.subplots import make_subplots

LOG_DIR = Path("./plot_logs")
OUT_PNG = Path("./plot_depth_time.png")
OUT_HTML = Path("./plot_depth_time.html")
OUT_MD = Path("./plot_depth_time_table.md")

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
                continue
            depth_time[depth] = total_time_ms
    return depth_time


def build_markdown_pivot(series: dict) -> str:
    # series: {name -> {depth: time_ms}}
    names = sorted(series.keys())
    all_depths = sorted({d for dt in series.values() for d in dt.keys()})

    # Header
    header = ["Bot"] + [str(d) for d in all_depths]
    md_lines = []
    md_lines.append("| " + " | ".join(header) + " |")
    md_lines.append("| " + " | ".join(["---"] * len(header)) + " |")

    # Rows
    for name in names:
        row = [name]
        for d in all_depths:
            val = series[name].get(d)
            row.append(str(val) if val is not None else "–")
        md_lines.append("| " + " | ".join(row) + " |")

    return "\n".join(md_lines)


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
        name = log.stem
        series[name] = data

    if not series:
        print("Keine verwertbaren info-Zeilen gefunden.")
        return

    # Plotly-Figur (nur Plot oben)
    fig = make_subplots(rows=1, cols=1)

    for name, dt in sorted(series.items()):
        depths = sorted(dt.keys())
        times = [dt[d] for d in depths]
        fig.add_trace(
            go.Scatter(
                x=depths,
                y=times,
                mode="lines+markers",
                name=name,
                line=dict(width=2),
                marker=dict(size=6),
                hovertemplate="depth=%{x}<br>time=%{y} ms<extra>" + name + "</extra>",
            )
        )

    fig.update_layout(
        title="Depth vs. Total time per Binary (timeout=true ignoriert)",
        xaxis_title="Depth",
        yaxis_title="Total time (ms)",
        template="plotly_white",
        legend=dict(font=dict(size=10)),
        margin=dict(l=50, r=20, t=60, b=50),
        height=450,
    )
    fig.update_xaxes(showgrid=True, gridwidth=1, gridcolor="rgba(0,0,0,0.1)")
    fig.update_yaxes(showgrid=True, gridwidth=1, gridcolor="rgba(0,0,0,0.1)")

    # Dateien speichern
    OUT_PNG.parent.mkdir(parents=True, exist_ok=True)
    try:
        # PNG-Export benötigt: pip install -U kaleido
        fig.write_image(str(OUT_PNG), scale=2)
        print(f"PNG gespeichert: {OUT_PNG}")
    except Exception as e:
        print(f"PNG-Export übersprungen (kaleido fehlt?): {e}")

    fig.write_html(str(OUT_HTML), include_plotlyjs="cdn", full_html=True)
    print(f"HTML gespeichert: {OUT_HTML}")

    # Markdown-Pivot erzeugen
    md = build_markdown_pivot(series)
    OUT_MD.write_text(md, encoding="utf-8")
    print(f"Markdown-Tabelle gespeichert: {OUT_MD}\n")
    print(md)

    try:
        fig.show()
    except Exception:
        pass


if __name__ == "__main__":
    main()
