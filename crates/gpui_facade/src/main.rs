//! Combined GPUI facade crate.
//!
//! This crate re-exports the public `gpui` API and additionally exposes
//! `gpui_platform` under the same crate namespace, so downstream code can use:
//!
//! ```ignore
//! use gpui_combined as gpui;
//!
//! let app = gpui::gpui_platform::application();
//! ```
//!
//! It intentionally does not modify the original `gpui` crate, avoiding the
//! dependency cycle `gpui -> gpui_platform -> gpui`.

pub use gpui::*;

/// Re-export of the `gpui_platform` convenience crate.
pub mod gpui_platform {
    pub use ::gpui_platform::*;
}
