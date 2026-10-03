/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The language server, `zkolang lsp`: the Language Server Protocol on standard input and
 * output, publishing the diagnostics of each open edition 2026 document as it changes and
 * laying documents out with `zkolang fmt`.
 */

mod check;
mod diagnostics;
mod docs;
mod documents;
mod json;
mod json_list;
mod json_read;
mod json_string;
#[cfg(test)]
mod json_tests;
mod json_write;
mod position;
mod reply;
mod rpc;
mod server;
mod uri;

pub(crate) use server::serve;
