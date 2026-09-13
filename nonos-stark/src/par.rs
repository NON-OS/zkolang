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

//! One dispatch point for the prover's data-parallel maps. With the `parallel`
//! feature the maps run across every core; without it they are the ordinary
//! serial iterators the kernel builds. Both forms are indexed and
//! order-preserving, so the output is identical either way — the callers pass
//! pure functions of the index or element, and a bit-exact gate proves the two
//! prover forms emit the same proof bytes.

use alloc::vec::Vec;

/// Map a pure function over `0..n`, collecting in index order.
#[cfg(feature = "parallel")]
pub fn map_index<T, F>(n: usize, f: F) -> Vec<T>
where
    T: Send,
    F: Fn(usize) -> T + Send + Sync,
{
    use rayon::prelude::*;
    (0..n).into_par_iter().map(f).collect()
}

#[cfg(not(feature = "parallel"))]
pub fn map_index<T, F>(n: usize, f: F) -> Vec<T>
where
    F: Fn(usize) -> T,
{
    (0..n).map(f).collect()
}

/// Map a pure function over a slice, collecting in element order.
#[cfg(feature = "parallel")]
pub fn map_slice<A, T, F>(items: &[A], f: F) -> Vec<T>
where
    A: Sync,
    T: Send,
    F: Fn(&A) -> T + Send + Sync,
{
    use rayon::prelude::*;
    items.par_iter().map(f).collect()
}

#[cfg(not(feature = "parallel"))]
pub fn map_slice<A, T, F>(items: &[A], f: F) -> Vec<T>
where
    F: Fn(&A) -> T,
{
    items.iter().map(f).collect()
}

/// Apply a mutation to each `size`-long chunk of a slice, the chunks disjoint. With
/// the `parallel` feature the chunks run across every core; without it they run in
/// order. No two chunks share an element, so the result is the same either way,
/// which is what lets the transform's butterflies, which touch one block at a
/// time, go wide without changing a bit of the proof.
#[cfg(feature = "parallel")]
pub fn for_each_chunk_mut<T, F>(items: &mut [T], size: usize, f: F)
where
    T: Send,
    F: Fn(&mut [T]) + Send + Sync,
{
    use rayon::prelude::*;
    items.par_chunks_mut(size).for_each(f);
}

#[cfg(not(feature = "parallel"))]
pub fn for_each_chunk_mut<T, F>(items: &mut [T], size: usize, f: F)
where
    F: Fn(&mut [T]),
{
    items.chunks_mut(size).for_each(f);
}

/// `for_each_chunk_mut` with each chunk told where it starts, so a task that
/// mutates its chunk can also read a matching index from something it does not
/// own. The chunks are disjoint and the base index is a pure function of the
/// chunk's position, so the result is the same in either form.
#[cfg(feature = "parallel")]
pub fn for_each_chunk_mut_indexed<T, F>(items: &mut [T], size: usize, f: F)
where
    T: Send,
    F: Fn(usize, &mut [T]) + Send + Sync,
{
    use rayon::prelude::*;
    items
        .par_chunks_mut(size)
        .enumerate()
        .for_each(|(k, chunk)| f(k * size, chunk));
}

#[cfg(not(feature = "parallel"))]
pub fn for_each_chunk_mut_indexed<T, F>(items: &mut [T], size: usize, f: F)
where
    F: Fn(usize, &mut [T]),
{
    items
        .chunks_mut(size)
        .enumerate()
        .for_each(|(k, chunk)| f(k * size, chunk));
}
