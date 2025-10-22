#!/usr/bin/env python3
"""
Script to plot chess engine performance data from tmp.md
Compares node counts at different depths for threading vs no threading scenarios.
"""

import matplotlib.pyplot as plt
import numpy as np


def parse_number(num_str):
    """Parse number string, handling spaces as thousands separators"""
    return int(num_str.replace(" ", "").replace(",", ""))


# Data from tmp.md
# First table (with threading)
threading_data = {
    "depth": [1, 2, 3, 4, 5, 6, 7],
    "nodes": [20, 428, 3028, 59109, 376210, 10563766, 50415959],
}

# Second table (no threading - Alpha Beta Pruning at root level)
no_threading_data = {
    "depth": [1, 2, 3, 4, 5, 6, 7],
    "nodes": [21, 146, 3028, 21993, 871367, 1877450, 21852777],
}

# Create the plot
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(15, 6))

# Plot 1: Linear scale comparison
ax1.plot(
    threading_data["depth"],
    threading_data["nodes"],
    marker="o",
    linewidth=2,
    label="With Threading",
    color="blue",
)
ax1.plot(
    no_threading_data["depth"],
    no_threading_data["nodes"],
    marker="s",
    linewidth=2,
    label="No Threading (Alpha Beta)",
    color="red",
)

ax1.set_xlabel("Depth")
ax1.set_ylabel("Nodes Visited")
ax1.set_title("Chess Engine Performance: Threading vs No Threading")
ax1.legend()
ax1.grid(True, alpha=0.3)
ax1.set_xticks(range(1, 8))


# Format y-axis to show numbers in millions/thousands
def format_nodes(x, p):
    if x >= 1_000_000:
        return f"{x/1_000_000:.1f}M"
    elif x >= 1_000:
        return f"{x/1_000:.0f}K"
    else:
        return f"{x:.0f}"


ax1.yaxis.set_major_formatter(plt.FuncFormatter(format_nodes))

# Plot 2: Logarithmic scale for better visualization
ax2.semilogy(
    threading_data["depth"],
    threading_data["nodes"],
    marker="o",
    linewidth=2,
    label="With Threading",
    color="blue",
)
ax2.semilogy(
    no_threading_data["depth"],
    no_threading_data["nodes"],
    marker="s",
    linewidth=2,
    label="No Threading (Alpha Beta)",
    color="red",
)

ax2.set_xlabel("Depth")
ax2.set_ylabel("Nodes Visited (log scale)")
ax2.set_title("Chess Engine Performance: Threading vs No Threading (Log Scale)")
ax2.legend()
ax2.grid(True, alpha=0.3)
ax2.set_xticks(range(1, 8))

plt.tight_layout()
plt.show()

# Print summary statistics
print("Performance Analysis Summary:")
print("=" * 50)
print(f"{'Depth':<6} {'Threading':<12} {'No Threading':<12} {'Ratio':<8}")
print("-" * 50)

for i in range(len(threading_data["depth"])):
    depth = threading_data["depth"][i]
    thread_nodes = threading_data["nodes"][i]
    no_thread_nodes = no_threading_data["nodes"][i]
    ratio = thread_nodes / no_thread_nodes if no_thread_nodes > 0 else 0

    print(f"{depth:<6} {thread_nodes:<12,} {no_thread_nodes:<12,} {ratio:<8.2f}")

print("\nKey Observations:")
print("- Alpha Beta Pruning significantly reduces node count at higher depths")
print(
    "- Threading appears to explore more nodes, possibly due to parallel search overhead"
)
print("- The efficiency of Alpha Beta Pruning becomes more pronounced at depths 4+")

# Create a third plot showing the ratio
fig2, ax3 = plt.subplots(figsize=(10, 6))

ratios = [
    threading_data["nodes"][i] / no_threading_data["nodes"][i]
    for i in range(len(threading_data["depth"]))
]

bars = ax3.bar(threading_data["depth"], ratios, alpha=0.7, color="green")
ax3.set_xlabel("Depth")
ax3.set_ylabel("Ratio (Threading / No Threading)")
ax3.set_title("Node Count Ratio: Threading vs No Threading")
ax3.grid(True, alpha=0.3, axis="y")
ax3.set_xticks(range(1, 8))

# Add value labels on bars
for bar, ratio in zip(bars, ratios):
    height = bar.get_height()
    ax3.text(
        bar.get_x() + bar.get_width() / 2.0,
        height + 0.05,
        f"{ratio:.2f}",
        ha="center",
        va="bottom",
    )

# Add horizontal line at y=1 for reference
ax3.axhline(y=1, color="black", linestyle="--", alpha=0.5, label="Equal performance")
ax3.legend()

plt.tight_layout()
# plt.show()
