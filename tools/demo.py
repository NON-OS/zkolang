#!/usr/bin/env python3
"""Drive the shield circuit for a screen recording.

Four acts: the machine-checked proofs, what the circuit refuses, the verifier
key, and the shape of a transfer. Every line is output from the tool that
produced it, not a reconstruction.

Build once so the recording holds no compile:

    cargo build --release -p stark_proofs --tests
    (cd lean && lake build)
    /usr/bin/python3 tools/demo.py
"""

import argparse
import os
import re
import subprocess
import sys
import threading
import time

C = "\033[38;2;102;255;255m"      # accent
I = "\033[38;2;232;241;242m"      # ink
G = "\033[38;2;147;167;170m"      # grey
D = "\033[38;2;78;97;100m"        # dim
R = "\033[38;2;255;107;91m"       # refusal
O = "\033[0m"
B = "\033[1m"

RESULT = re.compile(r"test result: (\w+)\. (\d+) passed; (\d+) failed")
AXIOM = re.compile(r"'([\w.]+)' (does not depend on any axioms|depends on axioms: \[(.*)\])")
SHAPE = re.compile(r"DEPLOYED trace_width=(\d+) degree=(\d+) log_trace_len=(\d+) t=(\d+) "
                   r"n_periodic=(\d+) eval_domain=(\d+)")

GATES = [
    ("a conserving transfer, owned and in the pool", "shield::test::conserves", True),
    ("the same note retired under a second position", "shield::test::double_spend", False),
    ("value moved from one asset into another", "shield::test::cross_asset", False),
    ("a note that was never deposited", "shield::test::foreign_pool", False),
    ("a key that did not open the note", "shield::test::not_owner", False),
    ("a path that walks away from the root", "shield::test::membership_scope", False),
]

THEOREMS = [
    ("Zkolang.Opening.injected_of_nodeHalf", "a state whose node half is the bound leaf is an injection of it"),
    ("Zkolang.Opening.nodeHalf_inject", "and the pin reads back what was injected"),
    ("Zkolang.Wiring.disjoint_class_preserves", "a disjoint class leaves an earlier one as it was"),
]


def slow(text, pace):
    if pace <= 0:
        sys.stdout.write(text)
    else:
        for ch in text:
            sys.stdout.write(ch)
            sys.stdout.flush()
            time.sleep(pace)
    sys.stdout.flush()


def act(n, title, pace):
    rule = "─" * (58 - len(title))
    slow(f"\n  {D}{n}{O}  {C}{B}{title}{O} {D}{rule}{O}\n\n", min(pace, 0.004))


def sh(cmd, env=None, cwd=None):
    # Both streams: the probes report on stderr, the harness on stdout.
    e = dict(os.environ, **(env or {}))
    r = subprocess.run(cmd, capture_output=True, text=True, env=e, cwd=cwd)
    return r.stdout + r.stderr


def cargo(f, extra=None):
    return sh(["cargo", "test", "--release", "-p", "stark_proofs", f] + (extra or []))


def spinner(label):
    """A clock on one line, so a wait reads as work rather than a hang. Carriage
    returns need a terminal, so a piped run gets nothing."""
    done = threading.Event()
    if not sys.stdout.isatty():
        return done, None
    t0 = time.monotonic()

    def tick():
        while not done.wait(0.1):
            sys.stdout.write(f"\r    {D}{label} {time.monotonic() - t0:4.1f}s{O}")
            sys.stdout.flush()
    th = threading.Thread(target=tick, daemon=True)
    th.start()
    return done, th


