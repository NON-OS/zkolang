/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The portability header for the emitted assembly. A .S file is run through the C
//! preprocessor before assembly, so one macro names the entry point and the C library
//! symbols the way each object format expects: a leading underscore on Mach-O, plain
//! on ELF. Every reference to the entry point and to the runtime goes through it, so
//! the same source assembles on either platform.

pub(super) const HEADER: &str = "\
#ifdef __APPLE__
# define SYM(x) _##x
#else
# define SYM(x) x
#endif
";
