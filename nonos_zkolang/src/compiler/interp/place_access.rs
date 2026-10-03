/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Reading and writing the part of a local an evaluated place names. */

use super::eval_place::Path;
use super::machine::Interp;
use super::Value;

impl<'e> Interp<'e> {
    /** The value at `path`. */
    pub(super) fn read(&self, path: &Path) -> Option<&Value> {
        let mut v = self.frame.get(path.root as usize)?;
        for &s in &path.steps {
            v = v.parts().get(s)?;
        }
        Some(v)
    }

    /** Replace the value at `path` with `new`. */
    pub(super) fn write(&mut self, path: &Path, new: Value) {
        let Some(mut v) = self.frame.get_mut(path.root as usize) else {
            return;
        };
        for &s in &path.steps {
            let part = match v {
                Value::Tuple(parts) | Value::Array(parts) => parts.get_mut(s),
                _ => None,
            };
            let Some(part) = part else {
                return;
            };
            v = part;
        }
        *v = new;
    }
}
