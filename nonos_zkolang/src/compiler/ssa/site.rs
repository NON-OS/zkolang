/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Where an instruction comes from (section 15.2): an index into the sites that lowering
 * records, each a place in the source and the calls inlined to reach it. The passes keep
 * each instruction's site, and give an instruction they write in place of another that
 * one's site.
 */

/** A site; `Site(0)`, the default, is the program's own, before any function runs. */
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Site(pub u32);
