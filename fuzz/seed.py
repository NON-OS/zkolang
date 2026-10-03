#!/usr/bin/env python3
# NONOS. AGPL-3.0-or-later.
"""Seed each fuzz target's corpus with the repository's own programs.

Each seed is a program's text after a zero byte, the mode in which a target reads
its bytes as UTF-8, so the fuzzer starts from programs that parse, check and build
and mutates them in place.
"""
import argparse
import hashlib
import pathlib

EDITION_2026 = ["nonos_zkolang_proofs/semantics", "nonos_zkolang_proofs/ui", "std"]
EDITION_2025 = ["examples", "circuits", "stdlib"]
TARGETS = {
    "front_end": EDITION_2026,
    "build_run": EDITION_2026,
    "fmt": EDITION_2026,
    "edition_2025": EDITION_2025,
}


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", default="..", help="the repository root")
    ap.add_argument("--out", default="corpus", help="where the corpora go")
    args = ap.parse_args()
    root = pathlib.Path(args.root)
    for target, dirs in TARGETS.items():
        out = pathlib.Path(args.out) / target
        out.mkdir(parents=True, exist_ok=True)
        count = 0
        for d in dirs:
            for f in sorted((root / d).rglob("*.zkl")):
                seed = b"\x00" + f.read_bytes()
                (out / hashlib.sha1(seed).hexdigest()).write_bytes(seed)
                count += 1
        print(f"{target}: {count} seeds")


if __name__ == "__main__":
    main()
