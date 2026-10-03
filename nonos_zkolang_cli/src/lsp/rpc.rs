/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The base protocol: messages framed by a `Content-Length` header, both ways. */

use std::io::{self, BufRead, Write};

use super::json::Json;
use super::json_read::parse;
use super::json_write::write;

/** The largest message read; a longer one ends the session rather than being buffered. */
const LIMIT: usize = 64 << 20;

/**
 * The next message on `input`, its body parsed; `Null` for a body that is not JSON, and
 * `None` at the end of the input.
 */
pub(crate) fn read(input: &mut impl BufRead) -> io::Result<Option<Json>> {
    let mut len = None;
    loop {
        let mut line = String::new();
        if input.read_line(&mut line)? == 0 {
            return Ok(None);
        }
        let line = line.trim_end();
        if line.is_empty() {
            break;
        }
        if let Some(v) = line.strip_prefix("Content-Length:") {
            len = v.trim().parse::<usize>().ok();
        }
    }
    let len = len.filter(|n| *n <= LIMIT).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "a message with no usable Content-Length",
        )
    })?;
    let mut body = vec![0; len];
    input.read_exact(&mut body)?;
    Ok(Some(
        std::str::from_utf8(&body)
            .ok()
            .and_then(parse)
            .unwrap_or(Json::Null),
    ))
}

/** Send `v` on `out` as one message. */
pub(crate) fn send(out: &mut impl Write, v: &Json) -> io::Result<()> {
    let mut body = String::new();
    write(v, &mut body);
    write!(out, "Content-Length: {}\r\n\r\n{body}", body.len())?;
    out.flush()
}
