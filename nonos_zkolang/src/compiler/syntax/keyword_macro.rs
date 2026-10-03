/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The macro that defines `Keyword` from a list of names and spellings. */

macro_rules! keywords {
    ($($name:ident = $text:literal,)*) => {
        /** A keyword. */
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub enum Keyword {
            $($name,)*
        }

        impl Keyword {
            /** The keyword spelled by `word`, if any. */
            pub fn from_word(word: &str) -> Option<Keyword> {
                match word {
                    $($text => Some(Keyword::$name),)*
                    _ => None,
                }
            }

            /** How the keyword is spelled, in backquotes for a diagnostic. */
            pub fn as_str(self) -> &'static str {
                match self {
                    $(Keyword::$name => concat!("`", $text, "`"),)*
                }
            }

            /** How the keyword is spelled. */
            pub fn text(self) -> &'static str {
                match self {
                    $(Keyword::$name => $text,)*
                }
            }
        }
    };
}

pub(super) use keywords;
