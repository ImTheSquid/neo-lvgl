//! Safe, idiomatic Rust bindings for LVGL 9.x
//!
//! This crate provides a safe, ergonomic API for building graphical user interfaces
//! with LVGL on embedded systems.
//!
//! # Features
//!
//! - `alloc` - Enable closure-based event handlers (requires allocator)
//! - `embedded-graphics` - Integration with the embedded-graphics ecosystem
//! - `widgets-core` - Core widgets (Button, Label, etc.) - enabled by default
//! - `widgets-extra` - Additional widgets (Chart, Calendar, etc.)
//!
//! # Example
//!
//! ```ignore
//! use lvgl::prelude::*;
//!
//! // Initialize LVGL
//! lvgl::init();
//!
//! // Create display
//! let display = Display::new(320, 240).unwrap();
//! display.set_default();
//!
//! // Get screen and create widgets
//! let screen = display.active_screen();
//! let btn = Button::new(&screen).unwrap();
//! let label = Label::new(&btn).unwrap();
//! label.set_text(c"Hello!");
//! ```

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod anim;
pub mod color;
pub mod display;
pub mod event;
pub mod font;
pub mod group;
pub mod indev;
pub mod layout;
pub mod observer;
pub mod prelude;
pub mod scroll;
pub mod style;
pub mod sync;
pub mod timer;
pub mod widgets;
pub mod xml;

/// Initialize LVGL.
///
/// This must be called before any other LVGL functions.
/// It is safe to call multiple times.
pub fn init() {
    unsafe {
        neo_lvgl_sys::lv_init();
    }
}

/// Check if LVGL is initialized.
pub fn is_initialized() -> bool {
    unsafe { neo_lvgl_sys::lv_is_initialized() }
}

/// Increment LVGL tick counter.
///
/// Call this periodically from your timer interrupt or main loop.
/// LVGL uses this for animations and timing.
///
/// # Arguments
///
/// * `ms` - Milliseconds elapsed since last call
pub fn tick_inc(ms: u32) {
    unsafe {
        neo_lvgl_sys::lv_tick_inc(ms);
    }
}

/// Run LVGL task handler.
///
/// Call this periodically in your main loop to handle:
/// - Display refresh
/// - Input device reading
/// - Animation updates
/// - Timer callbacks
///
/// Returns the time in milliseconds until the next scheduled task.
pub fn task_handler() -> u32 {
    unsafe { neo_lvgl_sys::lv_timer_handler() }
}

/// Get LVGL version as a string.
pub fn version_info() -> &'static str {
    // LVGL 9.x version
    "9.x"
}
