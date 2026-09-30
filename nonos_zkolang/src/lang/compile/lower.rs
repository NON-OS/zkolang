/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Lowering: count the inputs, walk the statements with the liveness table that frees
 * dead bindings, and finish the program with its advice plan.
 */

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
     * reads_after[i] is the sorted set of names read by statements i onward, so after
     * lowering statement i a binding is dead exactly when its name is absent from
     * reads_after[i + 1]. Building it from the end keeps the whole pass linear.
     */
    let stmts = &ast.stmts;
    let mut reads_after: Vec<Vec<String>> = Vec::with_capacity(stmts.len() + 1);
    reads_after.resize(stmts.len() + 1, Vec::new());
    for i in (0..stmts.len()).rev() {
        let mut names = reads_after[i + 1].clone();
        live::reads_of_stmt(&stmts[i], &mut names);
        names.sort_unstable();
        names.dedup();
        reads_after[i] = names;
    }
    for (i, s) in stmts.iter().enumerate() {
        c.stmt(s)?;
        c.free_dead(&reads_after[i + 1]);
    }
    Ok(c.finish())
}
