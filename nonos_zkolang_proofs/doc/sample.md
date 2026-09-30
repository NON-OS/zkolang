<!-- The reference of `sample`, made by `zkolang doc`. -->

# `sample`

A sample crate for the reference renderer.

### `pub struct Point`

A point of the plane.

- `pub x: u32`: Across.
- `pub y: u32`

### `impl Point`

#### `pub fn steps(self) -> u32`

Its distance from the origin, in steps.

### `pub struct Both<T>(pub T, pub T)`

A pair of one type, by position.

### `pub enum Shape`

A shape.

- `Empty`: Nothing at all.
- `Dot(Point)`

### `pub type Quad = [Point; 4]`

Four points.

### `pub const MAX: u32`

The largest coordinate.

### `pub fn origin(zero: u32) -> Point`

The origin.

## `sample::geo`

Geometry.

### `pub fn twice<const N: usize>(x: [u32; N]) -> u32`

Twice `x`.

