#!/usr/bin/env python3
# NONOS. AGPL-3.0-or-later.
"""Check a change against the repository's rules: no file of code over the line limit
that a change adds or pushes over it, the added lines' rules (rules_lines.py), and
commit subjects of at most 72 characters with no em-dash or banned word. Reports what
the change between --base and --head introduces, nothing older."""
import argparse
import subprocess
import sys

from rules_lines import WORD, diff_problems

CODE = (".rs", ".py", ".zkl", ".lean")


def git(*args):
    return subprocess.run(["git", *args], capture_output=True, text=True, check=True).stdout


def length_problems(base, head, limit):
    out = []
    for row in git("diff", "--name-status", "--no-renames", base, head).splitlines():
        status, path = row.split("\t")[0], row.split("\t")[-1]
        if status.startswith("D") or not path.endswith(CODE):
            continue
        n = git("show", f"{head}:{path}").count("\n")
        if n <= limit:
            continue
        if status.startswith("A"):
            out.append(f"{path}: a new file of {n} lines, over {limit}")
        elif git("show", f"{base}:{path}").count("\n") <= limit:
            out.append(f"{path}: grew to {n} lines, over {limit}")
    return out


def commit_problems(base, head):
    out = []
    for c in git("rev-list", f"{base}..{head}").split():
        msg = git("log", "-1", "--format=%B", c)
        subject = msg.splitlines()[0] if msg else ""
        if len(subject) > 72:
            out.append(f"commit {c[:7]}: a subject of {len(subject)} characters, over 72")
        if "\u2014" in msg:
            out.append(f"commit {c[:7]}: an em-dash")
        if WORD.search(msg):
            out.append(f"commit {c[:7]}: the banned word `{WORD.search(msg).group(0)}`")
    return out


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--base", required=True, help="the commit the change starts from")
    ap.add_argument("--head", default="HEAD", help="the commit the change ends at")
    ap.add_argument("--limit", type=int, default=75, help="the most lines a file of code has")
    a = ap.parse_args()
    problems = length_problems(a.base, a.head, a.limit)
    problems += diff_problems(git("diff", "-U0", "--no-renames", a.base, a.head))
    problems += commit_problems(a.base, a.head)
    for p in problems:
        print(p)
    print(f"{len(problems)} new")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
