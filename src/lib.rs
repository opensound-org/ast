#![cfg_attr(nightly, feature(doc_auto_cfg))]

//! **E**xtensions for the rust **S**tandard library and **T**okio.

/// Extensions to the [`std::collections`](https://doc.rust-lang.org/stable/std/collections/index.html) module.
pub mod collections;
/// Extensions to the [`std::result`](https://doc.rust-lang.org/stable/std/result/index.html) module.
pub mod result;

pub use result::AnyRes;
