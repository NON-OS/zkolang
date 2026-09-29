/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Expression lowering: the dispatcher routes each node to its own lowering file.

mod args;
mod array_return;
mod binary;
mod block;
mod block_close;
mod block_open;
mod call;
mod compare;
mod decompose;
mod dispatch;
mod div;
mod index;
mod inline;
mod inv;
mod ne;
mod neg;
mod num;
mod select;
mod tuple;
mod var;
