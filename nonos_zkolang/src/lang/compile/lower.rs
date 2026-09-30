/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Lowering: count the inputs, walk the statements with the liveness table that frees
 * dead bindings, and finish the program with its advice plan.
 */

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use super::compiled::Compiled;
use super::compiler::Compiler;
use super::{count_inputs, count_secrets, live};
use crate::lang::parse::Ast;
use crate::lang::CompileError;

/*
 * Lower an AST as given, without optimizing. The public inputs and secrets are counted
 * first, through any loops, so secrets index after the public prefix and comparison advice
 * indexes after the secrets.
 */
pub(super) fn lower(ast: &Ast) -> Result<Compiled, CompileError> {
    let n_public =
        u16::try_from(count_inputs::count_inputs(&ast.stmts)).map_err(|_| CompileError::IoLimit)?;
    let n_secret = u16::try_from(count_secrets::count_secrets(&ast.stmts))
        .map_err(|_| CompileError::IoLimit)?;
    let mut c = Compiler::new(ast.consts.clone(), ast.fns.clone(), n_public, n_secret);
    /*
     * last_read maps each name to the last statement that reads it, so after lowering
     * statement i a binding is dead exactly when no statement past i reads its name.
     */
    let stmts = &ast.stmts;
    let mut last_read: BTreeMap<String, usize> = BTreeMap::new();
    for (i, s) in stmts.iter().enumerate() {
        let mut names = Vec::new();
        live::reads_of_stmt(s, &mut names);
        last_read.extend(names.into_iter().map(|n| (n, i)));
    }
    for (i, s) in stmts.iter().enumerate() {
        c.stmt(s)?;
        c.free_dead(|n| last_read.get(n).is_some_and(|&j| j > i));
    }
    Ok(c.finish())
}
