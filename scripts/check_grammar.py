#!/usr/bin/env python3
# NONOS. AGPL-3.0-or-later.
"""Check the tree-sitter grammar against the repository's own programs: it generates,
its corpus tests pass, its highlight queries compile, and every .zkl file that marks
no lexical or syntax error (a `/*~ E00..` or `/*~ E01..` annotation) and every
Rust-fenced program of README.md parses with no error node. Takes the tree-sitter
command to run."""
import argparse
import pathlib
import re
import subprocess
import sys
import tempfile

ROOT = pathlib.Path(__file__).resolve().parent.parent
GRAMMAR = ROOT / "tree-sitter-zkolang"
SKIP = ("target", "nonos-data", "node_modules")
MARKED = re.compile(r"/\*~ E0[01]")


def programs():
    for p in sorted(ROOT.rglob("*.zkl")):
        rel = p.relative_to(ROOT)
        if rel.parts[0] in SKIP or MARKED.search(p.read_text(encoding="utf-8")):
            continue
        yield p


def readme_programs(out):
    text = (ROOT / "README.md").read_text(encoding="utf-8")
    for i, body in enumerate(re.findall(r"```rust\n(.*?)```", text, re.S)):
        path = out / f"readme_{i:02}.zkl"
        path.write_text(body, encoding="utf-8")
        yield path


def run(ts, *args):
    r = subprocess.run([ts, *args], cwd=GRAMMAR, capture_output=True, text=True)
    return r.returncode, r.stdout + r.stderr


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("tree_sitter", help="the tree-sitter command")
    ts = str(pathlib.Path(ap.parse_args().tree_sitter).resolve())
    problems = []
    for step in (["generate"], ["test"]):
        code, out = run(ts, *step)
        if code != 0:
            problems.append(f"tree-sitter {step[0]} failed:\n{out}")
    if problems:
        print("\n".join(problems))
        return 1
    with tempfile.TemporaryDirectory() as tmp:
        files = [str(p) for p in programs()]
        files += [str(p) for p in readme_programs(pathlib.Path(tmp))]
        code, out = run(ts, "parse", "--quiet", "--stat", *files)
        failed = [line for line in out.splitlines() if "(ERROR" in line or "(MISSING" in line]
        if code != 0 or failed:
            problems.append("programs that do not parse:\n" + "\n".join(failed or [out]))
        code, out = run(ts, "query", "queries/highlights.scm", files[0])
        if code != 0:
            problems.append(f"the highlight queries do not compile:\n{out}")
    print("\n".join(problems) or f"{len(files)} programs parse, the corpus and queries hold")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
