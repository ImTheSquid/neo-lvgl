//! Prelude module for convenient imports
//!
//! This module re-exports commonly used types and traits.
//!
//! # Example
//!
//! ```ignore
//! use lvgl::prelude::*;
//!
//! lvgl::init();
//! let display = Display::new(320, 240).unwrap();
//! let screen = display.active_screen();
//! let btn = Button::new(&screen).unwrap();
//! ```

// Core types
pub use crate::color::Color;
pub use crate::display::{Area, ColorFormat, Display, DisplayDriver, RenderMode};
#[cfg(feature = "alloc")]
pub use crate::display::ManagedDisplay;
pub use crate::style::{Style, StyleSelector};

// Widgets
pub use crate::widgets::{Obj, Widget};
pub use crate::widgets::{Button, Label};

// Events
pub use crate::event::{Event, EventCode};
#[cfg(feature = "alloc")]
pub use crate::event::ClosureEventHandler;

// Extension traits
pub use crate::layout::LayoutExt;
pub use crate::scroll::ScrollExt;
pub use crate::xml::NameExt;

// Layout
pub use crate::layout::{Direction, FlexAlign, FlexFlow, GridAlign};

// Scroll
pub use crate::scroll::{ScrollSnap, ScrollbarMode};

// Animation
pub use crate::anim::{Anim, AnimHandle, AnimPath, RepeatCount};

// Observer/binding
pub use crate::observer::{IntSubject, Observer, Subject};

// Input
pub use crate::indev::{Indev, IndevState, IndevType, Key};

// Groups
pub use crate::group::Group;

// Timer
pub use crate::timer::Timer;

// Font
pub use crate::font::Font;

// Thread safety
pub use crate::sync::{lvgl_lock, Locked, LockedGuard, LvglLockGuard};
