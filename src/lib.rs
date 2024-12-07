#![cfg_attr(nightly, feature(doc_auto_cfg))]

//! **E**xtensions for the rust **S**tandard library and **T**okio.

/// Extensions to the [`std::collections`](https://doc.rust-lang.org/stable/std/collections/index.html) module.
pub mod collections;
/// Extensions to the [`std::result`](https://doc.rust-lang.org/stable/std/result/index.html) module.
pub mod result;
/// Extensions to the [`std::task`](https://doc.rust-lang.org/stable/std/task/index.html) &
/// [`tokio::task`](https://docs.rs/tokio/latest/tokio/task/index.html) module.
pub mod task;
/// Extensions to the [`std::thread`](https://doc.rust-lang.org/stable/std/thread/index.html) module.
pub mod thread;

pub use result::AnyRes;
