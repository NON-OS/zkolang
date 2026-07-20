/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Array bindings: fixed-size vectors of field values, resolved at compile time. An
//! array names a run of registers, one per element, and a read with a static index
//! selects one. An array costs nothing at proof time beyond the registers its live
//! elements hold. One concern per file: lookup, bind, take, and element read.

mod bind;
mod element;
mod lookup;
mod take;
