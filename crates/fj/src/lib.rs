//! # Fornjot
//!
//! [Fornjot] is an early-stage b-rep CAD kernel written in Rust. The kernel is
//! split into multiple libraries that can be used semi-independently, and this
//! is one of those.
//!
//! This crate serves as a convenient entryway to Fornjot, re-exporting all
//! crates that make up Fornjot.
//!
//! [Fornjot]: https://www.fornjot.app/

mod args;
mod handle_model;

pub use self::{
    args::Args,
    handle_model::{handle_model, Error, Result},
};

pub use fj_core as core;
pub use fj_export as export;
pub use fj_interop as interop;
pub use fj_math as math;
pub use fj_viewer as viewer;
pub use fj_window as window;
