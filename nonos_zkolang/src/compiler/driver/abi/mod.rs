/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The ABI (section 12.2): `main`'s public inputs, secret inputs and result as lists of
 * scalar leaves in declaration order, a tuple, struct or array giving its parts' leaves.
 * A value is given per leaf; a 64-bit integer's leaf takes two slots, its pattern's
 * halves. An enum gives one leaf per slot: its tag, then its payload as laid out.
 */

mod decode;
mod encode;
mod leaf;
mod program;

pub use decode::decode;
pub use encode::encode;
pub use leaf::{leaves, slots, AbiError, Leaf};
pub use program::abi_of;
