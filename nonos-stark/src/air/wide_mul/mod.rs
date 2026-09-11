// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Wide multiplication over 16-bit limbs, the gadget behind the 256-bit value arithmetic a
//! note commitment needs. A field element is too narrow to hold a full 64-by-64 product
//! without wrapping, so a value is split into four limbs and multiplied schoolbook, each
//! output weight carrying its sum and carry. `limbs` holds the split and its range discipline,
//! `product` the schoolbook itself. Every limb is range-checked in circuit, because an
//! unbounded decomposition reassembles to anything, which is the wraparound the gadget exists
//! to prevent.

mod limbs;
mod product;

pub use limbs::{split, LIMB_BITS, LIMB_MASK, N_LIMBS};
pub use product::{wide_mul, Product, N_OUT};
