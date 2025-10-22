#!/usr/bin/env python3
import itertools
import os
import shutil
import subprocess
from pathlib import Path

# Pfade anpassen
SETTINGS = Path("../src/settings.rs")  # Datei mit deinen Konstanten
BINARY_NAME = "thunfisch"  # [[bin]] name aus Cargo.toml
TARGET_DIR = Path("../target/release")
OUT_DIR = Path("./bin")

TEMPLATE = """\
pub mod settings {{
    pub const QUIESCENCE_SEARCH: bool = {QUIESCENCE_SEARCH};
    pub const TRANSPOSITION_TABLE: bool = {TRANSPOSITION_TABLE};
    pub const MOVE_ORDERING: bool = {MOVE_ORDERING};
    pub const ALPHA_BETA: bool = {ALPHA_BETA};
}}
"""

FLAGS = ["QUIESCENCE_SEARCH", "TRANSPOSITION_TABLE", "MOVE_ORDERING", "ALPHA_BETA"]

ABBR = {
    "QUIESCENCE_SEARCH": "qs",
    "TRANSPOSITION_TABLE": "tt",
    "MOVE_ORDERING": "mo",
    "ALPHA_BETA": "ab",
}


def write_settings(values: dict[str, bool]):
    text = TEMPLATE.format(
        QUIESCENCE_SEARCH=str(values["QUIESCENCE_SEARCH"]).lower(),
        TRANSPOSITION_TABLE=str(values["TRANSPOSITION_TABLE"]).lower(),
        MOVE_ORDERING=str(values["MOVE_ORDERING"]).lower(),
        ALPHA_BETA=str(values["ALPHA_BETA"]).lower(),
    )
    SETTINGS.write_text(text, encoding="utf-8")


def label_bits(values: dict[str, bool]) -> str:
    # qs0_tt1_mo0_ab1 in FLAGS-Reihenfolge
    parts = []
    for key in FLAGS:
        parts.append(f"{ABBR[key]}{1 if values[key] else 0}")
    return "_".join(parts)


def main():
    if not SETTINGS.exists():
        print(f"Datei nicht gefunden: {SETTINGS}")
        return

    original = SETTINGS.read_text(encoding="utf-8")
    OUT_DIR.mkdir(parents=True, exist_ok=True)

    combos = list(itertools.product([False, True], repeat=len(FLAGS)))

    for bits in combos:
        vals = dict(zip(FLAGS, bits))
        label = label_bits(vals)
        print(f"Build {label} -> {vals}")

        try:
            write_settings(vals)
            res = subprocess.run(["cargo", "build", "--release"], text=True)
            if res.returncode != 0:
                print(f"Build FAILED for {label}")
                continue

            exe = TARGET_DIR / BINARY_NAME
            if os.name == "nt":
                exe = exe.with_suffix(".exe")

            if not exe.exists():
                print(f"Binary nicht gefunden: {exe} (check BINARY_NAME)")
                continue

            out = OUT_DIR / f"{BINARY_NAME}_{label}{exe.suffix}"
            shutil.copy2(exe, out)
            print(f" -> {out}")

        finally:
            pass

    # Original wiederherstellen
    SETTINGS.write_text(original, encoding="utf-8")
    print("Fertig.")


if __name__ == "__main__":
    main()
