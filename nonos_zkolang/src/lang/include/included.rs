/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A file an include names, as a resolver finds it. */

use alloc::string::String;

/** What a resolver returns for one include. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Included {
    /**
     * Names the file however its path is spelled, such as its canonical path. A file is
     * spliced in once per key, and the key is what its own includes resolve against.
     */
    pub key: String,
    /** The file's text. */
    pub text: String,
}
