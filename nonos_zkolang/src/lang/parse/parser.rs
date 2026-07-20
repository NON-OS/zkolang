/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Recursive-descent parser: a token stream to the abstract syntax tree the
//! compiler lowers. Precedence is encoded by the call chain, equality lowest,
//! then add and subtract, then multiply, then primaries.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use super::super::lex::Tok;
use super::super::CompileError;
use super::ast::{Ast, ConstDef, Expr, FnDef, Stmt};

struct Parser<'a> {
    toks: &'a [Tok],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.pos)
    }

    fn bump(&mut self) -> Option<&Tok> {
        let t = self.toks.get(self.pos);
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    // Consume a token that must be exactly `want`.
    fn expect(&mut self, want: &Tok) -> Result<(), CompileError> {
        match self.bump() {
            Some(t) if t == want => Ok(()),
            Some(_) => Err(CompileError::UnexpectedToken),
            None => Err(CompileError::UnexpectedEof),
        }
    }

    fn program(&mut self) -> Result<Ast, CompileError> {
        let mut consts = Vec::new();
        let mut fns = Vec::new();
        let mut stmts = Vec::new();
        while self.peek().is_some() {
            match self.peek() {
                Some(Tok::Const) => consts.push(self.const_def()?),
                Some(Tok::Fn) => fns.push(self.fn_def()?),
                _ => stmts.push(self.stmt()?),
            }
        }
        Ok(Ast { consts, fns, stmts })
    }

    // A constant table: `const name = [n0, n1, ...];`. The entries are decimal
    // literals in declaration order; a later index into the table folds to one of
    // them at compile time.
    fn const_def(&mut self) -> Result<ConstDef, CompileError> {
        self.pos += 1; // the `const`
        let name = self.ident()?;
        self.expect(&Tok::Assign)?;
        self.expect(&Tok::LBracket)?;
        let mut values = Vec::new();
        if !matches!(self.peek(), Some(Tok::RBracket)) {
            loop {
                values.push(self.number()?);
                if matches!(self.peek(), Some(Tok::Comma)) {
                    self.pos += 1;
                } else {
                    break;
                }
            }
        }
        self.expect(&Tok::RBracket)?;
        self.expect(&Tok::Semi)?;
        Ok(ConstDef { name, values })
    }

    // A function definition: `fn name(a, b) = expr;`. The body is one expression,
    // inlined at each call, so there is no statement block and no return keyword.
    fn fn_def(&mut self) -> Result<FnDef, CompileError> {
        self.pos += 1; // the `fn`
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
        self.expect(&Tok::Assign)?;
        let body = self.expr()?;
        self.expect(&Tok::Semi)?;
        Ok(FnDef { name, params, body })
    }

    // Consume an identifier, for a name or a parameter.
    fn ident(&mut self) -> Result<String, CompileError> {
        match self.bump() {
            Some(Tok::Ident(n)) => Ok(n.clone()),
            Some(_) => Err(CompileError::UnexpectedToken),
            None => Err(CompileError::UnexpectedEof),
        }
    }

    fn stmt(&mut self) -> Result<Stmt, CompileError> {
        match self.peek() {
            Some(Tok::Let) => {
                self.pos += 1;
                let name = match self.bump() {
                    Some(Tok::Ident(n)) => n.clone(),
                    Some(_) => return Err(CompileError::UnexpectedToken),
                    None => return Err(CompileError::UnexpectedEof),
                };
                self.expect(&Tok::Assign)?;
                let e = self.expr()?;
                self.expect(&Tok::Semi)?;
                Ok(Stmt::Let(name, e))
            }
            Some(Tok::Assert) => {
                self.pos += 1;
                let e = self.expr()?;
                self.expect(&Tok::Semi)?;
                Ok(Stmt::Assert(e))
            }
            Some(Tok::Input) => {
                self.pos += 1;
                let name = match self.bump() {
                    Some(Tok::Ident(n)) => n.clone(),
                    Some(_) => return Err(CompileError::UnexpectedToken),
                    None => return Err(CompileError::UnexpectedEof),
                };
                self.expect(&Tok::Semi)?;
                Ok(Stmt::Input(name))
            }
            Some(Tok::Secret) => {
                self.pos += 1;
                let name = match self.bump() {
                    Some(Tok::Ident(n)) => n.clone(),
                    Some(_) => return Err(CompileError::UnexpectedToken),
                    None => return Err(CompileError::UnexpectedEof),
                };
                self.expect(&Tok::Semi)?;
                Ok(Stmt::Secret(name))
            }
            Some(Tok::Output) => {
                self.pos += 1;
                let e = self.expr()?;
                self.expect(&Tok::Semi)?;
                Ok(Stmt::Output(e))
            }
            Some(Tok::For) => self.for_loop(),
            Some(_) => Err(CompileError::UnexpectedToken),
            None => Err(CompileError::UnexpectedEof),
        }
    }

    // A bounded loop: `for i in lo .. hi { stmt* }`. The bounds are literals, so
    // the iteration count is known at compile time and the compiler unrolls it.
    fn for_loop(&mut self) -> Result<Stmt, CompileError> {
        self.pos += 1; // the `for`
        let var = match self.bump() {
            Some(Tok::Ident(n)) => n.clone(),
            Some(_) => return Err(CompileError::UnexpectedToken),
            None => return Err(CompileError::UnexpectedEof),
        };
        self.expect(&Tok::In)?;
        let lo = self.number()?;
        self.expect(&Tok::DotDot)?;
        let hi = self.number()?;
        self.expect(&Tok::LBrace)?;
        let mut body = Vec::new();
        while !matches!(self.peek(), Some(Tok::RBrace)) {
            if self.peek().is_none() {
                return Err(CompileError::UnexpectedEof);
            }
            body.push(self.stmt()?);
        }
        self.expect(&Tok::RBrace)?;
        Ok(Stmt::For { var, lo, hi, body })
    }

    // Consume a single numeric literal, for a loop bound.
    fn number(&mut self) -> Result<u64, CompileError> {
        match self.bump() {
            Some(Tok::Num(v)) => Ok(*v),
            Some(_) => Err(CompileError::UnexpectedToken),
            None => Err(CompileError::UnexpectedEof),
        }
    }

    fn expr(&mut self) -> Result<Expr, CompileError> {
        self.equality()
    }

    // The lowest precedence: an optional single comparison of two sums. It is not
    // chainable, `a == b == c` is a parse error, because a comparison yields a bit
    // and comparing a bit to a third sum is almost never what a writer means.
    fn equality(&mut self) -> Result<Expr, CompileError> {
        let lhs = self.sum()?;
        match self.peek() {
            Some(Tok::EqEq) => {
                self.pos += 1;
                let rhs = self.sum()?;
                Ok(Expr::Eq(Box::new(lhs), Box::new(rhs)))
            }
            Some(Tok::BangEq) => {
                self.pos += 1;
                let rhs = self.sum()?;
                Ok(Expr::Ne(Box::new(lhs), Box::new(rhs)))
            }
            _ => Ok(lhs),
        }
    }

    fn sum(&mut self) -> Result<Expr, CompileError> {
        let mut lhs = self.product()?;
        loop {
            match self.peek() {
                Some(Tok::Plus) => {
                    self.pos += 1;
                    let rhs = self.product()?;
                    lhs = Expr::Add(Box::new(lhs), Box::new(rhs));
                }
                Some(Tok::Minus) => {
                    self.pos += 1;
                    let rhs = self.product()?;
                    lhs = Expr::Sub(Box::new(lhs), Box::new(rhs));
                }
                _ => return Ok(lhs),
            }
        }
    }

    // Multiply and divide, which bind tighter than add and subtract. Both are
    // left-associative, so `a / b / c` is `(a / b) / c`. Their operands are
    // unary expressions, so a leading minus binds tighter still.
    fn product(&mut self) -> Result<Expr, CompileError> {
        let mut lhs = self.unary()?;
        loop {
            match self.peek() {
                Some(Tok::Star) => {
                    self.pos += 1;
                    let rhs = self.unary()?;
                    lhs = Expr::Mul(Box::new(lhs), Box::new(rhs));
                }
                Some(Tok::Slash) => {
                    self.pos += 1;
                    let rhs = self.unary()?;
                    lhs = Expr::Div(Box::new(lhs), Box::new(rhs));
                }
                _ => return Ok(lhs),
            }
        }
    }

    // A prefix minus negates. It is right-recursive so `- - a` double-negates, and
    // it sits above the primaries so `-a * b` parses as `(-a) * b`.
    fn unary(&mut self) -> Result<Expr, CompileError> {
        if matches!(self.peek(), Some(Tok::Minus)) {
            self.pos += 1;
            let inner = self.unary()?;
            return Ok(Expr::Neg(Box::new(inner)));
        }
        self.primary()
    }

    // A primary is an atom followed by any number of `[index]` suffixes, so
    // `RC[i]` and, in principle, `T[i][j]` parse left to right. Indexing binds
    // tighter than the unary minus above it and the arithmetic below.
    fn primary(&mut self) -> Result<Expr, CompileError> {
        let mut base = self.atom()?;
        while matches!(self.peek(), Some(Tok::LBracket)) {
            self.pos += 1;
            let index = self.expr()?;
            self.expect(&Tok::RBracket)?;
            base = Expr::Index(Box::new(base), Box::new(index));
        }
        Ok(base)
    }

    fn atom(&mut self) -> Result<Expr, CompileError> {
        match self.bump() {
            Some(Tok::Num(v)) => Ok(Expr::Num(*v)),
            // An identifier is a call when followed by `(`, otherwise a variable.
            Some(Tok::Ident(n)) => {
                let name = n.clone();
                if matches!(self.peek(), Some(Tok::LParen)) {
                    self.pos += 1;
                    let mut args = Vec::new();
                    if !matches!(self.peek(), Some(Tok::RParen)) {
                        loop {
                            args.push(self.expr()?);
                            if matches!(self.peek(), Some(Tok::Comma)) {
                                self.pos += 1;
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(&Tok::RParen)?;
                    Ok(Expr::Call(name, args))
                } else {
                    Ok(Expr::Var(name))
                }
            }
            Some(Tok::LParen) => {
                let e = self.expr()?;
                self.expect(&Tok::RParen)?;
                Ok(e)
            }
            Some(Tok::Inv) => {
                self.expect(&Tok::LParen)?;
                let e = self.expr()?;
                self.expect(&Tok::RParen)?;
                Ok(Expr::Inv(Box::new(e)))
            }
            Some(Tok::Sel) => {
                self.expect(&Tok::LParen)?;
                let cond = self.expr()?;
                self.expect(&Tok::Comma)?;
                let a = self.expr()?;
                self.expect(&Tok::Comma)?;
                let b = self.expr()?;
                self.expect(&Tok::RParen)?;
                Ok(Expr::Sel(Box::new(cond), Box::new(a), Box::new(b)))
            }
            // A conditional expression: `if c { a } else { b }`. Both arms are
            // single expressions, because the lowering to `sel` evaluates both.
            Some(Tok::If) => {
                let cond = self.expr()?;
                self.expect(&Tok::LBrace)?;
                let a = self.expr()?;
                self.expect(&Tok::RBrace)?;
                self.expect(&Tok::Else)?;
                self.expect(&Tok::LBrace)?;
                let b = self.expr()?;
                self.expect(&Tok::RBrace)?;
                Ok(Expr::If(Box::new(cond), Box::new(a), Box::new(b)))
            }
            Some(_) => Err(CompileError::UnexpectedToken),
            None => Err(CompileError::UnexpectedEof),
        }
    }
}

/// Parse a token stream into an AST.
pub fn parse(toks: &[Tok]) -> Result<Ast, CompileError> {
    let mut p = Parser { toks, pos: 0 };
    p.program()
}
