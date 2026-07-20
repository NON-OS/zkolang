/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The zKolang command-line tool. `run` compiles a program, proves it, and reports the
//! result; `check` compiles without proving; `build` emits a native backend; `key`
//! prints a circuit's registration commitment and verifier key. Includes are resolved
//! from the program's directory and any `stdlib` folder above it, so a program runs the
//! same from a shell or an editor task.

use std::path::{Path, PathBuf};
use std::process::exit;
use std::{env, fs};

use nonos_zkolang::{
    commit, compile_source, expand_includes, prove_source_with_witness, render_error, to_asm, to_c,
    to_python, verifier_key, REGISTRATION_RATE,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let rest = if args.len() > 2 { &args[2..] } else { &[] };
    let code = match args.get(1).map(String::as_str) {
        Some("run") => cmd_run(rest),
        Some("check") => cmd_check(rest),
        Some("build") => cmd_build(rest),
        Some("key") => cmd_key(rest),
        Some("version" | "--version" | "-V") => {
            println!("zkolang {}", env!("CARGO_PKG_VERSION"));
            0
        }
        _ => {
            eprintln!("zkolang <run|check|build|key> <file.zkl> [options]");
            eprintln!("  run   <file> [--input a,b] [--witness x,y]   compile, prove, report");
            eprintln!("  check <file>                                 compile only");
            eprintln!("  build <file> --target c|asm|python [--out f] emit a native backend");
            eprintln!("  key   <file>                                 commitment and verifier key");
            1
        }
    };
    exit(code);
}

fn flag<'a>(a: &'a [String], name: &str) -> Option<&'a str> {
    a.iter()
        .position(|x| x == name)
        .and_then(|i| a.get(i + 1))
        .map(String::as_str)
}

fn file_arg(a: &[String]) -> Option<&str> {
    a.iter().find(|x| !x.starts_with('-')).map(String::as_str)
}

fn nums(s: Option<&str>) -> Vec<u64> {
    s.map(|v| v.split(',').filter_map(|t| t.trim().parse().ok()).collect())
        .unwrap_or_default()
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

// Read a program and expand its includes, resolving each from the file's directory or a
// `stdlib` folder in any ancestor.
fn load(file: &str) -> Result<String, String> {
    let path = Path::new(file);
    let src = fs::read_to_string(path).map_err(|e| format!("read {file}: {e}"))?;
    let dir = path.parent().map(PathBuf::from).unwrap_or_default();
    let mut resolve = |name: &str| resolve_include(&dir, name);
    expand_includes(&src, &mut resolve).map_err(|e| format!("include error: {e:?}"))
}

fn resolve_include(dir: &Path, name: &str) -> Option<String> {
    let mut d = dir.to_path_buf();
    loop {
        for cand in [d.join(name), d.join("stdlib").join(name)] {
            if let Ok(s) = fs::read_to_string(&cand) {
                return Some(s);
            }
        }
        if !d.pop() {
            return None;
        }
    }
}

fn err(msg: &str) -> i32 {
    eprintln!("{msg}");
    1
}

fn cmd_run(a: &[String]) -> i32 {
    let Some(file) = file_arg(a) else {
        return err("usage: zkolang run <file> [--input a,b] [--witness x,y]");
    };
    let src = match load(file) {
        Ok(s) => s,
        Err(e) => return err(&e),
    };
    let inputs = nums(flag(a, "--input"));
    let witness = nums(flag(a, "--witness"));
    match prove_source_with_witness(&src, &inputs, &witness) {
        Ok(r) if r.verified => {
            println!("verified");
            println!("outputs {:?}", r.outputs);
            println!("steps {}  trace 2^{}", r.steps, r.log_trace_len);
            0
        }
        Ok(_) => err("proof did not verify"),
        Err(e) => err(&format!("run error: {e:?}")),
    }
}

fn cmd_check(a: &[String]) -> i32 {
    let Some(file) = file_arg(a) else {
        return err("usage: zkolang check <file>");
    };
    let src = match load(file) {
        Ok(s) => s,
        Err(e) => return err(&e),
    };
    match compile_source(&src) {
        Ok(ops) => {
            println!("ok  {} instructions", ops.len());
            0
        }
        Err(e) => err(&render_error(&src, &e)),
    }
}

fn cmd_build(a: &[String]) -> i32 {
    let Some(file) = file_arg(a) else {
        return err("usage: zkolang build <file> --target c|asm|python [--out f]");
    };
    let src = match load(file) {
        Ok(s) => s,
        Err(e) => return err(&e),
    };
    let program = match compile_source(&src) {
        Ok(p) => p,
        Err(e) => return err(&render_error(&src, &e)),
    };
    let out = match flag(a, "--target").unwrap_or("c") {
        "c" => to_c(&program),
        "asm" => to_asm(&program),
        "python" => to_python(&program),
        t => return err(&format!("unknown target {t}, expected c, asm, or python")),
    };
    match flag(a, "--out") {
        Some(path) => match fs::write(path, out) {
            Ok(()) => {
                println!("wrote {path}");
                0
            }
            Err(e) => err(&format!("write {path}: {e}")),
        },
        None => {
            print!("{out}");
            0
        }
    }
}

fn cmd_key(a: &[String]) -> i32 {
    let Some(file) = file_arg(a) else {
        return err("usage: zkolang key <file>");
    };
    let src = match load(file) {
        Ok(s) => s,
        Err(e) => return err(&e),
    };
    let program = match compile_source(&src) {
        Ok(p) => p,
        Err(e) => return err(&render_error(&src, &e)),
    };
    match verifier_key(&program, REGISTRATION_RATE) {
        Ok(vk) => {
            println!("commit {}", hex(&commit(&program)));
            println!("vk     {}", hex(&vk));
            0
        }
        Err(e) => err(&format!("key error: {e:?}")),
    }
}
