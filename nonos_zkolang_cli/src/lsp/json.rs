/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A JSON value, as the language server protocol carries one. */

/** A JSON value. A number keeps its text, so an id goes back exactly as it came. */
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Json {
    Null,
    Bool(bool),
    Num(String),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

impl Json {
    /** The member `key` of an object. */
    pub(crate) fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Obj(fields) => fields.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    /** The value at the path `keys`, one member after another. */
    pub(crate) fn at(&self, keys: &[&str]) -> Option<&Json> {
        keys.iter().try_fold(self, |j, k| j.get(k))
    }

    /** The text of a string. */
    pub(crate) fn str(&self) -> Option<&str> {
        match self {
            Json::Str(s) => Some(s),
            _ => None,
        }
    }

    /** An object of `fields`, in order. */
    pub(crate) fn obj(fields: Vec<(&str, Json)>) -> Json {
        Json::Obj(
            fields
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
        )
    }

    /** The number `n`. */
    pub(crate) fn num(n: usize) -> Json {
        Json::Num(n.to_string())
    }

    /** The string `s`. */
    pub(crate) fn text(s: &str) -> Json {
        Json::Str(s.to_string())
    }
}
