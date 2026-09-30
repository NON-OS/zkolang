/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Structs and enums (sections 5.2 and 6). Each declaration, and each instance of a generic
 * one, is one entry of the type table, its fields at concrete types, so nothing that reads
 * a type substitutes generic parameters. A struct is an entry with one variant.
 */

use alloc::string::String;
use alloc::vec::Vec;

use super::TyId;
use crate::compiler::tir::Labels;

/** A struct or enum, as an index into the type table's entries. */
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AdtId(pub u32);

/** How a struct or variant writes its fields: none, by position, or by name. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Form {
    Unit,
    Tuple,
    Named,
}

/** A field: its name if it has one, its type, the labels its type writes, and whether `pub`. */
#[derive(Clone, Debug)]
pub struct AdtField {
    pub name: Option<String>,
    pub ty: TyId,
    pub labels: Labels,
    pub public: bool,
}

/** A variant of an enum, or the one shape of a struct. */
#[derive(Clone, Debug)]
pub struct AdtVariant {
    pub name: String,
    pub form: Form,
    pub fields: Vec<AdtField>,
}

/**
 * A struct or enum: its name as messages show it, the declaration and module it comes
 * from (as indices of the item table), and its variants.
 */
#[derive(Clone, Debug)]
pub struct Adt {
    pub name: String,
    pub def: u32,
    pub module: u32,
    pub is_enum: bool,
    pub variants: Vec<AdtVariant>,
}

impl Adt {
    /** The index and field of the named field `name` of a struct. */
    pub fn field(&self, name: &str) -> Option<(u32, &AdtField)> {
        let v = self.variants.first().filter(|_| !self.is_enum)?;
        let at = v
            .fields
            .iter()
            .position(|f| f.name.as_deref() == Some(name))?;
        Some((u32::try_from(at).ok()?, v.fields.get(at)?))
    }
}