def stop(done, th):
    done.set()
    if th is None:
        return
    th.join()
    sys.stdout.write("\r\033[2K")


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--pace", type=float, default=0.010, help="seconds per character, 0 for instant")
    ap.add_argument("--hold", type=float, default=0.55, help="seconds to hold each result")
    ap.add_argument("--raw", action="store_true", help="no screen clear, no final hold")
    a = ap.parse_args()

    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    elan = {"PATH": os.path.expanduser("~/.elan/bin") + ":" + os.environ.get("PATH", "")}

    if not a.raw:
        sys.stdout.write("\033[2J\033[3J\033[H\033]0;NOX SHIELD\007")
        sys.stdout.flush()

    print(f"\n  {C}{B}NØNOS{O}  {D}NOX SHIELD{O}    {G}transparent post-quantum STARK privacy{O}")
    print(f"  {G}A note exists in the pool, is spent by the key that owns it, is retired")
    print(f"  exactly once, and value is conserved. Without revealing which note,")
    print(f"  whose, or how much.{O}")

    ok = True

    # ---- I ----
    act("I", "MACHINE-CHECKED PROOFS", a.pace)
    slow(f"  {D}${O} {I}lake env lean axioms.lean{O}\n", a.pace)
    src = "import Zkolang\n" + "".join(f"#print axioms {n}\n" for n, _ in THEOREMS)
    path = os.path.join("/tmp", "nox_axioms.lean")
    with open(path, "w") as fh:
        fh.write(src)
    done, th = spinner("checking")
    out = sh(["lake", "env", "lean", path], env=elan, cwd=os.path.join(root, "lean"))
    stop(done, th)
    seen = {m.group(1): (m.group(2), m.group(3)) for m in AXIOM.finditer(out)}
    for name, gloss in THEOREMS:
        short = name.rsplit(".", 1)[-1]
        kind, deps = seen.get(name, ("", None))
        if not kind:
            ok = False
            plain, colour = "MISSING", R
        elif deps is None:
            plain, colour = "no axioms", C
        else:
            plain, colour = deps, C
        # Pad the visible text, then colour it: padding a string with escapes in
        # it counts the escapes.
        print(f"    {colour}{plain}{O}{' ' * (13 - len(plain))}{I}{short}{O}")
        print(f"    {' ' * 13}{D}{gloss}{O}\n")
        time.sleep(a.hold * 0.5)

    # ---- II ----
    act("II", "WHAT THE CIRCUIT REFUSES", a.pace)
    for label, f, accepts in GATES:
        verb, colour = ("accepts", C) if accepts else ("refuses", R)
        slow(f"  {D}${O} {I}cargo test {f}{O}\n", a.pace)
        m = RESULT.search(cargo(f))
        good = bool(m) and m.group(1) == "ok"
        ok &= good
        word = verb if good else "UNEXPECTED"
        print(f"    {colour if good else R}{word}{O}{' ' * (13 - len(word))}{G}{label}{O}\n")
        time.sleep(a.hold)

    # ---- III ----
    act("III", "ONE VERIFIER KEY", a.pace)
    slow(f"  {D}${O} {I}cargo test two_transfers_share_one_verifier_key{O}\n", a.pace)
    out = cargo("two_transfers_share_one_verifier_key", ["--", "--nocapture"])
    rows = [l.strip() for l in out.splitlines() if l.strip().startswith("transfer ")]
    for r in rows:
        print(f"    {I}{r}{O}")
    m = RESULT.search(out)
    good = bool(m) and m.group(1) == "ok"
    ok &= good
    print(f"\n    {C if good else R}{'identical' if good else 'DIVERGED'}{O}   "
          f"{G}one deployed key checks every transfer{O}\n")
    time.sleep(a.hold)

    # ---- IV ----
    act("IV", "A TRANSFER, AT THE DEPLOYED TREE DEPTH", a.pace)
    slow(f"  {D}${O} {I}cargo test probe_transfer_dims{O}\n", a.pace)
    m = SHAPE.search(cargo("probe_transfer_dims", ["--", "--ignored", "--nocapture"]))
    if m:
        w, deg, lt, t, np_, n = m.groups()
        for k, v in (("trace columns", w), ("constraint degree", deg),
                     ("periodic columns", np_), ("trace length", f"2^{lt}  ({t})"),
                     ("evaluation domain", f"{int(n):,}")):
            print(f"    {G}{k:<20}{O}{I}{v}{O}")
    else:
        ok = False
        print(f"    {R}UNEXPECTED{O}  {G}the probe did not report{O}")

    print(f"\n  {C if ok else R}{'every gate as specified' if ok else 'check the tree'}{O}")
    print(f"  {D}Ethereum L1. No rollup, no trusted setup.{O}\n")

    if not a.raw:
        try:
            input()
        except (EOFError, KeyboardInterrupt):
            pass
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
