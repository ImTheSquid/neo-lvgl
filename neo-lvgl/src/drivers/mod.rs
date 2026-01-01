//! Display and input driver implementations

#[cfg(feature = "embedded-graphics")]
pub mod embedded_graphics;

#[cfg(feature = "embedded-graphics")]
pub use embedded_graphics::{EmbeddedGraphicsDisplay, FromLvglRgb565, IntoLvglDisplay};
