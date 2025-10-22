#!/usr/bin/env python3
"""
Script to plot chess engine performance data parsed from engine output strings.
Compares node counts at different depths for threading vs no threading scenarios.
Plots total nodes, alpha-beta nodes, and quiescence search nodes.
"""

import matplotlib.pyplot as plt
import numpy as np
import re


def parse_engine_output(output_string):
    """Parse engine output string and extract depth, nodes, nodes_ab, and nodes_qs"""
    lines = output_string.strip().split("\n")
    data = {"depth": [], "nodes": [], "nodes_ab": [], "nodes_qs": []}

    for line in lines:
        if line.startswith("info") and "depth" in line:
            # Extract depth
            depth_match = re.search(r"depth (\d+)", line)
            if depth_match:
                depth = int(depth_match.group(1))
                data["depth"].append(depth)

                # Extract nodes
                nodes_match = re.search(r"nodes (\d+)", line)
                nodes = int(nodes_match.group(1)) if nodes_match else 0
                data["nodes"].append(nodes)

                # Extract nodes_ab
                nodes_ab_match = re.search(r"nodes_ab (\d+)", line)
                nodes_ab = int(nodes_ab_match.group(1)) if nodes_ab_match else 0
                data["nodes_ab"].append(nodes_ab)

                # Extract nodes_qs
                nodes_qs_match = re.search(r"nodes_qs (\d+)", line)
                nodes_qs = int(nodes_qs_match.group(1)) if nodes_qs_match else 0
                data["nodes_qs"].append(nodes_qs)

    return data


# Paste your engine output strings here:

# Non-threaded engine output
non_threaded_output = """
info  depth 1 seldepth 10  score cp 529 nodes 245 nps 733 time 333 tt 0 pv g2h1q | nodes_ab 1 nodes_qs 244
info  depth 2 seldepth 14  score cp 529 nodes 3251 nps 2232729 time 1 tt 0 pv g2h1q f3h1 | nodes_ab 45 nodes_qs 3206
info  depth 3 seldepth 15  score cp 529 nodes 5267 nps 1962832 time 2 tt 0 pv g2h1q f3h1 b4c3 | nodes_ab 128 nodes_qs 5139
info  depth 4 seldepth 16  score cp 529 nodes 214274 nps 2225563 time 96 tt 0 pv g2h1q f3h1 b4c3 d2c3 | nodes_ab 3247 nodes_qs 211027
info  depth 5 seldepth 17  score cp 529 nodes 274646 nps 2360115 time 116 tt 0 pv g2h1q f3h1 b4c3 d2c3 e6d5 | nodes_ab 8911 nodes_qs 265735
info  depth 6 seldepth 18  score cp 517 nodes 3375970 nps 2156792 time 1565 tt 0 pv g2h1q f3h1 b4c3 d2c3 e6d5 a6d3 | nodes_ab 133807 nodes_qs 3242163
info  depth 7 seldepth 19  score cp 526 nodes 6425725 nps 2384287 time 2695 tt 0 pv g2h1q f3h1 b4c3 d2c3 e6d5 a6b7 a8b8 | nodes_ab 301876 nodes_qs 6123849
info  depth 8 seldepth 20  score cp 530 nodes 52907319 nps 1946296 time 27183 tt 0 pv g2h1q f3h1 b4c3 d2c3 e6d5 a6d3 f6e4 d3e4 | nodes_ab 3870776 nodes_qs 49036543
"""

