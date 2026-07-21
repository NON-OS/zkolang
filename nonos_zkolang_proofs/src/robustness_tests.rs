/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Robustness fuzzing of the front end. The compiler is a boundary untrusted text crosses,
//! so it must answer every input with an Ok or an Err and never a panic. Random token soup,
//! character-boundary truncations of real programs, and multi-byte UTF-8 aimed at the
//! diagnostic renderer are thrown at compilation and error formatting. The multi-byte cases
//! matter because an error carries a byte offset and rendering slices the source at it, so a
//! slice that did not land on a character boundary would panic. A single panic fails the
//! suite and prints the input that caused it.

use std::panic::{catch_unwind, AssertUnwindSafe};

use nonos_zkolang::{compile_source, compile_source_unoptimized, expand_includes, render_error};

// A deterministic generator, so a failure reproduces from the seed.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0 >> 33
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

// Every token the grammar knows, mixed with whitespace, stray punctuation, oversized
// numbers, and multi-byte UTF-8, so a generated string can land as a valid fragment, a
// truncated one, or noise the lexer must reject without falling over.
const VOCAB: &[&str] = &[
    "input",
    "secret",
    "output",
    "let",
    "const",
    "fn",
    "for",
    "in",
    "include",
    "witness",
    "public",
    "reveal",
    "prove",
    "assert",
    "inv",
    "sel",
    "if",
    "else",
    "match",
    "a",
    "b",
    "c",
    "x",
    "v0",
    "acc",
    "ø",
    "Ø",
    "π",
    "名",
    "🔒",
    "0",
    "1",
    "9",
    "255",
    "18446744073709551615",
    "99999999999999999999999",
    "+",
    "-",
    "*",
    "/",
    "=",
    "==",
    "!=",
    "<",
    "<=",
    ">",
    ">=",
    "!",
    "&&",
    "||",
    "(",
    ")",
    "[",
    "]",
    "{",
    "}",
    ",",
    ";",
    "..",
    "=>",
    "\"",
    "\n",
    " ",
    "\t",
    "// comment\n",
    "\"str.zkl\"",
];

fn gen_soup(rng: &mut Rng) -> String {
    let n = rng.below(40) + 1;
    let mut s = String::new();
    for _ in 0..n {
        s.push_str(VOCAB[rng.below(VOCAB.len() as u64) as usize]);
        if rng.below(3) == 0 {
            s.push(' ');
        }
    }
    s
}

// Compile both ways, render any error, and run the include text step with a resolver that
// finds nothing. The only property under test is that none of this panics; the results
// themselves are unconstrained on garbage.
fn exercise(src: &str) {
    if let Err(e) = compile_source(src) {
        let _ = render_error(src, &e);
    }
    let _ = compile_source_unoptimized(src);
    let mut none = |_: &str| -> Option<String> { None };
    let _ = expand_includes(src, &mut none);
}

fn no_panic(src: &str, tag: &str) {
    let r = catch_unwind(AssertUnwindSafe(|| exercise(src)));
    assert!(r.is_ok(), "front end panicked on {tag}:\n{src:?}");
}

#[test]
fn the_front_end_never_panics_on_random_soup() {
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let mut rng = Rng(0xF07C0DE);
    for _ in 0..20000 {
        let src = gen_soup(&mut rng);
        no_panic(&src, "random soup");
    }
    std::panic::set_hook(prev);
}

#[test]
fn the_front_end_never_panics_on_truncation() {
    // Every character-boundary prefix of a real program, so the parser meets end of input at
    // every point a statement or an expression can be left half written.
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let programs = [
        "input x;\nlet y = x * x * x;\noutput y;\n",
        "const W = [3, 1, 4];\ninput x;\nfor i in 0 .. 3 { let x = x + W[i]; }\noutput x;\n",
        "public op;\npublic a;\npublic b;\nreveal match op { 0 => a + b, _ => a * b };\n",
        "witness k;\ninclude \"bits.zkl\";\nprove (k <= 255) - 1;\n",
        "fn sq(x) = x * x;\ninput a;\noutput sq(a) + sel(a, 1, 0);\n",
    ];
    for p in programs {
        for (i, _) in p.char_indices().chain(core::iter::once((p.len(), ' '))) {
            no_panic(&p[..i], "truncation");
        }
    }
    std::panic::set_hook(prev);
}

#[test]
fn diagnostic_rendering_never_panics_on_multibyte() {
    // Errors carry byte offsets and rendering slices the source at them. A program peppered
    // with multi-byte identifiers, comments, and strings forces those slices onto and across
    // character boundaries, where a naive byte slice would panic.
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let mut rng = Rng(0x0FF1CE);
    let noisy = [
        "øøø",
        "// π\n",
        "\"Ø\"",
        "名前",
        "🔒🔒",
        "let ø = ",
        "input Ø;",
        "ø + ",
        "\nØ\n",
    ];
    for _ in 0..5000 {
        let mut s = String::new();
        let n = rng.below(12) + 1;
        for _ in 0..n {
            s.push_str(noisy[rng.below(noisy.len() as u64) as usize]);
            s.push_str(VOCAB[rng.below(VOCAB.len() as u64) as usize]);
        }
        no_panic(&s, "multibyte diagnostic");
    }
    std::panic::set_hook(prev);
}
