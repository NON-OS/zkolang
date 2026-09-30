/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Positional fields, as in a tuple struct or a tuple variant. */

use alloc::format;

use super::parser::{PResult, Parser};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{FieldDecl, Fields, Visibility};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** `(T, pub U)`; in a variant, `(T, U)`, whose fields take no `pub` or attributes. */
    pub(super) fn tuple_fields(&mut self, in_variant: bool) -> PResult<Fields> {
        self.expect(TokenKind::LParen)?;
        let fields = self.decl_list(TokenKind::RParen, |p| {
            let start = p.span();
            let (doc, attrs) = p.doc_and_attrs();
            if in_variant {
                if let Some(a) = attrs.first() {
                    p.not_in_variant(
                        a.span,
                        "an attribute",
                        "a tuple variant's fields are types alone, with no attributes",
                    );
                }
            }
            let vis = p.field_vis(in_variant);
            let ty = p.ty()?;
            Ok(FieldDecl {
                attrs,
                vis,
                doc,
                name: None,
                ty,
                span: start.to(p.prev_span()),
            })
        })?;
        Ok(Fields::Tuple(fields))
    }

    /** The visibility of a field; `pub` is reported in a variant, whose fields have none. */
    pub(super) fn field_vis(&mut self, in_variant: bool) -> Visibility {
        let at = self.span();
        if !self.eat_kw(Keyword::Pub) {
            return Visibility::Private;
        }
        if in_variant {
            self.not_in_variant(
                at,
                "`pub`",
                "a variant's fields have the visibility of its enum",
            );
        }
        Visibility::Public
    }

    /** Report `what`, at `at`, on a variant's field, with `help` saying why it cannot be there. */
    fn not_in_variant(&mut self, at: Span, what: &str, help: &str) {
        let d = Diagnostic::error(
            Code::UNEXPECTED_TOKEN,
            format!("{what} on a variant's field"),
            at,
            "not allowed here",
        )
        .with_help(help);
        self.diags.push(d);
    }
}
