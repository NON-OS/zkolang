/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Paths: what one starts from, and the segments that follow. */

use alloc::vec::Vec;

use super::{GenericArg, Ident};
use crate::compiler::source::Span;

/** What a path starts from. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PathRoot {
    /** A plain name, resolved by scope. */
    Plain,
    /** `crate::` */
    Crate,
    /** `super::` */
    Super,
    /** `self::` */
    SelfModule,
    /** `Self::` */
    SelfType,
}

/**
 * One segment of a path, with its generic arguments when written with a turbofish or in a
 * type.
 */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PathSegment {
    pub ident: Ident,
    pub generics: Option<Vec<GenericArg>>,
}

/** A path such as `std::hash::mimc::permute`, or `size_of::<u8>` with generic arguments at its end. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Path {
    pub root: PathRoot,
    pub segments: Vec<PathSegment>,
    pub span: Span,
}

impl Path {
    /** The single name of a one-segment plain path without generics, if it is one. */
    pub fn as_ident(&self) -> Option<&Ident> {
        match (self.root, self.segments.as_slice()) {
            (PathRoot::Plain, [seg]) if seg.generics.is_none() => Some(&seg.ident),
            _ => None,
        }
    }

    /** The last segment's name. */
    pub fn last_name(&self) -> &str {
        self.segments
            .last()
            .map(|s| s.ident.name.as_str())
            .unwrap_or("")
    }
}
