//! Implementation of general [`Report`] serialization.
//!
//! The value can be of any type, currently only printable attachments and context are supported, in
//! the near future any values will be supported through the use of hooks.
//!
//! The serialized [`Report`] is a list of all current sources with the following output:
//!
//! ```json
//! {
//!     "context": "context display output",
//!     "attachments": ["all", "attachments", "leading", "up", "to", "this", "context"],
//!     "sources": [] // recursive render using `frame.sources()`
//! }
//! ```

#[cfg(any(feature = "std", feature = "hooks"))]
mod hook;

pub use hook::{HookContext, Serde, SerdeHooks, SerializeFn};

#[cfg(any(feature = "std", feature = "hooks"))]
pub(crate) use hook::install_builtin_serde_hooks;
