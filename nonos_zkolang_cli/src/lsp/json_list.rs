/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Reading the JSON values that are more than a word: arrays, objects and numbers. */

use super::json::Json;
use super::json_read::Reader;
use super::json_string::string;

impl Reader<'_> {
    /** The members of an array (keys empty) or an object, the opener current. */
    pub(super) fn list(&mut self, depth: usize, close: u8) -> Option<Vec<(String, Json)>> {
        self.at += 1;
        let mut out = Vec::new();
        if self.eat(close) {
            return Some(out);
        }
        loop {
            let key = if close == b'}' {
                self.key()?
            } else {
                String::new()
            };
            out.push((key, self.value(depth + 1)?));
            if self.eat(close) {
                return Some(out);
            }
            self.eat(b',').then_some(())?;
        }
    }

    fn key(&mut self) -> Option<String> {
        self.ws();
        (self.b.get(self.at) == Some(&b'"')).then_some(())?;
        let k = string(self)?;
        self.eat(b':').then_some(k)
    }

    pub(super) fn number(&mut self) -> Option<Json> {
        let start = self.at;
        while self
            .b
            .get(self.at)
            .is_some_and(|c| c.is_ascii_digit() || b"+-.eE".contains(c))
        {
            self.at += 1;
        }
        let s = std::str::from_utf8(self.b.get(start..self.at)?).ok()?;
        s.parse::<f64>().ok().filter(|n| n.is_finite())?;
        Some(Json::Num(s.to_string()))
    }
}
