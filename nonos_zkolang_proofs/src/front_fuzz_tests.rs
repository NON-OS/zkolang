/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The edition 2026 front end is total. Random token soup, however malformed, lexes and
 * parses without a panic, the parse ends, and every diagnostic has a primary label inside
 * the text.
 */

use crate::front_check::{check, WORDS};

#[test]
fn random_token_soup_parses_without_panic() {
    let mut state: u64 = 0x2545_F491_4F6C_DD1D;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    for _ in 0..10_000 {
        let len = (next() % 120) as usize;
        let words: Vec<&str> = (0..len)
            .map(|_| WORDS[(next() % WORDS.len() as u64) as usize])
            .collect();
        check(&words.join(" "));
    }
}
