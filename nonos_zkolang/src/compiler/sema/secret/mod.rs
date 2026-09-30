/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Secret flow (section 13): which values a secret reaches, and that none reaches a public
 * position unless `declassify` releases it.
 */

mod flow;
mod flow_bind;
mod flow_block;
mod flow_call;
mod flow_chain;
mod flow_exit;
mod flow_expr;
mod flow_if;
mod flow_iter;
mod flow_labels;
mod flow_loop;
mod flow_match;
mod flow_new;
mod flow_params;
mod flow_put;
mod flow_record;
mod flow_report;
mod flow_tested;
mod flow_values;
mod flow_variant;
mod flow_write;
mod order;
mod parts;
mod program;
mod shape;
mod shape_join;
mod shape_path;
mod slots;
mod summarize;
mod summary;
mod taint;

pub use program::check_program;
pub use shape::Shape;
pub use summary::Summary;
pub use taint::Taint;
