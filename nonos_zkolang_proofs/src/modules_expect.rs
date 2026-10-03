/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What a crate of several files reports, and what its files state it must, as (file, line, code). */

use nonos_zkolang::compiler::diag::Diagnostics;
use nonos_zkolang::compiler::source::SourceMap;

use crate::ui_expect::expected;

/** A diagnostic's place and code. */
pub(crate) type Seen = (String, usize, String);

/** Each diagnostic of `diags`, by the file and line of `map` it is at, sorted. */
pub(crate) fn reported(map: &SourceMap, diags: &Diagnostics) -> Vec<Seen> {
    let mut got: Vec<Seen> = diags
        .items()
        .iter()
        .filter_map(|d| {
            let (file, line, _) = map.locate(d.span())?;
            Some((file.to_string(), line, d.code.0.to_string()))
        })
        .collect();
    got.sort();
    got
}

/** Each `/*~ CODE */` comment of the files of `map`, sorted. */
pub(crate) fn stated(map: &SourceMap) -> Vec<Seen> {
    let mut want: Vec<Seen> = map
        .files()
        .iter()
        .flat_map(|f| {
            expected(&f.text)
                .into_iter()
                .map(|(l, c)| (f.name.clone(), l, c))
        })
        .collect();
    want.sort();
    want
}
