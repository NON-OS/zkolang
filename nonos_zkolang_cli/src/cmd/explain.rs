/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! `explain`: the long description of a diagnostic code (section 19). */

use nonos_zkolang::compiler::diag::explain as description;

const USAGE: &str = "usage: zkolang explain <code>, such as E0300";

/** Print the description of the code `args` names. */
pub(crate) fn explain(args: &[String]) -> Result<(), String> {
    let [code] = args else {
        return Err(String::from(USAGE));
    };
    let code = code.to_ascii_uppercase();
    let text =
        description(&code).ok_or_else(|| format!("`{code}` is no diagnostic code\n{USAGE}"))?;
    println!("{code}: {text}");
    Ok(())
}
