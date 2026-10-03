/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Reading JSON text, refusing anything malformed and anything nested past a bound. */

use super::json::Json;
use super::json_string::string;

/** The deepest nesting read; past it a message is refused rather than read. */
const DEPTH: usize = 128;

/** `src` as one JSON value, or `None` if it is not one. */
pub(crate) fn parse(src: &str) -> Option<Json> {
    let mut r = Reader {
        b: src.as_bytes(),
        at: 0,
    };
    let v = r.value(0)?;
    r.ws();
    (r.at == r.b.len()).then_some(v)
}

pub(super) struct Reader<'a> {
    pub(super) b: &'a [u8],
    pub(super) at: usize,
}

impl Reader<'_> {
    pub(super) fn ws(&mut self) {
        while self.b.get(self.at).is_some_and(|c| b" \t\r\n".contains(c)) {
            self.at += 1;
        }
    }

    pub(super) fn eat(&mut self, c: u8) -> bool {
        self.ws();
        let here = self.b.get(self.at) == Some(&c);
        self.at += usize::from(here);
        here
    }

    fn word(&mut self, w: &str, v: Json) -> Option<Json> {
        let end = self.at.checked_add(w.len())?;
        (self.b.get(self.at..end)? == w.as_bytes()).then_some(())?;
        self.at = end;
        Some(v)
    }

    pub(super) fn value(&mut self, depth: usize) -> Option<Json> {
        self.ws();
        match *self.b.get(self.at)? {
            _ if depth > DEPTH => None,
            b'n' => self.word("null", Json::Null),
            b't' => self.word("true", Json::Bool(true)),
            b'f' => self.word("false", Json::Bool(false)),
            b'"' => string(self).map(Json::Str),
            b'[' => self
                .list(depth, b']')
                .map(|v| Json::Arr(v.into_iter().map(|(_, v)| v).collect())),
            b'{' => self.list(depth, b'}').map(Json::Obj),
            _ => self.number(),
        }
    }
}
