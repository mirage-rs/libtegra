//! Driver for the Tegra X1 Security Engine.
//!
//! # Description
//!
//! The Security Engine is responsible for performing, cryptographic operations
//! in a secure, hardware-based environment. Communication is done over [`LinkedList`]s,
//! a data structure defined by the SE interface, which provides I/O buffers to
//! operate on.
//!
//! ## Hardware Operations
//!
//! As mentioned previously, the Security Engine performs hardware-based operations
//! to process DMA buffers of data. These operations can be performed through the
//! [`trigger_operation`] function.
//!
//! As this is a low-level interface that should only be used if no other possibility
//! exists, there are higher-level wrappers around commonly used operations available:
//!
//! ```no_run
//! // TODO
//! ```
//!
//! [`LinkedList`]: struct.LinkedList.html
//! [`trigger_operation`]: fn.trigger_operation.html

pub use self::core::*;
pub use registers::*;

#[allow(dead_code)]
mod constants;
mod core;
mod registers;
