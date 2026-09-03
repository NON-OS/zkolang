// NONOS Operating System (AGPL-3.0-or-later)
//! The witness-mode recursion assembly: nine regions and their grand-product
//! bindings, one region or binding family per file. `build` assembles the
//! wired AIR and witness from the real inner join-split proof; `tamper` names
//! the targeted forgeries the reject gate must catch.

pub mod auth;
pub mod build;
pub mod compose;
pub mod compose_step;
pub mod deep;
pub mod fri;
pub mod groups;
pub mod inner;
pub mod layout;
pub mod periodic;
pub mod points;
pub mod sponge;
pub mod tamper;
pub mod transcript;

pub use build::{
    assemble, assemble_capped, assemble_q, assemble_real, assemble_real_capped, build_groups_for, assemble_step,
};
pub use tamper::Tamper;
