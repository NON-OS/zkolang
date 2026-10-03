<!-- The reference of `std`, made by `zkolang doc`. -->

# `std`

The standard library of zKølang, edition 2026 (section 18), written in zKølang.

## `std::array`

Arrays (section 18.2): folds and searches over an array of any length `N`. A function
whose element type `T` must add, multiply or compare is checked for each `T` it is
called with (section 5.5), so it takes the integer types and `field` where they allow it.

### `pub fn sum<T, const N: usize>(a: [T; N]) -> T`

The sum of the elements of `a`; the run fails if an integer sum overflows `T`.

### `pub fn product<T, const N: usize>(a: [T; N]) -> T`

The product of the elements of `a`; the run fails if an integer product overflows `T`.

### `pub fn dot<T, const N: usize>(a: [T; N], b: [T; N]) -> T`

The sum of the products of the elements of `a` and `b` at each index.

### `pub fn position<T, const N: usize>(a: [T; N], x: T) -> Option<usize>`

The index of the first element of `a` equal to `x`, if one is.

### `pub fn contains<T, const N: usize>(a: [T; N], x: T) -> bool`

Whether an element of `a` equals `x`.

### `pub fn reverse<T, const N: usize>(a: [T; N]) -> [T; N]`

The elements of `a`, last first.

## `std::cmp`

Comparisons (section 18.2) of two or three values of one integer type `T`, checked for
each `T` a program calls them with (section 5.5).

### `pub fn clamp<T>(x: T, lo: T, hi: T) -> T`

`x` held within `lo..=hi`; the run fails if `lo` is above `hi`.

### `pub fn within<T>(x: T, lo: T, hi: T) -> bool`

Whether `x` lies within `lo..=hi`.

### `pub fn abs_diff<T>(a: T, b: T) -> T`

The larger of `a` and `b` less the smaller; the run fails if that does not fit `T`.

## `std::curve`

Elliptic curves y² = x³ + a·x + b over `field` (section 18.2), in affine coordinates,
`None` the point at infinity, by the formulas of the edition 2025 library's `curve.zkl`.

### `pub struct Curve`

The curve y² = x³ + a·x + b.

- `pub a: field`
- `pub b: field`

### `impl Curve`

#### `pub fn contains(self, p: Point) -> bool`

Whether `p` lies on the curve.

#### `pub fn add(self, p: Option<Point>, q: Option<Point>) -> Option<Point>`

The sum of the points `p` and `q` of the curve.

#### `pub fn double(self, p: Point) -> Option<Point>`

Twice the point `p` of the curve.

#### `pub fn mul<const N: usize>(self, p: Option<Point>, k: [bool; N]) -> Option<Point>`

`k` times the point `p`, `k` given by its bits, least significant first.

### `pub struct Point`

A point of a curve other than the point at infinity.

- `pub x: field`
- `pub y: field`

### `pub fn neg(p: Option<Point>) -> Option<Point>`

The negation of the point `p`.

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

## `std::poly`

Polynomials (section 18.2), each given by its coefficients, lowest degree first.

### `pub fn eval<T, const N: usize>(c: [T; N], x: T) -> T`

The value at `x` of the polynomial with coefficients `c`, by Horner's rule; for an
integer type, the run fails if a step overflows `T`.

## `std::prelude`

The prelude (section 18.1): what every module names without importing it. Each name
it exports stands after a module's own names and `std`; the variants of an enum it
exports stand as lone names.

### `pub use crate::option::Option`

### `pub use crate::result::Result`

## `std::result`

Results (section 18.1): a `Result<T, E>` holds a value `T`, or an error `E` saying why not.

### `pub enum Result<T, E>`

A value of type `T`, or the error of type `E` that stands for it.

- `Ok(T)`
- `Err(E)`

### `impl<T, E> Result<T, E>`

#### `pub fn is_ok(self) -> bool`

Whether the result holds a value.

#### `pub fn is_err(self) -> bool`

Whether the result holds an error.

#### `pub fn unwrap(self) -> T`

The value held; the run fails if the result holds an error.

#### `pub fn unwrap_err(self) -> E`

The error held; the run fails if the result holds a value.

#### `pub fn unwrap_or(self, default: T) -> T`

The value held, or `default`.

#### `pub fn ok(self) -> Option<T>`

The value held, as an option.

#### `pub fn err(self) -> Option<E>`

The error held, as an option.

