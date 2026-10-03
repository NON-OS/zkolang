/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Reading numbers from the command line. */

use crate::line::Line;

/**
 * The Goldilocks modulus. An input at or above it is not a field element; reducing it
 * silently would prove a statement about a different number than the one typed.
 */
const MODULUS: u64 = 0xFFFF_FFFF_0000_0001;

/**
 * Read the comma-separated list of field elements given for `flag`, or none if the flag
 * is absent. A token that is not a decimal number, or a number that is not below the
 * modulus, is an error naming it; dropping it would shift every later value to another
 * input.
 */
pub(crate) fn nums(line: &Line, flag: &str) -> Result<Vec<u64>, String> {
    let Some(list) = line.value(flag) else {
        return Ok(Vec::new());
    };
    if list.trim().is_empty() {
        return Ok(Vec::new());
    }
    list.split(',')
        .map(|t| {
            let t = t.trim();
            match t.parse::<u64>() {
                Ok(v) if v < MODULUS => Ok(v),
                Ok(_) => Err(format!(
                    "{flag}: {t} is not below the field modulus {MODULUS}"
                )),
                Err(_) => Err(format!("{flag}: `{t}` is not a decimal number")),
            }
        })
        .collect()
}
