/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The JSON reader and writer: escapes, surrogate pairs, the nesting bound, malformed text. */

use super::json::Json;
use super::json_read::parse;
use super::json_write::write;

fn round(src: &str) -> String {
    let mut out = String::new();
    write(&parse(src).expect("parses"), &mut out);
    out
}

#[test]
fn values_read_and_write_back_the_same() {
    let src = r#"{"a":[1,-2.5e3,true,false,null],"b":{"c":"d"},"e":""}"#;
    assert_eq!(round(src), src);
    assert_eq!(round(" [ 1 , 2 ] "), "[1,2]");
}

#[test]
fn escapes_and_surrogate_pairs_decode() {
    let v = parse(r#""a\"b\\c\/d\n\t\u00e9\ud83d\ude00""#).expect("parses");
    assert_eq!(v, Json::Str(String::from("a\"b\\c/d\n\té\u{1F600}")));
    assert_eq!(round(r#""\u0001""#), r#""\u0001""#);
}

#[test]
fn malformed_text_is_refused() {
    for bad in [
        "",
        "{",
        "[1,]",
        "{\"a\"}",
        "{\"a\":1,}",
        "tru",
        "\"open",
        "\"\\x\"",
        "\"\\ud83d\"",
        "1 2",
        "nan",
        "1e999",
        "\"a\nb\"",
    ] {
        assert!(parse(bad).is_none(), "{bad:?} parsed");
    }
}

#[test]
fn nesting_past_the_bound_is_refused_not_overflowed() {
    let deep = "[".repeat(100_000) + &"]".repeat(100_000);
    assert!(parse(&deep).is_none());
    let fine = "[".repeat(100) + &"]".repeat(100);
    assert!(parse(&fine).is_some());
}

#[test]
fn members_are_found_by_path() {
    let v = parse(r#"{"params":{"textDocument":{"uri":"a.zkl"}}}"#).expect("parses");
    let uri = v.at(&["params", "textDocument", "uri"]).and_then(Json::str);
    assert_eq!(uri, Some("a.zkl"));
    assert!(v.at(&["params", "missing"]).is_none());
}
