/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Reading a JSON string: its escapes, and `\u` escapes in surrogate pairs. */

use super::json_read::Reader;

/** The string whose opening quote is current, or `None` if it is malformed. */
pub(super) fn string(r: &mut Reader) -> Option<String> {
    r.at += 1;
    let mut out = String::new();
    loop {
        let start = r.at;
        while r
            .b
            .get(r.at)
            .is_some_and(|c| *c != b'"' && *c != b'\\' && *c >= 0x20)
        {
            r.at += 1;
        }
        out.push_str(std::str::from_utf8(r.b.get(start..r.at)?).ok()?);
        let c = *r.b.get(r.at)?;
        r.at += 1;
        match c {
            b'"' => return Some(out),
            b'\\' => out.push(escape(r)?),
            _ => return None,
        }
    }
}

fn escape(r: &mut Reader) -> Option<char> {
    let c = *r.b.get(r.at)?;
    r.at += 1;
    Some(match c {
        b'"' => '"',
        b'\\' => '\\',
        b'/' => '/',
        b'b' => '\u{8}',
        b'f' => '\u{c}',
        b'n' => '\n',
        b'r' => '\r',
        b't' => '\t',
        b'u' => return unit(r),
        _ => return None,
    })
}

/** The character a `\u` escape names, reading its low half when it is a high surrogate. */
fn unit(r: &mut Reader) -> Option<char> {
    let hi = hex4(r)?;
    if !(0xD800..0xDC00).contains(&hi) {
        return char::from_u32(hi);
    }
    (r.b.get(r.at..r.at + 2)? == b"\\u").then_some(())?;
    r.at += 2;
    let lo = hex4(r)?;
    (0xDC00..0xE000).contains(&lo).then_some(())?;
    char::from_u32(0x10000 + ((hi - 0xD800) << 10) + (lo - 0xDC00))
}

fn hex4(r: &mut Reader) -> Option<u32> {
    let s = std::str::from_utf8(r.b.get(r.at..r.at.checked_add(4)?)?).ok()?;
    let v = u32::from_str_radix(s, 16).ok()?;
    r.at += 4;
    Some(v)
}
