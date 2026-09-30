/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Host-runnable proofs for the zkolang VM. A real VM run is laid out for the
//! ALU step AIR and driven through the money-grade Poseidon-committed STARK, so
//! the honest trace is proven to accept and each tampered trace to be rejected.

pub mod mimc;

#[cfg(test)]
mod advice_fill_tests;
#[cfg(test)]
mod array_param_tests;
#[cfg(test)]
mod array_return_tests;
#[cfg(test)]
mod array_scope_tests;
#[cfg(test)]
mod array_tests;
#[cfg(test)]
mod assert_top_tests;
#[cfg(test)]
mod backend_tests;
#[cfg(test)]
mod block_propagation_tests;
#[cfg(test)]
mod block_tests;
#[cfg(test)]
mod commit_tests;
#[cfg(test)]
mod comparison_tests;
#[cfg(test)]
mod const_index_tests;
#[cfg(test)]
mod const_table_tests;
#[cfg(test)]
mod corpus_extra_tests;
#[cfg(test)]
mod corpus_tests;
#[cfg(test)]
mod cse_scale_tests;
#[cfg(test)]
mod curve_tests;
#[cfg(test)]
mod desugar_budget_tests;
#[cfg(test)]
mod diagnostic_tests;
#[cfg(test)]
mod double_free_tests;
#[cfg(test)]
mod embedded_stdlib_tests;
#[cfg(test)]
mod equivalence_tests;
#[cfg(test)]
mod feature_tests;
#[cfg(test)]
mod field_input_tests;
#[cfg(test)]
mod field_tests;
#[cfg(test)]
mod fn_tests;
#[cfg(test)]
mod fold_constraint_tests;
#[cfg(test)]
mod fold_meaning_tests;
#[cfg(test)]
mod front_ast_tests;
#[cfg(test)]
mod front_check;
#[cfg(test)]
mod front_deep_tests;
#[cfg(test)]
mod front_doc_tests;
#[cfg(test)]
mod front_fuzz_tests;
#[cfg(test)]
mod front_lex_foreign_tests;
#[cfg(test)]
mod front_lex_recovery_tests;
#[cfg(test)]
mod front_lex_tests;
#[cfg(test)]
mod front_nesting_tests;
#[cfg(test)]
mod front_place_tests;
#[cfg(test)]
mod front_recovery_tests;
#[cfg(test)]
mod front_render_check;
#[cfg(test)]
mod front_render_multi_tests;
#[cfg(test)]
mod front_render_tests;
#[cfg(test)]
mod front_render_text_tests;
#[cfg(test)]
mod front_render_width_tests;
#[cfg(test)]
mod front_render_window_tests;
#[cfg(test)]
mod front_source_tests;
#[cfg(test)]
mod fuzz_tests;
#[cfg(test)]
mod golden_corpus;
#[cfg(test)]
mod hash_tests;
#[cfg(test)]
mod include_key_tests;
#[cfg(test)]
mod include_tests;
#[cfg(test)]
mod input_count_tests;
#[cfg(test)]
mod io_tests;
#[cfg(test)]
mod kernel_tests;
#[cfg(test)]
mod lang_tests;
#[cfg(test)]
mod legacy_golden_tests;
#[cfg(test)]
mod lexical_scope_tests;
#[cfg(test)]
mod library_tests;
#[cfg(test)]
mod line_ending_tests;
#[cfg(test)]
mod liveness_scale_tests;
#[cfg(test)]
mod logic_tests;
#[cfg(test)]
mod loop_io_tests;
#[cfg(test)]
mod loop_tests;
#[cfg(test)]
mod match_default_tests;
#[cfg(test)]
mod name_check_tests;
#[cfg(test)]
mod native_compare_tests;
#[cfg(test)]
mod native_guard_tests;
#[cfg(test)]
mod nesting_tests;
#[cfg(test)]
mod note_commit_gen;
#[cfg(test)]
mod note_commit_tests;
#[cfg(test)]
mod nox_tests;
#[cfg(test)]
mod operator_tests;
#[cfg(test)]
mod optimize_tests;
#[cfg(test)]
mod optimizer_gate_tests;
#[cfg(test)]
mod owned_element_tests;
#[cfg(test)]
mod python_guard_tests;
#[cfg(test)]
mod recipes_tests;
#[cfg(test)]
mod recursion_tests;
#[cfg(test)]
mod register_file_tests;
#[cfg(test)]
mod render_tests;
#[cfg(test)]
mod robustness_tests;
#[cfg(test)]
mod sema_check;
#[cfg(test)]
mod sema_defs_check;
#[cfg(test)]
mod sema_defs_tests;
#[cfg(test)]
mod sema_run_tests;
#[cfg(test)]
mod semantics_tests;
#[cfg(test)]
mod shield_key_kat;
#[cfg(test)]
mod shield_membership_tests;
#[cfg(test)]
mod shield_tests;
#[cfg(test)]
mod small_stack;
#[cfg(test)]
mod stdlib_tests;
#[cfg(test)]
mod step_tests;
#[cfg(test)]
mod tuple_rebind_tests;
#[cfg(test)]
mod tuple_tests;
#[cfg(test)]
mod ui_expect;
#[cfg(test)]
mod ui_run;
#[cfg(test)]
mod ui_tests;
#[cfg(test)]
mod vkey_tests;
#[cfg(test)]
mod vm_tests;
#[cfg(test)]
mod wildcard_env_tests;
#[cfg(test)]
mod wildcard_tests;
#[cfg(test)]
mod witness_tests;
#[cfg(test)]
mod work_budget_tests;
