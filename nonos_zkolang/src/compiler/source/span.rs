/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A span: a half-open byte range in one source file. Every syntax node, type, IR
 * instruction and diagnostic carries one, so anything the compiler reports points at the
 * text that caused it.
 */

/** A source file's index in the source map. */
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct FileId(pub u32);

/** The bytes `lo..hi` of file `file`. */
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Span {
    pub file: FileId,
    pub lo: u32,
    pub hi: u32,
}

impl Span {
    /** A span over `lo..hi` in `file`. */
    pub const fn new(file: FileId, lo: u32, hi: u32) -> Span {
        Span { file, lo, hi }
    }

    /** The span a compiler-made node carries when nothing in the source caused it. */
    pub const DUMMY: Span = Span {
        file: FileId(u32::MAX),
        lo: 0,
        hi: 0,
    };

    /** The smallest span covering both, or `self` when they are in different files. */
    pub fn to(self, other: Span) -> Span {
        if self.file != other.file {
            return self;
        }
        Span {
            file: self.file,
            lo: self.lo.min(other.lo),
            hi: self.hi.max(other.hi),
        }
    }

    /** The empty span at this span's end. */
    pub fn end(self) -> Span {
        Span {
            file: self.file,
            lo: self.hi,
            hi: self.hi,
        }
    }

    /** Whether `self` lies within `other`. */
    pub fn within(self, other: Span) -> bool {
        self.file == other.file && other.lo <= self.lo && self.hi <= other.hi
    }

    /** The length in bytes. */
    pub fn len(self) -> u32 {
        self.hi.saturating_sub(self.lo)
    }

    /** Whether the span is empty. */
    pub fn is_empty(self) -> bool {
        self.hi <= self.lo
    }
}
