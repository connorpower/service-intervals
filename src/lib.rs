//! A library for tracking component service intervals.

#![warn(
    // Built-in lints
    missing_docs,
    rust_2018_idioms,
    // Clippy lint groups
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    // Specific clippy lints
    clippy::clone_on_ref_ptr,
    clippy::undocumented_unsafe_blocks,
    clippy::unwrap_used,
)]
#![cfg_attr(
    doc,
    warn(
        rustdoc::bare_urls,
        rustdoc::broken_intra_doc_links,
        rustdoc::invalid_codeblock_attributes,
        rustdoc::invalid_rust_codeblocks,
        rustdoc::missing_crate_level_docs,
    )
)]

pub mod db;
pub mod errors;
pub mod garmin;
