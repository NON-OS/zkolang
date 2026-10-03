"""Write the display width tables of the diagnostic renderer from Unicode data.

The tables are generated from the Unicode Character Database that Python's unicodedata
module carries, and name its version. A zero-width mark is a character of General_Category
Mn or Me; a wide character has East_Asian_Width W or F. Unassigned code points between two
ranges of one class join them, which keeps the tables short and does not change any
assigned character.
"""
import argparse
import unicodedata

HEADER = """/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * {what}.
 *
 * Sorted inclusive ranges of code points, each a start and an end, generated from
 * Unicode {version} by scripts/gen_display_widths.py; do not edit by hand.
 */

/** {doc} */
pub(super) const {name}: &[u32] = &[
"""


def ranges(test):
    """Inclusive ranges of the code points `test` holds for, bridging unassigned ones."""
    out, start, last = [], None, None
    for u in range(0x110000):
        c = chr(u)
        if unicodedata.category(c) == "Cn":
            continue
        if test(u, c):
            if start is None:
                start = u
            last = u
        elif start is not None:
            out.append((start, last))
            start = None
    if start is not None:
        out.append((start, last))
    return out


def write(path, what, doc, name, table):
    values = [f"0x{v:04X}" for pair in table for v in pair]
    lines, line = [], "   "
    for v in values:
        if len(line) + len(v) + 2 > 100:
            lines.append(line)
            line = "   "
        line += f" {v},"
    lines.append(line)
    text = HEADER.format(what=what, doc=doc, name=name, version=unicodedata.unidata_version)
    with open(path, "w", encoding="utf-8") as f:
        f.write(text + "\n".join(lines) + "\n];\n")


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--dir", default="nonos_zkolang/src/compiler/diag", help="where the tables go")
    a = ap.parse_args()
    marks = ranges(lambda u, c: unicodedata.category(c) in ("Mn", "Me"))
    wide = ranges(lambda u, c: unicodedata.east_asian_width(c) in ("W", "F")
                  and unicodedata.category(c) not in ("Mn", "Me"))
    write(f"{a.dir}/width_marks.rs", "Marks drawn over the character before them",
          "General_Category Mn and Me.", "MARKS", marks)
    write(f"{a.dir}/width_wide.rs", "Characters a terminal gives two columns",
          "East_Asian_Width W and F.", "WIDE", wide)


main()
