# NONOS. AGPL-3.0-or-later.
"""The rules a line added by a change must keep: comments in Rust are blocks, nothing
is silenced with an allow, a mod.rs holds declarations only, and no added text has an
em-dash or one of the banned words."""
import re

BANNED = [
    "robust", "leverage", "leverages", "leveraging", "seamless", "seamlessly", "delve",
    "utilize", "utilise", "streamline", "cutting-edge", "empower", "holistic", "pivotal",
    "paramount", "meticulous", "tapestry", "synergy", "game-changer", "elevate", "unlock",
    "crucial", "vital", "intricate", "landscape", "realm", "embark", "foster", "bolster",
    "showcase", "testament", "ever-evolving", "furthermore", "moreover", "harness",
    "navigate", "underscore", "comprehensive", "state-of-the-art", "groundbreaking",
]
WORD = re.compile(r"\b(" + "|".join(re.escape(w) for w in BANNED) + r")\b", re.I)
DECL = ("mod ", "pub mod ", "pub use ", "pub(crate) mod ", "pub(crate) use ",
        "pub(super) use ", "pub(super) mod ", "use ", "/*", "*", "#[cfg(")
WORDS_CHECKED = (".rs", ".md", ".zkl", ".lean", ".py", ".toml", ".yml")
SELF = ("scripts/rules_lines.py", ".github/workflows/hygiene.yml")


def line_problems(path, body, in_block):
    """What is wrong with `body`, an added line of `path`; `in_block` says it is inside
    a block comment."""
    out = []
    if path.endswith(".rs") and not in_block:
        code = re.sub(r'"(\\.|[^"\\])*"', '""', body)
        if re.search(r"(^|[^:\"'])//", code):
            out.append("a line comment; comments are /* */ blocks")
        if "#[allow(" in code:
            out.append("#[allow(...)]; fix the warning instead")
        t = body.strip()
        if path.endswith("mod.rs") and t and not t.startswith(DECL) \
                and not re.match(r"^[A-Za-z_0-9:{}, ]+[;,]?$", t) and t not in ("};", "}"):
            out.append("code in mod.rs, which holds declarations only")
    if "\u2014" in body:
        out.append("an em-dash")
    if path.endswith(WORDS_CHECKED) and path not in SELF:
        m = WORD.search(body)
        if m:
            out.append(f"the banned word `{m.group(0)}`")
    return out


def diff_problems(diff):
    """Every problem of the lines `diff`, a `git diff -U0`, adds."""
    out, path, line, block = [], None, 0, {}
    for l in diff.splitlines():
        if l.startswith("+++ "):
            path = l[6:] if l.startswith("+++ b/") else None
            continue
        m = re.match(r"^@@ -\d+(?:,\d+)? \+(\d+)", l)
        if m:
            line = int(m.group(1))
            continue
        if path is None or not l.startswith("+"):
            continue
        body = l[1:]
        opens = "/*" in body and "*/" not in body.split("/*", 1)[1]
        inside = block.get(path, False)
        out += [f"{path}:{line}: {p}" for p in line_problems(path, body, inside or opens)]
        if opens:
            block[path] = True
        elif inside and "*/" in body:
            block[path] = False
        line += 1
    return out
