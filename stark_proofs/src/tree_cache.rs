// NONOS Operating System (AGPL-3.0-or-later)
//! The periodic tree, kept on disk between proofs.
//!
//! The root cache keeps the one value a verifier bakes. The prover needs more
//! than the root: it opens a path in the periodic tree at every query, so
//! without the tree it rebuilds it, and on the settlement outer that rebuild
//! is half of the proof's running time spent on a constant of the circuit.
//! This keeps every level of the tree, leaves first, in a file named by the
//! root, beside the emissions in `.treecache`, the way `.rootcache` sits
//! beside them.
//!
//! The file is named by the root so a caller that already knows which root a
//! proof must verify against can ask for exactly that tree and nothing else.
//! Whether the loaded tree really has that root is checked by the caller when
//! it takes the tree's own root, and whether its levels are intact is checked
//! by the verifier, on the proof: a path opened from a corrupted level fails
//! there. Nothing here recomputes a hash.
//!
//! Layout: a magic, the level count, then each level as its length and its
//! digests in order. On the settlement outer that is about four gigabytes and
//! on the transfer-64 outer about nine; read and written through buffers, in
//! level-sized pieces, never held twice.

use crate::crypto::stark::merkle::MerkleTree;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

const MAGIC: &[u8; 8] = b"NOXTREE1";

/// The cache directory beside an emission: `.treecache` next to `out`.
pub fn dir_beside(out: &str) -> PathBuf {
    Path::new(out)
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_default()
        .join(".treecache")
}

/// Where the tree with this root lives, or would.
pub fn entry(dir: &Path, root_hex: &str) -> PathBuf {
    dir.join(format!("{root_hex}.tree"))
}

/// Load a tree written by `store`. `None` if the file is absent, is not one
/// of ours, or does not describe a tree of a valid shape.
pub fn load(path: &Path) -> Option<MerkleTree> {
    let mut r = BufReader::with_capacity(1 << 20, File::open(path).ok()?);
    let mut magic = [0u8; 8];
    r.read_exact(&mut magic).ok()?;
    if &magic != MAGIC {
        return None;
    }
    /*
     * Counts come off the file and the file is not trusted: a truncated or
     * foreign file with a matching magic must be refused, not allocated for.
     * A tree has one level per bit of its leaf count plus one, so more than
     * 65 levels is not a tree, and each level is half the one below, so the
     * leaf level bounds every other and the first level is bounded by what a
     * machine could ever have committed.
     */
    let n_layers = read_u64(&mut r)?;
    if n_layers == 0 || n_layers > 65 {
        return None;
    }
    let mut layers = Vec::with_capacity(n_layers as usize);
    let mut bound = 1u64 << 40;
    for _ in 0..n_layers {
        let len = read_u64(&mut r)?;
        if len == 0 || len > bound {
            return None;
        }
        let mut level = vec![[0u8; 32]; len as usize];
        for digest in level.iter_mut() {
            r.read_exact(digest).ok()?;
        }
        layers.push(level);
        bound = len;
    }
    MerkleTree::from_layers(layers)
}

/// Write a tree so `load` returns it. The file is written whole under a
/// temporary name and renamed into place, so a run killed mid-write leaves
/// no file that looks like a tree.
pub fn store(path: &Path, tree: &MerkleTree) -> io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let tmp = path.with_extension("tree.partial");
    {
        let mut w = BufWriter::with_capacity(1 << 20, File::create(&tmp)?);
        w.write_all(MAGIC)?;
        write_u64(&mut w, tree.layers().len() as u64)?;
        for level in tree.layers() {
            write_u64(&mut w, level.len() as u64)?;
            for digest in level {
                w.write_all(digest)?;
            }
        }
        w.flush()?;
    }
    std::fs::rename(&tmp, path)
}

fn read_u64<R: Read>(r: &mut R) -> Option<u64> {
    let mut b = [0u8; 8];
    r.read_exact(&mut b).ok()?;
    Some(u64::from_le_bytes(b))
}

fn write_u64<W: Write>(w: &mut W, v: u64) -> io::Result<()> {
    w.write_all(&v.to_le_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::stark::field::Fp;

    #[test]
    fn a_stored_tree_loads_with_its_root_and_paths() {
        let leaves: Vec<Fp> = (0..1000u64).map(Fp::from_u64).collect();
        let tree = MerkleTree::commit(&leaves);
        let dir = std::env::temp_dir().join(format!("nox-treecache-{}", std::process::id()));
        let path = entry(&dir, "abc");
        store(&path, &tree).expect("store");
        let back = load(&path).expect("load");
        assert_eq!(back.root(), tree.root());
        assert_eq!(back.len(), tree.len());
        for i in [0usize, 1, 511, 999, 1023] {
            assert_eq!(
                back.open(i),
                tree.open(i),
                "path {i} moved through the file"
            );
        }
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_file_that_is_not_a_tree_is_refused() {
        let dir = std::env::temp_dir().join(format!("nox-treecache-bad-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("dir");
        let path = entry(&dir, "bad");
        std::fs::write(&path, b"NOXTREE1 but not really").expect("write");
        assert!(load(&path).is_none());
        assert!(load(&entry(&dir, "absent")).is_none());
        let _ = std::fs::remove_dir_all(dir);
    }
}
