#!/usr/bin/env python3
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
    parts = []
    for key in FLAGS:
        parts.append(f"{ABBR[key]}{1 if values[key] else 0}")
    return "_".join(parts)


def build_and_copy(label: str) -> bool:
    print(f"Build {label}")
    res = subprocess.run(["cargo", "build", "--release"], text=True)
    if res.returncode != 0:
        print(f"Build FAILED for {label}")
        return False

    exe = TARGET_DIR / BINARY_NAME
    if os.name == "nt":
        exe = exe.with_suffix(".exe")

    if not exe.exists():
        print(f"Binary nicht gefunden: {exe} (check BINARY_NAME)")
        return False

    OUT_DIR.mkdir(parents=True, exist_ok=True)
    out = OUT_DIR / f"{BINARY_NAME}_{label}{exe.suffix}"
    shutil.copy2(exe, out)
    print(f" -> {out}")
    return True


def main():
    if not SETTINGS.exists():
        print(f"Datei nicht gefunden: {SETTINGS}")
        return

    original = SETTINGS.read_text(encoding="utf-8")

    try:
        # 1) Build: alle Features AN
        all_on = {flag: True for flag in FLAGS}
        write_settings(all_on)
        label_all_on = label_bits(all_on)  # z.B. qs1_tt1_mo1_ab1
        build_and_copy(label_all_on)

        # 2) Für jedes Feature: genau dieses AUS, alle anderen AN
        for off_flag in FLAGS:
            vals = {flag: True for flag in FLAGS}
            vals[off_flag] = False
            label = label_bits(vals)  # z.B. qs0_tt1_mo1_ab1
            print(f"Build mit {off_flag}=false -> {vals}")
            write_settings(vals)
            build_and_copy(label)

    finally:
        # Original wiederherstellen
        SETTINGS.write_text(original, encoding="utf-8")
        print("Original settings.rs wiederhergestellt. Fertig.")


if __name__ == "__main__":
    main()