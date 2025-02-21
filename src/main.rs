//! My base for executable programs.
//! TODO: Fix crate name in Cargo.toml

#![warn(
    clippy::create_dir,
    clippy::infinite_loop,
    clippy::let_underscore_must_use,
    clippy::missing_panics_doc,
    clippy::return_self_not_must_use,
    clippy::same_name_method,
    clippy::should_panic_without_expect,
    clippy::use_debug,
    missing_copy_implementations,
    rust_2018_idioms
)]
#![warn(clippy::missing_docs_in_private_items, missing_docs)]
#![deny(unsafe_op_in_unsafe_fn, missing_debug_implementations)]
#![cfg_attr(
    not(debug_assertions),
    deny(clippy::dbg_macro, clippy::todo, clippy::unimplemented)
)]
// The `lint_reasons` feature was stabilized in 1.81.0. These lints therefore can't be enabled
// except on nightly for earlier versions of `rustc`.
// #![deny(clippy::allow_attributes, clippy::allow_attributes_without_reason)]

fn main() {
    println!("Hello, world!");
    todo!("Fix crate name in Cargo.toml");
}
