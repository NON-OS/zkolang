/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The messages the server sends: a result, an error, and a notification. */

use std::io::{self, Write};

use super::json::Json;
use super::rpc::send;

fn message(mut fields: Vec<(&str, Json)>) -> Json {
    fields.insert(0, ("jsonrpc", Json::text("2.0")));
    Json::obj(fields)
}

/** The result of the request `id`. */
pub(crate) fn reply(out: &mut impl Write, id: Json, result: Json) -> io::Result<()> {
    send(out, &message(vec![("id", id), ("result", result)]))
}

/** The error `code`, saying `why`, for the request `id`. */
pub(crate) fn fail(out: &mut impl Write, id: Json, code: &str, why: &str) -> io::Result<()> {
    let error = Json::obj(vec![
        ("code", Json::Num(code.to_string())),
        ("message", Json::text(why)),
    ]);
    send(out, &message(vec![("id", id), ("error", error)]))
}

/** The notification `method` with `params`. */
pub(crate) fn notify(out: &mut impl Write, method: &str, params: Json) -> io::Result<()> {
    send(
        out,
        &message(vec![("method", Json::text(method)), ("params", params)]),
    )
}

/** What the server does: whole documents synchronised, diagnostics, and formatting. */
pub(crate) fn capabilities() -> Json {
    let sync = Json::obj(vec![
        ("openClose", Json::Bool(true)),
        ("change", Json::num(1)),
        ("save", Json::Bool(true)),
    ]);
    let caps = Json::obj(vec![
        ("textDocumentSync", sync),
        ("documentFormattingProvider", Json::Bool(true)),
    ]);
    let info = Json::obj(vec![
        ("name", Json::text("zkolang")),
        ("version", Json::text(env!("CARGO_PKG_VERSION"))),
    ]);
    Json::obj(vec![("capabilities", caps), ("serverInfo", info)])
}
