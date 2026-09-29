/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Reading numbers from the command line. */

/**
 * The Goldilocks modulus. An input at or above it is not a field element; reducing it
 * silently would prove a statement about a different number than the one typed.
 */
const MODULUS: u64 = 0xFFFF_FFFF_0000_0001;

/**
 * Read a comma-separated list of field elements for `flag`. A flag given without a value,
 * a token that is not a decimal number, or a number that is not below the modulus is an
 * error naming it; dropping it would shift every later value to another input.
 */
pub(crate) fn nums(a: &[String], flag: &str) -> Result<Vec<u64>, String> {
    let Some(i) = a.iter().position(|x| x == flag) else {
        return Ok(Vec::new());
    };
    let Some(list) = a.get(i + 1) else {
        return Err(format!("{flag} needs a comma-separated list of numbers"));
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