# Threaded engine output - replace this with your actual threaded output
threaded_output = """
info  depth 1 seldepth 10  score cp 529 nodes 4171 nps 733 time 333 tt 0 pv g2h1q | nodes_ab 100 nodes_qs 4071
info  depth 2 seldepth 14  score cp 529 nodes 25190 nps 2232729 time 1 tt 0 pv g2h1q f3h1 | nodes_ab 500 nodes_qs 24690
info  depth 3 seldepth 15  score cp 529 nodes 32742 nps 1962832 time 2 tt 0 pv g2h1q f3h1 b4c3 | nodes_ab 800 nodes_qs 31942
info  depth 4 seldepth 16  score cp 529 nodes 148362 nps 2225563 time 96 tt 0 pv g2h1q f3h1 b4c3 d2c3 | nodes_ab 5000 nodes_qs 143362
info  depth 5 seldepth 17  score cp 529 nodes 421265 nps 2360115 time 116 tt 0 pv g2h1q f3h1 b4c3 d2c3 e6d5 | nodes_ab 15000 nodes_qs 406265
info  depth 6 seldepth 18  score cp 517 nodes 3529536 nps 2156792 time 1565 tt 0 pv g2h1q f3h1 b4c3 d2c3 e6d5 a6d3 | nodes_ab 200000 nodes_qs 3329536
info  depth 7 seldepth 19  score cp 526 nodes 9838290 nps 2384287 time 2695 tt 0 pv g2h1q f3h1 b4c3 d2c3 e6d5 a6b7 a8b8 | nodes_ab 500000 nodes_qs 9338290
info  depth 8 seldepth 20  score cp 530 nodes 68067695 nps 1946296 time 27183 tt 0 pv g2h1q f3h1 b4c3 d2c3 e6d5 a6d3 f6e4 d3e4 | nodes_ab 6000000 nodes_qs 62067695
"""

# Parse the engine outputs
non_threaded_data = parse_engine_output(non_threaded_output)
threaded_data = parse_engine_output(threaded_output)

# Create the plots
fig, ((ax1, ax2), (ax3, ax4)) = plt.subplots(2, 2, figsize=(18, 12))

# Plot 1: Total nodes - Linear scale
ax1.plot(
    non_threaded_data["depth"],
    non_threaded_data["nodes"],
    marker="o",
    linewidth=2,
    label="Non-threaded",
    color="blue",
)
ax1.plot(
    threaded_data["depth"],
    threaded_data["nodes"],
    marker="s",
    linewidth=2,
    label="Threaded",
    color="red",
)

ax1.set_xlabel("Depth")
ax1.set_ylabel("Total Nodes Visited")
ax1.set_title("Total Nodes: Non-threaded vs Threaded")
ax1.legend()
ax1.grid(True, alpha=0.3)
ax1.set_xticks(range(1, max(non_threaded_data["depth"]) + 1))

# Plot 2: Alpha-Beta nodes
ax2.plot(
    non_threaded_data["depth"],
    non_threaded_data["nodes_ab"],
    marker="o",
    linewidth=2,
    label="Non-threaded",
    color="blue",
)
ax2.plot(
    threaded_data["depth"],
    threaded_data["nodes_ab"],
    marker="s",
    linewidth=2,
    label="Threaded",
    color="red",
)

ax2.set_xlabel("Depth")
ax2.set_ylabel("Alpha-Beta Nodes")
ax2.set_title("Alpha-Beta Nodes: Non-threaded vs Threaded")
ax2.legend()
ax2.grid(True, alpha=0.3)
ax2.set_xticks(range(1, max(non_threaded_data["depth"]) + 1))

# Plot 3: Quiescence Search nodes
ax3.plot(
    non_threaded_data["depth"],
    non_threaded_data["nodes_qs"],
    marker="o",
    linewidth=2,
    label="Non-threaded",
    color="blue",
)
ax3.plot(
    threaded_data["depth"],
    threaded_data["nodes_qs"],
    marker="s",
    linewidth=2,
    label="Threaded",
    color="red",
)

ax3.set_xlabel("Depth")
ax3.set_ylabel("Quiescence Search Nodes")
ax3.set_title("Quiescence Search Nodes: Non-threaded vs Threaded")
ax3.legend()
ax3.grid(True, alpha=0.3)
ax3.set_xticks(range(1, max(non_threaded_data["depth"]) + 1))

