<!-- The reference of `std`, made by `zkolang doc`. -->

# `std`

The standard library of zKølang, edition 2026 (section 18), written in zKølang.

## `std::hash`

MiMC over `field` (section 18.2): the sixteen round permutation and the compressions
built from it, the same functions as the edition 2025 standard library's `hash.zkl`.

### `pub const MIMC_C: [field; 16]`

The round constants, in the order the rounds use them.

### `pub fn mimc_round(s: field, c: field) -> field`

One round: the state plus the round constant `c`, to the seventh power.

### `pub fn rounds<const LO: usize, const HI: usize>(s: field) -> field`

The rounds whose constants are `MIMC_C[LO..HI]`, applied to `s` in order.

### `pub fn permute(s: field) -> field`

The sixteen round MiMC permutation of `s`.

### `pub fn compress_wide(left: field, right: field) -> field`

A two-to-one compression: `left` through the first eight rounds, then `right` added, then the last eight.

## `std::merkle`

Merkle trees over MiMC (section 18.2): a node's two-to-one compression, one level of an
authentication path, and the root a whole path climbs to, as the edition 2025 standard
library's `merkle.zkl` builds them.

### `pub fn compress2(left: field, right: field) -> field`

A node: `left` through four rounds, then `right` added, then four more.

### `pub fn step(node: field, sib: field, right: bool) -> field`

One level of a path: `node` and its sibling `sib`, ordered by `right`, which is true when
`node` is the right child, then compressed.

### `pub fn root<const N: usize>(leaf: field, path: [field; N], right: [bool; N]) -> field`

The root that `leaf` climbs to along the siblings `path`, bottom first, placed by `right`.

## `std::option`

Optional values (section 18.1): an `Option<T>` holds a `T` or nothing.

### `pub enum Option<T>`

A value of type `T`, or none.

- `None`
- `Some(T)`

### `impl<T> Option<T>`

#### `pub fn is_some(self) -> bool`

Whether the option holds a value.

#### `pub fn is_none(self) -> bool`

Whether the option holds nothing.

#### `pub fn unwrap(self) -> T`

The value held; the run fails if there is none.

#### `pub fn unwrap_or(self, default: T) -> T`

The value held, or `default`.

#### `pub fn or(self, other: Option<T>) -> Option<T>`

This option if it holds a value, else `other`.

#### `pub fn and(self, other: Option<T>) -> Option<T>`

`other` if this option holds a value, else `None`.

#### `pub fn xor(self, other: Option<T>) -> Option<T>`

The option that holds a value, if exactly one of the two does.

## `std::prelude`

The prelude (section 18.1): what every module names without importing it. Each name
it exports stands after a module's own names and `std`; the variants of an enum it
exports stand as lone names.

### `pub use crate::option::Option`

