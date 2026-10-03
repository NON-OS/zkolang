/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The zKolang command-line tool. `run` compiles a program, proves it, and reports the
 * result; `check` compiles without proving; `build` emits a native backend; `key`
 * prints a circuit's registration commitment and verifier key. An include resolves from
 * the file that writes it, a `stdlib` folder above it, or the standard library built into
 * the binary, so a program runs the same from a shell or an editor task.
 */

use std::env;
use std::process::exit;

mod args;
mod cmd;
mod line;
mod load;
mod lsp;
mod out;
mod render_run;

const USAGE: &str = "\
zkolang <run|check|test|explain|doc|abi|fmt|build|key|fee|lsp> <file.zkl> [options]
  run   <file> [--input a,b] [--witness x,y]     compile, prove, report
  check <file>                                   compile only
  test  <file>                                   run each #[test] of an edition 2026 crate
  explain <code>                                 the long description of a diagnostic code
  doc   <file> | --std                           a crate's reference, from its doc comments
  abi   <file>                                   the layout of an edition 2026 program's inputs
  fmt   <file> [--check]                         lay out an edition 2026 file's lines
  --edition 2026 compiles the new language: run takes [--public a,b] [--secret x,y],
  one value per scalar of main's public and secret parameters, and hides the secrets
  build <file> [--target c|asm|python] [--out f] emit a native backend
  key   <file>                                   commitment and verifier key
  fee   <file> [--input a,b] [--witness x,y]     the pay-to-prove cost in NOX
  lsp                                            serve the Language Server Protocol on stdio";

fn main() {
    let args: Vec<String> = env::args().collect();
    let rest = args.get(2..).unwrap_or(&[]);
    let done = match args.get(1).map(String::as_str) {
        Some("run") => cmd::run(rest),
        Some("check") => cmd::check(rest),
        Some("test") => cmd::test(rest),
        Some("explain") => cmd::explain(rest),
        Some("doc") => cmd::doc(rest),
        Some("abi") => cmd::abi(rest),
        Some("fmt") => cmd::fmt(rest),
        Some("build") => cmd::build(rest),
        Some("key") => cmd::key(rest),
        Some("fee") => cmd::fee(rest),
        Some("lsp") => lsp::serve(),
        Some("version" | "--version" | "-V") => {
            println!("zkolang {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        _ => Err(String::from(USAGE)),
    };
    exit(done.map_or_else(|e| out::err(&e), |()| 0));
}
