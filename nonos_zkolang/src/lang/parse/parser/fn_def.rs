/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A function definition.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::FnDef;
use crate::lang::CompileError;
use alloc::vec::Vec;

impl<'a> Parser<'a> {
    /// `fn name(a, b) = expr;` or `fn name(a, b) { let ...; return expr; }`: the body is
    /// either one expression or a block of local bindings and a result, inlined at each
    /// call. There is still no call stack and no recursion, only substitution.
    pub(crate) fn fn_def(&mut self) -> Result<FnDef, CompileError> {
        self.pos += 1;
        let name = self.ident()?;
        self.expect(&Tok::LParen)?;
        let mut params = Vec::new();
        if !matches!(self.peek(), Some(Tok::RParen)) {
            loop {
                params.push(self.ident()?);
                if matches!(self.peek(), Some(Tok::Comma)) {
                    self.pos += 1;
                } else {
                    break;
                }
            }
        }
        self.expect(&Tok::RParen)?;
        // Two body forms: `= expr;` for a one-line function, or a braced block for one
        // with local bindings and a result. A block reads to the closing brace itself.
        let body = if matches!(self.peek(), Some(Tok::LBrace)) {
            self.pos += 1;
            self.block_body()?
        } else {
            self.expect(&Tok::Assign)?;
            let e = self.expr()?;
            self.expect(&Tok::Semi)?;
            e
        };
        Ok(FnDef { name, params, body })
    }
}
