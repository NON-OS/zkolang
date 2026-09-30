/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Packages (section 4.1): manifests, and the crates of a package and its dependencies. */

mod crate_src;
mod graph;
mod graph_dep;
mod graph_entry;
mod load_package;
mod manifest;
mod manifest_check;
mod manifest_deps;
mod manifest_keys;
mod manifest_names;
mod manifest_required;
mod paths;
mod toml;
mod toml_cursor;
mod toml_key;
mod toml_line;
mod toml_skip;
mod toml_string;
mod toml_value;

pub use crate_src::CrateSrc;
pub use load_package::load_package;
pub use manifest::{manifest, Dep, Manifest};
pub use paths::normalize;