# Plot 4: All three on logarithmic scale for comparison
ax4.semilogy(
    non_threaded_data["depth"],
    non_threaded_data["nodes"],
    marker="o",
    linewidth=2,
    label="Total (Non-threaded)",
    color="blue",
    linestyle="-",
)
ax4.semilogy(
    threaded_data["depth"],
    threaded_data["nodes"],
    marker="s",
    linewidth=2,
    label="Total (Threaded)",
    color="red",
    linestyle="-",
)
ax4.semilogy(
    non_threaded_data["depth"],
    non_threaded_data["nodes_ab"],
    marker="^",
    linewidth=2,
    label="Alpha-Beta (Non-threaded)",
    color="blue",
    linestyle="--",
)
ax4.semilogy(
    threaded_data["depth"],
    threaded_data["nodes_ab"],
    marker="v",
    linewidth=2,
    label="Alpha-Beta (Threaded)",
    color="red",
    linestyle="--",
)
ax4.semilogy(
    non_threaded_data["depth"],
    non_threaded_data["nodes_qs"],
    marker="*",
    linewidth=2,
    label="Quiescence (Non-threaded)",
    color="blue",
    linestyle=":",
)
ax4.semilogy(
    threaded_data["depth"],
    threaded_data["nodes_qs"],
    marker="p",
    linewidth=2,
    label="Quiescence (Threaded)",
    color="red",
    linestyle=":",
)

ax4.set_xlabel("Depth")
ax4.set_ylabel("Nodes (log scale)")
ax4.set_title("All Node Types Comparison (Log Scale)")
ax4.legend()
ax4.grid(True, alpha=0.3)
ax4.set_xticks(range(1, max(non_threaded_data["depth"]) + 1))


# Format y-axis to show numbers in millions/thousands
def format_nodes(x, p):
    if x >= 1_000_000:
        return f"{x/1_000_000:.1f}M"
    elif x >= 1_000:
        return f"{x/1_000:.0f}K"
    else:
        return f"{x:.0f}"


# Apply formatting to linear scale plots
for ax in [ax1, ax2, ax3]:
    ax.yaxis.set_major_formatter(plt.FuncFormatter(format_nodes))

plt.tight_layout()
plt.show()

# Print summary
print("Engine Performance Analysis:")
print("=" * 60)
print(f"{'Depth':<6} {'Total':<12} {'Alpha-Beta':<12} {'Quiescence':<12} {'Type'}")
print("-" * 60)

for i in range(len(non_threaded_data["depth"])):
    depth = non_threaded_data["depth"][i]
    print(
        f"{depth:<6} {non_threaded_data['nodes'][i]:<12,} "
        f"{non_threaded_data['nodes_ab'][i]:<12,} "
        f"{non_threaded_data['nodes_qs'][i]:<12,} Non-threaded"
    )

    if i < len(threaded_data["depth"]):
        print(
            f"{depth:<6} {threaded_data['nodes'][i]:<12,} "
            f"{threaded_data['nodes_ab'][i]:<12,} "
            f"{threaded_data['nodes_qs'][i]:<12,} Threaded"
        )
    print()

print("\nQuiescence Search Percentage of Total Nodes:")
print("-" * 50)
print(f"{'Depth':<6} {'Non-threaded %':<15} {'Threaded %':<12}")
print("-" * 50)

for i in range(len(non_threaded_data["depth"])):
    depth = non_threaded_data["depth"][i]
    nt_qs_pct = (non_threaded_data["nodes_qs"][i] / non_threaded_data["nodes"][i]) * 100

    if i < len(threaded_data["depth"]):
        t_qs_pct = (threaded_data["nodes_qs"][i] / threaded_data["nodes"][i]) * 100
        print(f"{depth:<6} {nt_qs_pct:<15.1f} {t_qs_pct:<12.1f}")
    else:
        print(f"{depth:<6} {nt_qs_pct:<15.1f} {'N/A':<12}")
