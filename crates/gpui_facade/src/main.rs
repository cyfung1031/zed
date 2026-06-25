#![doc = "Facade crate that re-exports GPUI plus platform/web helpers."]

pub use gpui::*;
pub use gpui_macros::*;
pub use gpui_web::*;

pub mod gpui_platform {
    pub use ::gpui_platform::*;
}

pub mod gpui_macros {
    pub use ::gpui_macros::*;
}

pub mod gpui_web {
    pub use ::gpui_web::*;
}
